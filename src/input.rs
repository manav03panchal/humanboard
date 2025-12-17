//! Input Handling Module
//!
//! This module handles mouse and scroll events on the canvas,
//! including item selection, dragging, resizing, and drawing.

use crate::app::{Humanboard, SplitDirection};
use crate::hit_testing::{HitTestContentType, HitTestItem, HitTestResult, ItemHitArea, PreviewSplit};
use crate::types::{ArrowHead, ItemContent, ShapeType, ToolType};
use gpui::*;
use tracing::{debug, trace};

impl Humanboard {
    pub fn handle_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mouse_pos = event.position;
        let bounds = window.bounds();
        let window_size = bounds.size;

        // Extract data from board for hit testing (scoped borrow)
        let (hit_test_items, canvas_offset, zoom) = {
            let Some(ref board) = self.board else { return };
            let items: Vec<HitTestItem> = board.items.iter().map(|item| {
                HitTestItem {
                    id: item.id,
                    position: item.position,
                    size: item.size,
                    content_type: HitTestContentType::from_content(&item.content),
                }
            }).collect();
            (items, board.canvas_offset, board.zoom)
        };

        // Build preview split info for hit testing
        let preview_split = self.preview.as_ref().map(|preview| {
            match preview.split {
                SplitDirection::Vertical => {
                    let x = (1.0 - preview.size) * f32::from(window_size.width);
                    PreviewSplit::Vertical { x, width: preview.size * f32::from(window_size.width) }
                }
                SplitDirection::Horizontal => {
                    let y = (1.0 - preview.size) * f32::from(window_size.height);
                    PreviewSplit::Horizontal { y, height: preview.size * f32::from(window_size.height) }
                }
            }
        });

        // Perform hit test using the structured hit tester
        let hit_result = self.hit_tester.hit_test(
            mouse_pos,
            hit_test_items.into_iter(),
            canvas_offset,
            zoom,
            window_size,
            preview_split,
        );

        trace!("Hit test result: {:?}", hit_result);

        // Handle hit test results
        match hit_result {
            HitTestResult::Header => {
                // Header clicks handled by header UI
                return;
            }
            HitTestResult::Dock => {
                // Dock clicks handled by dock UI
                return;
            }
            HitTestResult::Splitter => {
                debug!("Starting splitter drag");
                self.dragging_splitter = true;
                self.splitter_drag_start = Some(mouse_pos);
                cx.notify();
                return;
            }
            HitTestResult::PreviewPanel => {
                // Preview panel handles its own clicks
                return;
            }
            HitTestResult::Canvas => {
                // Clicking on canvas - reset focus to canvas
                self.focus.force_canvas_focus(window);

                // Handle drawing tools or marquee selection
                self.handle_canvas_click(event, mouse_pos, cx);
            }
            HitTestResult::Item(item_hit) => {
                let item_id = item_hit.item_id;

                // If we're already editing this textbox, don't process further clicks
                // (prevents clicks from stealing focus from input)
                if self.editing_textbox_id == Some(item_id) {
                    return;
                }

                // If we're editing a DIFFERENT textbox, finish that edit first
                if self.editing_textbox_id.is_some() {
                    self.finish_textbox_editing_with_window(window, cx);
                }

                // Handle selection with Shift modifier for multi-select
                if event.modifiers.shift {
                    if self.selected_items.contains(&item_id) {
                        self.selected_items.remove(&item_id);
                    } else {
                        self.selected_items.insert(item_id);
                    }
                } else if !self.selected_items.contains(&item_id) {
                    self.selected_items.clear();
                    self.selected_items.insert(item_id);
                }

                // Check if clicking on resize corner FIRST - this takes priority
                // so users can resize textboxes without triggering edit mode
                if item_hit.area == ItemHitArea::ResizeCorner {
                    debug!("Click on resize corner of item {}", item_id);
                    self.handle_item_interaction(item_id, item_hit.area, mouse_pos);
                    cx.notify();
                    return;
                }

                // Single click on textbox body starts editing
                if let Some(ref board) = self.board {
                    let is_textbox = board
                        .get_item(item_id)
                        .map(|item| matches!(&item.content, ItemContent::TextBox { .. }))
                        .unwrap_or(false);

                    if is_textbox {
                        debug!("Single click on textbox {} body, starting edit", item_id);
                        self.start_textbox_editing(item_id, window, cx);
                        return;
                    }
                }

                // Double-click for preview (PDF, Markdown, Code)
                if event.click_count == 2 {
                    if self.handle_double_click(item_id, window, cx) {
                        return;
                    }
                }

                // Handle body drag for non-textbox items
                self.handle_item_interaction(item_id, item_hit.area, mouse_pos);
            }
        }

