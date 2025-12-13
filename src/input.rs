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
        // Account for 40px header offset
        let header_offset = 40.0;
        let clicked_item_id = board
            .items
            .iter()
            .rev()
            .find(|item| {
                let scaled_x = item.position.0 * board.zoom + f32::from(board.canvas_offset.x);
                let scaled_y =
                    item.position.1 * board.zoom + f32::from(board.canvas_offset.y) + header_offset;
                let scaled_width = item.size.0 * board.zoom;
                let scaled_height = item.size.1 * board.zoom;

                f32::from(mouse_pos.x) >= scaled_x
                    && f32::from(mouse_pos.x) <= scaled_x + scaled_width
                    && f32::from(mouse_pos.y) >= scaled_y
                    && f32::from(mouse_pos.y) <= scaled_y + scaled_height
            })
            .map(|item| item.id);

        if let Some(item_id) = clicked_item_id {
            // Handle selection with Shift modifier for multi-select
            if event.modifiers.shift {
                // Toggle selection: add if not selected, remove if already selected
                if self.selected_items.contains(&item_id) {
                    self.selected_items.remove(&item_id);
                } else {
                    self.selected_items.insert(item_id);
                }
            } else if self.selected_items.contains(&item_id) {
                // Clicked on an already-selected item - keep the selection for group move
                // (don't clear, don't change anything)
            } else {
                // Clicked on an unselected item - clear and select only this item
                self.selected_items.clear();
                self.selected_items.insert(item_id);
            }

            // Handle double-click for PDF/Markdown preview
            if event.click_count == 2 {
                let content_path = board
                    .get_item(item_id)
                    .and_then(|item| match &item.content {
                        ItemContent::Pdf { path, .. } => Some(path.clone()),
                        ItemContent::Markdown { path, .. } => Some(path.clone()),
                        _ => None,
                    });

                if let Some(path) = content_path {
                    self.open_preview(path, window, cx);
                    return;
                }
            }

            // Check if clicking on resize corner (bottom-right corner)
            // Use get_item for O(1) lookup - extract needed values
            let item_info = board
                .get_item(item_id)
                .map(|item| (item.position, item.size, &item.content));

            if let Some((position, size, _content)) = item_info {
                let is_resizable = true;
                let scaled_x = position.0 * board.zoom + f32::from(board.canvas_offset.x);
                let scaled_y =
                    position.1 * board.zoom + f32::from(board.canvas_offset.y) + header_offset;
                let scaled_width = size.0 * board.zoom;
                let scaled_height = size.1 * board.zoom;

                let corner_x = scaled_x + scaled_width;
                let corner_y = scaled_y + scaled_height;
                let corner_size = 30.0 * board.zoom;

                let in_corner = is_resizable
                    && f32::from(mouse_pos.x) >= corner_x - corner_size
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
            // Clicked on empty canvas - start marquee selection
            self.marquee_start = Some(mouse_pos);
            self.marquee_current = Some(mouse_pos);

            // Clear selection unless shift is held
            if !event.modifiers.shift {
                self.selected_items.clear();
            }
        }

        cx.notify();
    }

    pub fn handle_mouse_up(
        &mut self,
        event: &MouseUpEvent,
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

        // Finalize marquee selection
        if let (Some(start), Some(end)) = (self.marquee_start, self.marquee_current) {
            if let Some(ref board) = self.board {
                let header_offset = 40.0;

                // Calculate marquee bounds in screen space
                let min_x = f32::from(start.x).min(f32::from(end.x));
                let max_x = f32::from(start.x).max(f32::from(end.x));
                let min_y = f32::from(start.y).min(f32::from(end.y));
                let max_y = f32::from(start.y).max(f32::from(end.y));

                // Only select if marquee has some size (not just a click)
                if (max_x - min_x) > 5.0 || (max_y - min_y) > 5.0 {
                    // Find all items that intersect with marquee
                    for item in &board.items {
                        let item_x =
                            item.position.0 * board.zoom + f32::from(board.canvas_offset.x);
                        let item_y = item.position.1 * board.zoom
                            + f32::from(board.canvas_offset.y)
                            + header_offset;
                        let item_w = item.size.0 * board.zoom;
                        let item_h = item.size.1 * board.zoom;

                        // Check if item intersects with marquee rectangle
                        let intersects = !(item_x + item_w < min_x
                            || item_x > max_x
                            || item_y + item_h < min_y
                            || item_y > max_y);

                        if intersects {
                            if event.modifiers.shift {
                                // Toggle selection with shift
                                if self.selected_items.contains(&item.id) {
                                    self.selected_items.remove(&item.id);
                                } else {
                                    self.selected_items.insert(item.id);
                                }
                            } else {
                                self.selected_items.insert(item.id);
                            }
                        }
                    }
                }
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
        self.marquee_start = None;
        self.marquee_current = None;
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

                    // Check if this is a markdown item - if so, maintain aspect ratio
                    let is_markdown = board
                        .get_item(item_id)
                        .map(|item| matches!(&item.content, ItemContent::Markdown { .. }))
                        .unwrap_or(false);

                    let (new_width, new_height) = if is_markdown {
                        // Markdown cards have fixed aspect ratio of 200:36 (5.56:1)
                        const MD_ASPECT_RATIO: f32 = 200.0 / 36.0;
                        // Use width change to determine size, maintain aspect ratio
                        let width = (start_size.0 + delta_x).max(100.0);
                        let height = width / MD_ASPECT_RATIO;
                        (width, height)
                    } else {
                        // Other items can resize freely
                        let width = (start_size.0 + delta_x).max(50.0);
                        let height = (start_size.1 + delta_y).max(50.0);
                        (width, height)
                    };

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
                let header_offset = 40.0; // Account for header bar
                let new_x = (f32::from(event.position.x - offset.x) - canvas_offset_x) / zoom;
                let new_y =
                    (f32::from(event.position.y - offset.y) - canvas_offset_y - header_offset)
                        / zoom;

                // Get current position of dragged item to calculate delta
                let old_pos = board.get_item(item_id).map(|i| i.position);

                if let Some((old_x, old_y)) = old_pos {
                    let delta_x = new_x - old_x;
                    let delta_y = new_y - old_y;

                    // Move all selected items by the same delta
                    if self.selected_items.contains(&item_id) && self.selected_items.len() > 1 {
                        // Group move - move all selected items
                        let selected_ids: Vec<u64> = self.selected_items.iter().copied().collect();
                        for id in selected_ids {
                            if let Some(item) = board.get_item_mut(id) {
                                item.position.0 += delta_x;
                                item.position.1 += delta_y;
                            }
                        }
                    } else {
                        // Single item move
                        if let Some(item) = board.get_item_mut(item_id) {
                            item.position = (new_x, new_y);
                        }
                    }
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
        } else if self.marquee_start.is_some() {
            // Update marquee selection rectangle
            self.marquee_current = Some(event.position);
            cx.notify();
        }
    }

    pub fn handle_scroll(
        &mut self,
        event: &ScrollWheelEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Block canvas scroll when any modal/overlay is open
        if self.command_palette.is_some() || self.show_settings || self.show_shortcuts {
            return;
        }

        // Check if scrolling over preview panel - if so, let it handle its own scroll
        if let Some(ref preview) = self.preview {
            let bounds = window.bounds();
            let in_preview = match preview.split {
                crate::app::SplitDirection::Vertical => {
                    let preview_start = f32::from(bounds.size.width) * (1.0 - preview.size);
                    f32::from(event.position.x) > preview_start
                }
                crate::app::SplitDirection::Horizontal => {
                    let preview_start = f32::from(bounds.size.height) * (1.0 - preview.size);
                    f32::from(event.position.y) > preview_start
                }
            };
            if in_preview {
                return; // Let the preview panel handle scrolling
            }
        }

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
