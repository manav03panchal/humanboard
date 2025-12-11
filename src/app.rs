use crate::board::Board;
use crate::pdf_webview::PdfWebView;
use gpui::*;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

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
    pub board: Board,
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
}

impl Humanboard {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let board = Board::new();

        Self {
            board,
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
        }
    }

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
