use crate::app::{Humanboard, SplitDirection};
use crate::types::ItemContent;
use gpui::*;

impl Humanboard {
    pub fn handle_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(ref board) = self.board else { return };
        let mouse_pos = event.position;

        // Check if clicking on splitter bar
        if let Some(ref preview) = self.preview {
            let bounds = window.bounds();
            let window_size = bounds.size;

            let is_on_splitter = match preview.split {
                SplitDirection::Vertical => {
                    let splitter_x = (1.0 - preview.size) * f32::from(window_size.width);
                    (f32::from(mouse_pos.x) - splitter_x).abs() < 16.0
                }
                SplitDirection::Horizontal => {
                    let splitter_y = (1.0 - preview.size) * f32::from(window_size.height);
                    (f32::from(mouse_pos.y) - splitter_y).abs() < 16.0
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
        // Extract only the ID to avoid cloning the entire item
        let clicked_item_id = board
            .items
            .iter()
            .rev()
            .find(|item| {
                let scaled_x = item.position.0 * board.zoom + f32::from(board.canvas_offset.x);
                let scaled_y = item.position.1 * board.zoom + f32::from(board.canvas_offset.y);
                let scaled_width = item.size.0 * board.zoom;
                let scaled_height = item.size.1 * board.zoom;

                f32::from(mouse_pos.x) >= scaled_x
                    && f32::from(mouse_pos.x) <= scaled_x + scaled_width
                    && f32::from(mouse_pos.y) >= scaled_y
                    && f32::from(mouse_pos.y) <= scaled_y + scaled_height
            })
            .map(|item| item.id);

        if let Some(item_id) = clicked_item_id {
            self.selected_item = Some(item_id);

            // Handle double-click for PDF preview
            if event.click_count == 2 {
                let pdf_path = board.get_item(item_id).and_then(|item| {
                    if let ItemContent::Pdf { path, .. } = &item.content {
                        Some(path.clone())
                    } else {
                        None
                    }
                });

                if let Some(path) = pdf_path {
                    self.open_preview(path, cx);
                    return;
                }
            }

            // Check if clicking on resize corner (bottom-right corner)
            // Use get_item for O(1) lookup - extract needed values
            let item_info = board
                .get_item(item_id)
                .map(|item| (item.position, item.size));

            if let Some((position, size)) = item_info {
                let scaled_x = position.0 * board.zoom + f32::from(board.canvas_offset.x);
                let scaled_y = position.1 * board.zoom + f32::from(board.canvas_offset.y);
                let scaled_width = size.0 * board.zoom;
                let scaled_height = size.1 * board.zoom;

                let corner_x = scaled_x + scaled_width;
                let corner_y = scaled_y + scaled_height;
                let corner_size = 30.0 * board.zoom;

                let in_corner = f32::from(mouse_pos.x) >= corner_x - corner_size
                    && f32::from(mouse_pos.x) <= corner_x + 5.0
                    && f32::from(mouse_pos.y) >= corner_y - corner_size
                    && f32::from(mouse_pos.y) <= corner_y + 5.0;

                if in_corner {
                    self.resizing_item = Some(item_id);
                    self.resize_start_size = Some(size);
                    self.resize_start_pos = Some(mouse_pos);
                } else {
                    self.dragging_item = Some(item_id);
                    self.item_drag_offset = Some(point(
                        mouse_pos.x - px(scaled_x),
                        mouse_pos.y - px(scaled_y),
                    ));
                }
            }
        } else {
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
        // Only push history on mouse up if we were dragging/resizing
        let was_modifying = self.dragging_item.is_some() || self.resizing_item.is_some();

        if was_modifying {
            if let Some(ref mut board) = self.board {
                board.push_history();
                // Force save after drag/resize completes
                board.flush_save();
            }
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
        self.last_drop_pos = Some(event.position);

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

        let Some(ref mut board) = self.board else {
            return;
        };

        if let Some(item_id) = self.resizing_item {
            if let Some(start_size) = self.resize_start_size {
                if let Some(start_pos) = self.resize_start_pos {
                    // Capture values before mutable borrow
                    let zoom = board.zoom;
                    let delta_x = f32::from(event.position.x - start_pos.x) / zoom;
                    let delta_y = f32::from(event.position.y - start_pos.y) / zoom;
                    let new_width = (start_size.0 + delta_x).max(50.0);
                    let new_height = (start_size.1 + delta_y).max(50.0);

                    // Use get_item_mut for O(1) lookup
                    if let Some(item) = board.get_item_mut(item_id) {
                        item.size = (new_width, new_height);
                    }
                    // Mark dirty but don't save immediately (debounced)
                    board.mark_dirty();
                    cx.notify();
                }
            }
        } else if let Some(item_id) = self.dragging_item {
            if let Some(offset) = self.item_drag_offset {
                // Capture values before mutable borrow
                let zoom = board.zoom;
                let canvas_offset_x = f32::from(board.canvas_offset.x);
                let canvas_offset_y = f32::from(board.canvas_offset.y);
                let new_x = (f32::from(event.position.x - offset.x) - canvas_offset_x) / zoom;
                let new_y = (f32::from(event.position.y - offset.y) - canvas_offset_y) / zoom;

                // Use get_item_mut for O(1) lookup
                if let Some(item) = board.get_item_mut(item_id) {
                    item.position = (new_x, new_y);
                }
                // Mark dirty but don't save immediately (debounced)
                board.mark_dirty();
                cx.notify();
            }
        } else if self.dragging {
            if let Some(last_pos) = self.last_mouse_pos {
                let delta = event.position - last_pos;
                board.canvas_offset = board.canvas_offset + delta;
                self.last_mouse_pos = Some(event.position);
                // Mark dirty but don't save immediately (debounced)
                board.mark_dirty();
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
        let Some(ref mut board) = self.board else {
            return;
        };

        if event.modifiers.platform {
            // Pinch-to-zoom: calculate zoom factor from scroll delta
            let zoom_factor = match event.delta {
                ScrollDelta::Pixels(delta) => 1.0 - f32::from(delta.y) / 500.0,
                ScrollDelta::Lines(delta) => 1.0 - delta.y / 50.0,
            };

            if (zoom_factor - 1.0).abs() > 0.001 {
                if board.zoom_around(zoom_factor, event.position) {
                    cx.notify();
                }
            }
        } else {
            match event.delta {
                ScrollDelta::Pixels(delta) => {
                    board.canvas_offset.x += delta.x;
                    board.canvas_offset.y += delta.y;
                    board.mark_dirty();
                    cx.notify();
                }
                ScrollDelta::Lines(delta) => {
                    board.canvas_offset.x += px(delta.x * 20.0);
                    board.canvas_offset.y += px(delta.y * 20.0);
                    board.mark_dirty();
                    cx.notify();
                }
            }
        }
    }
}
