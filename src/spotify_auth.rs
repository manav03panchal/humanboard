//! Spotify OAuth authentication using PKCE flow
//!
//! This module handles:
//! - PKCE code generation
//! - OAuth authorization flow
//! - Token exchange and storage
//! - Token refresh

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

// Spotify app credentials
const CLIENT_ID: &str = "8c736b3890e34f31896df21317e54d49";
const REDIRECT_URI: &str = "http://127.0.0.1:5588/callback";
const SCOPES: &str = "streaming user-read-email user-read-private user-modify-playback-state user-read-playback-state user-library-read user-read-recently-played playlist-read-private playlist-read-collaborative";

/// Spotify authentication tokens
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpotifyTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: u64, // Unix timestamp
}

impl SpotifyTokens {
    /// Check if the access token is expired (with 5 min buffer)
    pub fn is_expired(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now >= self.expires_at.saturating_sub(300) // 5 min buffer
    }

    /// Get the tokens file path
    fn tokens_path() -> PathBuf {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("humanboard");
        let _ = fs::create_dir_all(&config_dir);
        config_dir.join("spotify_tokens.json")
    }

    /// Save tokens to disk
    pub fn save(&self) -> Result<(), String> {
        let path = Self::tokens_path();
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize tokens: {}", e))?;
        fs::write(&path, json)
            .map_err(|e| format!("Failed to write tokens: {}", e))?;
        Ok(())
    }

    /// Load tokens from disk
    pub fn load() -> Option<Self> {
        let path = Self::tokens_path();
        let json = fs::read_to_string(&path).ok()?;
        serde_json::from_str(&json).ok()
    }

    /// Delete tokens from disk
    pub fn delete() -> Result<(), String> {
        let path = Self::tokens_path();
        if path.exists() {
            fs::remove_file(&path)
                .map_err(|e| format!("Failed to delete tokens: {}", e))?;
        }
        Ok(())
    }
}

/// Generate a random code verifier for PKCE
fn generate_code_verifier() -> String {
    use rand::RngCore;
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 64];
    rng.fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(&bytes)
}

/// Generate code challenge from verifier (SHA256 + base64url)
fn generate_code_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let hash = hasher.finalize();
    URL_SAFE_NO_PAD.encode(&hash)
}

/// Build the Spotify authorization URL
pub fn build_auth_url(code_verifier: &str) -> String {
    let code_challenge = generate_code_challenge(code_verifier);

    format!(
        "https://accounts.spotify.com/authorize?\
         client_id={}&\
         response_type=code&\
         redirect_uri={}&\
         code_challenge_method=S256&\
         code_challenge={}&\
         scope={}",
        CLIENT_ID,
        urlencoding::encode(REDIRECT_URI),
        code_challenge,
        urlencoding::encode(SCOPES)
    )
}

/// Response from Spotify token endpoint
#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
    expires_in: u64,
}

