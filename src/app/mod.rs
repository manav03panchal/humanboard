//! Application module - the main Humanboard application state and logic.

mod types;
mod board_management;
mod settings_methods;
mod command_palette_methods;

pub use types::*;

use crate::audio_webview::AudioWebView;
use crate::background::BackgroundExecutor;
use crate::board::Board;
use crate::board_index::BoardIndex;
use crate::focus::{FocusContext, FocusManager};
use crate::hit_testing::HitTester;
use crate::notifications::{Toast, ToastManager};
use crate::pdf_webview::PdfWebView;
use crate::perf::PerfMonitor;
use crate::settings::Settings;
use crate::settings_watcher::{SettingsEvent, SettingsWatcher};
use crate::types::{ItemContent, ToolType};
use crate::video_webview::VideoWebView;
use crate::youtube_webview::YouTubeWebView;
use gpui::*;
use gpui_component::input::InputState;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};
use tracing::error;

pub struct Humanboard {
    // View state
    pub view: AppView,
    pub board_index: BoardIndex,

    // Landing page state
    pub editing_board_id: Option<String>,
    pub edit_input: Option<Entity<InputState>>,
    pub deleting_board_id: Option<String>,

    // Board creation modal state
    pub show_create_board_modal: bool,
    pub create_board_input: Option<Entity<InputState>>,
    pub create_board_location: StorageLocation,
    pub create_board_backdrop_clicked: bool,

    // Trash visibility on landing page
    pub show_trash: bool,

    // Board state (only populated when view is Board)
    pub board: Option<Board>,
    pub dragging: bool,
    pub last_mouse_pos: Option<Point<Pixels>>,
    pub dragging_item: Option<u64>,
    pub item_drag_offset: Option<Point<Pixels>>,
    pub resizing_item: Option<u64>,
    pub resize_start_size: Option<(f32, f32)>,
    pub resize_start_pos: Option<Point<Pixels>>,
    pub resize_start_font_size: Option<f32>,
    pub selected_items: HashSet<u64>,

    // Marquee selection state
    pub marquee_start: Option<Point<Pixels>>,
    pub marquee_current: Option<Point<Pixels>>,

    pub frame_times: Vec<Duration>,
    pub last_frame: Instant,
    pub frame_count: u64,
    /// Focus manager for handling focus across different contexts
    pub focus: FocusManager,
    pub preview: Option<PreviewPanel>,
    pub dragging_tab: Option<usize>,    // Index of tab being dragged
    pub tab_drag_target: Option<usize>, // Target position for tab drop
    pub tab_drag_split_zone: Option<SplitDropZone>, // Drop zone for creating split
    pub tab_drag_position: Option<Point<Pixels>>, // Current drag position for ghost
    pub preview_search: Option<Entity<InputState>>, // Search input for preview panel
    pub preview_search_query: String,   // Current search query
    pub preview_search_matches: Vec<(usize, usize)>, // (line, column) positions of matches
    pub preview_search_current: usize,  // Current match index
    pub dragging_splitter: bool,
    pub splitter_drag_start: Option<Point<Pixels>>,
    pub dragging_pane_splitter: bool, // Dragging the splitter between split panes
    pub last_drop_pos: Option<Point<Pixels>>,
    pub file_drop_rx: Option<Receiver<(Point<Pixels>, Vec<PathBuf>)>>,

    // UI overlays
    pub show_shortcuts: bool,
    pub command_palette: Option<Entity<InputState>>, // Command palette input
    pub pending_command: Option<String>, // Command to execute (deferred until we have window access)
    pub search_results: Vec<(u64, String)>, // Search results: (item_id, display_name)
    pub selected_result: usize,          // Currently selected search result index
    pub cmd_palette_mode: CmdPaletteMode, // Current mode: items or themes

    // YouTube WebViews (keyed by item ID)
    pub youtube_webviews: HashMap<u64, YouTubeWebView>,

    // Audio WebViews (keyed by item ID)
    pub audio_webviews: HashMap<u64, AudioWebView>,

    // Video WebViews (keyed by item ID)
    pub video_webviews: HashMap<u64, VideoWebView>,

    // Settings
    pub settings: Settings,
    pub show_settings: bool,
    pub settings_backdrop_clicked: bool,
    pub settings_tab: SettingsTab,
    pub settings_theme_index: usize,
    pub settings_theme_scroll: ScrollHandle,
    pub settings_font_index: usize,
    pub settings_font_scroll: ScrollHandle,

    // Toast notifications
    pub toast_manager: ToastManager,

    // Preview tab scroll handles (left/right for split pane)
    pub preview_tab_scroll: ScrollHandle,
    pub preview_right_tab_scroll: ScrollHandle,

    // Command palette scroll handle
    pub cmd_palette_scroll: ScrollHandle,

    // Pan animation state
    pub pan_animation: Option<PanAnimation>,

    // Tool dock state
    pub selected_tool: ToolType,
    pub drawing_start: Option<Point<Pixels>>, // Start position for drawing shapes/arrows
    pub drawing_current: Option<Point<Pixels>>, // Current position while drawing (for preview)
    pub editing_textbox_id: Option<u64>,      // ID of textbox being edited
    pub textbox_input: Option<Entity<gpui_component::input::InputState>>, // Input for editing textbox
    pub pending_textbox_drag: Option<(u64, Point<Pixels>)>, // Deferred drag for textboxes (to allow double-click)

    // Hit testing
    pub hit_tester: HitTester,

    // Performance monitoring
    pub perf_monitor: PerfMonitor,

    // Background task executor
    pub background: BackgroundExecutor,

    // Settings file watcher for hot-reload
    pub settings_watcher: Option<SettingsWatcher>,
}

impl Humanboard {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let board_index = BoardIndex::load();

