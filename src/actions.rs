use crate::app::Humanboard;
use gpui::*;
use std::sync::mpsc;

// Action definitions
actions!(
    humanboard,
    [
        Quit,
        ZoomIn,
        ZoomOut,
        ZoomReset,
        DeleteSelected,
        Undo,
        Redo,
        ClosePreview,
        ToggleSplit,
        NextPage,
        PrevPage,
        PdfZoomIn,
        PdfZoomOut,
        PdfZoomReset,
        NextTab,
        PrevTab,
        CloseTab,
        OpenFile
    ]
);

impl Humanboard {
    pub fn zoom_in(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let bounds = window.bounds();
        let window_size = bounds.size;

        // Calculate center point based on canvas area (accounting for preview panel if open)
        let (center_x, center_y) = if let Some(ref preview) = self.preview {
            match preview.split {
                crate::app::SplitDirection::Vertical => {
                    // Canvas is on the left side
                    let canvas_width = f32::from(window_size.width) * (1.0 - preview.size);
                    (
                        px(canvas_width / 2.0),
                        px(f32::from(window_size.height) / 2.0),
                    )
                }
                crate::app::SplitDirection::Horizontal => {
                    // Canvas is on the top
                    let canvas_height = f32::from(window_size.height) * (1.0 - preview.size);
                    (
                        px(f32::from(window_size.width) / 2.0),
                        px(canvas_height / 2.0),
                    )
                }
            }
        } else {
            // No preview panel, use full window center
            (
                px(f32::from(window_size.width) / 2.0),
                px(f32::from(window_size.height) / 2.0),
            )
        };

        let old_zoom = self.board.zoom;
        self.board.zoom = (self.board.zoom * 1.2).clamp(0.1, 10.0);

        let zoom_factor = self.board.zoom / old_zoom;
        let mouse_canvas_x = center_x - self.board.canvas_offset.x;
        let mouse_canvas_y = center_y - self.board.canvas_offset.y;

        self.board.canvas_offset.x = center_x - mouse_canvas_x * zoom_factor;
        self.board.canvas_offset.y = center_y - mouse_canvas_y * zoom_factor;

        self.board.save();
        cx.notify();
    }

    pub fn zoom_out(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let bounds = window.bounds();
        let window_size = bounds.size;

        // Calculate center point based on canvas area (accounting for preview panel if open)
        let (center_x, center_y) = if let Some(ref preview) = self.preview {
            match preview.split {
                crate::app::SplitDirection::Vertical => {
                    // Canvas is on the left side
                    let canvas_width = f32::from(window_size.width) * (1.0 - preview.size);
                    (
                        px(canvas_width / 2.0),
                        px(f32::from(window_size.height) / 2.0),
                    )
                }
                crate::app::SplitDirection::Horizontal => {
                    // Canvas is on the top
                    let canvas_height = f32::from(window_size.height) * (1.0 - preview.size);
                    (
                        px(f32::from(window_size.width) / 2.0),
                        px(canvas_height / 2.0),
                    )
                }
            }
        } else {
            // No preview panel, use full window center
            (
                px(f32::from(window_size.width) / 2.0),
                px(f32::from(window_size.height) / 2.0),
            )
        };

        let old_zoom = self.board.zoom;
        self.board.zoom = (self.board.zoom / 1.2).clamp(0.1, 10.0);

        let zoom_factor = self.board.zoom / old_zoom;
        let mouse_canvas_x = center_x - self.board.canvas_offset.x;
        let mouse_canvas_y = center_y - self.board.canvas_offset.y;

        self.board.canvas_offset.x = center_x - mouse_canvas_x * zoom_factor;
        self.board.canvas_offset.y = center_y - mouse_canvas_y * zoom_factor;

        self.board.save();
        cx.notify();
    }

    pub fn zoom_reset(&mut self, cx: &mut Context<Self>) {
        self.board.zoom = 1.0;
        self.board.save();
        cx.notify();
    }

    pub fn delete_selected(&mut self, cx: &mut Context<Self>) {
        if let Some(selected_id) = self.selected_item {
            self.board.items.retain(|item| item.id != selected_id);
            self.selected_item = None;
            self.board.push_history();
            self.board.save();
            cx.notify();
        }
    }

    pub fn undo(&mut self, cx: &mut Context<Self>) {
        if self.board.undo() {
            self.selected_item = None;
            cx.notify();
        }
    }

    pub fn redo(&mut self, cx: &mut Context<Self>) {
        if self.board.redo() {
            self.selected_item = None;
            cx.notify();
        }
    }

    pub fn open_file(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Show file picker for multiple files
        let paths_rx = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: true,
            prompt: None,
        });

        // Get window center for default drop position
        let bounds = window.bounds();
        let window_size = bounds.size;
        let center_x = f32::from(window_size.width) / 2.0;
        let center_y = f32::from(window_size.height) / 2.0;

        // Workaround for GPUI async limitations: use a channel to communicate back
        let center_pos = point(px(center_x), px(center_y));
        let (tx, rx) = mpsc::channel();

        // Spawn background task to wait for file selection
        cx.background_executor()
            .spawn(async move {
                if let Ok(Ok(Some(paths))) = paths_rx.await {
                    let _ = tx.send((center_pos, paths));
                }
            })
            .detach();

        // Store the receiver - we'll poll it in the render cycle
        self.file_drop_rx = Some(rx);
    }
}
