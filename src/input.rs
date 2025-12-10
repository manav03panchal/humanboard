use crate::app::{Humanboard, SplitDirection};
use crate::types::ItemContent;
use gpui::*;

impl Humanboard {
    pub fn handle_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mouse_pos = event.position;

        // Check if clicking on splitter bar
        if let Some(ref preview) = self.preview {
            let is_on_splitter = match preview.split {
                SplitDirection::Vertical => {
                    // Splitter is a vertical bar on the left edge of preview
                    let splitter_x = (1.0 - preview.size) * 1400.0; // Approximate window width
                    (f32::from(mouse_pos.x) - splitter_x).abs() < 8.0
                }
                SplitDirection::Horizontal => {
                    // Splitter is a horizontal bar on the top edge of preview
                    let splitter_y = (1.0 - preview.size) * 900.0; // Approximate window height
                    (f32::from(mouse_pos.y) - splitter_y).abs() < 8.0
                }
            };

            if is_on_splitter {
                self.dragging_splitter = true;
                self.splitter_drag_start = Some(mouse_pos);
                cx.notify();
                return;
            }
        }

        // Check if clicking on an item (in reverse order so top items are checked first)
        let clicked_item = self
            .board
            .items
            .iter()
            .rev()
            .find(|item| {
                let scaled_x =
                    item.position.0 * self.board.zoom + f32::from(self.board.canvas_offset.x);
                let scaled_y =
                    item.position.1 * self.board.zoom + f32::from(self.board.canvas_offset.y);
                let scaled_width = item.size.0 * self.board.zoom;
                let scaled_height = item.size.1 * self.board.zoom;

                f32::from(mouse_pos.x) >= scaled_x
                    && f32::from(mouse_pos.x) <= scaled_x + scaled_width
                    && f32::from(mouse_pos.y) >= scaled_y
                    && f32::from(mouse_pos.y) <= scaled_y + scaled_height
            })
            .cloned();

        if let Some(item) = clicked_item {
            self.selected_item = Some(item.id);

            // Handle double-click for PDF preview
            if event.click_count == 2 {
                if let ItemContent::Pdf { path, .. } = &item.content {
                    self.open_preview(path.clone(), cx);
                    return;
                }
            }

            let item_id = item.id;

            // Check if clicking on resize corner (bottom-right corner)
            if let Some(item) = self.board.items.iter().find(|i| i.id == item_id) {
                let scaled_x =
                    item.position.0 * self.board.zoom + f32::from(self.board.canvas_offset.x);
                let scaled_y =
                    item.position.1 * self.board.zoom + f32::from(self.board.canvas_offset.y);
                let scaled_width = item.size.0 * self.board.zoom;
                let scaled_height = item.size.1 * self.board.zoom;

                let corner_x = scaled_x + scaled_width;
                let corner_y = scaled_y + scaled_height;
                let corner_size = 30.0 * self.board.zoom;

                let in_corner = f32::from(mouse_pos.x) >= corner_x - corner_size
                    && f32::from(mouse_pos.x) <= corner_x + 5.0
                    && f32::from(mouse_pos.y) >= corner_y - corner_size
                    && f32::from(mouse_pos.y) <= corner_y + 5.0;

                if in_corner {
                    // Start resizing
                    self.resizing_item = Some(item_id);
                    self.resize_start_size = Some(item.size);
                    self.resize_start_pos = Some(mouse_pos);
                } else {
                    // Start dragging the item
                    self.dragging_item = Some(item_id);
                    let item_x =
                        item.position.0 * self.board.zoom + f32::from(self.board.canvas_offset.x);
                    let item_y =
                        item.position.1 * self.board.zoom + f32::from(self.board.canvas_offset.y);

                    self.item_drag_offset =
                        Some(point(mouse_pos.x - px(item_x), mouse_pos.y - px(item_y)));
                }
            }
        } else {
            // Start dragging the canvas and deselect
            self.dragging = true;
            self.last_mouse_pos = Some(mouse_pos);
            self.selected_item = None;
        }

