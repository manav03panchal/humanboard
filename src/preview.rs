//! Preview Panel Module
//!
//! This module manages the preview panel for viewing and editing files,
//! following Zed's pattern of separating panel concerns from the main app.
//!
//! ## Features
//!
//! - **Tab Management**: Multiple file tabs with switching
//! - **Split Views**: Horizontal/vertical split with canvas
//! - **File Types**: PDF, Markdown, Code files
//! - **Editing**: Inline editing for markdown and code

use crate::pdf_webview::PdfWebView;
use gpui::*;
use gpui_component::input::InputState;
use std::path::PathBuf;

/// Direction for splitting the preview panel with the canvas.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum SplitDirection {
    /// Panel on the right side
    #[default]
    Vertical,
    /// Panel on the bottom
    Horizontal,
}

impl SplitDirection {
    /// Toggle between vertical and horizontal split.
    pub fn toggle(&self) -> Self {
        match self {
            SplitDirection::Vertical => SplitDirection::Horizontal,
            SplitDirection::Horizontal => SplitDirection::Vertical,
        }
    }
}

/// A tab in the preview panel.
pub enum PreviewTab {
    /// PDF document
    Pdf {
        path: PathBuf,
        webview: Option<PdfWebView>,
    },
    /// Markdown file with optional editing
    Markdown {
        path: PathBuf,
        content: String,
        editing: bool,
        editor: Option<Entity<InputState>>,
    },
    /// Code file with syntax highlighting
    Code {
        path: PathBuf,
        language: String,
        content: String,
        editing: bool,
        dirty: bool,
        editor: Option<Entity<InputState>>,
    },
}

impl PreviewTab {
    /// Create a new PDF tab.
    pub fn pdf(path: PathBuf) -> Self {
        Self::Pdf { path, webview: None }
    }

    /// Create a new Markdown tab.
    pub fn markdown(path: PathBuf, content: String) -> Self {
        Self::Markdown {
            path,
            content,
            editing: false,
            editor: None,
        }
    }

    /// Create a new Code tab.
    pub fn code(path: PathBuf, language: String, content: String) -> Self {
        Self::Code {
            path,
            language,
            content,
            editing: true, // Code is always editable
            dirty: false,
            editor: None,
        }
    }

    /// Get the file path for this tab.
    pub fn path(&self) -> &PathBuf {
        match self {
            PreviewTab::Pdf { path, .. } => path,
            PreviewTab::Markdown { path, .. } => path,
            PreviewTab::Code { path, .. } => path,
        }
    }

    /// Get a display title for this tab.
    pub fn title(&self) -> String {
        match self {
            PreviewTab::Markdown { content, path, .. } => {
                // Try to extract title from first line (# Title)
                if let Some(first_line) = content.lines().next() {
                    if first_line.starts_with("# ") {
                        return first_line.trim_start_matches("# ").to_string();
                    }
                }
                // Fallback to filename without extension
                path.file_stem()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Untitled")
                    .to_string()
            }
            PreviewTab::Pdf { path, .. } | PreviewTab::Code { path, .. } => path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Untitled")
                .to_string(),
        }
    }

    /// Check if this tab is in editing mode.
    pub fn is_editing(&self) -> bool {
        match self {
            PreviewTab::Markdown { editing, .. } => *editing,
            PreviewTab::Code { editing, .. } => *editing,
            PreviewTab::Pdf { .. } => false,
        }
    }

    /// Check if this tab has unsaved changes.
    pub fn is_dirty(&self) -> bool {
        match self {
            PreviewTab::Code { dirty, .. } => *dirty,
            _ => false,
        }
    }

    /// Get the language identifier for code tabs.
    pub fn language(&self) -> Option<&str> {
        match self {
            PreviewTab::Code { language, .. } => Some(language),
            PreviewTab::Markdown { .. } => Some("markdown"),
            PreviewTab::Pdf { .. } => None,
        }
    }
}

