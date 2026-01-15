//! WebView-based YouTube video player.
//!
//! This module embeds YouTube videos using their official iframe API,
//! served through a local HTTP server to work around WebView security restrictions.
//!
//! ## Architecture
//!
//! Each YouTube player spawns a local HTTP server on a unique port that serves
//! an HTML page containing the YouTube iframe embed. This approach allows the
//! embedded player to function correctly within the native WebView.
//!
//! ## Features
//!
//! - YouTube iframe embed with full playback controls
//! - Autoplay disabled by default
//! - Modest branding (reduced YouTube UI)

use gpui::*;
use gpui_component::webview::WebView;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};
use std::thread::{self, JoinHandle};
use tiny_http::{Response, Server};
use tracing::error;
use wry::WebViewBuilder;

// Global port counter for unique server ports
static PORT_COUNTER: AtomicU16 = AtomicU16::new(19800);

/// Helper to create HTTP headers, returning None if the bytes are invalid
fn create_header(name: &[u8], value: &[u8]) -> Option<tiny_http::Header> {
    tiny_http::Header::from_bytes(name, value).ok()
}

/// WebView-based YouTube player with local HTTP server
pub struct YouTubeWebView {
    webview_entity: Entity<WebView>,
    video_id: String,
    port: u16,
    shutdown_flag: Arc<AtomicBool>,
    _server_thread: Option<JoinHandle<()>>,
}

impl YouTubeWebView {
    /// Create a new YouTube WebView with a local HTTP server
    pub fn new(video_id: String, window: &mut Window, cx: &mut App) -> Result<Self, String> {
        // Get a unique port for this instance
        let port = PORT_COUNTER.fetch_add(1, Ordering::SeqCst);
        let video_id_clone = video_id.clone();
        let shutdown_flag = Arc::new(AtomicBool::new(false));
        let shutdown_flag_clone = shutdown_flag.clone();

        // Start a local HTTP server in a background thread
        let server_thread = thread::spawn(move || {
            let addr = format!("127.0.0.1:{}", port);
            let server = match Server::http(&addr) {
                Ok(s) => s,
                Err(e) => {
                    error!("Failed to start YouTube embed server on port {}: {}", port, e);
                    return;
                }
            };

            // HTML with YouTube embed
            let html = format!(
                r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        html, body {{ width: 100%; height: 100%; overflow: hidden; background: #000; }}
        iframe {{ width: 100%; height: 100%; border: none; }}
    </style>
</head>
<body>
    <iframe
        src="https://www.youtube.com/embed/{video_id}?autoplay=0&rel=0&modestbranding=1&playsinline=1"
        title="YouTube video player"
        frameborder="0"
        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
        referrerpolicy="strict-origin-when-cross-origin"
        allowfullscreen>
    </iframe>
</body>
</html>"#,
                video_id = video_id_clone
            );

            // Serve requests with non-blocking check for shutdown
            // tiny_http doesn't have non-blocking recv, but we can use try_recv with timeout
            loop {
                // Check shutdown flag
                if shutdown_flag_clone.load(Ordering::Relaxed) {
                    break;
                }

                // Use recv_timeout to periodically check shutdown flag
                match server.recv_timeout(std::time::Duration::from_millis(100)) {
                    Ok(Some(request)) => {
                        let mut response = Response::from_string(&html);
                        if let Some(header) =
                            create_header(&b"Content-Type"[..], &b"text/html"[..])
                        {
                            response = response.with_header(header);
                        }
                        let _ = request.respond(response);
                    }
                    Ok(None) => {
                        // Timeout, loop continues to check shutdown
                    }
                    Err(_) => {
                        // Server error, exit
                        break;
                    }
                }
            }
        });

        // Give the server a moment to start
        std::thread::sleep(std::time::Duration::from_millis(50));

        // URL to our local server
        let url = format!("http://127.0.0.1:{}/", port);

        // Create WebView entity pointing to local server
        #[cfg(any(
            target_os = "macos",
            target_os = "windows",
            target_os = "ios",
            target_os = "android"
        ))]
        let webview = WebViewBuilder::new()
            .with_url(&url)
            .build_as_child(window)
            .map_err(|e| format!("Failed to create WebView: {:?}", e))?;

        #[cfg(not(any(
            target_os = "macos",
            target_os = "windows",
            target_os = "ios",
            target_os = "android"
        )))]
        return Err("WebView not supported on this platform".to_string());

        let webview_entity = cx.new(|cx| WebView::new(webview, window, cx));

        Ok(Self {
            webview_entity,
            video_id,
            port,
            shutdown_flag,
            _server_thread: Some(server_thread),
        })
    }

    /// Get the video ID
    pub fn video_id(&self) -> &str {
        &self.video_id
    }

    /// Get the port this server is running on
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Get the WebView entity for rendering
    pub fn webview(&self) -> Entity<WebView> {
        self.webview_entity.clone()
    }

    /// Shutdown the HTTP server
    pub fn shutdown(&self) {
        self.shutdown_flag.store(true, Ordering::Relaxed);
    }

    /// Hide the webview (should be called before dropping to prevent orphaned UI)
    pub fn hide(&self, cx: &mut App) {
        self.webview_entity.update(cx, |wv, _| wv.hide());
    }
}

impl Drop for YouTubeWebView {
    fn drop(&mut self) {
        // Signal server to shutdown
        self.shutdown_flag.store(true, Ordering::Relaxed);
        // Note: We don't join the thread here to avoid blocking
        // The thread will exit on its own within 100ms
    }
}
