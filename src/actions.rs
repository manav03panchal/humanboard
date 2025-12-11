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
        OpenFile,
        GoHome,
        NewBoard,
        ShowShortcuts,
        Paste,
        CommandPalette
    ]
);

impl Humanboard {
    /// Get the center point of the canvas area (accounting for preview panel)
    fn canvas_center(&self, window: &Window) -> Point<Pixels> {
        let bounds = window.bounds();
        let window_size = bounds.size;

        if let Some(ref preview) = self.preview {
            match preview.split {
                crate::app::SplitDirection::Vertical => {
                    let canvas_width = f32::from(window_size.width) * (1.0 - preview.size);
                    point(
                        px(canvas_width / 2.0),
                        px(f32::from(window_size.height) / 2.0),
                    )
                }
                crate::app::SplitDirection::Horizontal => {
                    let canvas_height = f32::from(window_size.height) * (1.0 - preview.size);
                    point(
                        px(f32::from(window_size.width) / 2.0),
                        px(canvas_height / 2.0),
                    )
                }
            }
        } else {
            point(
                px(f32::from(window_size.width) / 2.0),
                px(f32::from(window_size.height) / 2.0),
            )
        }
    }

    pub fn zoom_in(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let center = self.canvas_center(window);
        if let Some(ref mut board) = self.board {
            if board.zoom_in(center) {
                cx.notify();
            }
        }
    }

    pub fn zoom_out(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let center = self.canvas_center(window);
        if let Some(ref mut board) = self.board {
            if board.zoom_out(center) {
                cx.notify();
            }
        }
    }

    pub fn zoom_reset(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut board) = self.board {
            board.zoom_reset();
            cx.notify();
        }
    }

    pub fn delete_selected(&mut self, cx: &mut Context<Self>) {
        if let Some(selected_id) = self.selected_item {
            if let Some(ref mut board) = self.board {
                board.items.retain(|item| item.id != selected_id);
                self.selected_item = None;
                board.push_history();
                board.save();
                cx.notify();
            }
        }
    }

    pub fn undo(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut board) = self.board {
            if board.undo() {
                self.selected_item = None;
                cx.notify();
            }
        }
    }

    pub fn redo(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut board) = self.board {
            if board.redo() {
                self.selected_item = None;
                cx.notify();
            }
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
