//! Application state module.
//!
//! Contains the main Humanboard struct and application-level state management.

use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

use gpui::*;

use crate::board::Board;
use crate::pdf_webview::PdfWebView;

/// Maximum number of frame times to keep for FPS calculation
const MAX_FRAME_TIMES: usize = 60;

/// Split direction for the preview panel
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SplitDirection {
    /// Panel on the right side
    Vertical,
    /// Panel on the bottom
    Horizontal,
}

/// A tab in the PDF preview panel
pub struct PdfTab {
    pub path: PathBuf,
    pub webview: Option<PdfWebView>,
}

/// The PDF preview panel state
pub struct PreviewPanel {
    pub tabs: Vec<PdfTab>,
    pub active_tab: usize,
    pub split: SplitDirection,
    /// Size as percentage of window (0.0 to 1.0)
    pub size: f32,
}

impl PreviewPanel {
    /// Create a new preview panel with an initial tab
    pub fn new(path: PathBuf) -> Self {
        Self {
            tabs: vec![PdfTab {
                path,
                webview: None,
            }],
            active_tab: 0,
            split: SplitDirection::Vertical,
            size: 0.4,
        }
    }

    /// Add a new tab or switch to existing tab if path is already open
    pub fn add_or_switch_tab(&mut self, path: PathBuf) -> usize {
        if let Some(index) = self.tabs.iter().position(|tab| tab.path == path) {
            self.active_tab = index;
            index
        } else {
            self.tabs.push(PdfTab {
                path,
                webview: None,
            });
            self.active_tab = self.tabs.len() - 1;
            self.active_tab
        }
    }

    /// Get the active tab
    pub fn active_tab(&self) -> Option<&PdfTab> {
        self.tabs.get(self.active_tab)
    }

    /// Get the active tab mutably
    pub fn active_tab_mut(&mut self) -> Option<&mut PdfTab> {
        self.tabs.get_mut(self.active_tab)
    }
}

/// Main application state
pub struct Humanboard {
    /// The canvas board
    pub board: Board,

    // Mouse interaction state
    pub dragging: bool,
    pub last_mouse_pos: Option<Point<Pixels>>,
    pub dragging_item: Option<u64>,
    pub item_drag_offset: Option<Point<Pixels>>,
    pub resizing_item: Option<u64>,
    pub resize_start_size: Option<(f32, f32)>,
    pub resize_start_pos: Option<Point<Pixels>>,
    pub selected_item: Option<u64>,

    // Splitter drag state
    pub dragging_splitter: bool,
    pub splitter_drag_start: Option<Point<Pixels>>,

    // File drop tracking
    pub last_drop_pos: Option<Point<Pixels>>,
    pub file_drop_rx: Option<Receiver<(Point<Pixels>, Vec<PathBuf>)>>,

    // PDF preview panel
    pub preview: Option<PreviewPanel>,

    // Performance tracking (using VecDeque for O(1) operations)
    frame_times: VecDeque<Duration>,
    pub last_frame: Instant,
    pub frame_count: u64,

    // Focus handling
    pub focus_handle: FocusHandle,
}

impl Humanboard {
    /// Create a new Humanboard instance
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            board: Board::new(),

            dragging: false,
            last_mouse_pos: None,
            dragging_item: None,
            item_drag_offset: None,
            resizing_item: None,
            resize_start_size: None,
            resize_start_pos: None,
            selected_item: None,

            dragging_splitter: false,
            splitter_drag_start: None,

            last_drop_pos: None,
            file_drop_rx: None,

            preview: None,

            frame_times: VecDeque::with_capacity(MAX_FRAME_TIMES + 1),
            last_frame: Instant::now(),
            frame_count: 0,

