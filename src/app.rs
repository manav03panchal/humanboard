use crate::board::Board;
use crate::board_index::BoardIndex;
use crate::notifications::ToastManager;
use crate::pdf_webview::PdfWebView;
use crate::settings::Settings;
use crate::youtube_webview::YouTubeWebView;
use gpui::*;
use gpui_component::input::InputState;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub enum AppView {
    Landing,
    Board(String), // Board ID
}

#[derive(Clone, Copy, PartialEq)]
pub enum SplitDirection {
    Vertical,   // Panel on the right
    Horizontal, // Panel on the bottom
}

pub enum PreviewTab {
    Pdf {
        path: PathBuf,
        webview: Option<PdfWebView>,
    },
    Markdown {
        path: PathBuf,
        content: String,
        editing: bool,
        editor: Option<Entity<InputState>>,
    },
}

impl PreviewTab {
    pub fn path(&self) -> &PathBuf {
        match self {
            PreviewTab::Pdf { path, .. } => path,
            PreviewTab::Markdown { path, .. } => path,
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
        }
    }
}

pub struct PreviewPanel {
    pub tabs: Vec<PreviewTab>,
    pub active_tab: usize,
    pub split: SplitDirection,
    pub size: f32, // 0.0 to 1.0, percentage of window
}

pub struct Humanboard {
    // View state
    pub view: AppView,
    pub board_index: BoardIndex,

    // Landing page state
    pub editing_board_id: Option<String>,
    pub edit_input: Option<Entity<InputState>>,
    pub deleting_board_id: Option<String>,

    // Board state (only populated when view is Board)
    pub board: Option<Board>,
    pub dragging: bool,
    pub last_mouse_pos: Option<Point<Pixels>>,
    pub dragging_item: Option<u64>,
    pub item_drag_offset: Option<Point<Pixels>>,
    pub resizing_item: Option<u64>,
    pub resize_start_size: Option<(f32, f32)>,
    pub resize_start_pos: Option<Point<Pixels>>,
    pub selected_items: HashSet<u64>,

    // Marquee selection state
    pub marquee_start: Option<Point<Pixels>>,
    pub marquee_current: Option<Point<Pixels>>,

    pub frame_times: Vec<Duration>,
    pub last_frame: Instant,
    pub frame_count: u64,
    pub focus_handle: FocusHandle,
    pub preview: Option<PreviewPanel>,
    pub dragging_splitter: bool,
    pub splitter_drag_start: Option<Point<Pixels>>,
    pub last_drop_pos: Option<Point<Pixels>>,
    pub file_drop_rx: Option<Receiver<(Point<Pixels>, Vec<PathBuf>)>>,

    // UI overlays
    pub show_shortcuts: bool,
    pub command_palette: Option<Entity<InputState>>, // Command palette input
    pub pending_command: Option<String>, // Command to execute (deferred until we have window access)
    pub search_results: Vec<(u64, String)>, // Search results: (item_id, display_name)
    pub selected_result: usize,          // Currently selected search result index

    // YouTube WebViews (keyed by item ID)
    pub youtube_webviews: HashMap<u64, YouTubeWebView>,

    // Settings
    pub settings: Settings,
    pub show_settings: bool,

    // Toast notifications
    pub toast_manager: ToastManager,

    // Preview tab scroll handle
    pub preview_tab_scroll: ScrollHandle,

    // Command palette scroll handle
    pub cmd_palette_scroll: ScrollHandle,

    // Pan animation state
    pub pan_animation: Option<PanAnimation>,
}

