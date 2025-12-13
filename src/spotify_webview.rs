use crate::spotify_auth;
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

        // Check if user is authenticated with Spotify
        let access_token = spotify_auth::get_valid_token();

        let server_thread = thread::spawn(move || {
            let addr = format!("127.0.0.1:{}", port);
            let server = match Server::http(&addr) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to start Spotify embed server: {}", e);
                    return;
                }
            };

            // Use Web Playback SDK if authenticated, otherwise fall back to embed
            let html = if let Some(token) = access_token {
                generate_sdk_player_html(&content_type_str, &content_id_clone, &token)
            } else {
                generate_embed_html(&content_type_str, &content_id_clone)
            };

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

/// Generate HTML for the embed iframe (unauthenticated - 30 second previews)
fn generate_embed_html(content_type: &str, content_id: &str) -> String {
    format!(
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
        src="https://open.spotify.com/embed/{content_type}/{content_id}?utm_source=generator&theme=0"
        title="Spotify Embed"
        frameborder="0"
        allow="autoplay; clipboard-write; encrypted-media; fullscreen; picture-in-picture"
        loading="lazy">
    </iframe>
</body>
</html>"#,
        content_type = content_type,
        content_id = content_id
    )
}

/// Generate HTML for Web Playback SDK player (authenticated - full playback)
fn generate_sdk_player_html(content_type: &str, content_id: &str, access_token: &str) -> String {
    let uri = format!("spotify:{}:{}", content_type, content_id);

    format!(
        r##"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        html, body {{
            width: 100%; height: 100%; overflow: hidden;
            background: #121212;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
            color: #fff;
        }}
        .container {{ width: 100%; height: 100%; display: flex; flex-direction: column; }}
        .main {{ flex: 1; display: flex; align-items: center; padding: 12px; gap: 12px; background: linear-gradient(180deg, rgba(60,60,60,0.4) 0%, #121212 100%); }}
        .album-art {{ width: 64px; height: 64px; min-width: 64px; border-radius: 4px; background: #282828; object-fit: cover; box-shadow: 0 4px 12px rgba(0,0,0,0.5); }}
        .info {{ flex: 1; min-width: 0; display: flex; flex-direction: column; gap: 2px; }}
        .track-name {{ font-size: 14px; font-weight: 700; color: #fff; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }}
        .artist-name {{ font-size: 12px; color: #b3b3b3; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }}
        .controls {{ display: flex; align-items: center; gap: 12px; }}
        .ctrl-btn {{ background: none; border: none; cursor: pointer; padding: 4px; display: flex; opacity: 0.7; transition: opacity 0.15s, transform 0.1s; }}
        .ctrl-btn:hover {{ opacity: 1; }}
        .ctrl-btn:active {{ transform: scale(0.92); }}
        .ctrl-btn svg {{ width: 20px; height: 20px; fill: #fff; }}
        .play-btn {{ width: 32px; height: 32px; border-radius: 50%; background: #fff; border: none; cursor: pointer; display: flex; align-items: center; justify-content: center; transition: transform 0.1s; }}
        .play-btn:hover {{ transform: scale(1.06); }}
        .play-btn:active {{ transform: scale(0.96); }}
        .play-btn svg {{ width: 14px; height: 14px; fill: #000; }}
        .play-icon {{ margin-left: 2px; }}
        .progress {{ display: flex; align-items: center; gap: 8px; padding: 0 12px 10px; }}
        .time {{ font-size: 11px; color: #a0a0a0; min-width: 32px; font-variant-numeric: tabular-nums; }}
        .time.end {{ text-align: right; }}
        .bar {{ flex: 1; height: 4px; background: #404040; border-radius: 2px; cursor: pointer; position: relative; }}
        .bar:hover .fill {{ background: #1db954; }}
        .fill {{ height: 100%; background: #fff; border-radius: 2px; width: 0%; transition: width 0.1s linear; }}
        .loading, .error {{ display: flex; align-items: center; justify-content: center; width: 100%; height: 100%; color: #b3b3b3; font-size: 13px; text-align: center; padding: 16px; }}
        .hidden {{ display: none; }}
    </style>
</head>
<body>
    <div class="loading" id="loading">Connecting to Spotify...</div>
    <div class="error hidden" id="error"></div>
    <div class="container hidden" id="player">
        <div class="main">
            <img class="album-art" id="art" src="" alt="">
            <div class="info">
                <div class="track-name" id="track">Loading...</div>
                <div class="artist-name" id="artist"></div>
            </div>
            <div class="controls">
                <button class="ctrl-btn" id="prev" title="Previous">
                    <svg viewBox="0 0 16 16"><path d="M3.3 1a.7.7 0 0 1 .7.7v5.15l9.95-5.744a.7.7 0 0 1 1.05.606v12.575a.7.7 0 0 1-1.05.607L4 9.149V14.3a.7.7 0 0 1-.7.7H1.7a.7.7 0 0 1-.7-.7V1.7a.7.7 0 0 1 .7-.7h1.6z"/></svg>
                </button>
                <button class="play-btn" id="play" title="Play">
                    <svg class="play-icon" id="playIcon" viewBox="0 0 16 16"><path d="M3 1.713a.7.7 0 0 1 1.05-.607l10.89 6.288a.7.7 0 0 1 0 1.212L4.05 14.894A.7.7 0 0 1 3 14.288V1.713z"/></svg>
                    <svg id="pauseIcon" class="hidden" viewBox="0 0 16 16"><path d="M2.7 1a.7.7 0 0 0-.7.7v12.6a.7.7 0 0 0 .7.7h2.6a.7.7 0 0 0 .7-.7V1.7a.7.7 0 0 0-.7-.7H2.7zm8 0a.7.7 0 0 0-.7.7v12.6a.7.7 0 0 0 .7.7h2.6a.7.7 0 0 0 .7-.7V1.7a.7.7 0 0 0-.7-.7h-2.6z"/></svg>
                </button>
                <button class="ctrl-btn" id="next" title="Next">
                    <svg viewBox="0 0 16 16"><path d="M12.7 1a.7.7 0 0 0-.7.7v5.15L2.05 1.107A.7.7 0 0 0 1 1.712v12.575a.7.7 0 0 0 1.05.607L12 9.149V14.3a.7.7 0 0 0 .7.7h1.6a.7.7 0 0 0 .7-.7V1.7a.7.7 0 0 0-.7-.7h-1.6z"/></svg>
                </button>
            </div>
        </div>
        <div class="progress">
            <span class="time" id="cur">0:00</span>
            <div class="bar" id="bar"><div class="fill" id="fill"></div></div>
            <span class="time end" id="dur">0:00</span>
        </div>
    </div>

    <script src="https://sdk.scdn.co/spotify-player.js"></script>
    <script>
        const token = '{access_token}';
        const uri = '{uri}';
        const ctype = '{content_type}';
        const cid = '{content_id}';
        let player, devId, pos = 0, len = 0, paused = true;

        const fmt = ms => {{ const s = Math.floor(ms/1000); return Math.floor(s/60) + ':' + (s%60).toString().padStart(2,'0'); }};
        const $ = id => document.getElementById(id);

        const showErr = msg => {{ $('loading').classList.add('hidden'); $('player').classList.add('hidden'); $('error').classList.remove('hidden'); $('error').textContent = msg; }};
        const showPlayer = () => {{ $('loading').classList.add('hidden'); $('error').classList.add('hidden'); $('player').classList.remove('hidden'); }};

        setInterval(() => {{
            if (!paused && len > 0) {{
                pos = Math.min(pos + 1000, len);
                $('fill').style.width = (pos/len*100) + '%';
                $('cur').textContent = fmt(pos);
            }}
        }}, 1000);

        window.onSpotifyWebPlaybackSDKReady = async () => {{
            // Fetch initial info
            try {{
                const r = await fetch(`https://api.spotify.com/v1/${{ctype}}s/${{cid}}`, {{ headers: {{ Authorization: `Bearer ${{token}}` }} }});
                if (r.ok) {{
                    const d = await r.json();
                    $('art').src = d.album?.images?.[0]?.url || d.images?.[0]?.url || '';
                    $('track').textContent = d.name || '';
                    $('artist').textContent = d.artists?.map(a=>a.name).join(', ') || d.owner?.display_name || '';
                }}
            }} catch(e) {{}}

            player = new Spotify.Player({{ name: 'Humanboard', getOAuthToken: cb => cb(token), volume: 0.5 }});

            player.addListener('ready', ({{ device_id }}) => {{
                devId = device_id;
                showPlayer();
                // Start playback
                fetch(`https://api.spotify.com/v1/me/player/play?device_id=${{devId}}`, {{
                    method: 'PUT',
                    headers: {{ Authorization: `Bearer ${{token}}`, 'Content-Type': 'application/json' }},
                    body: JSON.stringify(uri.includes(':track:') ? {{ uris: [uri] }} : {{ context_uri: uri }})
                }});
            }});

            player.addListener('not_ready', () => showErr('Device offline'));
            player.addListener('initialization_error', ({{ message }}) => showErr(message));
            player.addListener('authentication_error', () => showErr('Reconnect Spotify in Settings'));
            player.addListener('account_error', () => showErr('Spotify Premium required'));

            player.addListener('player_state_changed', s => {{
                if (!s) return;
                const t = s.track_window.current_track;
                if (t) {{
                    $('art').src = t.album.images[0]?.url || '';
                    $('track').textContent = t.name;
                    $('artist').textContent = t.artists.map(a=>a.name).join(', ');
                }}
                pos = s.position; len = s.duration; paused = s.paused;
                $('fill').style.width = len > 0 ? (pos/len*100)+'%' : '0%';
                $('cur').textContent = fmt(pos);
                $('dur').textContent = fmt(len);
                $('playIcon').classList.toggle('hidden', !paused);
                $('pauseIcon').classList.toggle('hidden', paused);
            }});

            player.connect();
        }};

        $('play').onclick = () => player?.togglePlay();
        $('prev').onclick = () => player?.previousTrack();
        $('next').onclick = () => player?.nextTrack();
        $('bar').onclick = e => {{
            const pct = (e.clientX - e.currentTarget.getBoundingClientRect().left) / e.currentTarget.offsetWidth;
            player?.seek(Math.floor(pct * len));
        }};
    </script>
</body>
</html>"##,
        access_token = access_token,
        uri = uri,
        content_type = content_type,
        content_id = content_id
    )
}

/// Parse a Spotify URL and extract content type and ID
pub fn parse_spotify_url(url: &str) -> Option<(SpotifyContentType, String)> {
    let url = url.trim();
    if url.contains("open.spotify.com") {
        let parts: Vec<&str> = url.split('/').collect();
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

/// Full Spotify App WebView with Web Playback SDK
pub struct SpotifyAppWebView {
    webview_entity: Entity<WebView>,
    #[allow(dead_code)]
    port: u16,
    shutdown_flag: Arc<AtomicBool>,
    _server_thread: Option<JoinHandle<()>>,
}

impl SpotifyAppWebView {
    /// Create a new Spotify App WebView with full player UI
    pub fn new(window: &mut Window, cx: &mut App) -> Result<Self, String> {
        let access_token = crate::spotify_auth::get_valid_token()
            .ok_or("Not authenticated with Spotify")?;

        let port = PORT_COUNTER.fetch_add(1, Ordering::SeqCst);
        let shutdown_flag = Arc::new(AtomicBool::new(false));
        let shutdown_flag_clone = shutdown_flag.clone();

        let server_thread = thread::spawn(move || {
            let addr = format!("127.0.0.1:{}", port);
            let server = match Server::http(&addr) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to start Spotify App server: {}", e);
                    return;
                }
            };

            let html = generate_spotify_app_html(&access_token);

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
            let webview = wry::WebViewBuilder::new()
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
            port,
            shutdown_flag,
            _server_thread: Some(server_thread),
        })
    }

    /// Get the WebView entity for rendering
    pub fn webview(&self) -> Entity<WebView> {
        self.webview_entity.clone()
    }
}

impl Drop for SpotifyAppWebView {
    fn drop(&mut self) {
        self.shutdown_flag.store(true, Ordering::Relaxed);
    }
}

/// Generate HTML for full Spotify App player with Web Playback SDK
fn generate_spotify_app_html(access_token: &str) -> String {
    format!(
        r##"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        html, body {{
            width: 100%; height: 100%; overflow: hidden;
            background: #121212;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
            color: #fff;
        }}
        .app {{ width: 100%; height: 100%; display: flex; flex-direction: column; }}

        /* Tabs */
        .tabs {{
            display: flex;
            background: #181818;
            border-bottom: 1px solid #282828;
            padding: 0 12px;
        }}
        .tab {{
            padding: 12px 16px;
            font-size: 13px;
            font-weight: 600;
            color: #a0a0a0;
            cursor: pointer;
            border-bottom: 2px solid transparent;
            transition: color 0.15s, border-color 0.15s;
        }}
        .tab:hover {{ color: #fff; }}
        .tab.active {{ color: #fff; border-bottom-color: #1db954; }}

        /* Search bar */
        .search-bar {{
            padding: 12px 16px;
            background: #181818;
        }}
        .search-input {{
            width: 100%;
            padding: 10px 16px;
            border-radius: 20px;
            border: none;
            background: #242424;
            color: #fff;
            font-size: 14px;
            outline: none;
        }}
        .search-input::placeholder {{ color: #a0a0a0; }}
        .search-input:focus {{ background: #333; }}

        /* Content area */
        .content {{
            flex: 1;
            overflow-y: auto;
            padding: 12px 16px;
        }}
        .content::-webkit-scrollbar {{ width: 8px; }}
        .content::-webkit-scrollbar-track {{ background: transparent; }}
        .content::-webkit-scrollbar-thumb {{ background: #555; border-radius: 4px; }}
        .tab-content {{ display: none; }}
        .tab-content.active {{ display: block; }}

        .section-title {{
            font-size: 16px;
            font-weight: 700;
            margin: 16px 0 8px;
            color: #fff;
        }}
        .section-title:first-child {{ margin-top: 0; }}

        /* Playlist grid */
        .playlist-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
            gap: 16px;
        }}
        .playlist-card {{
            background: #181818;
            border-radius: 8px;
            padding: 12px;
            cursor: pointer;
            transition: background 0.2s;
        }}
        .playlist-card:hover {{ background: #282828; }}
        .playlist-art {{
            width: 100%;
            aspect-ratio: 1;
            border-radius: 4px;
            background: #333;
            object-fit: cover;
            margin-bottom: 8px;
        }}
        .playlist-name {{
            font-size: 13px;
            font-weight: 600;
            color: #fff;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }}
        .playlist-owner {{
            font-size: 11px;
            color: #a0a0a0;
            margin-top: 2px;
        }}

        .track-list {{ display: flex; flex-direction: column; gap: 2px; }}
        .track-item {{
            display: flex;
            align-items: center;
            gap: 12px;
            padding: 8px;
            border-radius: 4px;
            cursor: pointer;
            transition: background 0.15s;
        }}
        .track-item:hover {{ background: #282828; }}
        .track-item.playing {{ background: #282828; }}
        .track-idx {{
            width: 20px;
            font-size: 12px;
            color: #a0a0a0;
            text-align: right;
        }}
        .track-item.playing .track-idx {{ color: #1db954; }}
        .track-art {{
            width: 40px; height: 40px;
            border-radius: 4px;
            background: #333;
            object-fit: cover;
        }}
        .track-info {{ flex: 1; min-width: 0; }}
        .track-name {{
            font-size: 14px;
            font-weight: 500;
            color: #fff;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }}
        .track-item.playing .track-name {{ color: #1db954; }}
        .track-artist {{
            font-size: 12px;
            color: #a0a0a0;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }}
        .track-duration {{
            font-size: 12px;
            color: #a0a0a0;
            font-variant-numeric: tabular-nums;
        }}

        .back-btn {{
            display: inline-flex;
            align-items: center;
            gap: 4px;
            padding: 6px 12px;
            margin-bottom: 12px;
            background: #282828;
            border: none;
            border-radius: 16px;
            color: #fff;
            font-size: 12px;
            cursor: pointer;
        }}
        .back-btn:hover {{ background: #333; }}

        /* Now playing bar */
        .now-playing {{
            background: #181818;
            border-top: 1px solid #282828;
            padding: 10px 16px;
        }}
        .np-content {{
            display: flex;
            align-items: center;
            gap: 12px;
        }}
        .np-art {{
            width: 48px; height: 48px;
            border-radius: 4px;
            background: #333;
            object-fit: cover;
        }}
        .np-info {{ flex: 1; min-width: 0; }}
        .np-track {{
            font-size: 13px;
            font-weight: 600;
            color: #fff;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }}
        .np-artist {{
            font-size: 11px;
            color: #b3b3b3;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }}
        .np-controls {{
            display: flex;
            align-items: center;
            gap: 12px;
        }}
        .ctrl-btn {{
            background: none;
            border: none;
            cursor: pointer;
            padding: 6px;
            display: flex;
            opacity: 0.7;
            transition: opacity 0.15s, transform 0.1s;
        }}
        .ctrl-btn:hover {{ opacity: 1; }}
        .ctrl-btn:active {{ transform: scale(0.92); }}
        .ctrl-btn svg {{ width: 18px; height: 18px; fill: #fff; }}
        .play-btn {{
            width: 36px; height: 36px;
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
        .play-btn svg {{ width: 14px; height: 14px; fill: #000; }}

        .progress-bar {{
            margin-top: 6px;
            display: flex;
            align-items: center;
            gap: 6px;
        }}
        .time {{
            font-size: 10px;
            color: #a0a0a0;
            min-width: 32px;
            font-variant-numeric: tabular-nums;
        }}
        .time.end {{ text-align: right; }}
        .bar {{
            flex: 1;
            height: 4px;
            background: #404040;
            border-radius: 2px;
            cursor: pointer;
        }}
        .bar:hover .fill {{ background: #1db954; }}
        .fill {{
            height: 100%;
            background: #fff;
            border-radius: 2px;
            width: 0%;
            transition: width 0.1s linear;
        }}

        .loading, .error {{
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            width: 100%;
            height: 100%;
            color: #b3b3b3;
            font-size: 14px;
            gap: 8px;
        }}
        .spinner {{
            width: 32px; height: 32px;
            border: 3px solid #333;
            border-top-color: #1db954;
            border-radius: 50%;
            animation: spin 1s linear infinite;
        }}
        @keyframes spin {{ to {{ transform: rotate(360deg); }} }}
        .hidden {{ display: none; }}
        .empty {{ text-align: center; color: #a0a0a0; padding: 40px; }}
    </style>
</head>
<body>
    <div class="loading" id="loading">
        <div class="spinner"></div>
        <span>Connecting to Spotify...</span>
    </div>
    <div class="error hidden" id="error"></div>

    <div class="app hidden" id="app">
        <div class="tabs">
            <div class="tab active" data-tab="library">Library</div>
            <div class="tab" data-tab="playlists">Playlists</div>
            <div class="tab" data-tab="search">Search</div>
        </div>

        <div class="content">
            <div class="tab-content active" id="library">
                <div class="section-title">Liked Songs</div>
                <div id="likedSongs" class="track-list"></div>
                <div class="section-title">Recently Played</div>
                <div id="recentlyPlayed" class="track-list"></div>
            </div>

            <div class="tab-content" id="playlists">
                <div id="playlistsGrid" class="playlist-grid"></div>
                <div id="playlistView" class="hidden">
                    <button class="back-btn" onclick="showPlaylists()">← Back</button>
                    <div id="playlistTracks" class="track-list"></div>
                </div>
            </div>

            <div class="tab-content" id="search">
                <div class="search-bar" style="padding: 0 0 12px 0;">
                    <input type="text" class="search-input" id="searchInput" placeholder="Search songs, artists, albums...">
                </div>
                <div id="searchResults"></div>
            </div>
        </div>

        <div class="now-playing">
            <div class="np-content">
                <img class="np-art" id="npArt" src="data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7" alt="">
                <div class="np-info">
                    <div class="np-track" id="npTrack">Not playing</div>
                    <div class="np-artist" id="npArtist">-</div>
                </div>
                <div class="np-controls">
                    <button class="ctrl-btn" id="prevBtn">
                        <svg viewBox="0 0 24 24"><path d="M6 6h2v12H6zm3.5 6l8.5 6V6z"/></svg>
                    </button>
                    <button class="play-btn" id="playBtn">
                        <svg id="playIcon" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
                    </button>
                    <button class="ctrl-btn" id="nextBtn">
                        <svg viewBox="0 0 24 24"><path d="M6 18l8.5-6L6 6v12zM16 6v12h2V6h-2z"/></svg>
                    </button>
                </div>
            </div>
            <div class="progress-bar">
                <span class="time" id="currentTime">0:00</span>
                <div class="bar" id="progressBar">
                    <div class="fill" id="progressFill"></div>
                </div>
                <span class="time end" id="duration">0:00</span>
            </div>
        </div>
    </div>

    <script src="https://sdk.scdn.co/spotify-player.js"></script>
    <script>
        const token = '{access_token}';
        let player, deviceId, currentTrack = null, isPlaying = false;
        let currentQueue = [], currentQueueIndex = 0;

        const fmt = ms => {{ const s = Math.floor(ms/1000), m = Math.floor(s/60); return m + ':' + String(s%60).padStart(2,'0'); }};
        const esc = s => s.replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;').replace(/"/g,'&quot;');

        // Tabs
        document.querySelectorAll('.tab').forEach(t => t.onclick = () => {{
            document.querySelectorAll('.tab').forEach(x => x.classList.remove('active'));
            document.querySelectorAll('.tab-content').forEach(x => x.classList.remove('active'));
            t.classList.add('active');
            document.getElementById(t.dataset.tab).classList.add('active');
        }});

        window.onSpotifyWebPlaybackSDKReady = () => {{
            player = new Spotify.Player({{ name: 'Humanboard', getOAuthToken: cb => cb(token), volume: 0.5 }});

            player.addListener('ready', ({{ device_id }}) => {{
                deviceId = device_id;
                document.getElementById('loading').classList.add('hidden');
                document.getElementById('app').classList.remove('hidden');
                loadLibrary();
                loadPlaylists();
            }});

            player.addListener('initialization_error', ({{ message }}) => showError('Init error: ' + message));
            player.addListener('authentication_error', ({{ message }}) => showError('Auth error: ' + message));

            player.addListener('player_state_changed', state => {{
                if (!state) return;
                currentTrack = state.track_window.current_track;
                isPlaying = !state.paused;

                document.getElementById('npTrack').textContent = currentTrack.name;
                document.getElementById('npArtist').textContent = currentTrack.artists.map(a => a.name).join(', ');
                document.getElementById('npArt').src = currentTrack.album.images[0]?.url || '';
                document.getElementById('playIcon').innerHTML = isPlaying ? '<path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z"/>' : '<path d="M8 5v14l11-7z"/>';
                document.getElementById('currentTime').textContent = fmt(state.position);
                document.getElementById('duration').textContent = fmt(state.duration);
                document.getElementById('progressFill').style.width = (state.position / state.duration * 100) + '%';

                document.querySelectorAll('.track-item').forEach(el => el.classList.toggle('playing', el.dataset.uri === currentTrack.uri));
            }});

            player.connect();
        }};

        function showError(msg) {{
            document.getElementById('loading').classList.add('hidden');
            document.getElementById('error').textContent = msg;
            document.getElementById('error').classList.remove('hidden');
        }}

        setInterval(() => {{
            if (player && isPlaying) player.getCurrentState().then(s => {{
                if (s) {{
                    document.getElementById('currentTime').textContent = fmt(s.position);
                    document.getElementById('progressFill').style.width = (s.position / s.duration * 100) + '%';
                }}
            }});
        }}, 1000);

        document.getElementById('playBtn').onclick = () => player.togglePlay();
        document.getElementById('prevBtn').onclick = () => {{
            if (currentQueueIndex > 0) playFromQueue(currentQueueIndex - 1);
            else player.previousTrack();
        }};
        document.getElementById('nextBtn').onclick = () => {{
            if (currentQueueIndex < currentQueue.length - 1) playFromQueue(currentQueueIndex + 1);
            else player.nextTrack();
        }};
        document.getElementById('progressBar').onclick = e => {{
            player.getCurrentState().then(s => {{
                if (s) {{ const r = e.target.getBoundingClientRect(); player.seek((e.clientX - r.left) / r.width * s.duration); }}
            }});
        }};

        async function loadLibrary() {{
            try {{
                const [likedRes, recentRes] = await Promise.all([
                    fetch('https://api.spotify.com/v1/me/tracks?limit=50', {{ headers: {{ Authorization: 'Bearer ' + token }} }}),
                    fetch('https://api.spotify.com/v1/me/player/recently-played?limit=50', {{ headers: {{ Authorization: 'Bearer ' + token }} }})
                ]);

                if (likedRes.status === 403) {{
                    document.getElementById('likedSongs').innerHTML = '<div class="empty">Re-authenticate in Settings to access Liked Songs</div>';
                }} else if (likedRes.ok) {{
                    const liked = await likedRes.json();
                    renderTracks('likedSongs', liked.items?.map(i => i.track) || [], true);
                }} else {{
                    document.getElementById('likedSongs').innerHTML = '<div class="empty">Failed to load</div>';
                }}

                if (recentRes.status === 403) {{
                    document.getElementById('recentlyPlayed').innerHTML = '<div class="empty">Re-authenticate in Settings to access Recently Played</div>';
                }} else if (recentRes.ok) {{
                    const recent = await recentRes.json();
                    renderTracks('recentlyPlayed', recent.items?.map(i => i.track) || [], false);
                }} else {{
                    document.getElementById('recentlyPlayed').innerHTML = '<div class="empty">Failed to load</div>';
                }}
            }} catch (e) {{
                console.error('Load library error:', e);
                document.getElementById('likedSongs').innerHTML = '<div class="empty">Error loading library</div>';
            }}
        }}

        async function loadPlaylists() {{
            try {{
                const res = await fetch('https://api.spotify.com/v1/me/playlists?limit=50', {{ headers: {{ Authorization: 'Bearer ' + token }} }});
                if (res.status === 403) {{
                    document.getElementById('playlistsGrid').innerHTML = '<div class="empty">Re-authenticate in Settings to access Playlists</div>';
                    return;
                }}
                if (!res.ok) {{
                    document.getElementById('playlistsGrid').innerHTML = '<div class="empty">Failed to load playlists</div>';
                    return;
                }}
                const data = await res.json();
                const html = (data.items || []).map(p => `
                    <div class="playlist-card" onclick="openPlaylist('${{p.id}}', '${{esc(p.name)}}')">
                        <img class="playlist-art" src="${{p.images?.[0]?.url || ''}}" alt="">
                        <div class="playlist-name">${{esc(p.name)}}</div>
                        <div class="playlist-owner">${{p.tracks?.total || 0}} songs</div>
                    </div>
                `).join('');
                document.getElementById('playlistsGrid').innerHTML = html || '<div class="empty">No playlists found</div>';
            }} catch (e) {{
                console.error('Load playlists error:', e);
                document.getElementById('playlistsGrid').innerHTML = '<div class="empty">Error loading playlists</div>';
            }}
        }}

        async function openPlaylist(id, name) {{
            document.getElementById('playlistsGrid').classList.add('hidden');
            document.getElementById('playlistView').classList.remove('hidden');
            document.getElementById('playlistTracks').innerHTML = '<div class="empty">Loading...</div>';
            try {{
                const data = await fetch(`https://api.spotify.com/v1/playlists/${{id}}/tracks?limit=100`, {{ headers: {{ Authorization: 'Bearer ' + token }} }}).then(r => r.json());
                const tracks = (data.items || []).map(i => i.track).filter(t => t);
                currentQueue = tracks.map(t => t.uri);
                renderTracks('playlistTracks', tracks, true);
            }} catch (e) {{ console.error('Load playlist error:', e); }}
        }}

        function showPlaylists() {{
            document.getElementById('playlistsGrid').classList.remove('hidden');
            document.getElementById('playlistView').classList.add('hidden');
        }}

        function renderTracks(containerId, tracks, showIdx) {{
            if (!tracks.length) {{ document.getElementById(containerId).innerHTML = '<div class="empty">No tracks</div>'; return; }}
            document.getElementById(containerId).innerHTML = tracks.map((t, i) => `
                <div class="track-item${{currentTrack?.uri === t.uri ? ' playing' : ''}}" data-uri="${{t.uri}}" data-idx="${{i}}" onclick="playTrackFromList(this, ${{JSON.stringify(tracks.map(x=>x.uri)).replace(/"/g, '&quot;')}})">
                    ${{showIdx ? `<span class="track-idx">${{i+1}}</span>` : ''}}
                    <img class="track-art" src="${{t.album?.images?.[2]?.url || t.album?.images?.[0]?.url || ''}}" alt="">
                    <div class="track-info">
                        <div class="track-name">${{esc(t.name)}}</div>
                        <div class="track-artist">${{esc(t.artists?.map(a => a.name).join(', ') || '')}}</div>
                    </div>
                    <div class="track-duration">${{fmt(t.duration_ms)}}</div>
                </div>
            `).join('');
        }}

        async function playTrackFromList(el, queue) {{
            const idx = parseInt(el.dataset.idx);
            currentQueue = queue;
            currentQueueIndex = idx;
            await playQueue(idx);
        }}

        async function playQueue(startIdx) {{
            if (!currentQueue.length) return;
            currentQueueIndex = startIdx;
            try {{
                await fetch(`https://api.spotify.com/v1/me/player/play?device_id=${{deviceId}}`, {{
                    method: 'PUT',
                    headers: {{ Authorization: 'Bearer ' + token, 'Content-Type': 'application/json' }},
                    body: JSON.stringify({{ uris: currentQueue, offset: {{ position: startIdx }} }})
                }});
            }} catch (e) {{ console.error('Play error:', e); }}
        }}

        function playFromQueue(idx) {{
            if (idx >= 0 && idx < currentQueue.length) playQueue(idx);
        }}

        // Search
        let searchTimeout;
        document.getElementById('searchInput').oninput = e => {{
            clearTimeout(searchTimeout);
            const q = e.target.value.trim();
            if (!q) {{ document.getElementById('searchResults').innerHTML = ''; return; }}
            searchTimeout = setTimeout(() => searchSpotify(q), 300);
        }};

        async function searchSpotify(q) {{
            try {{
                const data = await fetch(`https://api.spotify.com/v1/search?q=${{encodeURIComponent(q)}}&type=track&limit=30`, {{ headers: {{ Authorization: 'Bearer ' + token }} }}).then(r => r.json());
                const tracks = data.tracks?.items || [];
                currentQueue = tracks.map(t => t.uri);
                if (!tracks.length) {{ document.getElementById('searchResults').innerHTML = '<div class="empty">No results</div>'; return; }}
                document.getElementById('searchResults').innerHTML = '<div class="section-title">Results</div><div id="searchTracks" class="track-list"></div>';
                renderTracks('searchTracks', tracks, false);
            }} catch (e) {{ console.error('Search error:', e); }}
        }}
    </script>
</body>
</html>"##,
        access_token = access_token
    )
}