            focus_handle: cx.focus_handle(),
        }
    }

    /// Open or focus a PDF in the preview panel
    pub fn open_preview(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            preview.add_or_switch_tab(path);
        } else {
            self.preview = Some(PreviewPanel::new(path));
        }
        cx.notify();
    }

    /// Ensure WebViews are created for all tabs
    pub fn ensure_pdf_webview(&mut self, window: &mut Window, cx: &mut App) {
        if let Some(ref mut preview) = self.preview {
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

    /// Close the preview panel
    pub fn close_preview(&mut self, cx: &mut Context<Self>) {
        self.preview = None;
        cx.notify();
    }

    /// Close a specific tab
    pub fn close_tab(&mut self, tab_index: usize, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if tab_index < preview.tabs.len() {
                preview.tabs.remove(tab_index);

                if preview.tabs.is_empty() {
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

    /// Switch to a specific tab
    pub fn switch_tab(&mut self, tab_index: usize, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if tab_index < preview.tabs.len() {
                preview.active_tab = tab_index;
                cx.notify();
            }
        }
    }

    /// Switch to the next tab
    pub fn next_tab(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if !preview.tabs.is_empty() {
                preview.active_tab = (preview.active_tab + 1) % preview.tabs.len();
                cx.notify();
            }
        }
    }

    /// Switch to the previous tab
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

    /// Close the currently active tab
    pub fn close_current_tab(&mut self, cx: &mut Context<Self>) {
        if let Some(ref preview) = self.preview {
            let active = preview.active_tab;
            self.close_tab(active, cx);
        }
    }

    /// Toggle between vertical and horizontal split
    pub fn toggle_split_direction(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            preview.split = match preview.split {
                SplitDirection::Vertical => SplitDirection::Horizontal,
                SplitDirection::Horizontal => SplitDirection::Vertical,
            };
            cx.notify();
        }
    }

    /// Update FPS tracking (O(1) with VecDeque)
    pub fn update_fps(&mut self) {
        let now = Instant::now();
        let frame_time = now.duration_since(self.last_frame);
        self.last_frame = now;
        self.frame_count += 1;

        // O(1) push_back
        self.frame_times.push_back(frame_time);

        // O(1) pop_front when over limit
        while self.frame_times.len() > MAX_FRAME_TIMES {
            self.frame_times.pop_front();
        }
    }

    /// Calculate current FPS
    pub fn calculate_fps(&self) -> f32 {
        if self.frame_times.is_empty() {
            return 0.0;
        }

        let total: Duration = self.frame_times.iter().sum();
        let avg_frame_time = total / self.frame_times.len() as u32;

        if avg_frame_time.as_secs_f32() > 0.0 {
            1.0 / avg_frame_time.as_secs_f32()
        } else {
            0.0
        }
    }

    // WebView handles these internally, but we keep the methods for API compatibility
    pub fn next_page(&mut self, _cx: &mut Context<Self>) {}
    pub fn prev_page(&mut self, _cx: &mut Context<Self>) {}
    pub fn pdf_zoom_in(&mut self, _cx: &mut Context<Self>) {}
    pub fn pdf_zoom_out(&mut self, _cx: &mut Context<Self>) {}
    pub fn pdf_zoom_reset(&mut self, _cx: &mut Context<Self>) {}

    /// Calculate the center of the canvas area (accounts for preview panel)
    pub fn calculate_canvas_center(&self, window: &Window) -> Point<Pixels> {
        let bounds = window.bounds();
        let window_size = bounds.size;

        let (center_x, center_y) = if let Some(ref preview) = self.preview {
            match preview.split {
                SplitDirection::Vertical => {
                    let canvas_width = f32::from(window_size.width) * (1.0 - preview.size);
                    (canvas_width / 2.0, f32::from(window_size.height) / 2.0)
                }
                SplitDirection::Horizontal => {
                    let canvas_height = f32::from(window_size.height) * (1.0 - preview.size);
                    (f32::from(window_size.width) / 2.0, canvas_height / 2.0)
                }
            }
        } else {
            (
                f32::from(window_size.width) / 2.0,
                f32::from(window_size.height) / 2.0,
            )
        };

        point(px(center_x), px(center_y))
    }

    /// Reset all interaction state (useful after completing an action)
    pub fn reset_interaction_state(&mut self) {
        self.dragging = false;
        self.last_mouse_pos = None;
        self.dragging_item = None;
        self.item_drag_offset = None;
        self.resizing_item = None;
        self.resize_start_size = None;
        self.resize_start_pos = None;
        self.dragging_splitter = false;
        self.splitter_drag_start = None;
    }
}
