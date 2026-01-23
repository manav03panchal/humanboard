//! Preview Panel Module
//!
//! This module provides helper functions and extensions for the preview panel.
//! The core `PreviewPanel` and `PreviewTab` types are defined in `app.rs` and
//! re-exported here for convenience.
//!
//! ## Features
//!
//! - **Tab Management**: Multiple file tabs with switching
//! - **Split Views**: Horizontal/vertical split with canvas
//! - **File Types**: PDF, Markdown, Code files
//! - **Editing**: Inline editing for markdown and code

// Re-export types from app module
pub use crate::app::{PreviewPanel, PreviewTab, SplitDirection};

use std::path::PathBuf;
use tracing::debug;

/// Extension trait for PreviewPanel with additional utility methods.
pub trait PreviewPanelExt {
    /// Check if the panel has any tabs.
    fn is_empty(&self) -> bool;

    /// Get the currently active tab.
    fn active_tab(&self) -> Option<&PreviewTab>;

    /// Find a tab by file path, returning its index.
    fn find_tab(&self, path: &PathBuf) -> Option<usize>;

    /// Switch to the next tab (wrapping around).
    fn next_tab(&mut self);

    /// Switch to the previous tab (wrapping around).
    fn prev_tab(&mut self);

    /// Close a specific tab by index.
    /// Returns true if the panel should be closed (no tabs left).
    fn close_tab(&mut self, index: usize) -> bool;

    /// Calculate the canvas bounds given window size.
    fn canvas_bounds(&self, window_width: f32, window_height: f32) -> (f32, f32);

    /// Calculate the panel bounds (x, y, width, height) given window size.
    fn panel_bounds(&self, window_width: f32, window_height: f32) -> (f32, f32, f32, f32);
}

impl PreviewPanelExt for PreviewPanel {
    fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }

    fn active_tab(&self) -> Option<&PreviewTab> {
        self.tabs.get(self.active_tab)
    }

    fn find_tab(&self, path: &PathBuf) -> Option<usize> {
        self.tabs.iter().position(|t| t.path() == Some(path))
    }

    fn next_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.active_tab = (self.active_tab + 1) % self.tabs.len();
            debug!("Switched to next tab: {}", self.active_tab);
        }
    }

    fn prev_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.active_tab = if self.active_tab == 0 {
                self.tabs.len() - 1
            } else {
                self.active_tab - 1
            };
            debug!("Switched to previous tab: {}", self.active_tab);
        }
    }

    fn close_tab(&mut self, index: usize) -> bool {
        if index < self.tabs.len() {
            debug!("Closing tab {}", index);
            self.tabs.remove(index);

            if self.tabs.is_empty() {
                return true;
            }

            // Adjust active tab if needed
            if self.active_tab >= self.tabs.len() {
                self.active_tab = self.tabs.len() - 1;
            } else if index < self.active_tab {
                self.active_tab -= 1;
            }
        }
        false
    }

    fn canvas_bounds(&self, window_width: f32, window_height: f32) -> (f32, f32) {
        match self.split {
            SplitDirection::Vertical => ((1.0 - self.size) * window_width, window_height),
            SplitDirection::Horizontal => (window_width, (1.0 - self.size) * window_height),
        }
    }

    fn panel_bounds(&self, window_width: f32, window_height: f32) -> (f32, f32, f32, f32) {
        match self.split {
            SplitDirection::Vertical => {
                let x = (1.0 - self.size) * window_width;
                let width = self.size * window_width;
                (x, 0.0, width, window_height)
            }
            SplitDirection::Horizontal => {
                let y = (1.0 - self.size) * window_height;
                let height = self.size * window_height;
                (0.0, y, window_width, height)
            }
        }
    }
}

/// Extension trait for PreviewTab with additional utility methods.
pub trait PreviewTabExt {
    /// Get the language identifier for syntax highlighting.
    fn language(&self) -> Option<&str>;

    /// Check if this is a code file tab.
    fn is_code(&self) -> bool;

    /// Check if this is a markdown tab.
    fn is_markdown(&self) -> bool;

    /// Check if this is a PDF tab.
    fn is_pdf(&self) -> bool;
}

impl PreviewTabExt for PreviewTab {
    fn language(&self) -> Option<&str> {
        match self {
            PreviewTab::Code { language, .. } => Some(language),
            PreviewTab::Markdown { .. } => Some("markdown"),
            PreviewTab::Pdf { .. } => None,
            PreviewTab::Table { .. } => None,
        }
    }

    fn is_code(&self) -> bool {
        matches!(self, PreviewTab::Code { .. })
    }

    fn is_markdown(&self) -> bool {
        matches!(self, PreviewTab::Markdown { .. })
    }

    fn is_pdf(&self) -> bool {
        matches!(self, PreviewTab::Pdf { .. })
    }
}

/// Create a PreviewTab from a file path based on extension.
pub fn tab_from_path(path: PathBuf) -> Option<PreviewTab> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    match ext {
        "md" | "markdown" => {
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            Some(PreviewTab::Markdown {
                path,
                content,
                editing: false,
                editor: None,
                meta: crate::app::TabMeta::default(),
            })
        }
        "pdf" => Some(PreviewTab::Pdf {
            path,
            webview: None,
            meta: crate::app::TabMeta::default(),
        }),
        _ => {
            // Try to detect code files
            if let Some(language) = crate::types::language_from_extension(ext) {
                let content = std::fs::read_to_string(&path).unwrap_or_default();
                Some(PreviewTab::Code {
                    path,
                    language: language.to_string(),
                    content,
                    editing: true,
                    dirty: false,
                    editor: None,
                    meta: crate::app::TabMeta::default(),
                })
            } else {
                None
            }
        }
    }
}

/// Toggle split direction helper.
pub fn toggle_split(direction: SplitDirection) -> SplitDirection {
    match direction {
        SplitDirection::Vertical => SplitDirection::Horizontal,
        SplitDirection::Horizontal => SplitDirection::Vertical,
    }
}
