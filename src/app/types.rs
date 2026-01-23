//! Types and enums used by the Humanboard application.

use crate::pdf_webview::PdfWebView;
use gpui::Point;
use gpui::Pixels;
use gpui_component::input::InputState;
use gpui::Entity;
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// The current view state of the application
#[derive(Clone, Debug)]
pub enum AppView {
    /// Onboarding flow for new users
    Onboarding,
    /// Home screen with countdown timer
    Home,
    /// Landing page showing all boards
    Landing,
    /// Board canvas view
    Board(String), // Board ID
}

/// Direction of the preview panel split with the canvas
#[derive(Clone, Copy, PartialEq)]
pub enum SplitDirection {
    Vertical,   // Panel on the right
    Horizontal, // Panel on the bottom
}

/// Current mode of the command palette
#[derive(Clone, Copy, PartialEq, Default)]
pub enum CmdPaletteMode {
    #[default]
    Items, // Searching canvas items (includes tables by CSV name)
    Themes, // Selecting theme
}

/// Tab in the settings modal
#[derive(Clone, Copy, PartialEq, Default)]
pub enum SettingsTab {
    #[default]
    Appearance,
    Integrations,
}

/// Storage location options for boards
#[derive(Clone, Debug, PartialEq, Default)]
pub enum StorageLocation {
    /// Default application data directory
    #[default]
    Default,
    /// iCloud Drive for cross-device sync
    ICloud,
    /// Custom user-specified path
    Custom(std::path::PathBuf),
}

impl StorageLocation {
    /// Get the display name for this location
    pub fn display_name(&self) -> &str {
        match self {
            StorageLocation::Default => "Local (Application Support)",
            StorageLocation::ICloud => "iCloud Drive",
            StorageLocation::Custom(_) => "Custom Location",
        }
    }

    /// Get the base path for this storage location
    pub fn base_path(&self) -> Option<std::path::PathBuf> {
        match self {
            StorageLocation::Default => {
                dirs::data_dir().map(|p| p.join("humanboard").join("boards"))
            }
            StorageLocation::ICloud => {
                // macOS iCloud Drive path
                dirs::home_dir().map(|p| {
                    p.join("Library")
                        .join("Mobile Documents")
                        .join("com~apple~CloudDocs")
                        .join("Humanboard")
                })
            }
            StorageLocation::Custom(path) => Some(path.clone()),
        }
    }

    /// Check if this location is available
    pub fn is_available(&self) -> bool {
        match self {
            StorageLocation::Default => true,
            StorageLocation::ICloud => {
                // Check if iCloud Drive exists
                if let Some(path) = self.base_path() {
                    path.parent().map(|p| p.exists()).unwrap_or(false)
                } else {
                    false
                }
            }
            StorageLocation::Custom(path) => {
                path.exists() || path.parent().map(|p| p.exists()).unwrap_or(false)
            }
        }
    }
}

/// Tab metadata shared across all tab types
#[derive(Clone, Copy, Default)]
pub struct TabMeta {
    /// Preview tabs are temporary and get replaced by the next preview open
    pub is_preview: bool,
    /// Pinned tabs resist close operations and stay at the left
    pub is_pinned: bool,
}

/// A tab in the preview panel
pub enum PreviewTab {
    Pdf {
        path: PathBuf,
        webview: Option<PdfWebView>,
        meta: TabMeta,
    },
    Markdown {
        path: PathBuf,
        content: String,
        editing: bool,
        editor: Option<Entity<InputState>>,
        meta: TabMeta,
    },
    Code {
        path: PathBuf,
        language: String,
        content: String,
        editing: bool,
        dirty: bool,
        editor: Option<Entity<InputState>>,
        meta: TabMeta,
    },
    Table {
        /// The data source ID this table references
        data_source_id: u64,
        /// Display name (from CSV filename)
        name: String,
        /// Table state for gpui-component Table
        table_state: Option<gpui::Entity<gpui_component::table::TableState<crate::data::DataSourceDelegate>>>,
        meta: TabMeta,
    },
}

impl PreviewTab {
    pub fn path(&self) -> Option<&PathBuf> {
        match self {
            PreviewTab::Pdf { path, .. } => Some(path),
            PreviewTab::Markdown { path, .. } => Some(path),
            PreviewTab::Code { path, .. } => Some(path),
            PreviewTab::Table { .. } => None, // Tables don't have file paths
        }
    }

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
            PreviewTab::Pdf { path, .. } => path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Untitled")
                .to_string(),
            PreviewTab::Code { path, .. } => path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Untitled")
                .to_string(),
            PreviewTab::Table { name, .. } => name.clone(),
        }
    }

    pub fn is_editing(&self) -> bool {
        matches!(self, PreviewTab::Code { editing: true, .. })
    }

    pub fn is_dirty(&self) -> bool {
        matches!(self, PreviewTab::Code { dirty: true, .. })
    }

    /// Get tab metadata
    pub fn meta(&self) -> &TabMeta {
        match self {
            PreviewTab::Pdf { meta, .. } => meta,
            PreviewTab::Markdown { meta, .. } => meta,
            PreviewTab::Code { meta, .. } => meta,
            PreviewTab::Table { meta, .. } => meta,
        }
    }

    /// Get mutable tab metadata
    pub fn meta_mut(&mut self) -> &mut TabMeta {
        match self {
            PreviewTab::Pdf { meta, .. } => meta,
            PreviewTab::Markdown { meta, .. } => meta,
            PreviewTab::Code { meta, .. } => meta,
            PreviewTab::Table { meta, .. } => meta,
        }
    }

    /// Check if this is a preview (temporary) tab
    pub fn is_preview(&self) -> bool {
        self.meta().is_preview
    }

    /// Check if this tab is pinned
    pub fn is_pinned(&self) -> bool {
        self.meta().is_pinned
    }

    /// Clean up resources (webviews, editors) before dropping the tab.
    /// This should be called before removing a tab to prevent memory leaks.
    pub fn cleanup(&mut self, cx: &mut gpui::App) {
        match self {
            PreviewTab::Pdf { webview, .. } => {
                // Hide and drop the webview to release resources
                if let Some(wv) = webview.take() {
                    wv.hide(cx);
                    // Entity will be dropped when wv goes out of scope
                }
            }
            PreviewTab::Markdown { editor, .. } => {
                // Clear the editor entity
                *editor = None;
            }
            PreviewTab::Code { editor, .. } => {
                // Clear the editor entity
                *editor = None;
            }
            PreviewTab::Table { table_state, .. } => {
                // Clear the table state entity
                *table_state = None;
            }
        }
    }

    /// Convert preview tab to permanent
    pub fn make_permanent(&mut self) {
        self.meta_mut().is_preview = false;
    }

    /// Toggle pinned state
    pub fn toggle_pinned(&mut self) {
        let meta = self.meta_mut();
        meta.is_pinned = !meta.is_pinned;
        // Pinned tabs are never previews
        if meta.is_pinned {
            meta.is_preview = false;
        }
    }
}

