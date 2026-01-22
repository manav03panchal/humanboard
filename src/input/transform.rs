//! Canvas transformations - scroll, zoom, coordinate conversion.

use crate::app::Humanboard;
use crate::constants::{DOCK_WIDTH, HEADER_HEIGHT};
use crate::data::{VirtualScrollState, ROW_HEIGHT};
use crate::types::ItemContent;
use gpui::*;

impl Humanboard {
    /// Convert screen position to canvas position.
    pub fn screen_to_canvas(&self, pos: Point<Pixels>, header_offset: f32) -> Point<Pixels> {
        if let Some(ref board) = self.board {
            let x = (f32::from(pos.x) - DOCK_WIDTH - f32::from(board.canvas_offset.x)) / board.zoom;
            let y =
                (f32::from(pos.y) - header_offset - f32::from(board.canvas_offset.y)) / board.zoom;
            point(px(x), px(y))
        } else {
            pos
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
                return;
            }
        }

        let Some(ref mut board) = self.board else {
            return;
        };

        // Zoom with Command (platform) or Control key
        if event.modifiers.platform || event.modifiers.control {
            let zoom_factor = match event.delta {
                ScrollDelta::Pixels(delta) => 1.0 - f32::from(delta.y) / 500.0,
                ScrollDelta::Lines(delta) => 1.0 - delta.y / 50.0,
            };

            if (zoom_factor - 1.0).abs() > 0.001 {
                if board.zoom_around(zoom_factor, event.position) {
                    cx.notify();
                }
            }
            return;
        }

        // Check if scrolling over a table item - scroll the table instead of canvas
        let scroll_delta_y = match event.delta {
            ScrollDelta::Pixels(delta) => f32::from(delta.y),
            ScrollDelta::Lines(delta) => delta.y * ROW_HEIGHT,
        };

        // Convert screen position to canvas coordinates
        let canvas_x = (f32::from(event.position.x) - DOCK_WIDTH - f32::from(board.canvas_offset.x)) / board.zoom;
        let canvas_y = (f32::from(event.position.y) - HEADER_HEIGHT - f32::from(board.canvas_offset.y)) / board.zoom;

        // Find if mouse is over a table
        let table_item = board.items.iter().find(|item| {
            if !matches!(item.content, ItemContent::Table { .. }) {
                return false;
            }
            let (ix, iy) = item.position;
            let (iw, ih) = item.size;
            canvas_x >= ix && canvas_x <= ix + iw && canvas_y >= iy && canvas_y <= iy + ih
        });

        if let Some(table) = table_item {
            // Scroll the table
            if let ItemContent::Table { data_source_id, .. } = &table.content {
                // Get row count from data source
                let row_count = board.data_sources
                    .get(data_source_id)
                    .map(|ds| ds.row_count())
                    .unwrap_or(0);

                if row_count > 0 {
                    let table_id = table.id;
                    let table_height = table.size.1;

                    // Get or create scroll state for this table
                    let scroll_state = self.table_scroll_states
                        .entry(table_id)
                        .or_insert_with(|| VirtualScrollState::new(table_height));

                    // Update scroll position (negative delta = scroll down)
                    scroll_state.scroll_by(-scroll_delta_y, row_count);
                    cx.notify();
                    return; // Don't pan canvas
                }
            }
        }

        // Default: Canvas panning
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