        cx.notify();
    }

    /// Handle item resize or drag interaction
    fn handle_item_interaction(
        &mut self,
        item_id: u64,
        area: ItemHitArea,
        mouse_pos: Point<Pixels>,
    ) {
        let Some(ref board) = self.board else { return };

        match area {
            ItemHitArea::ResizeCorner => {
                debug!("Starting resize of item {}", item_id);
                if let Some(item) = board.get_item(item_id) {
                    self.resizing_item = Some(item_id);
                    self.resize_start_size = Some(item.size);
                    self.resize_start_pos = Some(mouse_pos);
                    self.resize_start_font_size =
                        if let ItemContent::TextBox { font_size, .. } = &item.content {
                            Some(*font_size)
                        } else {
                            None
                        };
                }
            }
            ItemHitArea::Body | ItemHitArea::ShapeBorder => {
                debug!("Starting drag of item {}", item_id);
                if let Some(item) = board.get_item(item_id) {
                    // Convert item canvas position to screen position
                    let item_screen_pos = self.hit_tester.canvas_to_screen(
                        point(px(item.position.0), px(item.position.1)),
                        board.canvas_offset,
                        board.zoom,
                    );

                    self.dragging_item = Some(item_id);
                    self.item_drag_offset = Some(point(
                        mouse_pos.x - item_screen_pos.x,
                        mouse_pos.y - item_screen_pos.y,
                    ));
                }
            }
        }
    }

    /// Handle clicks on empty canvas area
    fn handle_canvas_click(
        &mut self,
        event: &MouseDownEvent,
        mouse_pos: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        match self.selected_tool {
            ToolType::Select => {
                // Start marquee selection
                trace!("Starting marquee selection at {:?}", mouse_pos);
                self.marquee_start = Some(mouse_pos);
                self.marquee_current = Some(mouse_pos);

                if !event.modifiers.shift {
                    self.selected_items.clear();
                }
            }
            ToolType::Text | ToolType::Arrow | ToolType::Shape => {
                trace!("Starting {:?} drawing at {:?}", self.selected_tool, mouse_pos);
                self.drawing_start = Some(mouse_pos);
                self.drawing_current = Some(mouse_pos);
            }
        }
        cx.notify();
    }

    /// Handle double-click on an item. Returns true if handled.
    fn handle_double_click(
        &mut self,
        item_id: u64,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(ref board) = self.board else { return false };

        // Check if it's a TextBox - start editing
        let is_textbox = board
            .get_item(item_id)
            .map(|item| matches!(&item.content, ItemContent::TextBox { .. }))
            .unwrap_or(false);

        if is_textbox {
            debug!("Starting textbox editing for item {}", item_id);
            self.start_textbox_editing(item_id, window, cx);
            return true;
        }

        // Check for previewable content (PDF, Markdown, Code)
        let content_path = board.get_item(item_id).and_then(|item| match &item.content {
            ItemContent::Pdf { path, .. } => Some(path.clone()),
            ItemContent::Markdown { path, .. } => Some(path.clone()),
            ItemContent::Code { path, .. } => Some(path.clone()),
            _ => None,
        });

        if let Some(path) = content_path {
            debug!("Opening preview for {:?}", path);
            self.open_preview(path, window, cx);
            return true;
        }

        false
    }

    pub fn handle_mouse_up(
        &mut self,
        event: &MouseUpEvent,
        window: &mut Window,
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
                // Calculate marquee bounds in screen space
                let min_x = f32::from(start.x).min(f32::from(end.x));
                let max_x = f32::from(start.x).max(f32::from(end.x));
                let min_y = f32::from(start.y).min(f32::from(end.y));
                let max_y = f32::from(start.y).max(f32::from(end.y));

                // Only select if marquee has some size (not just a click)
                if (max_x - min_x) > 5.0 || (max_y - min_y) > 5.0 {
                    // Find all items that intersect with marquee
                    for item in &board.items {
                        // Convert item position to screen coordinates using HitTester
                        let item_screen_pos = self.hit_tester.canvas_to_screen(
                            point(px(item.position.0), px(item.position.1)),
                            board.canvas_offset,
                            board.zoom,
                        );
                        let item_x = f32::from(item_screen_pos.x);
                        let item_y = f32::from(item_screen_pos.y);
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

        // Finalize arrow/shape drawing
        if let Some(start) = self.drawing_start {
            let end = event.position;

            // Calculate screen distance first (before converting to canvas coords)
            let screen_width = (f32::from(end.x) - f32::from(start.x)).abs();
            let screen_height = (f32::from(end.y) - f32::from(start.y)).abs();

            // Only create if dragged at least 10 pixels
            if screen_width < 10.0 && screen_height < 10.0 {
                self.drawing_start = None;
                self.drawing_current = None;
                self.selected_tool = ToolType::Select;
                cx.notify();
                return;
            }

            // Calculate canvas positions using HitTester
            let start_canvas = self.screen_to_canvas(start);
            let end_canvas = self.screen_to_canvas(end);

            let start_x = f32::from(start_canvas.x);
            let start_y = f32::from(start_canvas.y);
            let end_x = f32::from(end_canvas.x);
            let end_y = f32::from(end_canvas.y);

            // Calculate size in canvas coords
            let width = (end_x - start_x).abs().max(20.0);
            let height = (end_y - start_y).abs().max(20.0);

            // Get top-left corner for item position
            let pos_x = start_x.min(end_x);
            let pos_y = start_y.min(end_y);

            match self.selected_tool {
                ToolType::Arrow => {
                    if let Some(ref mut board) = self.board {
                        // Arrow: position is top-left of bounding box
                        // start_offset/end_offset are relative to the bounding box origin
                        let box_x = start_x.min(end_x);
                        let box_y = start_y.min(end_y);
                        let box_w = (end_x - start_x).abs().max(20.0);
                        let box_h = (end_y - start_y).abs().max(20.0);

                        // Calculate arrow start/end relative to bounding box top-left
                        let arrow_start = (start_x - box_x, start_y - box_y);
                        let arrow_end = (end_x - box_x, end_y - box_y);
                        // end_offset is vector from start to end (for rendering)
                        let end_offset = (arrow_end.0 - arrow_start.0, arrow_end.1 - arrow_start.1);

                        let id = board.add_item(
                            point(px(box_x), px(box_y)),
                            ItemContent::Arrow {
                                end_offset,
                                color: "".to_string(), // Empty = use theme foreground
                                thickness: 2.0,
                                head_style: ArrowHead::Arrow,
                            },
                        );
                        if let Some(item) = board.get_item_mut(id) {
                            item.size = (box_w, box_h);
                        }
                        self.selected_items.clear();
                        self.selected_items.insert(id);
                    }
                }
                ToolType::Shape => {
                    if let Some(ref mut board) = self.board {
                        let id = board.add_item(
                            point(px(pos_x), px(pos_y)),
                            ItemContent::Shape {
                                shape_type: ShapeType::Rectangle,
                                fill_color: None,
                                border_color: "".to_string(), // Empty = use theme foreground
                                border_width: 2.0,
                            },
                        );
                        // Update size based on drawn dimensions
                        if let Some(item) = board.get_item_mut(id) {
                            item.size = (width, height);
                        }
                        self.selected_items.clear();
                        self.selected_items.insert(id);
                    }
                }
                ToolType::Text => {
                    if let Some(ref mut board) = self.board {
                        let id = board.add_item(
                            point(px(pos_x), px(pos_y)),
                            ItemContent::TextBox {
                                text: "".to_string(),
                                font_size: 16.0,
                                color: "".to_string(), // Empty = use theme foreground
                            },
                        );
                        // Update size based on drawn dimensions
                        if let Some(item) = board.get_item_mut(id) {
                            item.size = (width.max(100.0), height.max(40.0));
                        }
                        self.selected_items.clear();
                        self.selected_items.insert(id);
                        // Start editing immediately
                        self.start_textbox_editing(id, window, cx);
                    }
                }
                _ => {}
            }

            // Switch back to select tool after drawing
            self.selected_tool = ToolType::Select;
            self.drawing_start = None;
            self.drawing_current = None;
        }

        self.dragging = false;
        self.last_mouse_pos = None;
        self.dragging_item = None;
        self.item_drag_offset = None;
        self.resizing_item = None;
        self.resize_start_size = None;
        self.resize_start_pos = None;
        self.resize_start_font_size = None;
        self.dragging_splitter = false;
        self.splitter_drag_start = None;
        self.marquee_start = None;
        self.marquee_current = None;
        cx.notify();
    }

    /// Convert screen position to canvas position using HitTester
    fn screen_to_canvas(&self, pos: Point<Pixels>) -> Point<Pixels> {
        if let Some(ref board) = self.board {
            self.hit_tester.screen_to_canvas(pos, board.canvas_offset, board.zoom)
        } else {
            pos
        }
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

                    // Check item type for special resize handling
                    let item_type = board.get_item(item_id).map(|item| match &item.content {
                        ItemContent::Markdown { .. } => "markdown",
                        ItemContent::TextBox { .. } => "textbox",
                        ItemContent::Arrow { end_offset, .. } => {
                            // Store arrow direction for later
                            if end_offset.0 >= 0.0 && end_offset.1 >= 0.0 {
                                "arrow_pp" // positive-positive (top-left to bottom-right)
                            } else if end_offset.0 < 0.0 && end_offset.1 >= 0.0 {
                                "arrow_np" // negative-positive
                            } else if end_offset.0 >= 0.0 && end_offset.1 < 0.0 {
                                "arrow_pn" // positive-negative
                            } else {
                                "arrow_nn" // negative-negative
                            }
                        }
                        _ => "other",
                    });

                    // Use stored font size from resize start
                    let original_font_size = self.resize_start_font_size;

                    let (new_width, new_height) = match item_type.as_deref() {
                        Some("markdown") => {
                            // Markdown cards have fixed aspect ratio of 200:36 (5.56:1)
                            const MD_ASPECT_RATIO: f32 = 200.0 / 36.0;
                            let width = (start_size.0 + delta_x).max(100.0);
                            let height = width / MD_ASPECT_RATIO;
                            (width, height)
                        }
                        _ => {
                            // Other items can resize freely
                            let width = (start_size.0 + delta_x).max(50.0);
                            let height = (start_size.1 + delta_y).max(50.0);
                            (width, height)
                        }
                    };

                    // Use get_item_mut for O(1) lookup
                    if let Some(item) = board.get_item_mut(item_id) {
                        // Calculate scale factor based on size change
                        let scale = new_height / start_size.1;

                        item.size = (new_width, new_height);

                        // For arrows, also update end_offset to match new size
                        if let ItemContent::Arrow { end_offset, .. } = &mut item.content {
                            // Preserve direction (sign) while updating magnitude
                            let sign_x = if end_offset.0 >= 0.0 { 1.0 } else { -1.0 };
                            let sign_y = if end_offset.1 >= 0.0 { 1.0 } else { -1.0 };
                            *end_offset = (new_width * sign_x, new_height * sign_y);
                        }

                        // For textboxes, scale font size proportionally
                        if let ItemContent::TextBox { font_size, .. } = &mut item.content {
                            if let Some(orig_size) = original_font_size {
                                *font_size = (orig_size * scale).max(8.0).min(200.0);
                            }
                        }
                    }
                    // Mark dirty but don't save immediately (debounced)
                    board.mark_dirty();
                    cx.notify();
                }
            }
        } else if let Some(item_id) = self.dragging_item {
            if let Some(offset) = self.item_drag_offset {
                // Convert screen position (minus drag offset) to canvas coordinates
                let adjusted_pos = point(event.position.x - offset.x, event.position.y - offset.y);
                let canvas_pos = self.hit_tester.screen_to_canvas(
                    adjusted_pos,
                    board.canvas_offset,
                    board.zoom,
                );
                let new_x = f32::from(canvas_pos.x);
                let new_y = f32::from(canvas_pos.y);

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
        } else if self.drawing_start.is_some() {
            // Update drawing preview position
            self.drawing_current = Some(event.position);
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
