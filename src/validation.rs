//! Property validation for canvas items.
//!
//! This module provides validation for item properties like sizes, colors, and fonts
//! to ensure they stay within acceptable bounds.

use crate::constants::{
    MAX_ARROW_THICKNESS, MAX_BORDER_WIDTH, MAX_FONT_SIZE, MIN_ARROW_THICKNESS, MIN_BORDER_WIDTH,
    MIN_FONT_SIZE,
};
use crate::types::{CanvasItem, ItemContent};

/// Validation constraints for item properties
pub struct ValidationConstraints {
    /// Minimum font size for text boxes
    pub min_font_size: f32,
    /// Maximum font size for text boxes
    pub max_font_size: f32,
    /// Minimum arrow thickness
    pub min_arrow_thickness: f32,
    /// Maximum arrow thickness
    pub max_arrow_thickness: f32,
    /// Minimum shape border width
    pub min_border_width: f32,
    /// Maximum shape border width
    pub max_border_width: f32,
    /// Minimum item width
    pub min_item_width: f32,
    /// Minimum item height
    pub min_item_height: f32,
    /// Maximum item dimension
    pub max_item_dimension: f32,
}

impl Default for ValidationConstraints {
    fn default() -> Self {
        Self {
            min_font_size: MIN_FONT_SIZE,
            max_font_size: MAX_FONT_SIZE,
            min_arrow_thickness: MIN_ARROW_THICKNESS,
            max_arrow_thickness: MAX_ARROW_THICKNESS,
            min_border_width: MIN_BORDER_WIDTH,
            max_border_width: MAX_BORDER_WIDTH,
            min_item_width: 20.0,
            min_item_height: 20.0,
            max_item_dimension: 10000.0,
        }
    }
}

/// Validation result with optional fixes applied
#[derive(Debug)]
pub struct ValidationResult {
    /// Whether the item was valid before any fixes
    pub was_valid: bool,
    /// List of issues found and fixed
    pub fixes_applied: Vec<String>,
}

impl ValidationResult {
    pub fn valid() -> Self {
        Self {
            was_valid: true,
            fixes_applied: Vec::new(),
        }
    }

    pub fn with_fix(mut self, fix: String) -> Self {
        self.was_valid = false;
        self.fixes_applied.push(fix);
        self
    }
}

/// Validate a hex color string (e.g., "#ffffff" or "#fff")
pub fn is_valid_hex_color(color: &str) -> bool {
    if !color.starts_with('#') {
        return false;
    }

    let hex = &color[1..];
    let valid_len = hex.len() == 3 || hex.len() == 6 || hex.len() == 8;
    let valid_chars = hex.chars().all(|c| c.is_ascii_hexdigit());

    valid_len && valid_chars
}

/// Normalize a hex color to 6-digit format with #
pub fn normalize_hex_color(color: &str) -> String {
    if !color.starts_with('#') {
        return format!("#{}", color);
    }

    let hex = &color[1..];
    match hex.len() {
        3 => {
            // Expand #RGB to #RRGGBB
            let chars: Vec<char> = hex.chars().collect();
            format!(
                "#{}{}{}{}{}{}",
                chars[0], chars[0], chars[1], chars[1], chars[2], chars[2]
            )
        }
        _ => color.to_string(),
    }
}

/// Clamp a value to a range
fn clamp(value: f32, min: f32, max: f32) -> f32 {
    value.max(min).min(max)
}

