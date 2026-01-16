//! WebView-based audio player with metadata display.
//!
//! This module provides an audio player implemented as a WebView with a local
//! HTTP server that supports range requests for seeking. It extracts and displays
//! metadata (title, artist, album art) from audio files.
//!
//! ## Architecture
//!
//! Each audio player spawns a local HTTP server on a unique port that serves:
//! - HTML/CSS/JS for the player UI
//! - Audio data with HTTP range request support for seeking
//!
//! ## Supported Formats
//!
//! MP3, WAV, OGG, M4A, AAC, FLAC

use base64::Engine;
use gpui::*;
use gpui_component::webview::WebView;
use lofty::{Accessor, PictureType, Probe, TaggedFileExt};
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};
use std::thread::{self, JoinHandle};
use tiny_http::{Header, Response, Server, StatusCode};
use tracing::error;
use wry::WebViewBuilder;

// Global port counter for unique server ports
static PORT_COUNTER: AtomicU16 = AtomicU16::new(19900);

/// Helper to create HTTP headers, returning None if the bytes are invalid
fn create_header(name: &[u8], value: &[u8]) -> Option<Header> {
    Header::from_bytes(name, value).ok()
}

/// WebView-based audio player with local HTTP server supporting range requests
pub struct AudioWebView {
    pub webview_entity: Entity<WebView>,
    pub audio_path: PathBuf,
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

        // Extract metadata from audio file
        let (title, artist, album_art_base64) = extract_audio_metadata(&audio_path);
        let display_title = title.unwrap_or_else(|| file_name.clone());
        let display_artist = artist.unwrap_or_else(|| "Audio File".to_string());

