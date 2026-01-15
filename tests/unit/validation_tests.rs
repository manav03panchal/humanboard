//! Unit tests for validation module.

use humanboard::types::{ArrowHead, CanvasItem, ItemContent, ShapeType};
use humanboard::validation::{
    is_valid_hex_color, normalize_hex_color, validate_item, ValidationConstraints,
};

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
            head_style: ArrowHead::Arrow,
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
