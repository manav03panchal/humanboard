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

    #[test]
    fn test_valid_hex_colors() {
        assert!(is_valid_hex_color("#fff"));
        assert!(is_valid_hex_color("#ffffff"));
        assert!(is_valid_hex_color("#FF00FF"));
        assert!(is_valid_hex_color("#00ff00ff")); // with alpha
        assert!(!is_valid_hex_color("fff"));
        assert!(!is_valid_hex_color("#gggggg"));
        assert!(!is_valid_hex_color("#ff"));
        assert!(!is_valid_hex_color(""));
    }

    #[test]
    fn test_normalize_hex_color() {
        assert_eq!(normalize_hex_color("#fff"), "#ffffff");
        assert_eq!(normalize_hex_color("#abc"), "#aabbcc");
        assert_eq!(normalize_hex_color("#ffffff"), "#ffffff");
        assert_eq!(normalize_hex_color("ff0000"), "#ff0000");
    }

    #[test]
    fn test_validate_textbox_font_size() {
        let constraints = ValidationConstraints::default();
        let mut item = CanvasItem {
            id: 1,
            position: (0.0, 0.0),
            size: (100.0, 50.0),
            content: ItemContent::TextBox {
                text: "Test".to_string(),
                font_size: 5.0, // Too small
                color: "#ffffff".to_string(),
            },
        };

        let result = validate_item(&mut item, &constraints);
        assert!(!result.was_valid);

        if let ItemContent::TextBox { font_size, .. } = &item.content {
            assert_eq!(*font_size, 8.0); // Clamped to minimum
        }
    }

    #[test]
    fn test_validate_textbox_color() {
        let constraints = ValidationConstraints::default();
        let mut item = CanvasItem {
            id: 1,
            position: (0.0, 0.0),
            size: (100.0, 50.0),
            content: ItemContent::TextBox {
                text: "Test".to_string(),
                font_size: 16.0,
                color: "invalid".to_string(),
            },
        };

        let result = validate_item(&mut item, &constraints);
        assert!(!result.was_valid);

        if let ItemContent::TextBox { color, .. } = &item.content {
            assert_eq!(color, "#ffffff");
        }
    }

    #[test]
    fn test_validate_arrow_thickness() {
        let constraints = ValidationConstraints::default();
        let mut item = CanvasItem {
            id: 1,
            position: (0.0, 0.0),
            size: (100.0, 50.0),
            content: ItemContent::Arrow {
                end_offset: (100.0, 50.0),
                color: "#ffffff".to_string(),
                thickness: 100.0, // Too thick
                head_style: crate::types::ArrowHead::Arrow,
            },
        };

        let result = validate_item(&mut item, &constraints);
        assert!(!result.was_valid);

        if let ItemContent::Arrow { thickness, .. } = &item.content {
            assert_eq!(*thickness, 20.0); // Clamped to maximum
        }
    }

    #[test]
    fn test_validate_shape_border() {
        let constraints = ValidationConstraints::default();
        let mut item = CanvasItem {
            id: 1,
            position: (0.0, 0.0),
            size: (100.0, 50.0),
            content: ItemContent::Shape {
                shape_type: ShapeType::Rectangle,
                fill_color: Some("notacolor".to_string()),
                border_color: "#ffffff".to_string(),
                border_width: 100.0, // Too wide
            },
        };

        let result = validate_item(&mut item, &constraints);
        assert!(!result.was_valid);

        if let ItemContent::Shape {
            border_width,
            fill_color,
            ..
        } = &item.content
        {
            assert_eq!(*border_width, 50.0); // Clamped
            assert!(fill_color.is_none()); // Invalid color removed
        }
    }

    #[test]
    fn test_validate_item_size() {
        let constraints = ValidationConstraints::default();
        let mut item = CanvasItem {
            id: 1,
            position: (0.0, 0.0),
            size: (5.0, 5.0), // Too small
            content: ItemContent::TextBox {
                text: "Test".to_string(),
                font_size: 16.0,
                color: "#ffffff".to_string(),
            },
        };

        let result = validate_item(&mut item, &constraints);
        assert!(!result.was_valid);
        assert!(item.size.0 >= 50.0); // TextBox minimum width
        assert!(item.size.1 >= 30.0); // TextBox minimum height
    }

    #[test]
    fn test_valid_item_passes() {
        let constraints = ValidationConstraints::default();
        let mut item = CanvasItem {
            id: 1,
            position: (0.0, 0.0),
            size: (200.0, 100.0),
            content: ItemContent::TextBox {
                text: "Test".to_string(),
                font_size: 16.0,
                color: "#ffffff".to_string(),
            },
        };

        let result = validate_item(&mut item, &constraints);
        assert!(result.was_valid);
        assert!(result.fixes_applied.is_empty());
    }
}
