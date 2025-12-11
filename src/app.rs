use crate::board::Board;
use crate::board_index::BoardIndex;
use crate::pdf_webview::PdfWebView;
use crate::youtube_webview::YouTubeWebView;
use gpui::*;
use gpui_component::input::InputState;
use std::collections::HashMap;
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

pub struct PdfTab {
    pub path: PathBuf,
    pub webview: Option<PdfWebView>,
}

pub struct PreviewPanel {
    pub tabs: Vec<PdfTab>,
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
    pub selected_item: Option<u64>,
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

    // YouTube WebViews (keyed by item ID)
    pub youtube_webviews: HashMap<u64, YouTubeWebView>,
    // Which YouTube item is currently active (showing WebView vs thumbnail)
    pub active_youtube_id: Option<u64>,
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
            selected_item: None,
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
            youtube_webviews: HashMap::new(),
            active_youtube_id: None,
        }
    }

    pub fn toggle_shortcuts(&mut self, cx: &mut Context<Self>) {
        self.show_shortcuts = !self.show_shortcuts;
        cx.notify();
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
        // Save current board before leaving
        if let Some(ref board) = self.board {
            board.save();
        }
        self.board = None;
        self.preview = None;
        self.youtube_webviews.clear(); // Clear YouTube WebViews when leaving board
        self.active_youtube_id = None;
        self.view = AppView::Landing;
        self.selected_item = None;
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

    pub fn open_preview(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            // Check if PDF is already open in a tab
            if let Some(index) = preview.tabs.iter().position(|tab| tab.path == path) {
                preview.active_tab = index;
            } else {
                // Add new tab
                preview.tabs.push(PdfTab {
                    path,
                    webview: None,
                });
                preview.active_tab = preview.tabs.len() - 1;
            }
        } else {
            // Create new preview panel with first tab
            self.preview = Some(PreviewPanel {
                tabs: vec![PdfTab {
                    path,
                    webview: None,
                }],
                active_tab: 0,
                split: SplitDirection::Vertical,
                size: 0.4,
            });
        }
        cx.notify();
    }

    pub fn ensure_pdf_webview(&mut self, window: &mut Window, cx: &mut App) {
        if let Some(ref mut preview) = self.preview {
            // Ensure all tabs have their WebViews created
            for tab in preview.tabs.iter_mut() {
                if tab.webview.is_none() {
                    match PdfWebView::new(tab.path.clone(), window, cx) {
                        Ok(webview) => {
                            tab.webview = Some(webview);
                        }
                        Err(e) => {
                            eprintln!("Failed to create PDF WebView: {}", e);
                        }
                    }
                }
            }
        }
    }

    pub fn ensure_youtube_webviews(&mut self, window: &mut Window, cx: &mut App) {
        use crate::types::ItemContent;

        // Only create WebView for the active YouTube item
        if let Some(active_id) = self.active_youtube_id {
            if let Some(ref board) = self.board {
                // Find the active YouTube item
                if let Some(item) = board.items.iter().find(|i| i.id == active_id) {
                    if let ItemContent::YouTube(video_id) = &item.content {
                        // Create WebView if it doesn't exist
                        if !self.youtube_webviews.contains_key(&active_id) {
                            match YouTubeWebView::new(video_id.clone(), window, cx) {
                                Ok(webview) => {
                                    self.youtube_webviews.insert(active_id, webview);
                                }
                                Err(e) => {
                                    eprintln!("Failed to create YouTube WebView: {}", e);
                                }
                            }
                        }
                    }
                }
            }

            // Remove WebViews for non-active items (keep only the active one)
            self.youtube_webviews.retain(|id, _| *id == active_id);
        } else {
            // No active YouTube, clear all WebViews
            self.youtube_webviews.clear();
        }
    }

    pub fn activate_youtube(&mut self, item_id: u64, cx: &mut Context<Self>) {
        self.active_youtube_id = Some(item_id);
        cx.notify();
    }

    pub fn deactivate_youtube(&mut self, cx: &mut Context<Self>) {
        self.active_youtube_id = None;
        self.youtube_webviews.clear();
        cx.notify();
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
