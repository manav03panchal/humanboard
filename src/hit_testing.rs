//! Hit Testing Module
//!
//! This module provides hit testing functionality for the canvas, determining
//! which UI elements and canvas items should respond to mouse events.
//!
//! ## Hit Testing Algorithm
//!
//! Following Zed's pattern, hit testing is performed in back-to-front order
//! (reverse of rendering order), so items painted on top take precedence.
//!
//! ## Hit Areas
//!
//! - **Item body**: The main clickable area of an item
//! - **Resize corner**: Bottom-right corner for resizing
//! - **Shape border**: For shape items, only the border is clickable
//! - **Splitter**: The divider between canvas and preview panel
//!
//! ## Performance Notes
//!
//! Hit testing is O(n) where n is the number of items. For boards with many
//! items (>500), consider implementing spatial indexing (quadtree/R-tree).
//! Current implementation iterates through all items in reverse order.
//!
//! Enable profiling with `cargo build --features profiling` to track hit test times.

use crate::constants::{DOCK_WIDTH, FOOTER_HEIGHT, HEADER_HEIGHT, MIN_HIT_AREA, SPLITTER_WIDTH};
use crate::profile_scope;
use crate::types::ItemContent;
use gpui::*;

/// The result of a hit test on the canvas.
#[derive(Debug, Clone, PartialEq)]
pub enum HitTestResult {
    /// No hit - empty canvas area
    Canvas,
    /// Hit the header bar
    Header,
    /// Hit the tool dock
    Dock,
    /// Hit the preview panel area
    PreviewPanel,
    /// Hit the splitter between canvas and preview
    Splitter,
    /// Hit a canvas item
    Item(ItemHit),
}

/// Information about a hit on a canvas item.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemHit {
    /// The ID of the hit item
    pub item_id: u64,
    /// The specific area that was hit
    pub area: ItemHitArea,
}

/// The specific area of an item that was hit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemHitArea {
    /// The main body of the item
    Body,
    /// The resize corner (bottom-right)
    ResizeCorner,
    /// The border of a shape (for click-through shapes)
    ShapeBorder,
}

/// Configuration for hit testing.
pub struct HitTestConfig {
    /// Header bar height in pixels
    pub header_height: f32,
    /// Dock width in pixels
    pub dock_width: f32,
    /// Footer height in pixels
    pub footer_height: f32,
    /// Size of the resize corner hit area
    pub resize_corner_size: f32,
    /// Width of the splitter hit area
    pub splitter_width: f32,
    /// Minimum border hit area for shapes
    pub min_border_hit_area: f32,
}

impl Default for HitTestConfig {
    fn default() -> Self {
        Self {
            header_height: HEADER_HEIGHT,
            dock_width: DOCK_WIDTH,
            footer_height: FOOTER_HEIGHT,
            resize_corner_size: 30.0,
            splitter_width: SPLITTER_WIDTH,
            min_border_hit_area: MIN_HIT_AREA,
        }
    }
}

/// Represents a canvas item for hit testing purposes.
#[derive(Debug, Clone)]
pub struct HitTestItem {
    pub id: u64,
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub content_type: HitTestContentType,
}

/// Content type information needed for hit testing.
#[derive(Debug, Clone)]
pub enum HitTestContentType {
    /// Standard item (images, PDFs, etc.)
    Standard,
    /// Shape with border - only border is clickable
    Shape { border_width: f32 },
    /// Arrow or line
    Arrow,
    /// Text box
    TextBox,
}

impl HitTestContentType {
    /// Create from ItemContent reference.
    pub fn from_content(content: &ItemContent) -> Self {
        match content {
            ItemContent::Shape { border_width, .. } => HitTestContentType::Shape {
                border_width: *border_width,
            },
            ItemContent::Arrow { .. } => HitTestContentType::Arrow,
            ItemContent::TextBox { .. } => HitTestContentType::TextBox,
            _ => HitTestContentType::Standard,
        }
    }
}

/// Hit tester for canvas items.
pub struct HitTester {
    config: HitTestConfig,
}

impl HitTester {
    /// Create a new hit tester with default configuration.
    pub fn new() -> Self {
        Self {
            config: HitTestConfig::default(),
        }
    }

    /// Create a new hit tester with custom configuration.
    pub fn with_config(config: HitTestConfig) -> Self {
        Self { config }
    }