        let server_thread = thread::spawn(move || {
            let addr = format!("127.0.0.1:{}", port);
            let server = match Server::http(&addr) {
                Ok(s) => s,
                Err(e) => {
                    error!("Failed to start audio server on port {}: {}", port, e);
                    return;
                }
            };

            let html = format!(r##"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        html, body {{
            width: 100%;
            height: 100%;
            background: #121212;
            overflow: hidden;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            color: #fff;
        }}
        .container {{
            width: 100%;
            height: 100%;
            display: flex;
            flex-direction: column;
            justify-content: center;
            padding: 16px;
        }}
        .player {{
            display: flex;
            align-items: center;
            gap: 12px;
        }}
        .icon {{
            width: 48px;
            height: 48px;
            min-width: 48px;
            border-radius: 4px;
            background: linear-gradient(135deg, #e91e63 0%, #9c27b0 100%);
            display: flex;
            align-items: center;
            justify-content: center;
            overflow: hidden;
        }}
        .icon svg {{
            width: 24px;
            height: 24px;
            fill: #fff;
        }}
        .icon img {{
            width: 100%;
            height: 100%;
            object-fit: cover;
        }}
        .info {{
            flex: 1;
            min-width: 0;
        }}
        .title {{
            font-size: 13px;
            font-weight: 600;
            color: #fff;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }}
        .subtitle {{
            font-size: 11px;
            color: #b3b3b3;
        }}
        .controls {{
            display: flex;
            align-items: center;
            gap: 8px;
        }}
        .play-btn {{
            width: 36px;
            height: 36px;
            border-radius: 50%;
            background: #fff;
            border: none;
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: center;
            transition: transform 0.1s;
        }}
        .play-btn:hover {{ transform: scale(1.06); }}
        .play-btn:active {{ transform: scale(0.96); }}
        .play-btn svg {{ width: 16px; height: 16px; fill: #000; }}
        .play-icon {{ margin-left: 2px; }}
        .progress {{
            margin-top: 12px;
            display: flex;
            align-items: center;
            gap: 8px;
        }}
        .time {{
            font-size: 10px;
            color: #a0a0a0;
            min-width: 36px;
            font-variant-numeric: tabular-nums;
        }}
        .time.end {{ text-align: right; }}
        .bar {{
            flex: 1;
            height: 4px;
            background: #404040;
            border-radius: 2px;
            cursor: pointer;
            position: relative;
        }}
        .bar:hover .fill {{ background: #e91e63; }}
        .fill {{
            height: 100%;
            background: #fff;
            border-radius: 2px;
            width: 0%;
            transition: width 0.1s linear;
        }}
        .hidden {{ display: none; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="player">
            <div class="icon">{album_art}</div>
            <div class="info">
                <div class="title">{title}</div>
                <div class="subtitle">{artist}</div>
            </div>
            <div class="controls">
                <button class="play-btn" id="playBtn">
                    <svg class="play-icon" id="playIcon" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
                    <svg class="hidden" id="pauseIcon" viewBox="0 0 24 24"><path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z"/></svg>
                </button>
            </div>
        </div>
        <div class="progress">
            <span class="time" id="cur">0:00</span>
            <div class="bar" id="bar">
                <div class="fill" id="fill"></div>
            </div>
            <span class="time end" id="dur">0:00</span>
        </div>
    </div>
    <audio id="audio" preload="metadata">
        <source src="/audio" type="audio/mpeg">
    </audio>
    <script>
        const audio = document.getElementById('audio');
        const playBtn = document.getElementById('playBtn');
        const playIcon = document.getElementById('playIcon');
        const pauseIcon = document.getElementById('pauseIcon');
        const fill = document.getElementById('fill');
        const cur = document.getElementById('cur');
        const dur = document.getElementById('dur');
        const bar = document.getElementById('bar');

        const fmt = s => {{
            const mins = Math.floor(s / 60);
            const secs = Math.floor(s % 60);
            return mins + ':' + String(secs).padStart(2, '0');
        }};

        audio.addEventListener('loadedmetadata', () => {{
            dur.textContent = fmt(audio.duration);
        }});

        audio.addEventListener('timeupdate', () => {{
            const pct = (audio.currentTime / audio.duration) * 100;
            fill.style.width = pct + '%';
            cur.textContent = fmt(audio.currentTime);
        }});

        audio.addEventListener('ended', () => {{
            playIcon.classList.remove('hidden');
            pauseIcon.classList.add('hidden');
        }});

        playBtn.onclick = () => {{
            if (audio.paused) {{
                audio.play();
                playIcon.classList.add('hidden');
                pauseIcon.classList.remove('hidden');
            }} else {{
                audio.pause();
                playIcon.classList.remove('hidden');
                pauseIcon.classList.add('hidden');
            }}
        }};

        bar.onclick = e => {{
            const rect = bar.getBoundingClientRect();
            const pct = (e.clientX - rect.left) / rect.width;
            audio.currentTime = pct * audio.duration;
        }};
    </script>
</body>
</html>"##,
                title = html_escape(&display_title),
                artist = html_escape(&display_artist),
                album_art = if let Some(ref art_data) = album_art_base64 {
                    // Format is "mime_type|base64_data"
                    let parts: Vec<&str> = art_data.splitn(2, '|').collect();
                    // Validate MIME type and base64 data to prevent XSS (CWE-79)
                    if parts.len() == 2 && is_valid_image_mime(parts[0]) && is_valid_base64(parts[1]) {
                        format!(r#"<img src="data:{};base64,{}" alt="">"#, parts[0], parts[1])
                    } else {
                        r#"<svg viewBox="0 0 24 24"><path d="M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z"/></svg>"#.to_string()
                    }
                } else {
                    r#"<svg viewBox="0 0 24 24"><path d="M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z"/></svg>"#.to_string()
                }
            );

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
                            let mut response = Response::from_string(&html);
                            if let Some(header) =
                                create_header(&b"Content-Type"[..], &b"text/html"[..])
                            {
                                response = response.with_header(header);
                            }
                            let _ = request.respond(response);
                        }
                    }
                    Ok(None) => {}
                    Err(_) => break,
                }
            }
        });

        std::thread::sleep(std::time::Duration::from_millis(100));

        let url = format!("http://127.0.0.1:{}/", port);

        #[cfg(any(
            target_os = "macos",
            target_os = "windows",
            target_os = "ios",
            target_os = "android"
        ))]
        let webview = WebViewBuilder::new()
            .with_url(&url)
            .with_autoplay(true)
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
            audio_path,
            shutdown_flag,
            _server_thread: Some(server_thread),
        })
    }

    /// Hide the webview (should be called before dropping to prevent orphaned UI)
    pub fn hide(&self, cx: &mut App) {
        self.webview_entity.update(cx, |wv, _| wv.hide());
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

                    let mut response = Response::from_data(buffer).with_status_code(StatusCode(206));
                    if let Some(h) = create_header(&b"Content-Type"[..], mime.as_bytes()) {
                        response = response.with_header(h);
                    }
                    if let Some(h) = create_header(&b"Content-Range"[..], content_range.as_bytes())
                    {
                        response = response.with_header(h);
                    }
                    if let Some(h) = create_header(&b"Accept-Ranges"[..], &b"bytes"[..]) {
                        response = response.with_header(h);
                    }
                    if let Some(h) =
                        create_header(&b"Content-Length"[..], length.to_string().as_bytes())
                    {
                        response = response.with_header(h);
                    }

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

        let mut response = Response::from_data(buffer);
        if let Some(h) = create_header(&b"Content-Type"[..], mime.as_bytes()) {
            response = response.with_header(h);
        }
        if let Some(h) = create_header(&b"Accept-Ranges"[..], &b"bytes"[..]) {
            response = response.with_header(h);
        }
        if let Some(h) = create_header(&b"Content-Length"[..], file_size.to_string().as_bytes()) {
            response = response.with_header(h);
        }

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

/// Validate that a MIME type is a safe image type to prevent XSS (CWE-79)
fn is_valid_image_mime(mime: &str) -> bool {
    matches!(
        mime,
        "image/jpeg" | "image/png" | "image/gif" | "image/webp" | "image/bmp" | "image/svg+xml"
    )
}

/// Validate that a string contains only valid base64 characters to prevent XSS (CWE-79)
fn is_valid_base64(s: &str) -> bool {
    s.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=')
}

/// Extract title, artist, and album art from audio file metadata
fn extract_audio_metadata(path: &PathBuf) -> (Option<String>, Option<String>, Option<String>) {
    let tagged_file = match Probe::open(path).and_then(|p| p.read()) {
        Ok(f) => f,
        Err(_) => return (None, None, None),
    };

    let tag = tagged_file.primary_tag().or_else(|| tagged_file.first_tag());

    let Some(tag) = tag else {
        return (None, None, None);
    };

    let title = tag.title().map(|s| s.to_string());
    let artist = tag.artist().map(|s| s.to_string());

    // Try to get album art - prefer front cover, fall back to any picture
    let album_art = tag
        .pictures()
        .iter()
        .find(|p| p.pic_type() == PictureType::CoverFront)
        .or_else(|| tag.pictures().first())
        .map(|p| {
            let mime = p.mime_type().map(|m| m.as_str()).unwrap_or("image/jpeg");
            let data = base64::engine::general_purpose::STANDARD.encode(p.data());
            format!("{}|{}", mime, data)
        });

    (title, artist, album_art)
}