        cx.notify();
    }

    pub fn handle_mouse_up(
        &mut self,
        _event: &MouseUpEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Push to history if we were dragging or resizing an item
        if self.dragging_item.is_some() || self.resizing_item.is_some() {
            self.board.push_history();
        }

        self.dragging = false;
        self.last_mouse_pos = None;
        self.dragging_item = None;
        self.item_drag_offset = None;
        self.resizing_item = None;
        self.resize_start_size = None;
        self.resize_start_pos = None;
        self.dragging_splitter = false;
        self.splitter_drag_start = None;
        cx.notify();
    }

    pub fn handle_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Handle splitter dragging
        if self.dragging_splitter {
            if let Some(ref mut preview) = self.preview {
                let bounds = window.bounds();
                let window_size = bounds.size;

                match preview.split {
                    SplitDirection::Vertical => {
                        let new_size =
                            1.0 - (f32::from(event.position.x) / f32::from(window_size.width));
                        preview.size = new_size.clamp(0.2, 0.8);
                    }
                    SplitDirection::Horizontal => {
                        let new_size =
                            1.0 - (f32::from(event.position.y) / f32::from(window_size.height));
                        preview.size = new_size.clamp(0.2, 0.8);
                    }
                }
                cx.notify();
            }
            return;
        }

        if let Some(item_id) = self.resizing_item {
            // Resizing an item
            if let Some(start_size) = self.resize_start_size {
                if let Some(start_pos) = self.resize_start_pos {
                    if let Some(item) = self.board.items.iter_mut().find(|i| i.id == item_id) {
                        // Calculate delta from start position
                        let delta_x = f32::from(event.position.x - start_pos.x) / self.board.zoom;
                        let delta_y = f32::from(event.position.y - start_pos.y) / self.board.zoom;

                        // Apply delta to original size, with minimum size of 50px
                        let new_width = (start_size.0 + delta_x).max(50.0);
                        let new_height = (start_size.1 + delta_y).max(50.0);

                        item.size = (new_width, new_height);
                        self.board.save();
                        cx.notify();
                    }
                }
            }
        } else if let Some(item_id) = self.dragging_item {
            // Dragging an item
            if let Some(offset) = self.item_drag_offset {
                if let Some(item) = self.board.items.iter_mut().find(|i| i.id == item_id) {
                    // Calculate new position in canvas coordinates
                    let new_x = (f32::from(event.position.x - offset.x)
                        - f32::from(self.board.canvas_offset.x))
                        / self.board.zoom;
                    let new_y = (f32::from(event.position.y - offset.y)
                        - f32::from(self.board.canvas_offset.y))
                        / self.board.zoom;

                    item.position = (new_x, new_y);
                    self.board.save();
                    cx.notify();
                }
            }
        } else if self.dragging {
            // Dragging the canvas
            if let Some(last_pos) = self.last_mouse_pos {
                let delta = event.position - last_pos;
                self.board.canvas_offset = self.board.canvas_offset + delta;
                self.last_mouse_pos = Some(event.position);
                self.board.save();
                cx.notify();
            }
        }
    }

    pub fn handle_scroll(
        &mut self,
        event: &ScrollWheelEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Native PDF view handles its own scrolling, we only handle canvas zoom
        let zoom_delta = match event.delta {
            ScrollDelta::Pixels(delta) => -f32::from(delta.y) / 500.0,
            ScrollDelta::Lines(delta) => -delta.y / 50.0,
        };

        if zoom_delta.abs() > 0.001 {
            let old_zoom = self.board.zoom;
            self.board.zoom = (self.board.zoom * (1.0 + zoom_delta)).clamp(0.1, 10.0);

            let zoom_factor = self.board.zoom / old_zoom;
            let mouse_canvas_x = event.position.x - self.board.canvas_offset.x;
            let mouse_canvas_y = event.position.y - self.board.canvas_offset.y;

            self.board.canvas_offset.x = event.position.x - mouse_canvas_x * zoom_factor;
            self.board.canvas_offset.y = event.position.y - mouse_canvas_y * zoom_factor;

            self.board.save();
            cx.notify();
        }
    }
}