/// Which pane is focused in split view
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum FocusedPane {
    #[default]
    Left,
    Right,
}

/// Drop zone for dragging tabs to create splits
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SplitDropZone {
    Left,
    Right,
    Top,
    Bottom,
}

/// State for the preview panel with tab management
pub struct PreviewPanel {
    // Left/primary pane
    pub tabs: Vec<PreviewTab>,
    pub active_tab: usize,
    pub back_stack: Vec<usize>,
    pub forward_stack: Vec<usize>,
    pub closed_tabs: Vec<PreviewTab>,

    // Right/secondary pane (only used when is_pane_split is true)
    pub right_tabs: Vec<PreviewTab>,
    pub right_active_tab: usize,
    pub right_back_stack: Vec<usize>,
    pub right_forward_stack: Vec<usize>,

    // Split state
    pub split: SplitDirection, // Split with canvas (vertical/horizontal)
    pub size: f32,             // Panel size (0.0 to 1.0)
    pub is_pane_split: bool,   // Whether panel itself is split into two panes
    pub pane_split_horizontal: bool, // True = top/bottom, False = left/right
    pub focused_pane: FocusedPane, // Which pane has focus
    pub pane_ratio: f32,       // Ratio between left/right panes (0.5 = equal)
}

impl PreviewPanel {
    pub fn new(split: SplitDirection, size: f32) -> Self {
        Self {
            tabs: Vec::new(),
            active_tab: 0,
            back_stack: Vec::new(),
            forward_stack: Vec::new(),
            closed_tabs: Vec::new(),

            right_tabs: Vec::new(),
            right_active_tab: 0,
            right_back_stack: Vec::new(),
            right_forward_stack: Vec::new(),

            split,
            size,
            is_pane_split: false,
            pane_split_horizontal: false,
            focused_pane: FocusedPane::Left,
            pane_ratio: 0.5,
        }
    }

    /// Clean up all resources before destroying the panel.
    /// This should be called before dropping the PreviewPanel to prevent memory leaks.
    pub fn cleanup(&mut self, cx: &mut gpui::App) {
        // Clean up all tabs in the left pane
        for tab in &mut self.tabs {
            tab.cleanup(cx);
        }
        // Clean up all tabs in the right pane
        for tab in &mut self.right_tabs {
            tab.cleanup(cx);
        }
        // Clean up all closed tabs
        for tab in &mut self.closed_tabs {
            tab.cleanup(cx);
        }
    }
}

/// Animation state for smooth panning to a target position
pub struct PanAnimation {
    pub start_offset: Point<Pixels>,
    pub target_offset: Point<Pixels>,
    pub start_time: Instant,
    pub duration: Duration,
}

/// State for the countdown timer on the home screen
#[derive(Clone, Debug)]
pub struct CountdownState {
    /// Target time to count down to
    pub target: std::time::SystemTime,
    /// Label/title for the countdown
    pub label: String,
    /// Whether the countdown is active
    pub active: bool,
}

impl CountdownState {
    /// Create a new countdown to a target time
    pub fn new(target: std::time::SystemTime, label: impl Into<String>) -> Self {
        Self {
            target,
            label: label.into(),
            active: true,
        }
    }

    /// Create a countdown to midnight tonight
    pub fn until_midnight() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};

        // Get current time
        let now = SystemTime::now();
        let duration_since_epoch = now.duration_since(UNIX_EPOCH).unwrap_or_default();
        let secs = duration_since_epoch.as_secs();

        // Calculate seconds until midnight (next day start)
        let secs_per_day = 24 * 60 * 60;
        let secs_today = secs % secs_per_day;
        let secs_until_midnight = secs_per_day - secs_today;

        let target = now + Duration::from_secs(secs_until_midnight);

        Self::new(target, "Until Midnight")
    }

    /// Get remaining time as (hours, minutes, seconds)
    pub fn remaining(&self) -> Option<(u64, u64, u64)> {
        let now = std::time::SystemTime::now();
        if let Ok(remaining) = self.target.duration_since(now) {
            let total_secs = remaining.as_secs();
            let hours = total_secs / 3600;
            let minutes = (total_secs % 3600) / 60;
            let seconds = total_secs % 60;
            Some((hours, minutes, seconds))
        } else {
            None // Countdown finished
        }
    }

    /// Check if countdown has finished
    pub fn is_finished(&self) -> bool {
        self.remaining().is_none()
    }
}