/// Preview panel state and logic.
pub struct PreviewPanel {
    /// Open tabs
    pub tabs: Vec<PreviewTab>,
    /// Currently active tab index
    pub active_tab: usize,
    /// Split direction with canvas
    pub split: SplitDirection,
    /// Panel size as percentage of window (0.0 to 1.0)
    pub size: f32,
    /// Scroll handle for tab bar
    pub tab_scroll: ScrollHandle,
}

impl PreviewPanel {
    /// Create a new preview panel with an initial tab.
    pub fn new(tab: PreviewTab) -> Self {
        Self {
            tabs: vec![tab],
            active_tab: 0,
            split: SplitDirection::default(),
            size: 0.4,
            tab_scroll: ScrollHandle::new(),
        }
    }

    /// Create a new preview panel with default settings but no tabs.
    pub fn empty() -> Self {
        Self {
            tabs: Vec::new(),
            active_tab: 0,
            split: SplitDirection::default(),
            size: 0.4,
            tab_scroll: ScrollHandle::new(),
        }
    }

    /// Check if the panel has any tabs.
    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }

    /// Get the currently active tab.
    pub fn active_tab(&self) -> Option<&PreviewTab> {
        self.tabs.get(self.active_tab)
    }

    /// Get the currently active tab mutably.
    pub fn active_tab_mut(&mut self) -> Option<&mut PreviewTab> {
        self.tabs.get_mut(self.active_tab)
    }

    /// Find a tab by file path.
    pub fn find_tab(&self, path: &PathBuf) -> Option<usize> {
        self.tabs.iter().position(|t| t.path() == path)
    }

    /// Open a file in the preview panel.
    /// If the file is already open, switch to that tab.
    /// Otherwise, add a new tab.
    pub fn open(&mut self, tab: PreviewTab) {
        let path = tab.path().clone();

        // Check if already open
        if let Some(index) = self.find_tab(&path) {
            self.active_tab = index;
        } else {
            self.tabs.push(tab);
            self.active_tab = self.tabs.len() - 1;
        }
    }

    /// Switch to a specific tab.
    pub fn switch_to(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.active_tab = index;
        }
    }

    /// Switch to the next tab.
    pub fn next_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.active_tab = (self.active_tab + 1) % self.tabs.len();
        }
    }

    /// Switch to the previous tab.
    pub fn prev_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.active_tab = if self.active_tab == 0 {
                self.tabs.len() - 1
            } else {
                self.active_tab - 1
            };
        }
    }

    /// Close a specific tab.
    /// Returns true if the panel should be closed (no tabs left).
    pub fn close_tab(&mut self, index: usize) -> bool {
        if index < self.tabs.len() {
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

    /// Close the currently active tab.
    /// Returns true if the panel should be closed.
    pub fn close_active_tab(&mut self) -> bool {
        self.close_tab(self.active_tab)
    }

    /// Toggle the split direction.
    pub fn toggle_split(&mut self) {
        self.split = self.split.toggle();
    }

    /// Set the panel size as a percentage (clamped to 0.2-0.8).
    pub fn set_size(&mut self, size: f32) {
        self.size = size.clamp(0.2, 0.8);
    }

    /// Calculate the canvas bounds given window size.
    pub fn canvas_bounds(&self, window_width: f32, window_height: f32) -> (f32, f32) {
        match self.split {
            SplitDirection::Vertical => ((1.0 - self.size) * window_width, window_height),
            SplitDirection::Horizontal => (window_width, (1.0 - self.size) * window_height),
        }
    }

    /// Calculate the panel bounds given window size.
    pub fn panel_bounds(&self, window_width: f32, window_height: f32) -> (f32, f32, f32, f32) {
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

/// Create a PreviewTab from a file path.
pub fn tab_from_path(path: PathBuf) -> Option<PreviewTab> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    match ext {
        "md" | "markdown" => {
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            Some(PreviewTab::markdown(path, content))
        }
        "pdf" => Some(PreviewTab::pdf(path)),
        _ => {
            // Try to detect code files
            if let Some(language) = crate::types::language_from_extension(ext) {
                let content = std::fs::read_to_string(&path).unwrap_or_default();
                Some(PreviewTab::code(path, language.to_string(), content))
            } else {
                None
            }
        }
    }
}
