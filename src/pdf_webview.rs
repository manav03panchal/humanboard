//! WebView-based PDF viewer using native platform rendering.
//!
//! This module provides PDF viewing by loading PDFs directly into a WebView,
//! which uses the platform's native PDF rendering (PDFKit on macOS via WKWebView).
//!
//! ## Security
//!
//! File paths are properly URL-encoded component-by-component to prevent
//! path traversal attacks (CWE-22).
//!
//! ## Features
//!
//! - Native PDF rendering with zoom and scroll
//! - Bounds control for positioning within the canvas
//! - Show/hide for visibility management

use gpui::*;
use gpui_component::webview::WebView;
use std::path::{Path, PathBuf};
use wry::dpi::{LogicalPosition, LogicalSize};
use wry::{Rect, WebViewBuilder};

/// Convert a path to a properly encoded file:// URL.
/// Each path component is URL-encoded to prevent path traversal attacks (CWE-22).
fn path_to_file_url(path: &Path) -> String {
    let encoded_components: Vec<String> = path
        .components()
        .map(|c| {
            match c {
                std::path::Component::Prefix(p) => {
                    // Windows drive prefix (e.g., "C:")
                    p.as_os_str().to_string_lossy().to_string()
                }
                std::path::Component::RootDir => String::new(),
                std::path::Component::Normal(s) => {
                    urlencoding::encode(&s.to_string_lossy()).into_owned()
                }
                std::path::Component::CurDir => ".".to_string(),
                std::path::Component::ParentDir => "..".to_string(),
            }
        })
        .collect();

    // Join with "/" for URL path separator
    let path_str = encoded_components.join("/");

    // Ensure path starts with "/" for file:// URLs
    if path_str.starts_with('/') || path_str.is_empty() {
        format!("file://{}", path_str)
    } else {
        format!("file:///{}", path_str)
    }
}

/// WebView-based PDF viewer using native WKWebView (which uses PDFKit on macOS)
pub struct PdfWebView {
    webview_entity: Entity<WebView>,
    path: PathBuf,
}

impl PdfWebView {
    /// Create a new PDF WebView
    pub fn new(path: PathBuf, window: &mut Window, cx: &mut App) -> Result<Self, String> {
        let file_url = path_to_file_url(&path);

        // Create WebView entity
        let builder = WebViewBuilder::new();

        #[cfg(any(
            target_os = "macos",
            target_os = "windows",
            target_os = "ios",
            target_os = "android"
        ))]
        let webview = builder
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

        // Load the PDF
        webview_entity.update(cx, |view, _| {
            view.load_url(&file_url);
        });

        Ok(Self {
            webview_entity,
            path,
        })
    }

    /// Get the path to the PDF
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Get the WebView entity for rendering
    pub fn webview(&self) -> Entity<WebView> {
        self.webview_entity.clone()
    }

    /// Reload the PDF in the WebView
    pub fn reload(&self, cx: &mut App) {
        let file_url = path_to_file_url(&self.path);
        self.webview_entity.update(cx, |view, _| {
            view.load_url(&file_url);
        });
    }

    /// Set the bounds of the webview explicitly (x, y, width, height in logical pixels)
    pub fn set_bounds(&self, x: f32, y: f32, width: f32, height: f32, cx: &mut App) {
        self.webview_entity.update(cx, |view, _| {
            let _ = view.raw().set_bounds(Rect {
                position: wry::dpi::Position::Logical(LogicalPosition::new(x as f64, y as f64)),
                size: wry::dpi::Size::Logical(LogicalSize::new(width as f64, height as f64)),
            });
        });
    }

    /// Show the webview
    pub fn show(&self, cx: &mut App) {
        self.webview_entity.update(cx, |view, _| view.show());
    }

    /// Hide the webview
    pub fn hide(&self, cx: &mut App) {
        self.webview_entity.update(cx, |view, _| view.hide());
    }
}
