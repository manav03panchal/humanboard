//! Drag operations - item dragging, resizing, splitter dragging.
//!
//! ## Performance Notes
//!
//! Mouse move is called very frequently during drag operations (potentially
//! 60+ times per second). Key optimizations:
//! - Early exit for non-drag states
//! - Minimal state updates per move
//! - Batched item position updates for group moves
//!
//! Enable profiling with `cargo build --features profiling` to see timing.

use crate::app::{Humanboard, SplitDirection};
use crate::constants::{DOCK_WIDTH, HEADER_HEIGHT};
use crate::profile_scope;
use crate::types::ItemContent;
use gpui::*;

impl Humanboard {
    pub fn handle_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        profile_scope!("handle_mouse_move");

        self.last_drop_pos = Some(event.position);

        // Handle splitter dragging (canvas/preview split)
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

        // Handle pane splitter dragging (between split panes)
        if self.dragging_pane_splitter {
            if let Some(ref mut preview) = self.preview {
                if preview.is_pane_split {
                    let bounds = window.bounds();
                    let window_width = f32::from(bounds.size.width);
                    let window_height = f32::from(bounds.size.height);
                    let mouse_x = f32::from(event.position.x);
                    let mouse_y = f32::from(event.position.y);

                    let header_height = HEADER_HEIGHT;
                    let footer_height = crate::constants::FOOTER_HEIGHT;
                    let dock_width = DOCK_WIDTH;

                    let (panel_start, panel_size) = match preview.split {
                        SplitDirection::Vertical => {
                            let panel_x =
                                dock_width + (window_width - dock_width) * (1.0 - preview.size);
                            let panel_width = (window_width - dock_width) * preview.size;
                            if preview.pane_split_horizontal {
                                let panel_y = header_height;
                                let panel_height = window_height - header_height - footer_height;
                                (panel_y, panel_height)
                            } else {
                                (panel_x, panel_width)
                            }
                        }
                        SplitDirection::Horizontal => {
                            let panel_y = header_height
                                + (window_height - header_height - footer_height)
                                    * (1.0 - preview.size);
                            let panel_height =
                                (window_height - header_height - footer_height) * preview.size;
                            if preview.pane_split_horizontal {
                                (panel_y, panel_height)
                            } else {
                                (dock_width, window_width - dock_width)
                            }
                        }
                    };

                    let new_ratio = if preview.pane_split_horizontal {
                        ((mouse_y - panel_start) / panel_size).clamp(0.2, 0.8)
                    } else {
                        ((mouse_x - panel_start) / panel_size).clamp(0.2, 0.8)
                    };

                    preview.pane_ratio = new_ratio;
                    cx.notify();
                }
            }
            return;
        }

        let Some(ref mut board) = self.board else {
            return;
        };

        // Handle item resizing
        if let Some(item_id) = self.resizing_item {
            profile_scope!("item_resize");

            if let Some(start_size) = self.resize_start_size {
                if let Some(start_pos) = self.resize_start_pos {
                    let zoom = board.zoom;
                    let delta_x = f32::from(event.position.x - start_pos.x) / zoom;
                    let delta_y = f32::from(event.position.y - start_pos.y) / zoom;

                    let item_type = board.get_item(item_id).map(|item| match &item.content {
                        ItemContent::Markdown { .. } => "markdown",
                        ItemContent::TextBox { .. } => "textbox",
                        ItemContent::Arrow { end_offset, .. } => {
                            if end_offset.0 >= 0.0 && end_offset.1 >= 0.0 {
                                "arrow_pp"
                            } else if end_offset.0 < 0.0 && end_offset.1 >= 0.0 {
                                "arrow_np"
                            } else if end_offset.0 >= 0.0 && end_offset.1 < 0.0 {
                                "arrow_pn"
                            } else {
                                "arrow_nn"
                            }
                        }
                        _ => "other",
                    });

                    let original_font_size = self.resize_start_font_size;

                    let (new_width, new_height) = match item_type.as_deref() {
                        Some("markdown") => {
                            const MD_ASPECT_RATIO: f32 = 200.0 / 36.0;
                            let width = (start_size.0 + delta_x).max(100.0);
                            let height = width / MD_ASPECT_RATIO;
                            (width, height)
                        }
                        Some(arrow_type) if arrow_type.starts_with("arrow_") => {
                            let scale_x = (start_size.0 + delta_x) / start_size.0;
                            let scale_y = (start_size.1 + delta_y) / start_size.1;
                            let scale = ((scale_x + scale_y) / 2.0).max(0.1);
                            let width = (start_size.0 * scale).max(20.0);
                            let height = (start_size.1 * scale).max(20.0);
                            (width, height)
                        }
                        _ => {
                            let width = (start_size.0 + delta_x).max(50.0);
                            let height = (start_size.1 + delta_y).max(50.0);
                            (width, height)
                        }
                    };

                    if let Some(item) = board.get_item_mut(item_id) {
                        let scale = new_height / start_size.1;
                        item.size = (new_width, new_height);

                        if let ItemContent::Arrow { end_offset, .. } = &mut item.content {
                            let sign_x = if end_offset.0 >= 0.0 { 1.0 } else { -1.0 };
                            let sign_y = if end_offset.1 >= 0.0 { 1.0 } else { -1.0 };
                            *end_offset = (new_width * sign_x, new_height * sign_y);
                        }

                        if let ItemContent::TextBox { font_size, .. } = &mut item.content {
                            if let Some(orig_size) = original_font_size {
                                *font_size = (orig_size * scale).max(8.0).min(200.0);
                            }
                        }
                    }
                    board.mark_dirty();
                    cx.notify();
                }
            }
        } else if let Some(item_id) = self.dragging_item {
            // Handle item dragging
            profile_scope!("item_drag");

            if let Some(offset) = self.item_drag_offset {
                let zoom = board.zoom;
                let canvas_offset_x = f32::from(board.canvas_offset.x);
                let canvas_offset_y = f32::from(board.canvas_offset.y);
                let header_offset = HEADER_HEIGHT;
                let new_x =
                    (f32::from(event.position.x - offset.x) - DOCK_WIDTH - canvas_offset_x) / zoom;
                let new_y =
                    (f32::from(event.position.y - offset.y) - canvas_offset_y - header_offset)
                        / zoom;

                let old_pos = board.get_item(item_id).map(|i| i.position);

                if let Some((old_x, old_y)) = old_pos {
                    let delta_x = new_x - old_x;
                    let delta_y = new_y - old_y;

                    if self.selected_items.contains(&item_id) && self.selected_items.len() > 1 {
                        // Group move
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

                board.mark_dirty();
                cx.notify();
            }
        } else if self.dragging {
            // Handle canvas panning
            if let Some(last_pos) = self.last_mouse_pos {
                let delta = event.position - last_pos;
                board.canvas_offset = board.canvas_offset + delta;
                self.last_mouse_pos = Some(event.position);
                board.mark_dirty();
                cx.notify();
            }
        } else if self.marquee_start.is_some() {
            // Update marquee selection rectangle
            self.marquee_current = Some(event.position);
            cx.notify();
        } else if self.drawing_start.is_some() {
            // Update drawing preview position
            self.drawing_current = Some(event.position);
            cx.notify();
        }
    }
}
