//! Linux WebView support using XWayland compatibility mode.
//!
//! Forces GDK_BACKEND=x11 to ensure WebKitGTK uses XWayland on Wayland systems,
//! allowing consistent use of wry's `build_as_child()` across all Linux display servers.

#[cfg(target_os = "linux")]
use std::sync::atomic::{AtomicBool, Ordering};

/// Flag indicating whether GTK has been initialized
#[cfg(target_os = "linux")]
static GTK_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Detected display server type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayServer {
    X11,
    Wayland,
    Unknown,
}

/// Detect the current display server at runtime
#[cfg(target_os = "linux")]
pub fn detect_display_server() -> DisplayServer {
    // Check XDG_SESSION_TYPE first (most reliable)
    if let Ok(session_type) = std::env::var("XDG_SESSION_TYPE") {
        match session_type.to_lowercase().as_str() {
            "wayland" => return DisplayServer::Wayland,
            "x11" => return DisplayServer::X11,
            _ => {}
        }
    }

    // Fallback to checking display environment variables
    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        return DisplayServer::Wayland;
    }

    if std::env::var("DISPLAY").is_ok() {
        return DisplayServer::X11;
    }

    DisplayServer::Unknown
}

#[cfg(not(target_os = "linux"))]
pub fn detect_display_server() -> DisplayServer {
    DisplayServer::Unknown
}

/// Check if we're running on Wayland
#[cfg(target_os = "linux")]
pub fn is_wayland() -> bool {
    detect_display_server() == DisplayServer::Wayland
}

#[cfg(not(target_os = "linux"))]
pub fn is_wayland() -> bool {
    false
}

/// Initialize GTK for Linux webview support
/// Must be called before creating any webviews on Linux
#[cfg(target_os = "linux")]
pub fn init_gtk() -> bool {
    if GTK_INITIALIZED.load(Ordering::Relaxed) {
        return true;
    }

    // Force WebKitGTK to use XWayland for compatibility
    // SAFETY: Called once during init, before any other threads access this env var
    unsafe { std::env::set_var("GDK_BACKEND", "x11") };

    match gtk::init() {
        Ok(()) => {
            GTK_INITIALIZED.store(true, Ordering::Relaxed);
            tracing::info!(
                "GTK initialized for Linux webview support (display: {:?})",
                detect_display_server()
            );
            true
        }
        Err(e) => {
            tracing::error!("Failed to initialize GTK: {:?}", e);
            false
        }
    }
}

#[cfg(not(target_os = "linux"))]
pub fn init_gtk() -> bool {
    true
}

/// Pump GTK events (required for webview functionality on Linux)
#[cfg(target_os = "linux")]
pub fn pump_gtk_events() {
    if !GTK_INITIALIZED.load(Ordering::Relaxed) {
        return;
    }

    while gtk::events_pending() {
        gtk::main_iteration_do(false);
    }
}

#[cfg(not(target_os = "linux"))]
pub fn pump_gtk_events() {
    // No-op on non-Linux platforms
}

/// Placeholder for Wayland container (unused - we use XWayland mode)
#[cfg(target_os = "linux")]
pub struct WaylandWebViewContainer;

/// Create a webview for Linux using XWayland compatibility mode
///
/// Uses build_as_child() with the gpui window handle. Works on both X11 and
/// Wayland (via XWayland, forced by GDK_BACKEND=x11 in init_gtk).
#[cfg(target_os = "linux")]
pub fn create_linux_webview<W: raw_window_handle::HasWindowHandle>(
    window: &W,
    builder: wry::WebViewBuilder,
) -> Result<(wry::WebView, Option<WaylandWebViewContainer>), String> {
    let webview = builder
        .build_as_child(window)
        .map_err(|e| format!("Failed to create webview: {:?}", e))?;

    Ok((webview, None))
}

/// Wrapper for Linux webview (container is unused in XWayland mode)
#[cfg(target_os = "linux")]
pub struct LinuxWebViewHandle {
    pub container: Option<WaylandWebViewContainer>,
}

#[cfg(target_os = "linux")]
impl LinuxWebViewHandle {
    pub fn new(container: Option<WaylandWebViewContainer>) -> Self {
        Self { container }
    }

    pub fn set_container_bounds(&self, _x: i32, _y: i32, _width: i32, _height: i32) {}
    pub fn show_container(&self) {}
    pub fn hide_container(&self) {}
}

// Stub implementations for non-Linux platforms

#[cfg(not(target_os = "linux"))]
pub struct WaylandWebViewContainer;

#[cfg(not(target_os = "linux"))]
pub struct LinuxWebViewHandle {
    pub container: Option<WaylandWebViewContainer>,
}

#[cfg(not(target_os = "linux"))]
impl LinuxWebViewHandle {
    pub fn new(_container: Option<WaylandWebViewContainer>) -> Self {
        Self { container: None }
    }

    pub fn set_container_bounds(&self, _x: i32, _y: i32, _width: i32, _height: i32) {}
    pub fn show_container(&self) {}
    pub fn hide_container(&self) {}
}
