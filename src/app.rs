use crate::board::Board;
use crate::pdf::PdfDocument;
use gpui::*;
use std::path::PathBuf;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq)]
pub enum SplitDirection {
    Vertical,   // Panel on the right
    Horizontal, // Panel on the bottom
}

pub struct PreviewPanel {
    pub path: PathBuf,
    pub split: SplitDirection,
    pub size: f32, // 0.0 to 1.0, percentage of window
    pub pdf_doc: Option<PdfDocument>,
    pub current_page_image: Option<PathBuf>,
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
        }
    }

    pub fn open_preview(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        let mut pdf_doc = PdfDocument::open(&path).ok();
        let current_page_image = pdf_doc
            .as_mut()
            .and_then(|pdf| pdf.get_current_page_image());

        self.preview = Some(PreviewPanel {
            path,
            split: SplitDirection::Vertical,
            size: 0.4, // 40% of window
            pdf_doc,
            current_page_image,
        });
        cx.notify();
    }

    fn update_page_image(&mut self) {
        if let Some(ref mut preview) = self.preview {
            if let Some(ref mut pdf) = preview.pdf_doc {
                preview.current_page_image = pdf.get_current_page_image();
            }
        }
    }

    pub fn close_preview(&mut self, cx: &mut Context<Self>) {
        self.preview = None;
        cx.notify();
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

    pub fn next_page(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if let Some(ref mut pdf) = preview.pdf_doc {
                pdf.next_page();
            }
        }
        self.update_page_image();
        cx.notify();
    }

    pub fn prev_page(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if let Some(ref mut pdf) = preview.pdf_doc {
                pdf.prev_page();
            }
        }
        self.update_page_image();
        cx.notify();
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
}