    /// Perform a hit test at the given screen position.
    ///
    /// ## Parameters
    /// - `mouse_pos`: The mouse position in screen coordinates
    /// - `items`: Iterator over canvas items in back-to-front order
    /// - `canvas_offset`: Current canvas pan offset
    /// - `zoom`: Current canvas zoom level
    /// - `window_size`: Size of the window
    /// - `preview_bounds`: Optional preview panel bounds (x, width) or (y, height)
    ///
    /// ## Returns
    /// The hit test result indicating what was hit.
    ///
    /// ## Performance
    /// O(n) where n is the number of items. UI chrome checks (header, dock, etc.)
    /// are O(1) and checked first for early exit.
    pub fn hit_test(
        &self,
        mouse_pos: Point<Pixels>,
        items: impl DoubleEndedIterator<Item = HitTestItem>,
        canvas_offset: Point<Pixels>,
        zoom: f32,
        window_size: Size<Pixels>,
        preview_split: Option<PreviewSplit>,
    ) -> HitTestResult {
        profile_scope!("hit_test");

        let mx = f32::from(mouse_pos.x);
        let my = f32::from(mouse_pos.y);
        let window_height = f32::from(window_size.height);

        // Check header
        if my < self.config.header_height {
            return HitTestResult::Header;
        }

        // Check preview panel and splitter
        if let Some(split) = preview_split {
            match split {
                PreviewSplit::Vertical { x, width: _ } => {
                    // Check splitter
                    if (mx - x).abs() < self.config.splitter_width / 2.0 {
                        return HitTestResult::Splitter;
                    }
                    // Check preview panel
                    if mx > x {
                        return HitTestResult::PreviewPanel;
                    }
                }
                PreviewSplit::Horizontal { y, height: _ } => {
                    // Check splitter
                    if (my - y).abs() < self.config.splitter_width / 2.0 {
                        return HitTestResult::Splitter;
                    }
                    // Check preview panel
                    if my > y {
                        return HitTestResult::PreviewPanel;
                    }
                }
            }
        }

        // Check dock (left side)
        if mx < self.config.dock_width {
            return HitTestResult::Dock;
        }

        // Check footer (bottom)
        if my > window_height - self.config.footer_height {
            // Still allow item hits in footer area, but this could be used
            // to prioritize footer UI elements
        }

        // Hit test items in reverse order (front-to-back for top items first)
        for item in items.rev() {
            if let Some(hit) = self.hit_test_item(&item, mouse_pos, canvas_offset, zoom) {
                return HitTestResult::Item(hit);
            }
        }

        HitTestResult::Canvas
    }

    /// Hit test a single item.
    fn hit_test_item(
        &self,
        item: &HitTestItem,
        mouse_pos: Point<Pixels>,
        canvas_offset: Point<Pixels>,
        zoom: f32,
    ) -> Option<ItemHit> {
        let mx = f32::from(mouse_pos.x);
        let my = f32::from(mouse_pos.y);

        // Calculate item screen bounds
        let scaled_x =
            item.position.0 * zoom + f32::from(canvas_offset.x) + self.config.dock_width;
        let scaled_y =
            item.position.1 * zoom + f32::from(canvas_offset.y) + self.config.header_height;
        let scaled_width = item.size.0 * zoom;
        let scaled_height = item.size.1 * zoom;

        // Basic bounds check
        let in_bounds = mx >= scaled_x
            && mx <= scaled_x + scaled_width
            && my >= scaled_y
            && my <= scaled_y + scaled_height;

        if !in_bounds {
            return None;
        }

        // Check for resize corner
        let corner_size = self.config.resize_corner_size * zoom;
        let corner_x = scaled_x + scaled_width;
        let corner_y = scaled_y + scaled_height;

        let in_resize_corner = mx >= corner_x - corner_size
            && mx <= corner_x + 5.0
            && my >= corner_y - corner_size
            && my <= corner_y + 5.0;

        if in_resize_corner {
            return Some(ItemHit {
                item_id: item.id,
                area: ItemHitArea::ResizeCorner,
            });
        }

        // Special handling for shapes - only hit on border
        if let HitTestContentType::Shape { border_width } = item.content_type {
            let border_hit_area = (border_width * zoom).max(self.config.min_border_hit_area);
            let near_left = mx - scaled_x < border_hit_area;
            let near_right = (scaled_x + scaled_width) - mx < border_hit_area;
            let near_top = my - scaled_y < border_hit_area;
            let near_bottom = (scaled_y + scaled_height) - my < border_hit_area;

            if near_left || near_right || near_top || near_bottom {
                return Some(ItemHit {
                    item_id: item.id,
                    area: ItemHitArea::ShapeBorder,
                });
            } else {
                // Inside shape but not on border - no hit (click through)
                return None;
            }
        }

        // Standard item body hit
        Some(ItemHit {
            item_id: item.id,
            area: ItemHitArea::Body,
        })
    }

    /// Check if a point is within the canvas area (not in UI chrome).
    pub fn is_in_canvas(&self, pos: Point<Pixels>, window_size: Size<Pixels>) -> bool {
        let x = f32::from(pos.x);
        let y = f32::from(pos.y);
        let window_height = f32::from(window_size.height);

        x >= self.config.dock_width
            && y >= self.config.header_height
            && y <= window_height - self.config.footer_height
    }

    /// Convert screen coordinates to canvas coordinates.
    pub fn screen_to_canvas(
        &self,
        pos: Point<Pixels>,
        canvas_offset: Point<Pixels>,
        zoom: f32,
    ) -> Point<Pixels> {
        let x = (f32::from(pos.x) - self.config.dock_width - f32::from(canvas_offset.x)) / zoom;
        let y = (f32::from(pos.y) - self.config.header_height - f32::from(canvas_offset.y)) / zoom;
        point(px(x), px(y))
    }

    /// Convert canvas coordinates to screen coordinates.
    pub fn canvas_to_screen(
        &self,
        pos: Point<Pixels>,
        canvas_offset: Point<Pixels>,
        zoom: f32,
    ) -> Point<Pixels> {
        let x = f32::from(pos.x) * zoom + f32::from(canvas_offset.x) + self.config.dock_width;
        let y = f32::from(pos.y) * zoom + f32::from(canvas_offset.y) + self.config.header_height;
        point(px(x), px(y))
    }
}

impl Default for HitTester {
    fn default() -> Self {
        Self::new()
    }
}

/// Preview panel split information for hit testing.
#[derive(Debug, Clone, Copy)]
pub enum PreviewSplit {
    /// Vertical split (panel on right)
    Vertical { x: f32, width: f32 },
    /// Horizontal split (panel on bottom)
    Horizontal { y: f32, height: f32 },
}
