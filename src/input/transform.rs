//! Canvas transformations - scroll, zoom, coordinate conversion.

use crate::app::Humanboard;
use crate::render::dock::DOCK_WIDTH;
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

        if event.modifiers.platform {
            // Pinch-to-zoom
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
            // Canvas panning
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
