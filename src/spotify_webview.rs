use gpui::*;
use gpui_component::webview::WebView;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};
use std::thread::{self, JoinHandle};
use tiny_http::{Response, Server};

// Global port counter for unique server ports
static PORT_COUNTER: AtomicU16 = AtomicU16::new(19850);

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