/// Validate and fix an item's properties in place
pub fn validate_item(item: &mut CanvasItem, constraints: &ValidationConstraints) -> ValidationResult {
    let mut result = ValidationResult::valid();

    // Validate size
    let (min_w, min_h) = get_min_size_for_content(&item.content, constraints);

    if item.size.0 < min_w {
        result = result.with_fix(format!(
            "Width {} below minimum {}, clamped",
            item.size.0, min_w
        ));
        item.size.0 = min_w;
    }

    if item.size.1 < min_h {
        result = result.with_fix(format!(
            "Height {} below minimum {}, clamped",
            item.size.1, min_h
        ));
        item.size.1 = min_h;
    }

    if item.size.0 > constraints.max_item_dimension {
        result = result.with_fix(format!(
            "Width {} exceeds maximum {}, clamped",
            item.size.0, constraints.max_item_dimension
        ));
        item.size.0 = constraints.max_item_dimension;
    }

    if item.size.1 > constraints.max_item_dimension {
        result = result.with_fix(format!(
            "Height {} exceeds maximum {}, clamped",
            item.size.1, constraints.max_item_dimension
        ));
        item.size.1 = constraints.max_item_dimension;
    }

    // Validate content-specific properties
    match &mut item.content {
        ItemContent::TextBox {
            font_size, color, ..
        } => {
            let clamped_size = clamp(
                *font_size,
                constraints.min_font_size,
                constraints.max_font_size,
            );
            if (*font_size - clamped_size).abs() > 0.01 {
                result = result.with_fix(format!(
                    "Font size {} out of range [{}, {}], clamped to {}",
                    font_size, constraints.min_font_size, constraints.max_font_size, clamped_size
                ));
                *font_size = clamped_size;
            }

            if !is_valid_hex_color(color) {
                result = result.with_fix(format!(
                    "Invalid color '{}', reset to white",
                    color
                ));
                *color = "#ffffff".to_string();
            }
        }

        ItemContent::Arrow {
            color, thickness, ..
        } => {
            let clamped_thickness = clamp(
                *thickness,
                constraints.min_arrow_thickness,
                constraints.max_arrow_thickness,
            );
            if (*thickness - clamped_thickness).abs() > 0.01 {
                result = result.with_fix(format!(
                    "Arrow thickness {} out of range [{}, {}], clamped to {}",
                    thickness,
                    constraints.min_arrow_thickness,
                    constraints.max_arrow_thickness,
                    clamped_thickness
                ));
                *thickness = clamped_thickness;
            }

            if !is_valid_hex_color(color) {
                result = result.with_fix(format!(
                    "Invalid arrow color '{}', reset to white",
                    color
                ));
                *color = "#ffffff".to_string();
            }
        }

        ItemContent::Shape {
            fill_color,
            border_color,
            border_width,
            ..
        } => {
            let clamped_width = clamp(
                *border_width,
                constraints.min_border_width,
                constraints.max_border_width,
            );
            if (*border_width - clamped_width).abs() > 0.01 {
                result = result.with_fix(format!(
                    "Border width {} out of range [{}, {}], clamped to {}",
                    border_width,
                    constraints.min_border_width,
                    constraints.max_border_width,
                    clamped_width
                ));
                *border_width = clamped_width;
            }

            if !is_valid_hex_color(border_color) {
                result = result.with_fix(format!(
                    "Invalid border color '{}', reset to white",
                    border_color
                ));
                *border_color = "#ffffff".to_string();
            }

            if let Some(fill) = fill_color {
                if !is_valid_hex_color(fill) {
                    result = result.with_fix(format!(
                        "Invalid fill color '{}', removed",
                        fill
                    ));
                    *fill_color = None;
                }
            }
        }

        // Other content types don't have configurable properties to validate
        _ => {}
    }

    result
}

/// Get minimum size for a content type
fn get_min_size_for_content(content: &ItemContent, constraints: &ValidationConstraints) -> (f32, f32) {
    match content {
        ItemContent::TextBox { .. } => (50.0, 30.0),
        ItemContent::Arrow { .. } => (constraints.min_item_width, constraints.min_item_height),
        ItemContent::Shape { .. } => (30.0, 30.0),
        ItemContent::Markdown { .. } => (100.0, 36.0),
        ItemContent::Code { .. } => (100.0, 36.0),
        _ => (constraints.min_item_width, constraints.min_item_height),
    }
}