        Self {
            view: AppView::Landing,
            board_index,
            editing_board_id: None,
            edit_input: None,
            deleting_board_id: None,
            show_create_board_modal: false,
            create_board_input: None,
            create_board_location: StorageLocation::default(),
            create_board_backdrop_clicked: false,
            show_trash: false,
            board: None,
            dragging: false,
            last_mouse_pos: None,
            dragging_item: None,
            item_drag_offset: None,
            resizing_item: None,
            resize_start_size: None,
            resize_start_pos: None,
            resize_start_font_size: None,
            selected_items: HashSet::new(),
            marquee_start: None,
            marquee_current: None,
            frame_times: Vec::with_capacity(60),
            last_frame: Instant::now(),
            frame_count: 0,
            focus: FocusManager::new(cx),
            preview: None,
            dragging_tab: None,
            tab_drag_target: None,
            tab_drag_split_zone: None,
            tab_drag_position: None,
            preview_search: None,
            preview_search_query: String::new(),
            preview_search_matches: Vec::new(),
            preview_search_current: 0,
            dragging_splitter: false,
            splitter_drag_start: None,
            dragging_pane_splitter: false,
            last_drop_pos: None,
            file_drop_rx: None,
            show_shortcuts: false,
            command_palette: None,
            pending_command: None,
            search_results: Vec::new(),
            selected_result: 0,
            cmd_palette_mode: CmdPaletteMode::default(),
            youtube_webviews: HashMap::new(),
            audio_webviews: HashMap::new(),
            video_webviews: HashMap::new(),

            settings: Settings::load(),
            show_settings: false,
            settings_backdrop_clicked: false,
            settings_tab: SettingsTab::default(),
            settings_theme_index: 0,
            settings_theme_scroll: ScrollHandle::new(),
            settings_font_index: 0,
            settings_font_scroll: ScrollHandle::new(),

            toast_manager: ToastManager::new(),
            preview_tab_scroll: ScrollHandle::new(),
            preview_right_tab_scroll: ScrollHandle::new(),
            cmd_palette_scroll: ScrollHandle::new(),
            pan_animation: None,
            selected_tool: ToolType::default(),
            drawing_start: None,
            drawing_current: None,
            editing_textbox_id: None,
            textbox_input: None,
            pending_textbox_drag: None,
            hit_tester: HitTester::new(),
            perf_monitor: PerfMonitor::new(),
            background: BackgroundExecutor::with_default_workers(),
            settings_watcher: crate::settings_watcher::default_settings_path()
                .and_then(|p| SettingsWatcher::new(p).ok()),
        }
    }

    /// Check for settings file changes and reload if needed.
    pub fn check_settings_reload(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut watcher) = self.settings_watcher {
            if let Some(event) = watcher.poll() {
                match event {
                    SettingsEvent::Modified | SettingsEvent::Created => {
                        tracing::info!("Settings file changed, reloading...");
                        // Reload settings
                        self.settings = Settings::load();
                        self.toast_manager.push(Toast::info("Settings reloaded"));
                        cx.notify();
                    }
                    SettingsEvent::Deleted => {
                        tracing::warn!("Settings file deleted");
                        self.toast_manager
                            .push(Toast::warning("Settings file deleted"));
                    }
                    SettingsEvent::Error(e) => {
                        tracing::error!("Settings watch error: {}", e);
                    }
                }
            }
        }
    }

    /// Returns true if a code editor is currently in edit mode
    pub fn is_code_editing(&self) -> bool {
        self.preview
            .as_ref()
            .and_then(|p| p.tabs.get(p.active_tab))
            .map(|tab| tab.is_editing())
            .unwrap_or(false)
    }

    // ==================== Board Methods (Preview Panel) ====================

    /// Open a file in the preview panel as a permanent tab
    pub fn open_preview(&mut self, path: PathBuf, window: &mut Window, cx: &mut Context<Self>) {
        self.open_preview_internal(path, false, window, cx);
    }

    /// Open a file as a preview (temporary) tab - will replace existing preview tab
    pub fn open_as_preview_tab(
        &mut self,
        path: PathBuf,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_preview_internal(path, true, window, cx);
    }

    /// Internal method to open preview with preview/permanent mode option
    fn open_preview_internal(
        &mut self,
        path: PathBuf,
        as_preview: bool,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Determine tab type based on extension
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        let meta = TabMeta {
            is_preview: as_preview,
            is_pinned: false,
        };

        let tab = if ext == "md" {
            // Load markdown content
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            // Create editor immediately for edit mode
            let content_clone = content.clone();
            let editor = Some(cx.new(|cx| {
                InputState::new(_window, cx)
                    .code_editor("markdown")
                    .soft_wrap(true)
                    .line_number(true)
                    .default_value(content_clone)
            }));
            PreviewTab::Markdown {
                path: path.clone(),
                content,
                editing: true, // Open in edit mode
                editor,
                meta,
            }
        } else if ext == "pdf" {
            PreviewTab::Pdf {
                path: path.clone(),
                webview: None,
                meta,
            }
        } else if let Some(language) = crate::types::language_from_extension(ext) {
            // Code file - load content
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            PreviewTab::Code {
                path: path.clone(),
                language: language.to_string(),
                content,
                editing: true, // Always editable
                dirty: false,
                editor: None,
                meta,
            }
        } else {
            // Default to PDF for unknown types (or could be Text)
            PreviewTab::Pdf {
                path: path.clone(),
                webview: None,
                meta,
            }
        };

        if let Some(ref mut preview) = self.preview {
            // Check if file is already open in left pane
            if let Some(index) = preview.tabs.iter().position(|t| t.path() == &path) {
                // File already open - just switch to it and make permanent if not preview mode
                if !as_preview {
                    preview.tabs[index].make_permanent();
                }
                preview.active_tab = index;
                preview.focused_pane = FocusedPane::Left;
                cx.notify();
                return;
            }
            // Check if file is already open in right pane (when split)
            if preview.is_pane_split {
                if let Some(index) = preview.right_tabs.iter().position(|t| t.path() == &path) {
                    if !as_preview {
                        preview.right_tabs[index].make_permanent();
                    }
                    preview.right_active_tab = index;
                    preview.focused_pane = FocusedPane::Right;
                    cx.notify();
                    return;
                }
            }
            // File not open yet - add it
            if as_preview {
                // Preview mode: replace existing preview tab if one exists
                if let Some(preview_idx) = preview.tabs.iter().position(|t| t.is_preview()) {
                    preview.tabs[preview_idx] = tab;
                    preview.active_tab = preview_idx;
                } else {
                    // No existing preview tab, add new one
                    preview.tabs.push(tab);
                    preview.active_tab = preview.tabs.len() - 1;
                }
            } else {
                // Permanent mode: add new tab
                preview.tabs.push(tab);
                preview.active_tab = preview.tabs.len() - 1;
            }
        } else {
            // Create new preview panel with first tab
            let mut panel = PreviewPanel::new(SplitDirection::Vertical, 0.4);
            panel.tabs.push(tab);
            self.preview = Some(panel);
        }
        cx.notify();
    }

    pub fn ensure_pdf_webview(&mut self, window: &mut Window, cx: &mut App) {
        if let Some(ref mut preview) = self.preview {
            let active_tab = preview.active_tab;
            let right_active_tab = preview.right_active_tab;
            let is_pane_split = preview.is_pane_split;
            let is_horizontal_split = preview.pane_split_horizontal;

            // Calculate preview panel bounds
            let bounds = window.bounds();
            let window_width = f32::from(bounds.size.width);
            let window_height = f32::from(bounds.size.height);

            // Preview panel position and size
            let header_height = 40.0;
            let footer_height = 28.0;
            let dock_width = 40.0; // Tool dock on left
            let tab_bar_height = 36.0;

            let (panel_x, panel_y, panel_width, panel_height) = match preview.split {
                SplitDirection::Vertical => {
                    let panel_x = dock_width + (window_width - dock_width) * (1.0 - preview.size);
                    let panel_y = header_height;
                    let panel_width = (window_width - dock_width) * preview.size;
                    let panel_height = window_height - header_height - footer_height;
                    (panel_x, panel_y, panel_width, panel_height)
                }
                SplitDirection::Horizontal => {
                    let panel_x = dock_width;
                    let panel_y = header_height
                        + (window_height - header_height - footer_height) * (1.0 - preview.size);
                    let panel_width = window_width - dock_width;
                    let panel_height =
                        (window_height - header_height - footer_height) * preview.size;
                    (panel_x, panel_y, panel_width, panel_height)
                }
            };

            // Calculate pane bounds (left/top pane and right/bottom pane)
            let pane_ratio = preview.pane_ratio;
            let splitter_size = 6.0; // Splitter handle size
            let (
                left_pane_x,
                left_pane_y,
                left_pane_w,
                left_pane_h,
                right_pane_x,
                right_pane_y,
                right_pane_w,
                right_pane_h,
            ) = if is_pane_split {
                if is_horizontal_split {
                    // Top/Bottom split - use pane_ratio for heights
                    let available_height = panel_height - tab_bar_height * 2.0 - splitter_size;
                    let first_pane_height = available_height * pane_ratio;
                    let second_pane_height = available_height * (1.0 - pane_ratio);
                    (
                        panel_x,
                        panel_y + tab_bar_height,
                        panel_width,
                        first_pane_height,
                        panel_x,
                        panel_y
                            + tab_bar_height
                            + first_pane_height
                            + splitter_size
                            + tab_bar_height,
                        panel_width,
                        second_pane_height,
                    )
                } else {
                    // Left/Right split - use pane_ratio for widths
                    let available_width = panel_width - splitter_size;
                    let first_pane_width = available_width * pane_ratio;
                    let second_pane_width = available_width * (1.0 - pane_ratio);
                    (
                        panel_x,
                        panel_y + tab_bar_height,
                        first_pane_width,
                        panel_height - tab_bar_height,
                        panel_x + first_pane_width + splitter_size,
                        panel_y + tab_bar_height,
                        second_pane_width,
                        panel_height - tab_bar_height,
                    )
                }
            } else {
                // Single pane
                (
                    panel_x,
                    panel_y + tab_bar_height,
                    panel_width,
                    panel_height - tab_bar_height,
                    0.0,
                    0.0,
                    0.0,
                    0.0, // Right pane not used
                )
            };

            // Ensure all PDF tabs in left pane have their WebViews created and positioned
            for (idx, tab) in preview.tabs.iter_mut().enumerate() {
                if let PreviewTab::Pdf { path, webview, .. } = tab {
                    if webview.is_none() {
                        match PdfWebView::new(path.clone(), window, cx) {
                            Ok(wv) => {
                                if idx != active_tab {
                                    wv.hide(cx);
                                }
                                *webview = Some(wv);
                            }
                            Err(e) => {
                                error!("Failed to create PDF WebView: {}", e);
                            }
                        }
                    }

                    if let Some(wv) = webview {
                        if idx == active_tab {
                            wv.set_bounds(left_pane_x, left_pane_y, left_pane_w, left_pane_h, cx);
                            wv.show(cx);
                        } else {
                            wv.hide(cx);
                        }
                    }
                }
            }

            // Ensure all PDF tabs in right pane have their WebViews created and positioned (when split)
            if is_pane_split {
                for (idx, tab) in preview.right_tabs.iter_mut().enumerate() {
                    if let PreviewTab::Pdf { path, webview, .. } = tab {
                        if webview.is_none() {
                            match PdfWebView::new(path.clone(), window, cx) {
                                Ok(wv) => {
                                    if idx != right_active_tab {
                                        wv.hide(cx);
                                    }
                                    *webview = Some(wv);
                                }
                                Err(e) => {
                                    error!("Failed to create PDF WebView for right pane: {}", e);
                                }
                            }
                        }

                        if let Some(wv) = webview {
                            if idx == right_active_tab {
                                wv.set_bounds(
                                    right_pane_x,
                                    right_pane_y,
                                    right_pane_w,
                                    right_pane_h,
                                    cx,
                                );
                                wv.show(cx);
                            } else {
                                wv.hide(cx);
                            }
                        }
                    }
                }
            }
        }
    }

    /// Ensure code editors are created for code tabs (for syntax-highlighted viewing)
    pub fn ensure_code_editors(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            let is_split = preview.is_pane_split;

            // Ensure code editors for left pane
            for tab in preview.tabs.iter_mut() {
                if let PreviewTab::Code {
                    content,
                    language,
                    editor,
                    dirty,
                    ..
                } = tab
                {
                    if editor.is_none() {
                        // Create editor with syntax highlighting
                        let content_clone = content.clone();
                        let lang = language.clone();
                        *editor = Some(cx.new(|cx| {
                            InputState::new(_window, cx)
                                .code_editor(lang)
                                .line_number(true)
                                .default_value(content_clone)
                        }));
                    } else if let Some(ed) = editor {
                        // Check if content changed (for dirty indicator)
                        let editor_content = ed.read(cx).text().to_string();
                        *dirty = editor_content != *content;
                    }
                }
            }

            // Ensure code editors for right pane (when split)
            if is_split {
                for tab in preview.right_tabs.iter_mut() {
                    if let PreviewTab::Code {
                        content,
                        language,
                        editor,
                        dirty,
                        ..
                    } = tab
                    {
                        if editor.is_none() {
                            let content_clone = content.clone();
                            let lang = language.clone();
                            *editor = Some(cx.new(|cx| {
                                InputState::new(_window, cx)
                                    .code_editor(lang)
                                    .line_number(true)
                                    .default_value(content_clone)
                            }));
                        } else if let Some(ed) = editor {
                            let editor_content = ed.read(cx).text().to_string();
                            *dirty = editor_content != *content;
                        }
                    }
                }
            }
        }
    }

    // Markdown editing methods
    pub fn toggle_markdown_edit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if let Some(tab) = preview.tabs.get_mut(preview.active_tab) {
                if let PreviewTab::Markdown {
                    editing,
                    content,
                    editor,
                    ..
                } = tab
                {
                    *editing = !*editing;
                    if *editing {
                        // Set focus context to Preview for editor input
                        self.focus.focus(FocusContext::Preview, window);

                        if editor.is_none() {
                            // Create editor with current content - use code_editor for multiline support
                            let content_clone = content.clone();
                            *editor = Some(cx.new(|cx| {
                                InputState::new(window, cx)
                                    .code_editor("markdown")
                                    .soft_wrap(true)
                                    .line_number(true)
                                    .default_value(content_clone)
                            }));
                        }
                        // Focus the editor so user can type immediately
                        if let Some(ed) = editor {
                            ed.update(cx, |state, cx| {
                                state.focus(window, cx);
                            });
                        }
                    } else {
                        // Release focus back to canvas when exiting edit mode
                        self.focus.release(FocusContext::Preview, window);
                    }
                }
            }
        }
        cx.notify();
    }

    pub fn save_markdown(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if let Some(tab) = preview.tabs.get_mut(preview.active_tab) {
                if let PreviewTab::Markdown {
                    path,
                    content,
                    editor,
                    editing,
                    ..
                } = tab
                {
                    if let Some(ed) = editor {
                        let new_content = ed.read(cx).text().to_string();
                        *content = new_content.clone();
                        // Save to file
                        let path_clone = path.clone();
                        let _ = std::fs::write(&path_clone, &new_content);
                        // Also update board item if exists
                        if let Some(ref mut board) = self.board {
                            for item in board.items.iter_mut() {
                                if let crate::types::ItemContent::Markdown {
                                    path: item_path,
                                    content: item_content,
                                    ..
                                } = &mut item.content
                                {
                                    if *item_path == path_clone {
                                        *item_content = new_content.clone();
                                    }
                                }
                            }
                        }
                        *editing = false;
                    }
                }
            }
        }
        cx.notify();
    }

    pub fn save_code(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if let Some(tab) = preview.tabs.get_mut(preview.active_tab) {
                match tab {
                    PreviewTab::Code {
                        path,
                        content,
                        editor,
                        dirty,
                        ..
                    } => {
                        if let Some(ed) = editor {
                            let new_content = ed.read(cx).text().to_string();
                            *content = new_content.clone();
                            // Save to file
                            let path_clone = path.clone();
                            if let Err(e) = std::fs::write(&path_clone, &new_content) {
                                error!("Failed to save code file: {}", e);
                            }
                            *dirty = false;
                            // Keep editor for viewing
                        }
                    }
                    PreviewTab::Markdown {
                        path,
                        content,
                        editor,
                        editing,
                        ..
                    } => {
                        // Also handle Cmd+S for markdown files
                        if *editing {
                            if let Some(ed) = editor {
                                let new_content = ed.read(cx).text().to_string();
                                *content = new_content.clone();
                                // Save to file
                                let path_clone = path.clone();
                                let _ = std::fs::write(&path_clone, &new_content);
                                // Also update board item if exists
                                if let Some(ref mut board) = self.board {
                                    for item in board.items.iter_mut() {
                                        if let crate::types::ItemContent::Markdown {
                                            path: item_path,
                                            content: item_content,
                                            ..
                                        } = &mut item.content
                                        {
                                            if *item_path == path_clone {
                                                *item_content = new_content.clone();
                                            }
                                        }
                                    }
                                }
                                *editing = false; // Exit edit mode to show preview
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        cx.notify();
    }

    pub fn ensure_youtube_webviews(&mut self, window: &mut Window, cx: &mut App) {
        use crate::types::ItemContent;

        let Some(ref board) = self.board else {
            self.youtube_webviews.clear();
            return;
        };

        // Collect YouTube item IDs and video IDs
        let youtube_items: Vec<(u64, String)> = board
            .items
            .iter()
            .filter_map(|item| {
                if let ItemContent::YouTube(video_id) = &item.content {
                    Some((item.id, video_id.clone()))
                } else {
                    None
                }
            })
            .collect();

        // Create WebViews for new YouTube items
        for (item_id, video_id) in &youtube_items {
            if !self.youtube_webviews.contains_key(item_id) {
                match YouTubeWebView::new(video_id.clone(), window, cx) {
                    Ok(webview) => {
                        self.youtube_webviews.insert(*item_id, webview);
                    }
                    Err(e) => {
                        error!(
                            "Failed to create YouTube WebView for video {}: {}",
                            video_id, e
                        );
                    }
                }
            }
        }

        // Remove WebViews for deleted items
        let youtube_ids: std::collections::HashSet<u64> =
            youtube_items.iter().map(|(id, _)| *id).collect();
        self.youtube_webviews
            .retain(|id, _| youtube_ids.contains(id));
    }

    pub fn ensure_audio_webviews(&mut self, window: &mut Window, cx: &mut App) {
        use crate::types::ItemContent;

        let Some(ref board) = self.board else {
            self.audio_webviews.clear();
            return;
        };

        // Collect Audio item IDs and paths
        let audio_items: Vec<(u64, std::path::PathBuf)> = board
            .items
            .iter()
            .filter_map(|item| {
                if let ItemContent::Audio(path) = &item.content {
                    Some((item.id, path.clone()))
                } else {
                    None
                }
            })
            .collect();

        // Create WebViews for new Audio items
        for (item_id, path) in &audio_items {
            if !self.audio_webviews.contains_key(item_id) {
                match AudioWebView::new(path.clone(), window, cx) {
                    Ok(webview) => {
                        self.audio_webviews.insert(*item_id, webview);
                    }
                    Err(e) => {
                        error!("Failed to create Audio WebView for {:?}: {}", path, e);
                    }
                }
            }
        }

        // Remove WebViews for deleted items
        let audio_ids: std::collections::HashSet<u64> =
            audio_items.iter().map(|(id, _)| *id).collect();
        self.audio_webviews.retain(|id, _| audio_ids.contains(id));
    }

    pub fn ensure_video_webviews(&mut self, window: &mut Window, cx: &mut App) {
        use crate::types::ItemContent;

        let Some(ref board) = self.board else {
            self.video_webviews.clear();
            return;
        };

        // Collect Video item IDs and paths
        let video_items: Vec<(u64, std::path::PathBuf)> = board
            .items
            .iter()
            .filter_map(|item| {
                if let ItemContent::Video(path) = &item.content {
                    Some((item.id, path.clone()))
                } else {
                    None
                }
            })
            .collect();

        // Create WebViews for new Video items
        for (item_id, path) in &video_items {
            if !self.video_webviews.contains_key(item_id) {
                match VideoWebView::new(path.clone(), window, cx) {
                    Ok(webview) => {
                        self.video_webviews.insert(*item_id, webview);
                    }
                    Err(e) => {
                        error!("Failed to create Video WebView for {:?}: {}", path, e);
                    }
                }
            }
        }

        // Remove WebViews for deleted items
        let video_ids: std::collections::HashSet<u64> =
            video_items.iter().map(|(id, _)| *id).collect();
        self.video_webviews.retain(|id, _| video_ids.contains(id));
    }

    /// Update webview visibility based on canvas viewport
    /// Hides webviews that are scrolled out of view to prevent z-index issues
    pub fn update_webview_visibility(&mut self, window: &mut Window, cx: &mut App) {
        let Some(ref board) = self.board else { return };

        // Hide all webviews when settings modal or shortcuts overlay is open
        if self.show_settings || self.show_shortcuts {
            for (_, webview) in &self.youtube_webviews {
                webview.webview().update(cx, |wv, _| wv.hide());
            }
            for (_, webview) in &self.audio_webviews {
                webview.webview_entity.update(cx, |wv, _| wv.hide());
            }
            for (_, webview) in &self.video_webviews {
                webview.webview_entity.update(cx, |wv, _| wv.hide());
            }
            // Also hide PDF webviews in preview panel
            if let Some(ref preview) = self.preview {
                for tab in &preview.tabs {
                    if let PreviewTab::Pdf {
                        webview: Some(wv), ..
                    } = tab
                    {
                        wv.webview().update(cx, |view, _| view.hide());
                    }
                }
            }
            return;
        }

        let bounds = window.bounds();
        let window_width = f32::from(bounds.size.width);
        let window_height = f32::from(bounds.size.height);

        // Account for preview panel if open
        let (canvas_width, canvas_height) = if let Some(ref preview) = self.preview {
            match preview.split {
                SplitDirection::Vertical => ((1.0 - preview.size) * window_width, window_height),
                SplitDirection::Horizontal => (window_width, (1.0 - preview.size) * window_height),
            }
        } else {
            (window_width, window_height)
        };

        // Header offset
        let header_height = 40.0;
        let canvas_top = header_height;

        let zoom = board.zoom;
        let offset_x = f32::from(board.canvas_offset.x);
        let offset_y = f32::from(board.canvas_offset.y);

        // Check each item with a webview
        for item in &board.items {
            let item_x = item.position.0 * zoom + offset_x;
            let item_y = item.position.1 * zoom + offset_y + header_height;
            let item_w = item.size.0 * zoom;
            let item_h = item.size.1 * zoom;

            // Check if item is visible and not overlapping UI chrome
            // Webviews don't clip, so hide if any edge goes outside canvas bounds
            let footer_height = 28.0;
            let canvas_bottom = canvas_height - footer_height;

            let overlaps_header = item_y < canvas_top;
            let overlaps_footer = item_y + item_h > canvas_bottom;
            let overlaps_left = item_x < 0.0;
            let overlaps_right = item_x + item_w > canvas_width;

            let is_visible =
                !overlaps_header && !overlaps_footer && !overlaps_left && !overlaps_right;

            // Update YouTube webview visibility
            if let Some(webview) = self.youtube_webviews.get(&item.id) {
                webview.webview().update(cx, |wv, _| {
                    if is_visible {
                        wv.show();
                    } else {
                        wv.hide();
                    }
                });
            }

            // Update Audio webview visibility
            if let Some(webview) = self.audio_webviews.get(&item.id) {
                webview.webview_entity.update(cx, |wv, _| {
                    if is_visible {
                        wv.show();
                    } else {
                        wv.hide();
                    }
                });
            }

            // Update Video webview visibility
            if let Some(webview) = self.video_webviews.get(&item.id) {
                webview.webview_entity.update(cx, |wv, _| {
                    if is_visible {
                        wv.show();
                    } else {
                        wv.hide();
                    }
                });
            }
        }

        // Show PDF webviews in preview panel (active tab only)
        if let Some(ref preview) = self.preview {
            for (idx, tab) in preview.tabs.iter().enumerate() {
                if let PreviewTab::Pdf {
                    webview: Some(wv), ..
                } = tab
                {
                    if idx == preview.active_tab {
                        wv.webview().update(cx, |view, _| view.show());
                    } else {
                        wv.webview().update(cx, |view, _| view.hide());
                    }
                }
            }
        }
    }

    pub fn close_preview(&mut self, cx: &mut Context<Self>) {
        self.preview = None;
        cx.notify();
    }

    pub fn close_tab(&mut self, tab_index: usize, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            // Close tab in the focused pane
            if preview.is_pane_split && preview.focused_pane == FocusedPane::Right {
                // Closing from right pane
                if tab_index < preview.right_tabs.len() {
                    if preview.right_tabs[tab_index].is_pinned() {
                        return;
                    }

                    let closed_tab = preview.right_tabs.remove(tab_index);
                    preview.closed_tabs.push(closed_tab);
                    if preview.closed_tabs.len() > 20 {
                        preview.closed_tabs.remove(0);
                    }

                    if preview.right_tabs.is_empty() {
                        // Right pane empty, close the split
                        preview.is_pane_split = false;
                        preview.focused_pane = FocusedPane::Left;
                    } else {
                        if preview.right_active_tab >= preview.right_tabs.len() {
                            preview.right_active_tab = preview.right_tabs.len() - 1;
                        } else if tab_index < preview.right_active_tab {
                            preview.right_active_tab -= 1;
                        }
                    }
                    cx.notify();
                }
            } else {
                // Closing from left pane
                if tab_index < preview.tabs.len() {
                    if preview.tabs[tab_index].is_pinned() {
                        return;
                    }

                    let closed_tab = preview.tabs.remove(tab_index);
                    preview.closed_tabs.push(closed_tab);
                    if preview.closed_tabs.len() > 20 {
                        preview.closed_tabs.remove(0);
                    }

                    if preview.tabs.is_empty() {
                        if preview.is_pane_split && !preview.right_tabs.is_empty() {
                            // Left pane empty but right has tabs - move right to left
                            preview.tabs = preview.right_tabs.drain(..).collect();
                            preview.active_tab = preview.right_active_tab;
                            preview.is_pane_split = false;
                            preview.focused_pane = FocusedPane::Left;
                        } else {
                            // No tabs anywhere, close preview panel
                            self.preview = None;
                        }
                    } else {
                        if preview.active_tab >= preview.tabs.len() {
                            preview.active_tab = preview.tabs.len() - 1;
                        } else if tab_index < preview.active_tab {
                            preview.active_tab -= 1;
                        }
                    }
                    cx.notify();
                }
            }
        }
    }

    /// Convert a preview tab to a permanent tab
    pub fn make_tab_permanent(&mut self, tab_index: usize, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if let Some(tab) = preview.tabs.get_mut(tab_index) {
                tab.make_permanent();
                cx.notify();
            }
        }
    }

    /// Toggle the pinned state of a tab
    pub fn toggle_tab_pinned(&mut self, tab_index: usize, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if let Some(tab) = preview.tabs.get_mut(tab_index) {
                let was_pinned = tab.is_pinned();
                tab.toggle_pinned();

                // Reorder tabs: pinned tabs stay at the left
                if !was_pinned && tab.is_pinned() {
                    // Tab was just pinned - move it to after the last pinned tab
                    let pinned_count = preview.tabs.iter().filter(|t| t.is_pinned()).count();
                    if tab_index >= pinned_count {
                        let tab = preview.tabs.remove(tab_index);
                        preview.tabs.insert(pinned_count - 1, tab);
                        // Adjust active tab index
                        if preview.active_tab == tab_index {
                            preview.active_tab = pinned_count - 1;
                        } else if preview.active_tab < tab_index
                            && preview.active_tab >= pinned_count - 1
                        {
                            preview.active_tab += 1;
                        }
                    }
                }
                cx.notify();
            }
        }
    }

    /// Reopen the most recently closed tab
    pub fn reopen_closed_tab(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if let Some(tab) = preview.closed_tabs.pop() {
                preview.tabs.push(tab);
                preview.active_tab = preview.tabs.len() - 1;
                cx.notify();
            }
        }
    }

    /// Navigate back in tab history
    pub fn go_back(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if let Some(prev_index) = preview.back_stack.pop() {
                if prev_index < preview.tabs.len() {
                    // Push current to forward stack
                    preview.forward_stack.push(preview.active_tab);
                    preview.active_tab = prev_index;
                    cx.notify();
                }
            }
        }
    }

    /// Navigate forward in tab history
    pub fn go_forward(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if let Some(next_index) = preview.forward_stack.pop() {
                if next_index < preview.tabs.len() {
                    // Push current to back stack
                    preview.back_stack.push(preview.active_tab);
                    preview.active_tab = next_index;
                    cx.notify();
                }
            }
        }
    }

    /// Start dragging a tab for reordering
    pub fn start_tab_drag(
        &mut self,
        tab_index: usize,
        position: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        tracing::debug!("start_tab_drag: index={}", tab_index);
        self.dragging_tab = Some(tab_index);
        self.tab_drag_target = Some(tab_index);
        self.tab_drag_split_zone = None;
        self.tab_drag_position = Some(position);
        cx.notify();
    }

    /// Update drag position as mouse moves
    pub fn update_tab_drag_position(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        if self.dragging_tab.is_some() {
            self.tab_drag_position = Some(position);
            cx.notify();
        }
    }

    /// Update the drag target position as mouse moves over tabs
    pub fn update_tab_drag_target(&mut self, target_index: usize, cx: &mut Context<Self>) {
        if self.dragging_tab.is_some() {
            // Clear split zone when hovering over tabs
            if self.tab_drag_split_zone.is_some() {
                self.tab_drag_split_zone = None;
            }
            if self.tab_drag_target != Some(target_index) {
                self.tab_drag_target = Some(target_index);
            }
            cx.notify();
        }
    }

    /// Set the split drop zone when dragging to edges
    pub fn set_tab_drag_split_zone(&mut self, zone: Option<SplitDropZone>, cx: &mut Context<Self>) {
        if self.dragging_tab.is_some() && self.tab_drag_split_zone != zone {
            tracing::debug!("set_tab_drag_split_zone: {:?}", zone);
            self.tab_drag_split_zone = zone;
            // Clear tab target when in split zone
            if zone.is_some() {
                self.tab_drag_target = None;
            }
            cx.notify();
        }
    }

    /// Finish tab drag - either reorder or create split
    pub fn finish_tab_drag(&mut self, cx: &mut Context<Self>) {
        tracing::debug!(
            "finish_tab_drag: dragging={:?}, split_zone={:?}, target={:?}",
            self.dragging_tab,
            self.tab_drag_split_zone,
            self.tab_drag_target
        );
        // Check if dropping on a split zone
        if let (Some(from), Some(zone)) = (self.dragging_tab, self.tab_drag_split_zone) {
            if let Some(ref mut preview) = self.preview {
                // Determine which pane we're dragging from
                let from_right_pane =
                    preview.is_pane_split && preview.focused_pane == FocusedPane::Right;

                // Get the source tabs list
                let (source_tabs, source_active) = if from_right_pane {
                    (&mut preview.right_tabs, &mut preview.right_active_tab)
                } else {
                    (&mut preview.tabs, &mut preview.active_tab)
                };

                if from < source_tabs.len() {
                    let tab = source_tabs.remove(from);

                    // Update source active tab
                    if *source_active >= source_tabs.len() {
                        *source_active = source_tabs.len().saturating_sub(1);
                    } else if *source_active > from {
                        *source_active -= 1;
                    }

                    // Hide webview before move
                    if let PreviewTab::Pdf {
                        webview: Some(wv), ..
                    } = &tab
                    {
                        wv.hide(cx);
                    }

                    // Determine target based on zone
                    let target_is_right =
                        matches!(zone, SplitDropZone::Right | SplitDropZone::Bottom);

                    if !preview.is_pane_split {
                        // Create new split
                        preview.is_pane_split = true;
                        preview.pane_ratio = 0.5;
                        preview.pane_split_horizontal =
                            matches!(zone, SplitDropZone::Top | SplitDropZone::Bottom);

                        if target_is_right {
                            preview.right_tabs.push(tab);
                            preview.right_active_tab = preview.right_tabs.len() - 1;
                            preview.focused_pane = FocusedPane::Right;
                        } else {
                            // Move existing tabs to right, put dragged tab in left
                            let left_tabs: Vec<PreviewTab> = preview.tabs.drain(..).collect();
                            preview.tabs.push(tab);
                            preview.active_tab = 0;
                            for t in left_tabs {
                                preview.right_tabs.push(t);
                            }
                            preview.right_active_tab = 0;
                            preview.focused_pane = FocusedPane::Left;
                        }
                    } else {
                        // Already split - move tab to target pane
                        if target_is_right {
                            preview.right_tabs.push(tab);
                            preview.right_active_tab = preview.right_tabs.len() - 1;
                            preview.focused_pane = FocusedPane::Right;
                        } else {
                            preview.tabs.push(tab);
                            preview.active_tab = preview.tabs.len() - 1;
                            preview.focused_pane = FocusedPane::Left;
                        }

                        // Check if source pane is now empty - close split
                        if from_right_pane && preview.right_tabs.is_empty() {
                            preview.is_pane_split = false;
                            preview.focused_pane = FocusedPane::Left;
                        } else if !from_right_pane && preview.tabs.is_empty() {
                            // Move right tabs to left
                            preview.tabs = preview.right_tabs.drain(..).collect();
                            preview.active_tab = preview.right_active_tab;
                            preview.is_pane_split = false;
                            preview.focused_pane = FocusedPane::Left;
                        }
                    }
                }
            }
            self.dragging_tab = None;
            self.tab_drag_target = None;
            self.tab_drag_split_zone = None;
            self.tab_drag_position = None;
            cx.notify();
            return;
        }

        // Normal tab reorder
        if let (Some(from), Some(to)) = (self.dragging_tab, self.tab_drag_target) {
            if from != to {
                if let Some(ref mut preview) = self.preview {
                    // Don't allow moving tabs before pinned tabs (unless the dragged tab is pinned)
                    let pinned_count = preview.tabs.iter().filter(|t| t.is_pinned()).count();
                    let is_dragged_pinned = preview
                        .tabs
                        .get(from)
                        .map(|t| t.is_pinned())
                        .unwrap_or(false);

                    let effective_to = if !is_dragged_pinned && to < pinned_count {
                        pinned_count // Can't move before pinned tabs
                    } else {
                        to
                    };

                    if from != effective_to && from < preview.tabs.len() {
                        let tab = preview.tabs.remove(from);
                        let insert_pos = if effective_to > from {
                            effective_to.min(preview.tabs.len())
                        } else {
                            effective_to
                        };
                        preview.tabs.insert(insert_pos, tab);

                        // Update active tab index
                        if preview.active_tab == from {
                            preview.active_tab = insert_pos;
                        } else if from < preview.active_tab && insert_pos >= preview.active_tab {
                            preview.active_tab -= 1;
                        } else if from > preview.active_tab && insert_pos <= preview.active_tab {
                            preview.active_tab += 1;
                        }
                    }
                }
            }
        }
        self.dragging_tab = None;
        self.tab_drag_target = None;
        self.tab_drag_split_zone = None;
        self.tab_drag_position = None;
        cx.notify();
    }

    /// Cancel tab drag without reordering
    pub fn cancel_tab_drag(&mut self, cx: &mut Context<Self>) {
        self.dragging_tab = None;
        self.tab_drag_target = None;
        self.tab_drag_split_zone = None;
        self.tab_drag_position = None;
        cx.notify();
    }

    /// Toggle the preview search bar
    pub fn toggle_preview_search(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.preview_search.is_some() {
            self.close_preview_search(cx);
        } else {
            self.open_preview_search(window, cx);
        }
    }

    /// Open the preview search bar
    pub fn open_preview_search(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.preview.is_none() {
            return;
        }

        let input = cx.new(|cx| InputState::new(window, cx).placeholder("Find in file..."));

        // Focus the input
        input.update(cx, |state, cx| {
            state.focus(window, cx);
        });

        // Subscribe to input changes
        cx.subscribe(
            &input,
            |this, input, event: &gpui_component::input::InputEvent, cx| {
                match event {
                    gpui_component::input::InputEvent::Change { .. } => {
                        let query = input.read(cx).text().to_string();
                        this.update_preview_search(&query, cx);
                    }
                    gpui_component::input::InputEvent::PressEnter { .. } => {
                        // Go to next match
                        this.next_search_match(cx);
                    }
                    gpui_component::input::InputEvent::Blur => {
                        // Don't close on blur - let user click away
                    }
                    _ => {}
                }
            },
        )
        .detach();

        self.preview_search = Some(input);
        self.preview_search_query.clear();
        self.preview_search_matches.clear();
        self.preview_search_current = 0;
        cx.notify();
    }

    /// Close the preview search bar
    pub fn close_preview_search(&mut self, cx: &mut Context<Self>) {
        self.preview_search = None;
        self.preview_search_query.clear();
        self.preview_search_matches.clear();
        self.preview_search_current = 0;
        cx.notify();
    }

    /// Update search matches based on query
    fn update_preview_search(&mut self, query: &str, cx: &mut Context<Self>) {
        self.preview_search_query = query.to_string();
        self.preview_search_matches.clear();
        self.preview_search_current = 0;

        if query.is_empty() {
            cx.notify();
            return;
        }

        // Get content from active tab
        let content = if let Some(ref preview) = self.preview {
            if let Some(tab) = preview.tabs.get(preview.active_tab) {
                match tab {
                    PreviewTab::Markdown { content, .. } => Some(content.clone()),
                    PreviewTab::Code { content, .. } => Some(content.clone()),
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        };

        if let Some(content) = content {
            let query_lower = query.to_lowercase();
            for (line_idx, line) in content.lines().enumerate() {
                let line_lower = line.to_lowercase();
                let mut start = 0;
                while let Some(col) = line_lower[start..].find(&query_lower) {
                    self.preview_search_matches.push((line_idx, start + col));
                    start += col + 1;
                }
            }
        }

        cx.notify();
    }

    /// Go to next search match
    pub fn next_search_match(&mut self, cx: &mut Context<Self>) {
        if !self.preview_search_matches.is_empty() {
            self.preview_search_current =
                (self.preview_search_current + 1) % self.preview_search_matches.len();
            cx.notify();
        }
    }

    /// Go to previous search match
    pub fn prev_search_match(&mut self, cx: &mut Context<Self>) {
        if !self.preview_search_matches.is_empty() {
            self.preview_search_current = if self.preview_search_current == 0 {
                self.preview_search_matches.len() - 1
            } else {
                self.preview_search_current - 1
            };
            cx.notify();
        }
    }

    pub fn switch_tab(&mut self, tab_index: usize, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if tab_index < preview.tabs.len() && tab_index != preview.active_tab {
                // Record in back stack for navigation history
                preview.back_stack.push(preview.active_tab);
                // Clear forward stack when user manually switches tabs
                preview.forward_stack.clear();

                // Hide/show PDF webviews based on active tab
                for (idx, tab) in preview.tabs.iter().enumerate() {
                    if let PreviewTab::Pdf {
                        webview: Some(wv), ..
                    } = tab
                    {
                        if idx == tab_index {
                            wv.webview().update(cx, |view, _| view.show());
                        } else {
                            wv.webview().update(cx, |view, _| view.hide());
                        }
                    }
                }

                preview.active_tab = tab_index;
                cx.notify();
            }
        }
    }

    pub fn next_tab(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if !preview.tabs.is_empty() {
                preview.active_tab = (preview.active_tab + 1) % preview.tabs.len();
                cx.notify();
            }
        }
    }

    pub fn prev_tab(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if !preview.tabs.is_empty() {
                preview.active_tab = if preview.active_tab == 0 {
                    preview.tabs.len() - 1
                } else {
                    preview.active_tab - 1
                };
                cx.notify();
            }
        }
    }

    pub fn close_current_tab(&mut self, cx: &mut Context<Self>) {
        if let Some(ref preview) = self.preview {
            let active = if preview.is_pane_split && preview.focused_pane == FocusedPane::Right {
                preview.right_active_tab
            } else {
                preview.active_tab
            };
            self.close_tab(active, cx);
        }
    }

    pub fn toggle_split_direction(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            preview.split = match preview.split {
                SplitDirection::Vertical => SplitDirection::Horizontal,
                SplitDirection::Horizontal => SplitDirection::Vertical,
            };
            cx.notify();
        }
    }

    /// Split the preview panel into two panes
    pub fn split_pane(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if !preview.is_pane_split {
                preview.is_pane_split = true;
                preview.pane_ratio = 0.5;
                // Focus stays on left pane
                cx.notify();
            }
        }
    }

    /// Close the split and merge into single pane
    pub fn close_split_pane(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if preview.is_pane_split {
                // Move right pane tabs to left pane
                let right_tabs: Vec<PreviewTab> = preview.right_tabs.drain(..).collect();
                for tab in right_tabs {
                    preview.tabs.push(tab);
                }
                preview.right_active_tab = 0;
                preview.right_back_stack.clear();
                preview.right_forward_stack.clear();
                preview.is_pane_split = false;
                preview.focused_pane = FocusedPane::Left;
                cx.notify();
            }
        }
    }

    /// Toggle split pane on/off
    pub fn toggle_pane_split(&mut self, cx: &mut Context<Self>) {
        if let Some(ref preview) = self.preview {
            if preview.is_pane_split {
                self.close_split_pane(cx);
            } else {
                self.split_pane(cx);
            }
        }
    }

    /// Focus the left pane
    pub fn focus_left_pane(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            preview.focused_pane = FocusedPane::Left;
            cx.notify();
        }
    }

    /// Focus the right pane
    pub fn focus_right_pane(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if preview.is_pane_split {
                preview.focused_pane = FocusedPane::Right;
                cx.notify();
            }
        }
    }

    /// Move current tab to the other pane
    pub fn move_tab_to_other_pane(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if !preview.is_pane_split {
                // Auto-split when moving tab
                preview.is_pane_split = true;
                preview.pane_ratio = 0.5;
            }

            match preview.focused_pane {
                FocusedPane::Left => {
                    if !preview.tabs.is_empty() && preview.active_tab < preview.tabs.len() {
                        let tab = preview.tabs.remove(preview.active_tab);
                        preview.right_tabs.push(tab);
                        preview.right_active_tab = preview.right_tabs.len() - 1;
                        // Adjust left active tab
                        if preview.active_tab >= preview.tabs.len() && !preview.tabs.is_empty() {
                            preview.active_tab = preview.tabs.len() - 1;
                        }
                        // Focus the right pane where we moved the tab
                        preview.focused_pane = FocusedPane::Right;
                    }
                }
                FocusedPane::Right => {
                    if !preview.right_tabs.is_empty()
                        && preview.right_active_tab < preview.right_tabs.len()
                    {
                        let tab = preview.right_tabs.remove(preview.right_active_tab);
                        preview.tabs.push(tab);
                        preview.active_tab = preview.tabs.len() - 1;
                        // Adjust right active tab
                        if preview.right_active_tab >= preview.right_tabs.len()
                            && !preview.right_tabs.is_empty()
                        {
                            preview.right_active_tab = preview.right_tabs.len() - 1;
                        }
                        // Focus the left pane where we moved the tab
                        preview.focused_pane = FocusedPane::Left;
                    }
                }
            }

            // If right pane is empty after move, close split
            if preview.right_tabs.is_empty() {
                preview.is_pane_split = false;
                preview.focused_pane = FocusedPane::Left;
            }

            cx.notify();
        }
    }

    pub fn next_page(&mut self, _cx: &mut Context<Self>) {
        // WebView handles PDF navigation internally
    }

    pub fn prev_page(&mut self, _cx: &mut Context<Self>) {
        // WebView handles PDF navigation internally
    }

    pub fn update_fps(&mut self) {
        let now = Instant::now();
        let frame_time = now.duration_since(self.last_frame);
        self.last_frame = now;
        self.frame_count += 1;

        self.frame_times.push(frame_time);
        if self.frame_times.len() > 60 {
            self.frame_times.remove(0);
        }
    }

    pub fn calculate_fps(&self) -> f32 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        let avg_frame_time: Duration =
            self.frame_times.iter().sum::<Duration>() / self.frame_times.len() as u32;
        if avg_frame_time.as_secs_f32() > 0.0 {
            1.0 / avg_frame_time.as_secs_f32()
        } else {
            0.0
        }
    }

    pub fn pdf_zoom_in(&mut self, _cx: &mut Context<Self>) {
        // WebView handles PDF zoom internally
    }

    pub fn pdf_zoom_out(&mut self, _cx: &mut Context<Self>) {
        // WebView handles PDF zoom internally
    }

    pub fn pdf_zoom_reset(&mut self, _cx: &mut Context<Self>) {
        // WebView handles PDF zoom internally
    }

    // ==================== TextBox Editing Methods ====================

    /// Start editing a textbox inline on the canvas
    pub fn start_textbox_editing(
        &mut self,
        item_id: u64,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Clear any pending drag/resize state from the first click of the double-click
        self.dragging_item = None;
        self.item_drag_offset = None;
        self.resizing_item = None;
        self.resize_start_size = None;
        self.resize_start_pos = None;
        self.resize_start_font_size = None;

        // Get the current text from the item
        let current_text = if let Some(ref board) = self.board {
            board.get_item(item_id).and_then(|item| {
                if let ItemContent::TextBox { text, .. } = &item.content {
                    Some(text.clone())
                } else {
                    None
                }
            })
        } else {
            None
        };

        if let Some(text) = current_text {
            // Count lines to position cursor at end
            let lines: Vec<&str> = text.lines().collect();
            let last_line = lines.len().saturating_sub(1) as u32;
            let last_char = lines.last().map(|l| l.len() as u32).unwrap_or(0);

            // Create the input with multiline support (code_editor enables multiline)
            let input = cx.new(|cx| {
                InputState::new(window, cx)
                    .code_editor("plaintext") // Enable multiline editing
                    .line_number(false) // No line numbers for textbox
                    .soft_wrap(true) // Wrap long lines
                    .default_value(text)
            });

            // Focus the input and position cursor at end after it's been added to the render tree
            // We need to defer this because the Input isn't mounted yet
            let input_clone = input.clone();
            window.defer(cx, move |window, cx| {
                input_clone.update(cx, |state, cx| {
                    // Position cursor at the end of the text
                    let end_pos = gpui_component::input::Position {
                        line: last_line,
                        character: last_char,
                    };
                    state.set_cursor_position(end_pos, window, cx);
                });
            });

            // Subscribe to input events
            // Note: PressEnter is NOT handled here - Enter creates new lines in multiline mode
            // Use Escape or click outside to finish editing
            cx.subscribe(
                &input,
                |this, input, event: &gpui_component::input::InputEvent, cx| {
                    match event {
                        gpui_component::input::InputEvent::Change { .. } => {
                            // Update text as user types
                            if let Some(item_id) = this.editing_textbox_id {
                                let new_text = input.read(cx).text().to_string();
                                if let Some(ref mut board) = this.board {
                                    if let Some(item) = board.get_item_mut(item_id) {
                                        if let ItemContent::TextBox { text, .. } = &mut item.content
                                        {
                                            *text = new_text;
                                        }
                                    }
                                    board.mark_dirty();
                                }
                            }
                        }
                        gpui_component::input::InputEvent::Blur => {
                            // Save on blur (click outside)
                            this.finish_textbox_editing(cx);
                        }
                        _ => {}
                    }
                },
            )
            .detach();

            // Set focus context to TextboxEditing so keyboard shortcuts don't intercept input
            self.focus
                .set_context_without_focus(FocusContext::TextboxEditing);

            self.editing_textbox_id = Some(item_id);
            self.textbox_input = Some(input);
            cx.notify();
        }
    }

    /// Finish editing and save the textbox content
    pub fn finish_textbox_editing(&mut self, cx: &mut Context<Self>) {
        if let Some(item_id) = self.editing_textbox_id.take() {
            if let Some(input) = self.textbox_input.take() {
                let new_text = input.read(cx).text().to_string();

                if let Some(ref mut board) = self.board {
                    if let Some(item) = board.get_item_mut(item_id) {
                        if let ItemContent::TextBox { text, .. } = &mut item.content {
                            *text = new_text;
                        }
                    }
                    board.push_history();
                    if let Err(e) = board.flush_save() {
                        self.toast_manager
                            .push(crate::notifications::Toast::error(format!(
                                "Save failed: {}",
                                e
                            )));
                    }
                }
            }
        }

        // Release focus back to canvas (mark for restore since we don't have window)
        self.focus.mark_needs_canvas_focus();
        cx.notify();
    }

    /// Finish editing with explicit window access (preferred when window available)
    pub fn finish_textbox_editing_with_window(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(item_id) = self.editing_textbox_id.take() {
            if let Some(input) = self.textbox_input.take() {
                let new_text = input.read(cx).text().to_string();

                if let Some(ref mut board) = self.board {
                    if let Some(item) = board.get_item_mut(item_id) {
                        if let ItemContent::TextBox { text, .. } = &mut item.content {
                            *text = new_text;
                        }
                    }
                    board.push_history();
                    if let Err(e) = board.flush_save() {
                        self.toast_manager
                            .push(crate::notifications::Toast::error(format!(
                                "Save failed: {}",
                                e
                            )));
                    }
                }
            }
        }

        // Release focus back to canvas
        self.focus.release(FocusContext::TextboxEditing, window);
        cx.notify();
    }

    /// Cancel textbox editing without saving
    pub fn cancel_textbox_editing(&mut self, cx: &mut Context<Self>) {
        self.editing_textbox_id = None;
        self.textbox_input = None;

        // Release focus back to canvas (mark for restore since we don't have window)
        self.focus.mark_needs_canvas_focus();
        cx.notify();
    }

    /// Cancel textbox editing with explicit window access (preferred when window available)
    pub fn cancel_textbox_editing_with_window(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editing_textbox_id = None;
        self.textbox_input = None;

        // Release focus back to canvas
        self.focus.release(FocusContext::TextboxEditing, window);
        cx.notify();
    }
}
