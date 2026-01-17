//! Linux WebView support with X11 and Wayland compatibility.
//!
//! This module provides WebView creation that works on both X11 and Wayland display servers.
//!
//! ## Architecture
//!
//! - **X11**: Uses wry's `build_as_child()` which creates a child X11 window
//! - **Wayland**: Uses wry's `build_gtk()` with a standalone GTK window
//!
//! On Wayland, since gpui doesn't use GTK internally, we create a separate GTK popup
//! window that hosts the webview and position it to overlay the gpui window area.

#[cfg(target_os = "linux")]
use gtk::prelude::*;
#[cfg(target_os = "linux")]
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(target_os = "linux")]
use wry::WebViewBuilderExtUnix;

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

/// GTK window container for Wayland webviews
/// This wraps a GTK window and Fixed container for hosting webviews on Wayland
#[cfg(target_os = "linux")]
pub struct WaylandWebViewContainer {
    window: gtk::Window,
    fixed: gtk::Fixed,
}

#[cfg(target_os = "linux")]
impl WaylandWebViewContainer {
    /// Create a new GTK container for hosting a webview on Wayland
    pub fn new() -> Result<Self, String> {
        if !GTK_INITIALIZED.load(Ordering::Relaxed) {
            init_gtk();
        }

        // Create a popup-style window (no decorations, stays on top)
        let window = gtk::Window::new(gtk::WindowType::Popup);
        window.set_decorated(false);
        window.set_skip_taskbar_hint(true);
        window.set_skip_pager_hint(true);
        window.set_accept_focus(true);
        window.set_keep_above(true);

        // Create Fixed container for the webview
        let fixed = gtk::Fixed::new();
        window.add(&fixed);

        Ok(Self { window, fixed })
    }

    /// Get a reference to the Fixed container for building webviews
    pub fn container(&self) -> &gtk::Fixed {
        &self.fixed
    }

    /// Show the container window
    pub fn show(&self) {
        self.window.show_all();
    }

    /// Hide the container window
    pub fn hide(&self) {
        self.window.hide();
    }

    /// Set the position and size of the container window
    pub fn set_bounds(&self, x: i32, y: i32, width: i32, height: i32) {
        self.window.move_(x, y);
        self.window.resize(width.max(1), height.max(1));
    }

    /// Close and destroy the container
    pub fn close(&self) {
        self.window.close();
    }
}

#[cfg(target_os = "linux")]
impl Drop for WaylandWebViewContainer {
    fn drop(&mut self) {
        self.window.close();
    }
}

/// Create a webview appropriate for the current Linux display server
///
/// On X11: Uses build_as_child() with the gpui window handle
/// On Wayland: Creates a GTK container and uses build_gtk()
///
/// Returns the wry::WebView and optionally the GTK container (for Wayland)
#[cfg(target_os = "linux")]
pub fn create_linux_webview<W: raw_window_handle::HasWindowHandle>(
    window: &W,
    builder: wry::WebViewBuilder,
) -> Result<(wry::WebView, Option<WaylandWebViewContainer>), String> {
    let display = detect_display_server();

    match display {
        DisplayServer::X11 | DisplayServer::Unknown => {
            // Use standard build_as_child for X11
            let webview = builder
                .build_as_child(window)
                .map_err(|e| format!("Failed to create X11 webview: {:?}", e))?;

            Ok((webview, None))
        }
        DisplayServer::Wayland => {
            // Create GTK container for Wayland
            let container = WaylandWebViewContainer::new()?;

            let webview = builder
                .build_gtk(container.container())
                .map_err(|e| format!("Failed to create Wayland webview: {:?}", e))?;

            container.show();

            Ok((webview, Some(container)))
        }
    }
}

/// Wrapper that holds both the webview and its Wayland container (if any)
#[cfg(target_os = "linux")]
pub struct LinuxWebViewHandle {
    /// The Wayland GTK container (None on X11)
    pub container: Option<WaylandWebViewContainer>,
}

#[cfg(target_os = "linux")]
impl LinuxWebViewHandle {
    pub fn new(container: Option<WaylandWebViewContainer>) -> Self {
        Self { container }
    }

    /// Update bounds for the Wayland container
    pub fn set_container_bounds(&self, x: i32, y: i32, width: i32, height: i32) {
        if let Some(ref container) = self.container {
            container.set_bounds(x, y, width, height);
        }
    }

    /// Show the Wayland container
    pub fn show_container(&self) {
        if let Some(ref container) = self.container {
            container.show();
        }
    }

    /// Hide the Wayland container
    pub fn hide_container(&self) {
        if let Some(ref container) = self.container {
            container.hide();
        }
    }
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