/// Animation state for smooth panning to a target position
pub struct PanAnimation {
    pub start_offset: Point<Pixels>,
    pub target_offset: Point<Pixels>,
    pub start_time: Instant,
    pub duration: Duration,
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
            board: None,
            dragging: false,
            last_mouse_pos: None,
            dragging_item: None,
            item_drag_offset: None,
            resizing_item: None,
            resize_start_size: None,
            resize_start_pos: None,
            selected_items: HashSet::new(),
            marquee_start: None,
            marquee_current: None,
            frame_times: Vec::with_capacity(60),
            last_frame: Instant::now(),
            frame_count: 0,
            focus_handle: cx.focus_handle(),
            preview: None,
            dragging_splitter: false,
            splitter_drag_start: None,
            last_drop_pos: None,
            file_drop_rx: None,
            show_shortcuts: false,
            command_palette: None,
            pending_command: None,
            search_results: Vec::new(),
            selected_result: 0,
            youtube_webviews: HashMap::new(),
            settings: Settings::load(),
            show_settings: false,
            toast_manager: ToastManager::new(),
            preview_tab_scroll: ScrollHandle::new(),
            cmd_palette_scroll: ScrollHandle::new(),
            pan_animation: None,
        }
    }

    pub fn toggle_settings(&mut self, cx: &mut Context<Self>) {
        self.show_settings = !self.show_settings;
        cx.notify();
    }

    /// Show a toast notification
    pub fn show_toast(&mut self, toast: crate::notifications::Toast) {
        self.toast_manager.push(toast);
    }

    pub fn set_theme(&mut self, theme_name: String, cx: &mut Context<Self>) {
        self.settings.theme = theme_name.clone();
        self.settings.save();

        // Apply theme using the App context
        let theme_name = gpui::SharedString::from(theme_name);
        let config = gpui_component::theme::ThemeRegistry::global(cx)
            .themes()
            .get(&theme_name)
            .cloned();

        if let Some(config) = config {
            let mode = config.mode;
            if mode.is_dark() {
                gpui_component::theme::Theme::global_mut(cx).dark_theme = config.clone();
            } else {
                gpui_component::theme::Theme::global_mut(cx).light_theme = config.clone();
            }
            gpui_component::theme::Theme::global_mut(cx).mode = mode;
            gpui_component::theme::Theme::global_mut(cx).apply_config(&config);
        }

        cx.notify();
    }

    pub fn toggle_shortcuts(&mut self, cx: &mut Context<Self>) {
        self.show_shortcuts = !self.show_shortcuts;
        cx.notify();
    }

    pub fn show_command_palette(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let input = cx
            .new(|cx| InputState::new(window, cx).placeholder("Type to search or use commands..."));

        // Focus the input
        input.update(cx, |state, cx| {
            state.focus(window, cx);
        });

        // Subscribe to input events
        cx.subscribe(
            &input,
            |this, input, event: &gpui_component::input::InputEvent, cx| {
                match event {
                    gpui_component::input::InputEvent::PressEnter { .. } => {
                        // Note: Enter is handled by the action handler on the container
                        // which runs before this subscription. Don't double-handle.
                        // Only handle if pending_command wasn't already set by action handler.
                        if this.pending_command.is_none() && this.command_palette.is_some() {
                            this.execute_command_from_subscription(cx);
                        }
                    }
                    gpui_component::input::InputEvent::Change { .. } => {
                        // Update search results as user types
                        let text = input.read(cx).text().to_string();
                        this.update_search_results(&text, cx);
                    }
                    gpui_component::input::InputEvent::Blur => {
                        // Close command palette when input loses focus (click outside)
                        this.hide_command_palette(cx);
                    }
                    _ => {}
                }
            },
        )
        .detach();

        self.command_palette = Some(input);

        // Show all items initially
        self.update_search_results("", cx);
    }

    pub fn hide_command_palette(&mut self, cx: &mut Context<Self>) {
        self.command_palette = None;
        self.search_results.clear();
        self.selected_result = 0;
        cx.notify();
    }

    /// Update search results based on input text
    fn update_search_results(&mut self, text: &str, cx: &mut Context<Self>) {
        let text = text.trim();

        // Don't search if it's a command
        if text.starts_with("md ") || text == "md" {
            self.search_results.clear();
            self.selected_result = 0;
            cx.notify();
            return;
        }

        // Search canvas items (empty string shows all items)
        if let Some(ref board) = self.board {
            if text.is_empty() {
                // Show all items when no search text
                self.search_results = board
                    .items
                    .iter()
                    .map(|item| (item.id, item.content.display_name()))
                    .collect();
            } else {
                self.search_results = board.find_items(text);
            }
            self.selected_result = 0;
        } else {
            self.search_results.clear();
        }
        cx.notify();
    }

    /// Navigate search results
    pub fn select_next_result(&mut self, cx: &mut Context<Self>) {
        if !self.search_results.is_empty() {
            self.selected_result = (self.selected_result + 1) % self.search_results.len();
            self.cmd_palette_scroll.scroll_to_item(self.selected_result);
            cx.notify();
        }
    }

    pub fn select_prev_result(&mut self, cx: &mut Context<Self>) {
        if !self.search_results.is_empty() {
            self.selected_result = if self.selected_result == 0 {
                self.search_results.len() - 1
            } else {
                self.selected_result - 1
            };
            self.cmd_palette_scroll.scroll_to_item(self.selected_result);
            cx.notify();
        }
    }

    /// Called from action when Enter is pressed - stores command for deferred execution
    pub fn execute_command_from_action(&mut self, cx: &mut Context<Self>) {
        self.execute_command_from_subscription(cx);
    }

    /// Called from subscription when Enter is pressed - stores command for deferred execution
    fn execute_command_from_subscription(&mut self, cx: &mut Context<Self>) {
        // If we have search results selected, jump to that item
        if !self.search_results.is_empty() {
            let (item_id, _) = self.search_results[self.selected_result];
            self.pending_command = Some(format!("__jump:{}", item_id));
        } else {
            let command = self
                .command_palette
                .as_ref()
                .map(|input| input.read(cx).text().to_string())
                .unwrap_or_default();
            self.pending_command = Some(command);
        }

        self.command_palette = None;
        self.search_results.clear();
        self.selected_result = 0;
        cx.notify();
    }

    /// Process any pending command (called from render where we have window access)
    pub fn process_pending_command(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(command) = self.pending_command.take() {
            let command = command.trim();

            // Handle jump command (from search result selection)
            if command.starts_with("__jump:") {
                if let Ok(item_id) = command
                    .strip_prefix("__jump:")
                    .unwrap_or("0")
                    .parse::<u64>()
                {
                    self.jump_to_item(item_id, window, cx);
                }
            } else if command.starts_with("md ") {
                let name = command.strip_prefix("md ").unwrap_or("Untitled");
                self.create_markdown_note(name.to_string(), window, cx);
            } else if command == "md" {
                self.create_markdown_note("Untitled".to_string(), window, cx);
            }
        }
    }

    /// Jump to and select an item by ID with smooth animation
    fn jump_to_item(&mut self, item_id: u64, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ref board) = self.board {
            if let Some(item) = board.items.iter().find(|i| i.id == item_id) {
                // Get window size for centering
                let bounds = window.bounds();
                let screen_size = bounds.size;

                // Calculate the center of the item in canvas coordinates
                let item_center_x = item.position.0 + item.size.0 / 2.0;
                let item_center_y = item.position.1 + item.size.1 / 2.0;

                // Calculate target offset to center item on screen
                let screen_center_x = f32::from(screen_size.width) / 2.0;
                let screen_center_y = f32::from(screen_size.height) / 2.0;

                let target_offset = point(
                    px(screen_center_x - item_center_x * board.zoom),
                    px(screen_center_y - item_center_y * board.zoom),
                );

                // Start animation from current position to target
                self.pan_animation = Some(PanAnimation {
                    start_offset: board.canvas_offset,
                    target_offset,
                    start_time: Instant::now(),
                    duration: Duration::from_millis(300),
                });

                // Select the item
                self.selected_items.clear();
                self.selected_items.insert(item_id);

                // Trigger first frame
                cx.notify();
            }
        }
    }

    /// Update pan animation, returns true if animation is active
    pub fn update_pan_animation(&mut self) -> bool {
        if let Some(ref anim) = self.pan_animation {
            let elapsed = anim.start_time.elapsed();
            let progress = (elapsed.as_secs_f32() / anim.duration.as_secs_f32()).min(1.0);

            // Ease out cubic for smooth deceleration
            let eased = 1.0 - (1.0 - progress).powi(3);

            if let Some(ref mut board) = self.board {
                // Interpolate between start and target
                let start_x = f32::from(anim.start_offset.x);
                let start_y = f32::from(anim.start_offset.y);
                let target_x = f32::from(anim.target_offset.x);
                let target_y = f32::from(anim.target_offset.y);

                board.canvas_offset = point(
                    px(start_x + (target_x - start_x) * eased),
                    px(start_y + (target_y - start_y) * eased),
                );
            }

            if progress >= 1.0 {
                // Animation complete
                self.pan_animation = None;
                return false;
            }
            return true;
        }
        false
    }

    pub fn execute_command(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let command = self
            .command_palette
            .as_ref()
            .map(|input| input.read(cx).text().to_string())
            .unwrap_or_default();

        self.command_palette = None;

        // Parse command
        let command = command.trim();
        if command.starts_with("md ") {
            let name = command.strip_prefix("md ").unwrap_or("Untitled");
            self.create_markdown_note(name.to_string(), window, cx);
        } else if command == "md" {
            self.create_markdown_note("Untitled".to_string(), window, cx);
        }
        // Add more commands here as needed

        cx.notify();
    }

    fn create_markdown_note(&mut self, name: String, window: &mut Window, cx: &mut Context<Self>) {
        // Get board ID from current view
        let board_id = match &self.view {
            AppView::Board(id) => id.clone(),
            _ => return,
        };

        // Clean up name - remove .md extension if user added it
        let name = name.trim().trim_end_matches(".md").trim();
        let name = if name.is_empty() {
            "Untitled".to_string()
        } else {
            name.to_string()
        };

        if let Some(ref mut board) = self.board {
            // Create markdown file in the board's files directory
            let files_dir = crate::board_index::BoardIndex::board_files_dir(&board_id);
            let _ = std::fs::create_dir_all(&files_dir);

            // Generate safe filename - just use the name, add short suffix if exists
            let safe_name: String = name
                .chars()
                .map(|c| {
                    if c.is_alphanumeric() || c == '-' || c == '_' {
                        c
                    } else {
                        '-'
                    }
                })
                .collect();

            // Try just the name first, add short timestamp suffix if file exists
            let mut filename = format!("{}.md", safe_name);
            let mut path = files_dir.join(&filename);
            if path.exists() {
                let short_id = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
                    % 10000; // Last 4 digits
                filename = format!("{}-{}.md", safe_name, short_id);
                path = files_dir.join(&filename);
            }

            // Create markdown file with title
            let initial_content = format!("# {}\n\n", name);
            let _ = std::fs::write(&path, &initial_content);

            // Add to board at center of visible canvas (accounting for pan/zoom)
            let center_screen = point(px(600.0), px(400.0));
            let canvas_pos = board.screen_to_canvas(center_screen);
            board.add_item(
                canvas_pos,
                crate::types::ItemContent::Markdown {
                    path: path.clone(),
                    title: name.clone(),
                    content: initial_content, // Store actual content for preview
                },
            );

            // Open in preview and immediately switch to edit mode
            self.open_preview(path, window, cx);
            self.toggle_markdown_edit(window, cx);
        }
    }

    pub fn paste(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let clipboard = cx.read_from_clipboard();
        if let Some(item) = clipboard {
            if let Some(text) = item.text() {
                let text = text.trim();
                // Check if it's a URL
                if text.starts_with("http://") || text.starts_with("https://") {
                    // Get center of window for paste position
                    let bounds = window.bounds();
                    let center = point(
                        px(f32::from(bounds.size.width) / 2.0),
                        px(f32::from(bounds.size.height) / 2.0),
                    );
                    if let Some(ref mut board) = self.board {
                        board.add_url(text, center);
                        cx.notify();
                    }
                }
            }
        }
    }

    // ==================== Landing Page Methods ====================

    pub fn create_new_board(&mut self, cx: &mut Context<Self>) {
        let metadata = self.board_index.create_board("Untitled Board".to_string());
        // Open the new board immediately
        self.open_board(metadata.id, cx);
    }

    pub fn open_board(&mut self, id: String, cx: &mut Context<Self>) {
        let board = Board::load(id.clone());
        self.board = Some(board);
        self.board_index.touch_board(&id);
        self.view = AppView::Board(id);
        cx.notify();
    }

    pub fn go_home(&mut self, cx: &mut Context<Self>) {
        // Force save current board before leaving
        if let Some(ref mut board) = self.board {
            board.flush_save();
        }
        self.board = None;
        self.preview = None;
        self.youtube_webviews.clear(); // Clear YouTube WebViews when leaving board
        self.view = AppView::Landing;
        self.selected_items.clear();
        // Reload index to get any changes
        self.board_index = BoardIndex::load();
        cx.notify();
    }

    pub fn start_editing_board(&mut self, id: String, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(meta) = self.board_index.get_board(&id) {
            let name = meta.name.clone();
            let input = cx.new(|cx| InputState::new(window, cx).default_value(name));
            // Focus the input so user can type immediately
            input.update(cx, |state, cx| {
                state.focus(window, cx);
            });
            self.edit_input = Some(input);
            self.editing_board_id = Some(id);
            cx.notify();
        }
    }

    pub fn finish_editing_board(&mut self, cx: &mut Context<Self>) {
        if let Some(ref id) = self.editing_board_id.clone() {
            if let Some(ref input) = self.edit_input {
                let new_name = input.read(cx).value().to_string();
                if !new_name.trim().is_empty() {
                    self.board_index.rename_board(id, new_name);
                }
            }
        }
        self.editing_board_id = None;
        self.edit_input = None;
        cx.notify();
    }

    pub fn cancel_editing(&mut self, cx: &mut Context<Self>) {
        self.editing_board_id = None;
        self.edit_input = None;
        cx.notify();
    }

    pub fn confirm_delete_board(&mut self, id: String, cx: &mut Context<Self>) {
        self.deleting_board_id = Some(id);
        cx.notify();
    }

    pub fn delete_board(&mut self, id: String, cx: &mut Context<Self>) {
        self.board_index.delete_board(&id);
        self.deleting_board_id = None;
        cx.notify();
    }

    pub fn cancel_delete(&mut self, cx: &mut Context<Self>) {
        self.deleting_board_id = None;
        cx.notify();
    }

    // ==================== Board Methods (existing) ====================

    pub fn open_preview(&mut self, path: PathBuf, _window: &mut Window, cx: &mut Context<Self>) {
        // Determine tab type based on extension
        let tab = if path.extension().and_then(|e| e.to_str()) == Some("md") {
            // Load markdown content
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            PreviewTab::Markdown {
                path: path.clone(),
                content,
                editing: false,
                editor: None,
            }
        } else {
            PreviewTab::Pdf {
                path: path.clone(),
                webview: None,
            }
        };

        if let Some(ref mut preview) = self.preview {
            // Check if file is already open in a tab
            if let Some(index) = preview.tabs.iter().position(|t| t.path() == &path) {
                preview.active_tab = index;
            } else {
                // Add new tab
                preview.tabs.push(tab);
                preview.active_tab = preview.tabs.len() - 1;
            }
        } else {
            // Create new preview panel with first tab
            self.preview = Some(PreviewPanel {
                tabs: vec![tab],
                active_tab: 0,
                split: SplitDirection::Vertical,
                size: 0.4,
            });
        }
        cx.notify();
    }

    pub fn ensure_pdf_webview(&mut self, window: &mut Window, cx: &mut App) {
        if let Some(ref mut preview) = self.preview {
            let active_tab = preview.active_tab;

            // Ensure all PDF tabs have their WebViews created, and hide/show based on active
            for (idx, tab) in preview.tabs.iter_mut().enumerate() {
                if let PreviewTab::Pdf { path, webview } = tab {
                    if webview.is_none() {
                        match PdfWebView::new(path.clone(), window, cx) {
                            Ok(wv) => {
                                // Hide if not active tab
                                if idx != active_tab {
                                    wv.webview().update(cx, |view, _| view.hide());
                                }
                                *webview = Some(wv);
                            }
                            Err(e) => {
                                eprintln!("Failed to create PDF WebView: {}", e);
                            }
                        }
                    } else if let Some(wv) = webview {
                        // Show/hide existing webviews based on active tab
                        if idx == active_tab {
                            wv.webview().update(cx, |view, _| view.show());
                        } else {
                            wv.webview().update(cx, |view, _| view.hide());
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
                        eprintln!("Failed to create YouTube WebView: {}", e);
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

    pub fn close_preview(&mut self, cx: &mut Context<Self>) {
        self.preview = None;
        cx.notify();
    }

    pub fn close_tab(&mut self, tab_index: usize, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if tab_index < preview.tabs.len() {
                preview.tabs.remove(tab_index);

                if preview.tabs.is_empty() {
                    // No more tabs, close preview panel
                    self.preview = None;
                } else {
                    // Adjust active tab if needed
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

    pub fn switch_tab(&mut self, tab_index: usize, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if tab_index < preview.tabs.len() {
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
            let active = preview.active_tab;
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
}