/// Validate all items in a list, returning the count of items that needed fixes
pub fn validate_items(items: &mut [CanvasItem]) -> usize {
    let constraints = ValidationConstraints::default();
    let mut fixed_count = 0;

    for item in items.iter_mut() {
        let result = validate_item(item, &constraints);
        if !result.was_valid {
            fixed_count += 1;
            for fix in &result.fixes_applied {
                tracing::warn!("Item {}: {}", item.id, fix);
            }
        }
    }

    fixed_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ShapeType;

    // ========================================================================
    // Edge Case Tests for Hex Color Validation
    // ========================================================================

    #[test]
    fn test_hex_color_edge_empty_string() {
        assert!(!is_valid_hex_color(""));
    }

    #[test]
    fn test_hex_color_edge_just_hash() {
        assert!(!is_valid_hex_color("#"));
    }

    #[test]
    fn test_hex_color_edge_single_char() {
        assert!(!is_valid_hex_color("#a"));
    }

    #[test]
    fn test_hex_color_edge_two_chars() {
        assert!(!is_valid_hex_color("#ab"));
    }

    #[test]
    fn test_hex_color_edge_four_chars() {
        // 4-digit hex (#rgba shorthand) is not standard, should fail
        assert!(!is_valid_hex_color("#abcd"));
    }

    #[test]
    fn test_hex_color_edge_five_chars() {
        assert!(!is_valid_hex_color("#abcde"));
    }

    #[test]
    fn test_hex_color_edge_seven_chars() {
        assert!(!is_valid_hex_color("#abcdeff"));
    }

    #[test]
    fn test_hex_color_edge_nine_chars() {
        assert!(!is_valid_hex_color("#abcdef012"));
    }

    #[test]
    fn test_hex_color_edge_mixed_case() {
        assert!(is_valid_hex_color("#AaBbCc"));
        assert!(is_valid_hex_color("#aAbBcC"));
    }

    #[test]
    fn test_hex_color_edge_invalid_chars_start() {
        assert!(!is_valid_hex_color("#gfffff"));
    }

    #[test]
    fn test_hex_color_edge_invalid_chars_middle() {
        assert!(!is_valid_hex_color("#ffgfff"));
    }

    #[test]
    fn test_hex_color_edge_invalid_chars_end() {
        assert!(!is_valid_hex_color("#fffffg"));
    }

    #[test]
    fn test_hex_color_edge_special_chars() {
        assert!(!is_valid_hex_color("#ff!fff"));
        assert!(!is_valid_hex_color("#ff fff"));
        assert!(!is_valid_hex_color("#ff\nfff"));
    }

    #[test]
    fn test_hex_color_edge_no_hash_valid_content() {
        // Valid hex digits but missing #
        assert!(!is_valid_hex_color("ffffff"));
        assert!(!is_valid_hex_color("fff"));
    }

    #[test]
    fn test_hex_color_edge_all_zeros() {
        assert!(is_valid_hex_color("#000"));
        assert!(is_valid_hex_color("#000000"));
        assert!(is_valid_hex_color("#00000000"));
    }

    #[test]
    fn test_hex_color_edge_all_f() {
        assert!(is_valid_hex_color("#fff"));
        assert!(is_valid_hex_color("#ffffff"));
        assert!(is_valid_hex_color("#ffffffff"));
    }

    // ========================================================================
    // Edge Case Tests for Hex Color Normalization
    // ========================================================================

    #[test]
    fn test_normalize_hex_color_edge_empty() {
        assert_eq!(normalize_hex_color(""), "#");
    }

    #[test]
    fn test_normalize_hex_color_edge_just_hash() {
        assert_eq!(normalize_hex_color("#"), "#");
    }

    #[test]
    fn test_normalize_hex_color_edge_no_hash() {
        assert_eq!(normalize_hex_color("abc"), "#abc");
    }

    #[test]
    fn test_normalize_hex_color_edge_eight_digits() {
        // 8-digit hex (with alpha) should pass through unchanged
        assert_eq!(normalize_hex_color("#aabbccdd"), "#aabbccdd");
    }

    #[test]
    fn test_normalize_hex_color_edge_mixed_case() {
        assert_eq!(normalize_hex_color("#AbC"), "#AAbbCC");
    }

    // ========================================================================
    // Edge Case Tests for Boundary Values
    // ========================================================================

    #[test]
    fn test_font_size_exactly_at_min() {
        let constraints = ValidationConstraints::default();
        let mut item = CanvasItem {
            id: 1,
            position: (0.0, 0.0),
            size: (100.0, 50.0),
            content: ItemContent::TextBox {
                text: "Test".to_string(),
                font_size: constraints.min_font_size,
                color: "#ffffff".to_string(),
            },
        };

        let result = validate_item(&mut item, &constraints);
        assert!(result.was_valid);
    }

    #[test]
    fn test_font_size_exactly_at_max() {
        let constraints = ValidationConstraints::default();
        let mut item = CanvasItem {
            id: 1,
            position: (0.0, 0.0),
            size: (100.0, 50.0),
            content: ItemContent::TextBox {
                text: "Test".to_string(),
                font_size: constraints.max_font_size,
                color: "#ffffff".to_string(),
            },
        };

        let result = validate_item(&mut item, &constraints);
        assert!(result.was_valid);
    }

    #[test]
    fn test_font_size_just_below_min() {
        let constraints = ValidationConstraints::default();
        let mut item = CanvasItem {
            id: 1,
            position: (0.0, 0.0),
            size: (100.0, 50.0),
            content: ItemContent::TextBox {
                text: "Test".to_string(),
                font_size: constraints.min_font_size - 0.1,
                color: "#ffffff".to_string(),
            },
        };

        let result = validate_item(&mut item, &constraints);
        assert!(!result.was_valid);
        if let ItemContent::TextBox { font_size, .. } = &item.content {
            assert_eq!(*font_size, constraints.min_font_size);
        }
    }

    #[test]
    fn test_font_size_just_above_max() {
        let constraints = ValidationConstraints::default();
        let mut item = CanvasItem {
            id: 1,
            position: (0.0, 0.0),
            size: (100.0, 50.0),
            content: ItemContent::TextBox {
                text: "Test".to_string(),
                font_size: constraints.max_font_size + 0.1,
                color: "#ffffff".to_string(),
            },
        };

        let result = validate_item(&mut item, &constraints);
        assert!(!result.was_valid);
        if let ItemContent::TextBox { font_size, .. } = &item.content {
            assert_eq!(*font_size, constraints.max_font_size);
        }
    }

    #[test]
    fn test_arrow_thickness_exactly_at_boundaries() {
        let constraints = ValidationConstraints::default();

        // Test at min
        let mut item_min = CanvasItem {
            id: 1,
            position: (0.0, 0.0),
            size: (100.0, 50.0),
            content: ItemContent::Arrow {
                end_offset: (100.0, 50.0),
                color: "#ffffff".to_string(),
                thickness: constraints.min_arrow_thickness,
                head_style: crate::types::ArrowHead::Arrow,
            },
        };
        assert!(validate_item(&mut item_min, &constraints).was_valid);

        // Test at max
        let mut item_max = CanvasItem {
            id: 2,
            position: (0.0, 0.0),
            size: (100.0, 50.0),
            content: ItemContent::Arrow {
                end_offset: (100.0, 50.0),
                color: "#ffffff".to_string(),
                thickness: constraints.max_arrow_thickness,
                head_style: crate::types::ArrowHead::Arrow,
            },
        };
        assert!(validate_item(&mut item_max, &constraints).was_valid);
    }

    #[test]
    fn test_border_width_at_zero() {
        let constraints = ValidationConstraints::default();
        let mut item = CanvasItem {
            id: 1,
            position: (0.0, 0.0),
            size: (100.0, 50.0),
            content: ItemContent::Shape {
                shape_type: ShapeType::Rectangle,
                fill_color: Some("#ff0000".to_string()),
                border_color: "#ffffff".to_string(),
                border_width: 0.0, // Min is 0.0
            },
        };

        let result = validate_item(&mut item, &constraints);
        assert!(result.was_valid);
    }

    #[test]
    fn test_negative_values() {
        let constraints = ValidationConstraints::default();
        let mut item = CanvasItem {
            id: 1,
            position: (0.0, 0.0),
            size: (100.0, 50.0),
            content: ItemContent::TextBox {
                text: "Test".to_string(),
                font_size: -10.0,
                color: "#ffffff".to_string(),
            },
        };

        let result = validate_item(&mut item, &constraints);
        assert!(!result.was_valid);
        if let ItemContent::TextBox { font_size, .. } = &item.content {
            assert_eq!(*font_size, constraints.min_font_size);
        }
    }

    #[test]
    fn test_zero_font_size() {
        let constraints = ValidationConstraints::default();
        let mut item = CanvasItem {
            id: 1,
            position: (0.0, 0.0),
            size: (100.0, 50.0),
            content: ItemContent::TextBox {
                text: "Test".to_string(),
                font_size: 0.0,
                color: "#ffffff".to_string(),
            },
        };

        let result = validate_item(&mut item, &constraints);
        assert!(!result.was_valid);
        if let ItemContent::TextBox { font_size, .. } = &item.content {
            assert_eq!(*font_size, constraints.min_font_size);
        }
    }

    #[test]
    fn test_very_large_values() {
        let constraints = ValidationConstraints::default();
        let mut item = CanvasItem {
            id: 1,
            position: (0.0, 0.0),
            size: (100.0, 50.0),
            content: ItemContent::TextBox {
                text: "Test".to_string(),
                font_size: f32::MAX / 2.0,
                color: "#ffffff".to_string(),
            },
        };

        let result = validate_item(&mut item, &constraints);
        assert!(!result.was_valid);
        if let ItemContent::TextBox { font_size, .. } = &item.content {
            assert_eq!(*font_size, constraints.max_font_size);
        }
    }

    #[test]
    fn test_item_size_at_max_dimension() {
        let constraints = ValidationConstraints::default();
        let mut item = CanvasItem {
            id: 1,
            position: (0.0, 0.0),
            size: (constraints.max_item_dimension, constraints.max_item_dimension),
            content: ItemContent::TextBox {
                text: "Test".to_string(),
                font_size: 16.0,
                color: "#ffffff".to_string(),
            },
        };

        let result = validate_item(&mut item, &constraints);
        assert!(result.was_valid);
    }

    #[test]
    fn test_item_size_exceeds_max() {
        let constraints = ValidationConstraints::default();
        let mut item = CanvasItem {
            id: 1,
            position: (0.0, 0.0),
            size: (constraints.max_item_dimension + 1.0, constraints.max_item_dimension + 1.0),
            content: ItemContent::TextBox {
                text: "Test".to_string(),
                font_size: 16.0,
                color: "#ffffff".to_string(),
            },
        };

        let result = validate_item(&mut item, &constraints);
        assert!(!result.was_valid);
        assert_eq!(item.size.0, constraints.max_item_dimension);
        assert_eq!(item.size.1, constraints.max_item_dimension);
    }

    // ========================================================================
    // Edge Case Tests for Multiple Fixes
    // ========================================================================

    #[test]
    fn test_multiple_fixes_in_single_item() {
        let constraints = ValidationConstraints::default();
        let mut item = CanvasItem {
            id: 1,
            position: (0.0, 0.0),
            size: (5.0, 5.0), // Too small
            content: ItemContent::TextBox {
                text: "Test".to_string(),
                font_size: 1.0,          // Too small
                color: "invalid".to_string(), // Invalid
            },
        };

        let result = validate_item(&mut item, &constraints);
        assert!(!result.was_valid);
        assert!(result.fixes_applied.len() >= 3);
    }

    #[test]
    fn test_shape_multiple_invalid_colors() {
        let constraints = ValidationConstraints::default();
        let mut item = CanvasItem {
            id: 1,
            position: (0.0, 0.0),
            size: (100.0, 100.0),
            content: ItemContent::Shape {
                shape_type: ShapeType::Ellipse,
                fill_color: Some("bad".to_string()),
                border_color: "also_bad".to_string(),
                border_width: 2.0,
            },
        };

        let result = validate_item(&mut item, &constraints);
        assert!(!result.was_valid);

        if let ItemContent::Shape {
            fill_color,
            border_color,
            ..
        } = &item.content
        {
            assert!(fill_color.is_none());
            assert_eq!(border_color, "#ffffff");
        }
    }

    // ========================================================================
    // Edge Case Tests for validate_items
    // ========================================================================

    #[test]
    fn test_validate_items_empty_slice() {
        let mut items: Vec<CanvasItem> = vec![];
        let fixed_count = validate_items(&mut items);
        assert_eq!(fixed_count, 0);
    }

    #[test]
    fn test_validate_items_all_valid() {
        let mut items = vec![
            CanvasItem {
                id: 1,
                position: (0.0, 0.0),
                size: (100.0, 50.0),
                content: ItemContent::TextBox {
                    text: "Test".to_string(),
                    font_size: 16.0,
                    color: "#ffffff".to_string(),
                },
            },
            CanvasItem {
                id: 2,
                position: (0.0, 0.0),
                size: (100.0, 50.0),
                content: ItemContent::Arrow {
                    end_offset: (100.0, 50.0),
                    color: "#ff0000".to_string(),
                    thickness: 2.0,
                    head_style: crate::types::ArrowHead::Arrow,
                },
            },
        ];

        let fixed_count = validate_items(&mut items);
        assert_eq!(fixed_count, 0);
    }

    #[test]
    fn test_validate_items_all_invalid() {
        let mut items = vec![
            CanvasItem {
                id: 1,
                position: (0.0, 0.0),
                size: (100.0, 50.0),
                content: ItemContent::TextBox {
                    text: "Test".to_string(),
                    font_size: 1.0, // Invalid
                    color: "#ffffff".to_string(),
                },
            },
            CanvasItem {
                id: 2,
                position: (0.0, 0.0),
                size: (100.0, 50.0),
                content: ItemContent::Arrow {
                    end_offset: (100.0, 50.0),
                    color: "bad".to_string(), // Invalid
                    thickness: 2.0,
                    head_style: crate::types::ArrowHead::Arrow,
                },
            },
        ];

        let fixed_count = validate_items(&mut items);
        assert_eq!(fixed_count, 2);
    }

    #[test]
    fn test_validate_items_mixed_validity() {
        let mut items = vec![
            CanvasItem {
                id: 1,
                position: (0.0, 0.0),
                size: (100.0, 50.0),
                content: ItemContent::TextBox {
                    text: "Valid".to_string(),
                    font_size: 16.0,
                    color: "#ffffff".to_string(),
                },
            },
            CanvasItem {
                id: 2,
                position: (0.0, 0.0),
                size: (100.0, 50.0),
                content: ItemContent::TextBox {
                    text: "Invalid".to_string(),
                    font_size: 1.0, // Invalid
                    color: "#ffffff".to_string(),
                },
            },
            CanvasItem {
                id: 3,
                position: (0.0, 0.0),
                size: (100.0, 50.0),
                content: ItemContent::TextBox {
                    text: "Valid".to_string(),
                    font_size: 20.0,
                    color: "#00ff00".to_string(),
                },
            },
        ];

        let fixed_count = validate_items(&mut items);
        assert_eq!(fixed_count, 1);
    }

    // ========================================================================
    // Edge Case Tests for Different Content Types
    // ========================================================================

    #[test]
    fn test_content_types_without_validation() {
        use std::path::PathBuf;

        let constraints = ValidationConstraints::default();

        // Image - no content-specific validation
        let mut image_item = CanvasItem {
            id: 1,
            position: (0.0, 0.0),
            size: (100.0, 100.0),
            content: ItemContent::Image(PathBuf::from("/test/image.png")),
        };
        assert!(validate_item(&mut image_item, &constraints).was_valid);

        // YouTube - no content-specific validation
        let mut youtube_item = CanvasItem {
            id: 2,
            position: (0.0, 0.0),
            size: (100.0, 100.0),
            content: ItemContent::YouTube("dQw4w9WgXcQ".to_string()),
        };
        assert!(validate_item(&mut youtube_item, &constraints).was_valid);

        // Link - no content-specific validation
        let mut link_item = CanvasItem {
            id: 3,
            position: (0.0, 0.0),
            size: (100.0, 100.0),
            content: ItemContent::Link("https://example.com".to_string()),
        };
        assert!(validate_item(&mut link_item, &constraints).was_valid);
    }

    #[test]
    fn test_min_size_for_different_content_types() {
        use std::path::PathBuf;

        let constraints = ValidationConstraints::default();

        // TextBox has 50x30 minimum
        let mut textbox = CanvasItem {
            id: 1,
            position: (0.0, 0.0),
            size: (10.0, 10.0),
            content: ItemContent::TextBox {
                text: "".to_string(),
                font_size: 16.0,
                color: "#fff".to_string(),
            },
        };
        validate_item(&mut textbox, &constraints);
        assert!(textbox.size.0 >= 50.0);
        assert!(textbox.size.1 >= 30.0);

        // Shape has 30x30 minimum
        let mut shape = CanvasItem {
            id: 2,
            position: (0.0, 0.0),
            size: (10.0, 10.0),
            content: ItemContent::Shape {
                shape_type: ShapeType::Rectangle,
                fill_color: None,
                border_color: "#fff".to_string(),
                border_width: 1.0,
            },
        };
        validate_item(&mut shape, &constraints);
        assert!(shape.size.0 >= 30.0);
        assert!(shape.size.1 >= 30.0);

        // Markdown has 100x36 minimum
        let mut markdown = CanvasItem {
            id: 3,
            position: (0.0, 0.0),
            size: (10.0, 10.0),
            content: ItemContent::Markdown {
                path: PathBuf::from("/test.md"),
                title: "Test".to_string(),
                content: "# Test".to_string(),
            },
        };
        validate_item(&mut markdown, &constraints);
        assert!(markdown.size.0 >= 100.0);
        assert!(markdown.size.1 >= 36.0);

        // Code has 100x36 minimum
        let mut code = CanvasItem {
            id: 4,
            position: (0.0, 0.0),
            size: (10.0, 10.0),
            content: ItemContent::Code {
                path: PathBuf::from("/test.rs"),
                language: "rust".to_string(),
                content: "fn main() {}".to_string(),
            },
        };
        validate_item(&mut code, &constraints);
        assert!(code.size.0 >= 100.0);
        assert!(code.size.1 >= 36.0);
    }

    // ========================================================================
    // Edge Case Tests for ValidationResult
    // ========================================================================

    #[test]
    fn test_validation_result_chaining() {
        let result = ValidationResult::valid()
            .with_fix("Fix 1".to_string())
            .with_fix("Fix 2".to_string())
            .with_fix("Fix 3".to_string());

        assert!(!result.was_valid);
        assert_eq!(result.fixes_applied.len(), 3);
        assert_eq!(result.fixes_applied[0], "Fix 1");
        assert_eq!(result.fixes_applied[1], "Fix 2");
        assert_eq!(result.fixes_applied[2], "Fix 3");
    }

    // ========================================================================
    // Edge Case Tests for Shape Types
    // ========================================================================

    #[test]
    fn test_all_shape_types_validate() {
        let constraints = ValidationConstraints::default();

        for shape_type in [ShapeType::Rectangle, ShapeType::RoundedRect, ShapeType::Ellipse] {
            let mut item = CanvasItem {
                id: 1,
                position: (0.0, 0.0),
                size: (100.0, 100.0),
                content: ItemContent::Shape {
                    shape_type,
                    fill_color: Some("#ff0000".to_string()),
                    border_color: "#ffffff".to_string(),
                    border_width: 2.0,
                },
            };

            let result = validate_item(&mut item, &constraints);
            assert!(result.was_valid, "Shape type {:?} should validate", shape_type);
        }
    }

    #[test]
    fn test_all_arrow_head_styles_validate() {
        use crate::types::ArrowHead;

        let constraints = ValidationConstraints::default();

        for head_style in [ArrowHead::None, ArrowHead::Arrow, ArrowHead::Diamond, ArrowHead::Circle] {
            let mut item = CanvasItem {
                id: 1,
                position: (0.0, 0.0),
                size: (100.0, 100.0),
                content: ItemContent::Arrow {
                    end_offset: (100.0, 50.0),
                    color: "#ffffff".to_string(),
                    thickness: 2.0,
                    head_style,
                },
            };

            let result = validate_item(&mut item, &constraints);
            assert!(result.was_valid, "Arrow head style {:?} should validate", head_style);
        }
    }

    #[test]
    fn test_shape_with_none_fill_color() {
        let constraints = ValidationConstraints::default();
        let mut item = CanvasItem {
            id: 1,
            position: (0.0, 0.0),
            size: (100.0, 100.0),
            content: ItemContent::Shape {
                shape_type: ShapeType::Rectangle,
                fill_color: None,
                border_color: "#ffffff".to_string(),
                border_width: 2.0,
            },
        };

        let result = validate_item(&mut item, &constraints);
        assert!(result.was_valid);
    }
}
