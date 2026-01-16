//! Mouse down event handling - selection, drag/resize initiation, drawing tools.

use crate::app::{Humanboard, SplitDirection};
use crate::constants::{HEADER_HEIGHT, SPLITTER_WIDTH};
use crate::render::dock::DOCK_WIDTH;
use crate::types::{ItemContent, ToolType};
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
                    (f32::from(mouse_pos.x) - splitter_x).abs() < SPLITTER_WIDTH
                }
                SplitDirection::Horizontal => {
                    let splitter_y = (1.0 - preview.size) * f32::from(window_size.height);
                    (f32::from(mouse_pos.y) - splitter_y).abs() < SPLITTER_WIDTH
                }
            };

            if is_on_splitter {
                self.dragging_splitter = true;
                self.splitter_drag_start = Some(mouse_pos);
                cx.notify();
                return;
            }

            // Check if click is in preview panel area - if so, don't handle here
            let in_preview = match preview.split {
                SplitDirection::Vertical => {
                    let preview_start = (1.0 - preview.size) * f32::from(window_size.width);
                    f32::from(mouse_pos.x) > preview_start
                }
                SplitDirection::Horizontal => {
                    let preview_start = (1.0 - preview.size) * f32::from(window_size.height);
                    f32::from(mouse_pos.y) > preview_start
                }
            };
            if in_preview {
                return;
            }
        }

        let header_offset = HEADER_HEIGHT;
        let dock_offset = DOCK_WIDTH;

        // If a drawing tool is selected, prioritize drawing over item selection
        if matches!(
            self.selected_tool,
            ToolType::Text | ToolType::Arrow | ToolType::Shape
        ) {
            self.drawing_start = Some(mouse_pos);
            self.drawing_current = Some(mouse_pos);
            self.selected_items.clear();
            cx.notify();
            return;
        }

        // Check if clicking on an item (in reverse order so top items are checked first)
        let clicked_item_id = board
            .items
            .iter()
            .rev()
            .find(|item| {
                let scaled_x =
                    item.position.0 * board.zoom + f32::from(board.canvas_offset.x) + dock_offset;
                let scaled_y =
                    item.position.1 * board.zoom + f32::from(board.canvas_offset.y) + header_offset;
                let scaled_width = item.size.0 * board.zoom;
                let scaled_height = item.size.1 * board.zoom;

                let mx = f32::from(mouse_pos.x);
                let my = f32::from(mouse_pos.y);

                let in_bounds = mx >= scaled_x
                    && mx <= scaled_x + scaled_width
                    && my >= scaled_y
                    && my <= scaled_y + scaled_height;

                if !in_bounds {
                    return false;
                }

                // For Shape items, only select if clicking on the border (not interior)
                if let ItemContent::Shape { border_width, .. } = &item.content {
                    let border_hit_area = (border_width * board.zoom).max(8.0);
                    let near_left = mx - scaled_x < border_hit_area;
                    let near_right = (scaled_x + scaled_width) - mx < border_hit_area;
                    let near_top = my - scaled_y < border_hit_area;
                    let near_bottom = (scaled_y + scaled_height) - my < border_hit_area;
                    return near_left || near_right || near_top || near_bottom;
                }

                true
            })
            .map(|item| item.id);

        if let Some(item_id) = clicked_item_id {
            // Handle selection with Shift modifier for multi-select
            if event.modifiers.shift {
                if self.selected_items.contains(&item_id) {
                    self.selected_items.remove(&item_id);
                } else {
                    self.selected_items.insert(item_id);
                }
            } else if self.selected_items.contains(&item_id) {
                // Clicked on already-selected item - keep selection for group move
            } else {
                self.selected_items.clear();
                self.selected_items.insert(item_id);
            }

            // Handle double-click for preview or TextBox editing
            if event.click_count == 2 {
                let is_textbox = board
                    .get_item(item_id)
                    .map(|item| matches!(&item.content, ItemContent::TextBox { .. }))
                    .unwrap_or(false);

                if is_textbox {
                    self.start_textbox_editing(item_id, window, cx);
                    return;
                }

                let content_path = board
                    .get_item(item_id)
                    .and_then(|item| match &item.content {
                        ItemContent::Pdf { path, .. } => Some(path.clone()),
                        ItemContent::Markdown { path, .. } => Some(path.clone()),
                        ItemContent::Code { path, .. } => Some(path.clone()),
                        _ => None,
                    });

                if let Some(path) = content_path {
                    self.open_preview(path, window, cx);
                    return;
                }
            }

            // Check if clicking on resize corner (bottom-right)
            let item_info = board
                .get_item(item_id)
                .map(|item| (item.position, item.size, &item.content));

            if let Some((position, size, content)) = item_info {
                let scaled_x =
                    position.0 * board.zoom + f32::from(board.canvas_offset.x) + dock_offset;
                let scaled_y =
                    position.1 * board.zoom + f32::from(board.canvas_offset.y) + header_offset;
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
                    self.resize_start_font_size =
                        if let ItemContent::TextBox { font_size, .. } = content {
                            Some(*font_size)
                        } else {
                            None
                        };
                } else {
                    self.dragging_item = Some(item_id);
                    self.item_drag_offset = Some(point(
                        mouse_pos.x - px(scaled_x),
                        mouse_pos.y - px(scaled_y),
                    ));
                }
            }
            self.focus.force_canvas_focus(window);
        } else {
            // Clicked on empty canvas
            self.focus.force_canvas_focus(window);

            match self.selected_tool {
                ToolType::Select => {
                    self.marquee_start = Some(mouse_pos);
                    self.marquee_current = Some(mouse_pos);
                    if !event.modifiers.shift {
                        self.selected_items.clear();
                    }
                }
                ToolType::Text | ToolType::Arrow | ToolType::Shape => {
                    self.drawing_start = Some(mouse_pos);
                    self.drawing_current = Some(mouse_pos);
                }
            }
        }

        cx.notify();
    }
}
