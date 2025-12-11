//! Input handling module.
//!
//! Handles mouse, keyboard, and scroll events with improved modularity.

use gpui::*;

use crate::app::{Humanboard, SplitDirection};
use crate::types::ItemContent;

/// Splitter hit test threshold in pixels
const SPLITTER_HIT_THRESHOLD: f32 = 16.0;

/// Resize handle size multiplier (relative to zoom)
const RESIZE_HANDLE_SIZE: f32 = 30.0;

/// Minimum item size in canvas units
const MIN_ITEM_SIZE: f32 = 50.0;

impl Humanboard {
    /// Handle mouse down events
    pub fn handle_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mouse_pos = event.position;

        // Check splitter first (highest priority)
        if self.try_start_splitter_drag(mouse_pos, window, cx) {
            return;
        }

        // Check if clicking on an item
        if let Some(item) = self.find_item_at_position(mouse_pos) {
            self.handle_item_click(item.id, item.clone(), event, mouse_pos, cx);
        } else {
            // Start canvas dragging
            self.start_canvas_drag(mouse_pos, cx);
        }
    }

    /// Try to start splitter dragging if mouse is on splitter
    fn try_start_splitter_drag(
        &mut self,
        mouse_pos: Point<Pixels>,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(ref preview) = self.preview else {
            return false;
        };

        let bounds = window.bounds();
        let window_size = bounds.size;

        let is_on_splitter = match preview.split {
            SplitDirection::Vertical => {
                let splitter_x = (1.0 - preview.size) * f32::from(window_size.width);
                (f32::from(mouse_pos.x) - splitter_x).abs() < SPLITTER_HIT_THRESHOLD
            }
            SplitDirection::Horizontal => {
                let splitter_y = (1.0 - preview.size) * f32::from(window_size.height);
                (f32::from(mouse_pos.y) - splitter_y).abs() < SPLITTER_HIT_THRESHOLD
            }
        };

        if is_on_splitter {
            self.dragging_splitter = true;
            self.splitter_drag_start = Some(mouse_pos);
            cx.notify();
            true
        } else {
            false
        }
    }

    /// Find the topmost item at a screen position
    fn find_item_at_position(&self, pos: Point<Pixels>) -> Option<&crate::types::CanvasItem> {
        self.board
            .item_at_screen_pos_precise(f32::from(pos.x), f32::from(pos.y))
    }

    /// Handle click on an item
    fn handle_item_click(
        &mut self,
        item_id: u64,
        item: crate::types::CanvasItem,
        event: &MouseDownEvent,
        mouse_pos: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        self.selected_item = Some(item_id);

        // Handle double-click for PDF preview
        if event.click_count == 2 {
            if let ItemContent::Pdf { path, .. } = &item.content {
                self.open_preview(path.clone(), cx);
                return;
            }
        }

        // Check if clicking on resize corner
        if self.is_in_resize_corner(&item, mouse_pos) {
            self.start_item_resize(item_id, item.size, mouse_pos);
        } else {
            self.start_item_drag(item_id, &item, mouse_pos);
        }

        cx.notify();
    }

    /// Check if mouse is in the resize corner of an item
    fn is_in_resize_corner(
        &self,
        item: &crate::types::CanvasItem,
        mouse_pos: Point<Pixels>,
    ) -> bool {
        let scaled_x = item.position.0 * self.board.zoom + f32::from(self.board.canvas_offset.x);
        let scaled_y = item.position.1 * self.board.zoom + f32::from(self.board.canvas_offset.y);
        let scaled_width = item.size.0 * self.board.zoom;
        let scaled_height = item.size.1 * self.board.zoom;

        let corner_x = scaled_x + scaled_width;
        let corner_y = scaled_y + scaled_height;
        let corner_size = RESIZE_HANDLE_SIZE * self.board.zoom;

        f32::from(mouse_pos.x) >= corner_x - corner_size
            && f32::from(mouse_pos.x) <= corner_x + 5.0
            && f32::from(mouse_pos.y) >= corner_y - corner_size
            && f32::from(mouse_pos.y) <= corner_y + 5.0
    }

    /// Start resizing an item
    fn start_item_resize(&mut self, item_id: u64, size: (f32, f32), mouse_pos: Point<Pixels>) {
        self.resizing_item = Some(item_id);
        self.resize_start_size = Some(size);
        self.resize_start_pos = Some(mouse_pos);
    }

    /// Start dragging an item
    fn start_item_drag(
        &mut self,
        item_id: u64,
        item: &crate::types::CanvasItem,
        mouse_pos: Point<Pixels>,
    ) {
        self.dragging_item = Some(item_id);

        let item_screen_x =
            item.position.0 * self.board.zoom + f32::from(self.board.canvas_offset.x);
        let item_screen_y =
            item.position.1 * self.board.zoom + f32::from(self.board.canvas_offset.y);

        self.item_drag_offset = Some(point(
            mouse_pos.x - px(item_screen_x),
            mouse_pos.y - px(item_screen_y),
        ));
    }

    /// Start dragging the canvas
    fn start_canvas_drag(&mut self, mouse_pos: Point<Pixels>, cx: &mut Context<Self>) {
        self.dragging = true;
        self.last_mouse_pos = Some(mouse_pos);
        self.selected_item = None;
        cx.notify();
    }

    /// Handle mouse up events
    pub fn handle_mouse_up(
        &mut self,
        _event: &MouseUpEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Record history if we were dragging or resizing an item
        if self.dragging_item.is_some() || self.resizing_item.is_some() {
            self.board.push_history();
        }

        // Reset all interaction state
        self.reset_interaction_state();
        cx.notify();
    }

    /// Handle mouse move events
    pub fn handle_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Always track mouse position for file drops
        self.last_drop_pos = Some(event.position);

        if self.dragging_splitter {
            self.handle_splitter_drag(event.position, window, cx);
        } else if let Some(item_id) = self.resizing_item {
            self.handle_item_resize(item_id, event.position, cx);
        } else if let Some(item_id) = self.dragging_item {
            self.handle_item_drag(item_id, event.position, cx);
        } else if self.dragging {
            self.handle_canvas_drag(event.position, cx);
        }
    }

    /// Handle splitter dragging
    fn handle_splitter_drag(
        &mut self,
        pos: Point<Pixels>,
        window: &Window,
        cx: &mut Context<Self>,
    ) {
        let Some(ref mut preview) = self.preview else {
            return;
        };

        let bounds = window.bounds();
        let window_size = bounds.size;

        let new_size = match preview.split {
            SplitDirection::Vertical => 1.0 - (f32::from(pos.x) / f32::from(window_size.width)),
            SplitDirection::Horizontal => 1.0 - (f32::from(pos.y) / f32::from(window_size.height)),
        };

        preview.size = new_size.clamp(0.2, 0.8);
        cx.notify();
    }

    /// Handle item resizing
    fn handle_item_resize(&mut self, item_id: u64, pos: Point<Pixels>, cx: &mut Context<Self>) {
        let Some(start_size) = self.resize_start_size else {
            return;
        };
        let Some(start_pos) = self.resize_start_pos else {
            return;
        };

        let delta_x = f32::from(pos.x - start_pos.x) / self.board.zoom;
        let delta_y = f32::from(pos.y - start_pos.y) / self.board.zoom;

        let new_width = (start_size.0 + delta_x).max(MIN_ITEM_SIZE);
        let new_height = (start_size.1 + delta_y).max(MIN_ITEM_SIZE);

        self.board.update_item_size(item_id, new_width, new_height);
        cx.notify();
    }

    /// Handle item dragging
    fn handle_item_drag(&mut self, item_id: u64, pos: Point<Pixels>, cx: &mut Context<Self>) {
        let Some(offset) = self.item_drag_offset else {
            return;
        };

        let new_x =
            (f32::from(pos.x - offset.x) - f32::from(self.board.canvas_offset.x)) / self.board.zoom;
        let new_y =
            (f32::from(pos.y - offset.y) - f32::from(self.board.canvas_offset.y)) / self.board.zoom;

        self.board.update_item_position(item_id, new_x, new_y);
        cx.notify();
    }

    /// Handle canvas dragging (panning)
    fn handle_canvas_drag(&mut self, pos: Point<Pixels>, cx: &mut Context<Self>) {
        let Some(last_pos) = self.last_mouse_pos else {
            return;
        };

        let delta = pos - last_pos;
        let new_offset = self.board.canvas_offset + delta;

        self.board.update_offset(new_offset);
        self.last_mouse_pos = Some(pos);
        cx.notify();
    }

    /// Handle scroll wheel events
    pub fn handle_scroll(
        &mut self,
        event: &ScrollWheelEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if event.modifiers.platform {
            self.handle_zoom_scroll(event, cx);
        } else {
            self.handle_pan_scroll(event, cx);
        }
    }

    /// Handle zoom via scroll (Cmd + scroll)
    fn handle_zoom_scroll(&mut self, event: &ScrollWheelEvent, cx: &mut Context<Self>) {
        let zoom_delta = match event.delta {
            ScrollDelta::Pixels(delta) => -f32::from(delta.y) / 500.0,
            ScrollDelta::Lines(delta) => -delta.y / 50.0,
        };

        if zoom_delta.abs() > 0.001 {
            let old_zoom = self.board.zoom;
            let new_zoom = (old_zoom * (1.0 + zoom_delta)).clamp(0.1, 10.0);

            // Zoom toward mouse position
            let zoom_factor = new_zoom / old_zoom;
            let mouse_canvas_x = event.position.x - self.board.canvas_offset.x;
            let mouse_canvas_y = event.position.y - self.board.canvas_offset.y;

            let new_offset = point(
                event.position.x - mouse_canvas_x * zoom_factor,
                event.position.y - mouse_canvas_y * zoom_factor,
            );

            self.board.update_zoom(new_zoom);
            self.board.update_offset(new_offset);
            cx.notify();
        }
    }

    /// Handle pan via scroll (normal scroll)
    fn handle_pan_scroll(&mut self, event: &ScrollWheelEvent, cx: &mut Context<Self>) {
        let delta = match event.delta {
            ScrollDelta::Pixels(delta) => delta,
            ScrollDelta::Lines(delta) => point(px(delta.x * 20.0), px(delta.y * 20.0)),
        };

        let new_offset = point(
            self.board.canvas_offset.x + delta.x,
            self.board.canvas_offset.y + delta.y,
        );

        self.board.update_offset(new_offset);
        cx.notify();
    }
}
