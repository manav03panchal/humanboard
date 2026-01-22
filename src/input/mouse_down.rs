//! Mouse down event handling - selection, drag/resize initiation, drawing tools.
//!
//! ## Performance Notes
//!
//! Mouse down is a hot path during user interaction. Key optimizations:
//! - O(log n) hit testing via R-tree spatial index
//! - Coordinate transformations for zoom/pan
//!
//! Enable profiling with `cargo build --features profiling` to see timing.

use crate::app::{Humanboard, SplitDirection};
use crate::constants::{DOCK_WIDTH, HEADER_HEIGHT, SPLITTER_WIDTH};
use crate::profile_scope;
use crate::types::{ItemContent, ToolType};
use gpui::*;

impl Humanboard {
    pub fn handle_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        profile_scope!("handle_mouse_down");

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

        // Check if clicking on an item using spatial index for O(log n) lookup
        profile_scope!("hit_test_items");

        // Convert mouse position to canvas coordinates for spatial query
        // Mouse coordinates are window-relative. Items are rendered within canvas_area
        // which is offset by dock_width from the left edge, so we must subtract it.
        // Formula: canvas_pos = (screen_pos - dock - canvas_offset) / zoom
        let canvas_x = (f32::from(mouse_pos.x) - dock_offset - f32::from(board.canvas_offset.x)) / board.zoom;
        let canvas_y = (f32::from(mouse_pos.y) - header_offset - f32::from(board.canvas_offset.y)) / board.zoom;

        // Get candidate items from spatial index (O(log n))
        let candidates: std::collections::HashSet<u64> = board
            .query_items_at_point(canvas_x, canvas_y)
            .into_iter()
            .collect();

        // Check candidates in reverse z-order (front to back) for the topmost hit
        let clicked_item_id = board
            .items
            .iter()
            .rev()
            .filter(|item| candidates.contains(&item.id))
            .find(|item| {
                // For Shape items, only select if clicking on the border (not interior)
                if let ItemContent::Shape { border_width, .. } = &item.content {
                    let scaled_x =
                        item.position.0 * board.zoom + f32::from(board.canvas_offset.x) + dock_offset;
                    let scaled_y =
                        item.position.1 * board.zoom + f32::from(board.canvas_offset.y) + header_offset;
                    let scaled_width = item.size.0 * board.zoom;
                    let scaled_height = item.size.1 * board.zoom;

                    let mx = f32::from(mouse_pos.x);
                    let my = f32::from(mouse_pos.y);

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

                // Check for table items - open in preview panel
                if let Some(item) = board.get_item(item_id) {
                    if let ItemContent::Table { data_source_id, .. } = &item.content {
                        // Get table name from data source (CSV filename)
                        let name = board.data_sources.get(data_source_id)
                            .and_then(|ds| ds.file_path())
                            .and_then(|p| p.file_stem())
                            .and_then(|n| n.to_str())
                            .unwrap_or("Table")
                            .to_string();
                        self.open_table_preview(*data_source_id, name, window, cx);
                        return;
                    }
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
                ToolType::Table | ToolType::Chart => {
                    // Table and Chart creation handled via dedicated UI
                    // Clicking on canvas just places a default item
                    self.drawing_start = Some(mouse_pos);
                    self.drawing_current = Some(mouse_pos);
                }
            }
        }

        cx.notify();
    }
}
