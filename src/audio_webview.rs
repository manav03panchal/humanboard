use gpui::*;
use gpui_component::webview::WebView;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};
use std::thread::{self, JoinHandle};
use tiny_http::{Header, Response, Server, StatusCode};
use wry::WebViewBuilder;

// Global port counter for unique server ports
static PORT_COUNTER: AtomicU16 = AtomicU16::new(19900);

/// WebView-based audio player with local HTTP server supporting range requests
pub struct AudioWebView {
    pub webview_entity: Entity<WebView>,
    pub audio_path: PathBuf,
    #[allow(dead_code)]
    port: u16,
    shutdown_flag: Arc<AtomicBool>,
    _server_thread: Option<JoinHandle<()>>,
}

impl AudioWebView {
    pub fn new(audio_path: PathBuf, window: &mut Window, cx: &mut App) -> Result<Self, String> {
        let port = PORT_COUNTER.fetch_add(1, Ordering::SeqCst);
        let audio_path_clone = audio_path.clone();
        let file_name = audio_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Audio")
            .to_string();
        let shutdown_flag = Arc::new(AtomicBool::new(false));
        let shutdown_flag_clone = shutdown_flag.clone();

        let server_thread = thread::spawn(move || {
            let addr = format!("127.0.0.1:{}", port);
            let server = match Server::http(&addr) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to start audio server: {}", e);
                    return;
                }
            };

            let html = format!(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        html, body {{
            width: 100%;
            height: 100%;
            background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
            overflow: hidden;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        }}
        .container {{
            width: 100%;
            height: 100%;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            padding: 16px;
        }}
        .title {{
            color: #e0e0e0;
            font-size: 13px;
            font-weight: 500;
            text-align: center;
            margin-bottom: 16px;
            max-width: 100%;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
            padding: 0 8px;
        }}
        audio {{
            width: 100%;
            max-width: 280px;
            height: 40px;
            border-radius: 20px;
            outline: none;
        }}
        audio::-webkit-media-controls-panel {{
            background: rgba(255,255,255,0.1);
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="title">{}</div>
        <audio controls>
            <source src="/audio" type="audio/mpeg">
        </audio>
    </div>
</body>
</html>"#, html_escape(&file_name));

            loop {
                if shutdown_flag_clone.load(Ordering::Relaxed) {
                    break;
                }

                match server.recv_timeout(std::time::Duration::from_millis(100)) {
                    Ok(Some(request)) => {
                        let url = request.url();
                        if url.starts_with("/audio") {
                            Self::serve_audio_file(&audio_path_clone, request);
                        } else {
                            let response = Response::from_string(&html).with_header(
                                Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..])
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
            audio_path,
            port,
            shutdown_flag,
            _server_thread: Some(server_thread),
        })
    }

    fn serve_audio_file(path: &PathBuf, request: tiny_http::Request) {
        let mut file = match std::fs::File::open(path) {
            Ok(f) => f,
            Err(_) => {
                let _ = request.respond(Response::empty(StatusCode(404)));
                return;
            }
        };

        let file_size = match file.metadata() {
            Ok(m) => m.len(),
            Err(_) => {
                let _ = request.respond(Response::empty(StatusCode(500)));
                return;
            }
        };

        let mime = match path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .as_deref()
        {
            Some("mp3") => "audio/mpeg",
            Some("wav") => "audio/wav",
            Some("ogg") => "audio/ogg",
            Some("m4a") => "audio/mp4",
            Some("aac") => "audio/aac",
            Some("flac") => "audio/flac",
            _ => "audio/mpeg",
        };

        // Check for Range header
        let range_header = request
            .headers()
            .iter()
            .find(|h| h.field.as_str() == "Range" || h.field.as_str() == "range")
            .map(|h| h.value.as_str().to_string());

        if let Some(range) = range_header {
            if let Some(range_spec) = range.strip_prefix("bytes=") {
                let parts: Vec<&str> = range_spec.split('-').collect();
                if !parts.is_empty() {
                    let start: u64 = parts[0].parse().unwrap_or(0);
                    let end: u64 = if parts.len() > 1 && !parts[1].is_empty() {
                        parts[1].parse().unwrap_or(file_size - 1)
                    } else {
                        file_size - 1
                    };

                    let length = end - start + 1;

                    if file.seek(SeekFrom::Start(start)).is_err() {
                        let _ = request.respond(Response::empty(StatusCode(500)));
                        return;
                    }

                    let mut buffer = vec![0u8; length as usize];
                    if file.read_exact(&mut buffer).is_err() {
                        let _ = file.seek(SeekFrom::Start(start));
                        buffer.clear();
                        let _ = file.take(length).read_to_end(&mut buffer);
                    }

                    let content_range = format!("bytes {}-{}/{}", start, end, file_size);

                    let response = Response::from_data(buffer)
                        .with_status_code(StatusCode(206))
                        .with_header(
                            Header::from_bytes(&b"Content-Type"[..], mime.as_bytes()).unwrap(),
                        )
                        .with_header(
                            Header::from_bytes(&b"Content-Range"[..], content_range.as_bytes())
                                .unwrap(),
                        )
                        .with_header(
                            Header::from_bytes(&b"Accept-Ranges"[..], &b"bytes"[..]).unwrap(),
                        )
                        .with_header(
                            Header::from_bytes(
                                &b"Content-Length"[..],
                                length.to_string().as_bytes(),
                            )
                            .unwrap(),
                        );

                    let _ = request.respond(response);
                    return;
                }
            }
        }

        // No range request - serve entire file
        let mut buffer = Vec::new();
        if file.read_to_end(&mut buffer).is_err() {
            let _ = request.respond(Response::empty(StatusCode(500)));
            return;
        }

        let response = Response::from_data(buffer)
            .with_header(Header::from_bytes(&b"Content-Type"[..], mime.as_bytes()).unwrap())
            .with_header(Header::from_bytes(&b"Accept-Ranges"[..], &b"bytes"[..]).unwrap())
            .with_header(
                Header::from_bytes(&b"Content-Length"[..], file_size.to_string().as_bytes())
                    .unwrap(),
            );

        let _ = request.respond(response);
    }
}

impl Drop for AudioWebView {
    fn drop(&mut self) {
        self.shutdown_flag.store(true, Ordering::Relaxed);
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
