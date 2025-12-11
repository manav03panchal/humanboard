use gpui::*;
use gpui_component::webview::WebView;
use std::sync::atomic::{AtomicU16, Ordering};
use std::thread;
use tiny_http::{Response, Server};
use wry::WebViewBuilder;

// Global port counter for unique server ports
static PORT_COUNTER: AtomicU16 = AtomicU16::new(19800);

/// WebView-based YouTube player with local HTTP server
pub struct YouTubeWebView {
    webview_entity: Entity<WebView>,
    video_id: String,
    _port: u16, // Keep track of port (server runs in background thread)
}

impl YouTubeWebView {
    /// Create a new YouTube WebView with a local HTTP server
    pub fn new(video_id: String, window: &mut Window, cx: &mut App) -> Result<Self, String> {
        // Get a unique port for this instance
        let port = PORT_COUNTER.fetch_add(1, Ordering::SeqCst);
        let video_id_clone = video_id.clone();

        // Start a local HTTP server in a background thread
        thread::spawn(move || {
            let addr = format!("127.0.0.1:{}", port);
            let server = match Server::http(&addr) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to start YouTube embed server: {}", e);
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
        src="https://www.youtube.com/embed/{video_id}?autoplay=1&rel=0&modestbranding=1&playsinline=1"
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

            // Serve requests (will serve until program exits)
            for request in server.incoming_requests() {
                let response = Response::from_string(&html).with_header(
                    tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap(),
                );
                let _ = request.respond(response);
            }
        });

        // Give the server a moment to start
        std::thread::sleep(std::time::Duration::from_millis(50));

        // URL to our local server
        let url = format!("http://127.0.0.1:{}/", port);

        // Create WebView entity pointing to local server
        let webview_entity = cx.new(|cx| {
            #[cfg(any(
                target_os = "macos",
                target_os = "windows",
                target_os = "ios",
                target_os = "android"
            ))]
            let webview = {
                WebViewBuilder::new()
                    .with_url(&url)
                    .build_as_child(window)
                    .map_err(|e| format!("Failed to create WebView: {:?}", e))
            };

            #[cfg(not(any(
                target_os = "macos",
                target_os = "windows",
                target_os = "ios",
                target_os = "android"
            )))]
            let webview = Err("WebView not supported on this platform".to_string());

            let webview = webview.expect("Failed to create webview");
            WebView::new(webview, window, cx)
        });

        Ok(Self {
            webview_entity,
            video_id,
            _port: port,
        })
    }

    /// Get the video ID
    pub fn video_id(&self) -> &str {
        &self.video_id
    }

    /// Get the WebView entity for rendering
    pub fn webview(&self) -> Entity<WebView> {
        self.webview_entity.clone()
    }
}
