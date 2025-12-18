use crate::app::{Humanboard, SplitDirection};
use crate::constants::{DEFAULT_FONT_SIZE, HEADER_HEIGHT, SPLITTER_WIDTH};
use crate::render::dock::DOCK_WIDTH;
use crate::types::{ArrowHead, ItemContent, ShapeType, ToolType};
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
                return; // Let preview panel handle its own clicks
            }
        }

        // Don't reset focus yet - we need to check for textbox editing first
        // self.focus.force_canvas_focus(window) is called later if needed

        // If a drawing tool is selected, prioritize drawing over item selection
        let header_offset = HEADER_HEIGHT;
        let dock_offset = DOCK_WIDTH;

        if matches!(
            self.selected_tool,
            ToolType::Text | ToolType::Arrow | ToolType::Shape
        ) {
            // Start drawing regardless of what's under the cursor
            self.drawing_start = Some(mouse_pos);
            self.drawing_current = Some(mouse_pos);
            self.selected_items.clear();
            cx.notify();
            return;
        }

        // Check if clicking on an item (in reverse order so top items are checked first)
        // Extract only the ID to avoid cloning the entire item
        // Account for 40px header offset and dock width
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
                // This allows clicking through to items inside the shape
                if let ItemContent::Shape { border_width, .. } = &item.content {
                    let border_hit_area = (border_width * board.zoom).max(8.0); // Min 8px hit area
                    let near_left = mx - scaled_x < border_hit_area;
                    let near_right = (scaled_x + scaled_width) - mx < border_hit_area;
                    let near_top = my - scaled_y < border_hit_area;
                    let near_bottom = (scaled_y + scaled_height) - my < border_hit_area;

                    // Only hit if near any edge
                    return near_left || near_right || near_top || near_bottom;
                }

                true
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

            // Handle double-click for PDF/Markdown/Code preview or TextBox editing
            if event.click_count == 2 {
                // Check if it's a TextBox - start editing
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

            // Check if clicking on resize corner (bottom-right corner)
            // Use get_item for O(1) lookup - extract needed values
            let item_info = board
                .get_item(item_id)
                .map(|item| (item.position, item.size, &item.content));

            if let Some((position, size, content)) = item_info {
                let is_resizable = true;
                let scaled_x =
                    position.0 * board.zoom + f32::from(board.canvas_offset.x) + dock_offset;
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
                    // Store font size for TextBox scaling
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
            // Reset focus to canvas for non-textbox interactions
            self.focus.force_canvas_focus(window);
        } else {
            // Clicked on empty canvas - reset focus
            self.focus.force_canvas_focus(window);

            // Check if we have a drawing tool selected
            match self.selected_tool {
                ToolType::Select => {
                    // Start marquee selection
                    self.marquee_start = Some(mouse_pos);
                    self.marquee_current = Some(mouse_pos);

                    // Clear selection unless shift is held
                    if !event.modifiers.shift {
                        self.selected_items.clear();
                    }
                }
                ToolType::Text => {
                    // Start drawing a text box (drag to size)
                    self.drawing_start = Some(mouse_pos);
                    self.drawing_current = Some(mouse_pos);
                }
                ToolType::Arrow => {
                    // Start drawing an arrow
                    self.drawing_start = Some(mouse_pos);
                    self.drawing_current = Some(mouse_pos);
                }
                ToolType::Shape => {
                    // Start drawing a shape
                    self.drawing_start = Some(mouse_pos);
                    self.drawing_current = Some(mouse_pos);
                }
            }
        }

        cx.notify();
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
                if let Err(e) = board.flush_save() {
                    self.toast_manager
                        .push(crate::notifications::Toast::error(format!(
                            "Save failed: {}",
                            e
                        )));
                }
            }
        }

        // Finalize marquee selection
        if let (Some(start), Some(end)) = (self.marquee_start, self.marquee_current) {
            if let Some(ref board) = self.board {
                let header_offset = HEADER_HEIGHT;

                // Calculate marquee bounds in screen space
                let min_x = f32::from(start.x).min(f32::from(end.x));
                let max_x = f32::from(start.x).max(f32::from(end.x));
                let min_y = f32::from(start.y).min(f32::from(end.y));
                let max_y = f32::from(start.y).max(f32::from(end.y));

                // Only select if marquee has some size (not just a click)
                if (max_x - min_x) > 5.0 || (max_y - min_y) > 5.0 {
                    // Find all items that intersect with marquee
                    for item in &board.items {
                        let item_x = item.position.0 * board.zoom
                            + f32::from(board.canvas_offset.x)
                            + DOCK_WIDTH;
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

        // Finalize arrow/shape drawing
        if let Some(start) = self.drawing_start {
            let end = event.position;
            let header_offset = HEADER_HEIGHT;

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

            // Calculate canvas positions
            let start_canvas = self.screen_to_canvas(start, header_offset);
            let end_canvas = self.screen_to_canvas(end, header_offset);

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
                                font_size: DEFAULT_FONT_SIZE,
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
        self.dragging_pane_splitter = false;
        self.splitter_drag_start = None;
        self.marquee_start = None;
        self.marquee_current = None;
        cx.notify();
    }

    /// Convert screen position to canvas position (returns Point<Pixels>)
    fn screen_to_canvas(&self, pos: Point<Pixels>, header_offset: f32) -> Point<Pixels> {
        if let Some(ref board) = self.board {
            // Account for dock width on the left side
            let x = (f32::from(pos.x) - DOCK_WIDTH - f32::from(board.canvas_offset.x)) / board.zoom;
            let y =
                (f32::from(pos.y) - header_offset - f32::from(board.canvas_offset.y)) / board.zoom;
            point(px(x), px(y))
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

                    // Calculate panel bounds
                    let header_height = 40.0;
                    let footer_height = 28.0;
                    let dock_width = 40.0;

                    let (panel_start, panel_size) = match preview.split {
                        SplitDirection::Vertical => {
                            let panel_x =
                                dock_width + (window_width - dock_width) * (1.0 - preview.size);
                            let panel_width = (window_width - dock_width) * preview.size;
                            if preview.pane_split_horizontal {
                                // Top/bottom split within vertical panel
                                let panel_y = header_height;
                                let panel_height = window_height - header_height - footer_height;
                                (panel_y, panel_height)
                            } else {
                                // Left/right split within vertical panel
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
                                // Top/bottom split
                                (panel_y, panel_height)
                            } else {
                                // Left/right split
                                (dock_width, window_width - dock_width)
                            }
                        }
                    };

                    // Calculate new ratio based on mouse position
                    let new_ratio = if preview.pane_split_horizontal {
                        // Horizontal pane split (top/bottom)
                        ((mouse_y - panel_start) / panel_size).clamp(0.2, 0.8)
                    } else {
                        // Vertical pane split (left/right)
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
                // Capture values before mutable borrow
                let zoom = board.zoom;
                let canvas_offset_x = f32::from(board.canvas_offset.x);
                let canvas_offset_y = f32::from(board.canvas_offset.y);
                let header_offset = HEADER_HEIGHT; // Account for header bar
                let new_x =
                    (f32::from(event.position.x - offset.x) - DOCK_WIDTH - canvas_offset_x) / zoom;
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
