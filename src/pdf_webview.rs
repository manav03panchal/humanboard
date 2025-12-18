use gpui::*;
use gpui_component::webview::WebView;
use std::path::PathBuf;
use wry::dpi::{LogicalPosition, LogicalSize};
use wry::{Rect, WebViewBuilder};

/// WebView-based PDF viewer using native WKWebView (which uses PDFKit on macOS)
pub struct PdfWebView {
    webview_entity: Entity<WebView>,
    path: PathBuf,
}

impl PdfWebView {
    /// Create a new PDF WebView
    pub fn new(path: PathBuf, window: &mut Window, cx: &mut App) -> Result<Self, String> {
        let file_url = format!("file://{}", path.display());

        // Create WebView entity
        let webview_entity = cx.new(|cx| {
            let builder = WebViewBuilder::new();

            #[cfg(any(
                target_os = "macos",
                target_os = "windows",
                target_os = "ios",
                target_os = "android"
            ))]
            let webview = {
                builder
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
        let file_url = format!("file://{}", self.path.display());
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
