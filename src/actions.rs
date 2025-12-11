//! Action definitions and handlers.
//!
//! Defines all keyboard-bindable actions and their implementations.

use std::sync::mpsc;

use gpui::*;

use crate::app::Humanboard;

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
    /// Zoom in toward the center of the canvas area
    pub fn zoom_in(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let center = self.calculate_canvas_center(window);
        self.zoom_toward_point(center, 1.2, cx);
    }

    /// Zoom out from the center of the canvas area
    pub fn zoom_out(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let center = self.calculate_canvas_center(window);
        self.zoom_toward_point(center, 1.0 / 1.2, cx);
    }

    /// Reset zoom to 1.0x
    pub fn zoom_reset(&mut self, cx: &mut Context<Self>) {
        self.board.update_zoom(1.0);
        cx.notify();
    }

    /// Delete the currently selected item
    pub fn delete_selected(&mut self, cx: &mut Context<Self>) {
        if let Some(selected_id) = self.selected_item {
            self.board.delete_item(selected_id);
            self.selected_item = None;
            cx.notify();
        }
    }

    /// Undo the last action
    pub fn undo(&mut self, cx: &mut Context<Self>) {
        if self.board.undo() {
            self.selected_item = None;
            cx.notify();
        }
    }

    /// Redo the last undone action
    pub fn redo(&mut self, cx: &mut Context<Self>) {
        if self.board.redo() {
            self.selected_item = None;
            cx.notify();
        }
    }

    /// Open a file picker and add selected files to the canvas
    pub fn open_file(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let paths_rx = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: true,
            prompt: None,
        });

        // Calculate center position for file drop
        let center_pos = self.calculate_canvas_center(window);

        // Use channel to communicate back from async task
        let (tx, rx) = mpsc::channel();

        cx.background_executor()
            .spawn(async move {
                if let Ok(Ok(Some(paths))) = paths_rx.await {
                    let _ = tx.send((center_pos, paths));
                }
            })
            .detach();

        // Store receiver to poll in render cycle
        self.file_drop_rx = Some(rx);
    }

    /// Zoom toward a specific point
    fn zoom_toward_point(&mut self, center: Point<Pixels>, factor: f32, cx: &mut Context<Self>) {
        let old_zoom = self.board.zoom;
        let new_zoom = (old_zoom * factor).clamp(0.1, 10.0);

        let zoom_factor = new_zoom / old_zoom;
        let mouse_canvas_x = center.x - self.board.canvas_offset.x;
        let mouse_canvas_y = center.y - self.board.canvas_offset.y;

        let new_offset = point(
            center.x - mouse_canvas_x * zoom_factor,
            center.y - mouse_canvas_y * zoom_factor,
        );

        self.board.update_zoom(new_zoom);
        self.board.update_offset(new_offset);
        cx.notify();
    }
}
