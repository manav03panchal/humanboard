use gpui::*;
use gpui_component::webview::WebView;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};
use std::thread::{self, JoinHandle};
use tiny_http::{Response, Server};
use wry::WebViewBuilder;

// Global port counter for unique server ports
static PORT_COUNTER: AtomicU16 = AtomicU16::new(19900);

/// WebView-based audio player with local HTTP server
pub struct AudioWebView {
    pub webview_entity: Entity<WebView>,
    pub audio_path: PathBuf,
    #[allow(dead_code)]
    port: u16,
    shutdown_flag: Arc<AtomicBool>,
    _server_thread: Option<JoinHandle<()>>,
}

impl AudioWebView {
    /// Create a new Audio WebView with a local HTTP server
    pub fn new(audio_path: PathBuf, window: &mut Window, cx: &mut App) -> Result<Self, String> {
        // Get a unique port for this instance
        let port = PORT_COUNTER.fetch_add(1, Ordering::SeqCst);
        let audio_path_clone = audio_path.clone();
        let shutdown_flag = Arc::new(AtomicBool::new(false));
        let shutdown_flag_clone = shutdown_flag.clone();

        let file_name = audio_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Audio")
            .to_string();

        // Start a local HTTP server in a background thread
        let server_thread = thread::spawn(move || {
            let addr = format!("127.0.0.1:{}", port);
            let server = match Server::http(&addr) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to start audio server: {}", e);
                    return;
                }
            };

            // HTML with audio player
            let html = format!(
                r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        html, body {{
            width: 100%;
            height: 100%;
            background: #1a1a2e;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            color: #eee;
            padding: 16px;
        }}
        .icon {{
            font-size: 48px;
            margin-bottom: 12px;
        }}
        .title {{
            font-size: 14px;
            margin-bottom: 16px;
            text-align: center;
            word-break: break-word;
            max-width: 100%;
            opacity: 0.9;
        }}
        audio {{
            width: 100%;
            max-width: 280px;
        }}
        audio::-webkit-media-controls-panel {{
            background: #2a2a4a;
        }}
    </style>
</head>
<body>
    <div class="icon">🎵</div>
    <div class="title">{file_name}</div>
    <audio controls>
        <source src="/audio" type="audio/mpeg">
        Your browser does not support the audio element.
    </audio>
</body>
</html>"#,
                file_name = file_name
            );

            loop {
                if shutdown_flag_clone.load(Ordering::Relaxed) {
                    break;
                }

                match server.recv_timeout(std::time::Duration::from_millis(100)) {
                    Ok(Some(request)) => {
                        let url = request.url();
                        if url == "/audio" {
                            // Serve the audio file
                            match std::fs::read(&audio_path_clone) {
                                Ok(data) => {
                                    let mime = if audio_path_clone
                                        .extension()
                                        .map(|e| e == "mp3")
                                        .unwrap_or(false)
                                    {
                                        "audio/mpeg"
                                    } else if audio_path_clone
                                        .extension()
                                        .map(|e| e == "wav")
                                        .unwrap_or(false)
                                    {
                                        "audio/wav"
                                    } else if audio_path_clone
                                        .extension()
                                        .map(|e| e == "ogg")
                                        .unwrap_or(false)
                                    {
                                        "audio/ogg"
                                    } else if audio_path_clone
                                        .extension()
                                        .map(|e| e == "m4a" || e == "aac")
                                        .unwrap_or(false)
                                    {
                                        "audio/mp4"
                                    } else {
                                        "audio/mpeg"
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
            audio_path,
            port,
            shutdown_flag,
            _server_thread: Some(server_thread),
        })
    }
}

impl Drop for AudioWebView {
    fn drop(&mut self) {
        self.shutdown_flag.store(true, Ordering::Relaxed);
    }
}