/// Exchange authorization code for tokens
pub fn exchange_code_for_tokens(code: &str, code_verifier: &str) -> Result<SpotifyTokens, String> {
    let client = reqwest::blocking::Client::new();

    let mut params = HashMap::new();
    params.insert("grant_type", "authorization_code");
    params.insert("code", code);
    params.insert("redirect_uri", REDIRECT_URI);
    params.insert("client_id", CLIENT_ID);
    params.insert("code_verifier", code_verifier);

    let response = client
        .post("https://accounts.spotify.com/api/token")
        .form(&params)
        .send()
        .map_err(|e| format!("Token request failed: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().unwrap_or_default();
        return Err(format!("Token exchange failed: {}", error_text));
    }

    let token_response: TokenResponse = response
        .json()
        .map_err(|e| format!("Failed to parse token response: {}", e))?;

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    Ok(SpotifyTokens {
        access_token: token_response.access_token,
        refresh_token: token_response.refresh_token,
        expires_at: now + token_response.expires_in,
    })
}

/// Refresh the access token using the refresh token
pub fn refresh_access_token(refresh_token: &str) -> Result<SpotifyTokens, String> {
    let client = reqwest::blocking::Client::new();

    let mut params = HashMap::new();
    params.insert("grant_type", "refresh_token");
    params.insert("refresh_token", refresh_token);
    params.insert("client_id", CLIENT_ID);

    let response = client
        .post("https://accounts.spotify.com/api/token")
        .form(&params)
        .send()
        .map_err(|e| format!("Refresh request failed: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().unwrap_or_default();
        return Err(format!("Token refresh failed: {}", error_text));
    }

    let token_response: TokenResponse = response
        .json()
        .map_err(|e| format!("Failed to parse refresh response: {}", e))?;

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    Ok(SpotifyTokens {
        access_token: token_response.access_token,
        refresh_token: token_response.refresh_token,
        expires_at: now + token_response.expires_in,
    })
}

/// State for the OAuth flow
pub struct SpotifyAuthFlow {
    code_verifier: String,
    result: Arc<Mutex<Option<Result<String, String>>>>,
}

impl SpotifyAuthFlow {
    /// Start a new OAuth flow
    pub fn new() -> Self {
        Self {
            code_verifier: generate_code_verifier(),
            result: Arc::new(Mutex::new(None)),
        }
    }

    /// Get the authorization URL to open in browser
    pub fn get_auth_url(&self) -> String {
        build_auth_url(&self.code_verifier)
    }

    /// Start the callback server in a background thread
    /// Returns immediately, call check_result() to poll for completion
    pub fn start_callback_server(&self) {
        let result = self.result.clone();

        thread::spawn(move || {
            let listener = match TcpListener::bind("127.0.0.1:5588") {
                Ok(l) => l,
                Err(e) => {
                    *result.lock().unwrap() = Some(Err(format!("Failed to bind: {}", e)));
                    return;
                }
            };

            // Set a timeout so we don't block forever
            listener.set_nonblocking(false).ok();

            // Wait for the callback (with timeout handled by dropping listener)
            if let Ok((mut stream, _)) = listener.accept() {
                let mut reader = BufReader::new(&stream);
                let mut request_line = String::new();

                if reader.read_line(&mut request_line).is_ok() {
                    // Parse the authorization code from the callback URL
                    // GET /callback?code=xxx HTTP/1.1
                    if let Some(code) = extract_code_from_request(&request_line) {
                        // Send success response to browser
                        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n\
                            <html><body style='font-family: system-ui; display: flex; justify-content: center; align-items: center; height: 100vh; margin: 0; background: #121212; color: white;'>\
                            <div style='text-align: center;'>\
                            <h1 style='color: #1DB954;'>Connected to Spotify!</h1>\
                            <p>You can close this window and return to Humanboard.</p>\
                            </div></body></html>";
                        let _ = stream.write_all(response.as_bytes());

                        *result.lock().unwrap() = Some(Ok(code));
                    } else if request_line.contains("error=") {
                        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n\
                            <html><body style='font-family: system-ui; display: flex; justify-content: center; align-items: center; height: 100vh; margin: 0; background: #121212; color: white;'>\
                            <div style='text-align: center;'>\
                            <h1 style='color: #e74c3c;'>Authorization Failed</h1>\
                            <p>You can close this window and try again.</p>\
                            </div></body></html>";
                        let _ = stream.write_all(response.as_bytes());

                        *result.lock().unwrap() = Some(Err("User denied authorization".to_string()));
                    }
                }
            }
        });
    }

    /// Check if the OAuth flow has completed
    /// Returns None if still waiting, Some(Ok(code)) on success, Some(Err(msg)) on failure
    pub fn check_result(&self) -> Option<Result<String, String>> {
        self.result.lock().unwrap().clone()
    }

    /// Complete the flow by exchanging the code for tokens
    pub fn complete(&self, code: &str) -> Result<SpotifyTokens, String> {
        exchange_code_for_tokens(code, &self.code_verifier)
    }
}

/// Extract the authorization code from the callback request
fn extract_code_from_request(request: &str) -> Option<String> {
    // Request looks like: GET /callback?code=AQD...&state=... HTTP/1.1
    let url_part = request.split_whitespace().nth(1)?;

    if !url_part.starts_with("/callback?") {
        return None;
    }

    let query = url_part.strip_prefix("/callback?")?;

    for param in query.split('&') {
        if let Some(code) = param.strip_prefix("code=") {
            return Some(code.to_string());
        }
    }

    None
}

/// Get a valid access token, refreshing if necessary
pub fn get_valid_token() -> Option<String> {
    let tokens = SpotifyTokens::load()?;

    if tokens.is_expired() {
        // Try to refresh
        match refresh_access_token(&tokens.refresh_token) {
            Ok(new_tokens) => {
                let _ = new_tokens.save();
                Some(new_tokens.access_token)
            }
            Err(_) => {
                // Refresh failed, tokens are invalid
                let _ = SpotifyTokens::delete();
                None
            }
        }
    } else {
        Some(tokens.access_token)
    }
}

/// Check if user is connected to Spotify
pub fn is_connected() -> bool {
    SpotifyTokens::load().is_some()
}

/// Disconnect from Spotify (delete tokens)
pub fn disconnect() -> Result<(), String> {
    SpotifyTokens::delete()
}
