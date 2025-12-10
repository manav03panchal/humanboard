use crate::app::Humanboard;
use gpui::*;

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
        PrevPage
    ]
);

impl Humanboard {
    pub fn zoom_in(&mut self, cx: &mut Context<Self>) {
        let center_x = px(700.0);
        let center_y = px(450.0);

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

    pub fn zoom_out(&mut self, cx: &mut Context<Self>) {
        let center_x = px(700.0);
        let center_y = px(450.0);

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
}
