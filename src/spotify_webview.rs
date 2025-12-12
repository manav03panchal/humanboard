use gpui::*;
use gpui_component::webview::WebView;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};
use std::thread::{self, JoinHandle};
use tiny_http::{Response, Server};
use wry::WebViewBuilder;

// Global port counter for unique server ports
static PORT_COUNTER: AtomicU16 = AtomicU16::new(19850);

/// Spotify content type for embeds
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SpotifyContentType {
    Track,
    Album,
    Playlist,
    Artist,
    Episode,
    Show,
}

impl SpotifyContentType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SpotifyContentType::Track => "track",
            SpotifyContentType::Album => "album",
            SpotifyContentType::Playlist => "playlist",
            SpotifyContentType::Artist => "artist",
            SpotifyContentType::Episode => "episode",
            SpotifyContentType::Show => "show",
        }
    }
}

/// WebView-based Spotify player with local HTTP server
pub struct SpotifyWebView {
    webview_entity: Entity<WebView>,
    content_type: SpotifyContentType,
    content_id: String,
    #[allow(dead_code)]
    port: u16,
    shutdown_flag: Arc<AtomicBool>,
    _server_thread: Option<JoinHandle<()>>,
}

impl SpotifyWebView {
    /// Create a new Spotify WebView with a local HTTP server
    pub fn new(
        content_type: SpotifyContentType,
        content_id: String,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<Self, String> {
        let port = PORT_COUNTER.fetch_add(1, Ordering::SeqCst);
        let content_type_str = content_type.as_str().to_string();
        let content_id_clone = content_id.clone();
        let shutdown_flag = Arc::new(AtomicBool::new(false));
        let shutdown_flag_clone = shutdown_flag.clone();

        let server_thread = thread::spawn(move || {
            let addr = format!("127.0.0.1:{}", port);
            let server = match Server::http(&addr) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to start Spotify embed server: {}", e);
                    return;
                }
            };

            // HTML with Spotify embed iframe - use compact style
            let html = format!(
                r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        html, body {{ width: 100%; height: 100%; overflow: hidden; background: #000; }}
        iframe {{ width: 100%; height: 176px; border: none; }}
    </style>
</head>
<body>
    <iframe
        src="https://open.spotify.com/embed/{content_type}/{content_id}?utm_source=generator&theme=0"
        title="Spotify Embed"
        frameborder="0"
        allow="autoplay; clipboard-write; encrypted-media; fullscreen; picture-in-picture"
        loading="lazy">
    </iframe>
</body>
</html>"#,
                content_type = content_type_str,
                content_id = content_id_clone
            );

            loop {
                if shutdown_flag_clone.load(Ordering::Relaxed) {
                    break;
                }

                match server.recv_timeout(std::time::Duration::from_millis(100)) {
                    Ok(Some(request)) => {
                        let response = Response::from_string(&html).with_header(
                            tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..])
                                .unwrap(),
                        );
                        let _ = request.respond(response);
                    }
                    Ok(None) => {}
                    Err(_) => break,
                }
            }
        });

        std::thread::sleep(std::time::Duration::from_millis(50));

        let url = format!("http://127.0.0.1:{}/", port);

        let webview_entity = cx.new(|cx| {
            #[cfg(any(
                target_os = "macos",
                target_os = "windows",
                target_os = "ios",
                target_os = "android"
            ))]
            let webview = WebViewBuilder::new()
                .with_url(&url)
                .build_as_child(window)
                .map_err(|e| format!("Failed to create WebView: {:?}", e));

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
            content_type,
            content_id,
            port,
            shutdown_flag,
            _server_thread: Some(server_thread),
        })
    }

    /// Get the WebView entity for rendering
    pub fn webview(&self) -> Entity<WebView> {
        self.webview_entity.clone()
    }

    /// Get the content type
    pub fn content_type(&self) -> &SpotifyContentType {
        &self.content_type
    }

    /// Get the content ID
    pub fn content_id(&self) -> &str {
        &self.content_id
    }
}

impl Drop for SpotifyWebView {
    fn drop(&mut self) {
        self.shutdown_flag.store(true, Ordering::Relaxed);
    }
}

/// Parse a Spotify URL and extract content type and ID
/// Supports formats like:
/// - https://open.spotify.com/track/4iV5W9uYEdYUVa79Axb7Rh
/// - https://open.spotify.com/album/1DFixLWuPkv3KT3TnV35m3
/// - https://open.spotify.com/playlist/37i9dQZF1DXcBWIGoYBM5M
/// - https://open.spotify.com/artist/0OdUWJ0sBjDrqHygGUXeCF
/// - https://open.spotify.com/episode/512ojhOuo1ktJprKbVcKyQ
/// - https://open.spotify.com/show/2MAi0BvDc6GTFvKFPXnkCL
pub fn parse_spotify_url(url: &str) -> Option<(SpotifyContentType, String)> {
    // Handle both open.spotify.com and spotify.link URLs
    let url = url.trim();

    // Check for open.spotify.com format
    if url.contains("open.spotify.com") {
        let parts: Vec<&str> = url.split('/').collect();

        // Find content type and ID
        for i in 0..parts.len() - 1 {
            let content_type = match parts[i] {
                "track" => Some(SpotifyContentType::Track),
                "album" => Some(SpotifyContentType::Album),
                "playlist" => Some(SpotifyContentType::Playlist),
                "artist" => Some(SpotifyContentType::Artist),
                "episode" => Some(SpotifyContentType::Episode),
                "show" => Some(SpotifyContentType::Show),
                _ => None,
            };

            if let Some(ct) = content_type {
                // Get the ID (remove any query params)
                let id = parts[i + 1].split('?').next().unwrap_or("");
                if !id.is_empty() {
                    return Some((ct, id.to_string()));
                }
            }
        }
    }

    None
}

/// Check if a URL is a Spotify URL
pub fn is_spotify_url(url: &str) -> bool {
    url.contains("open.spotify.com") || url.contains("spotify.link")
}
