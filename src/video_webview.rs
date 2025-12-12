use gpui::*;
use gpui_component::webview::WebView;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};
use std::thread::{self, JoinHandle};
use tiny_http::{Response, Server};
use wry::WebViewBuilder;

// Global port counter for unique server ports
static PORT_COUNTER: AtomicU16 = AtomicU16::new(19950);

/// WebView-based video player with local HTTP server
pub struct VideoWebView {
    pub webview_entity: Entity<WebView>,
    pub video_path: PathBuf,
    #[allow(dead_code)]
    port: u16,
    shutdown_flag: Arc<AtomicBool>,
    _server_thread: Option<JoinHandle<()>>,
}

impl VideoWebView {
    /// Create a new Video WebView with a local HTTP server
    pub fn new(video_path: PathBuf, window: &mut Window, cx: &mut App) -> Result<Self, String> {
        // Get a unique port for this instance
        let port = PORT_COUNTER.fetch_add(1, Ordering::SeqCst);
        let video_path_clone = video_path.clone();
        let shutdown_flag = Arc::new(AtomicBool::new(false));
        let shutdown_flag_clone = shutdown_flag.clone();

        // Start a local HTTP server in a background thread
        let server_thread = thread::spawn(move || {
            let addr = format!("127.0.0.1:{}", port);
            let server = match Server::http(&addr) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to start video server: {}", e);
                    return;
                }
            };

            // HTML with video player
            let html = r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        html, body {
            width: 100%;
            height: 100%;
            background: #000;
            display: flex;
            align-items: center;
            justify-content: center;
            overflow: hidden;
        }
        video {
            width: 100%;
            height: 100%;
            object-fit: contain;
        }
    </style>
</head>
<body>
    <video controls>
        <source src="/video" type="video/mp4">
        Your browser does not support the video element.
    </video>
</body>
</html>"#
                .to_string();

            loop {
                if shutdown_flag_clone.load(Ordering::Relaxed) {
                    break;
                }

                match server.recv_timeout(std::time::Duration::from_millis(100)) {
                    Ok(Some(request)) => {
                        let url = request.url();
                        if url == "/video" {
                            // Serve the video file
                            match std::fs::read(&video_path_clone) {
                                Ok(data) => {
                                    let mime = if video_path_clone
                                        .extension()
                                        .map(|e| e == "mp4")
                                        .unwrap_or(false)
                                    {
                                        "video/mp4"
                                    } else if video_path_clone
                                        .extension()
                                        .map(|e| e == "webm")
                                        .unwrap_or(false)
                                    {
                                        "video/webm"
                                    } else if video_path_clone
                                        .extension()
                                        .map(|e| e == "mov")
                                        .unwrap_or(false)
                                    {
                                        "video/quicktime"
                                    } else if video_path_clone
                                        .extension()
                                        .map(|e| e == "avi")
                                        .unwrap_or(false)
                                    {
                                        "video/x-msvideo"
                                    } else if video_path_clone
                                        .extension()
                                        .map(|e| e == "mkv")
                                        .unwrap_or(false)
                                    {
                                        "video/x-matroska"
                                    } else {
                                        "video/mp4"
                                    };
                                    let response = Response::from_data(data).with_header(
                                        tiny_http::Header::from_bytes(
                                            &b"Content-Type"[..],
                                            mime.as_bytes(),
                                        )
                                        .unwrap(),
                                    );
                                    let _ = request.respond(response);
                                }
                                Err(_) => {
                                    let _ = request.respond(Response::empty(404));
                                }
                            }
                        } else {
                            // Serve HTML
                            let response = Response::from_string(&html).with_header(
                                tiny_http::Header::from_bytes(
                                    &b"Content-Type"[..],
                                    &b"text/html"[..],
                                )
                                .unwrap(),
                            );
                            let _ = request.respond(response);
                        }
                    }
                    Ok(None) => {}
                    Err(_) => break,
                }
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
            video_path,
            port,
            shutdown_flag,
            _server_thread: Some(server_thread),
        })
    }
}

impl Drop for VideoWebView {
    fn drop(&mut self) {
        self.shutdown_flag.store(true, Ordering::Relaxed);
    }
}
