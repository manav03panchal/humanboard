//! Selection Integration Tests

use humanboard::board::Board;
use humanboard::selection::{MarqueeState, SelectionManager};
use humanboard::types::ItemContent;
use gpui::{point, px, size, Bounds};

#[test]
fn test_select_board_items() {
    let mut board = Board::new_for_test();
    let mut selection = SelectionManager::new();

    board.add_item(point(px(0.0), px(0.0)), ItemContent::Text("Item 0".to_string()));
    board.add_item(point(px(100.0), px(0.0)), ItemContent::Text("Item 1".to_string()));

    selection.select(board.items[0].id);
    assert!(selection.is_selected(0));
    assert_eq!(selection.count(), 1);
}

#[test]
fn test_multi_select_toggle() {
    let mut selection = SelectionManager::new();

    selection.select(0);
    selection.toggle(2);
    selection.toggle(4);

    assert!(selection.is_selected(0));
    assert!(selection.is_selected(2));
    assert!(selection.is_selected(4));
    assert_eq!(selection.count(), 3);
}

#[test]
fn test_marquee_bounds_calculation() {
    let marquee = MarqueeState {
        start: point(px(300.0), px(300.0)),
        current: point(px(100.0), px(100.0)),
    };

    let bounds = marquee.bounds();
    assert_eq!(f32::from(bounds.origin.x), 100.0);
    assert_eq!(f32::from(bounds.origin.y), 100.0);
    assert_eq!(f32::from(bounds.size.width), 200.0);
}

#[test]
fn test_marquee_intersects() {
    let marquee = MarqueeState {
        start: point(px(50.0), px(50.0)),
        current: point(px(250.0), px(250.0)),
    };

    let inside = Bounds { origin: point(px(100.0), px(100.0)), size: size(px(50.0), px(50.0)) };
    let outside = Bounds { origin: point(px(500.0), px(500.0)), size: size(px(50.0), px(50.0)) };

    assert!(marquee.intersects(inside));
    assert!(!marquee.intersects(outside));
}

#[test]
fn test_additive_marquee() {
    let mut selection = SelectionManager::new();

    selection.select(10);
    selection.toggle(11);

    selection.start_marquee(point(px(0.0), px(0.0)), true);
    selection.finish_marquee(vec![0, 1, 2]);

    assert!(selection.is_selected(10));
    assert!(selection.is_selected(11));
    assert!(selection.is_selected(0));
    assert_eq!(selection.count(), 5);
}
