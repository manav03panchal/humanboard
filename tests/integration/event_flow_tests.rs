//! Event Flow Integration Tests

use humanboard::board::Board;
use humanboard::types::ItemContent;
use gpui::{point, px};

#[test]
fn test_nudge_items_workflow() {
    let mut board = Board::new_for_test();

    board.add_item(point(px(100.0), px(100.0)), ItemContent::Text("Item".to_string()));

    // Simulate nudge operations
    board.items[0].position.0 += 10.0; // Right
    board.items[0].position.1 -= 10.0; // Up

    assert_eq!(board.items[0].position, (110.0, 90.0));
}

#[test]
fn test_duplicate_workflow() {
    let mut board = Board::new_for_test();

    board.add_item(point(px(100.0), px(100.0)), ItemContent::Text("Original".to_string()));

    // Simulate duplicate
    let original = board.items[0].clone();
    let mut duplicate = original.clone();
    duplicate.id = board.next_item_id;
    board.next_item_id += 1;
    duplicate.position.0 += 20.0;
    duplicate.position.1 += 20.0;
    board.items.push(duplicate);

    assert_eq!(board.items.len(), 2);
    assert_eq!(board.items[1].position, (120.0, 120.0));
}

#[test]
fn test_delete_selected_workflow() {
    let mut board = Board::new_for_test();

    for i in 0..5 {
        board.add_item(point(px(i as f32 * 100.0), px(0.0)), ItemContent::Text(format!("Item {}", i)));
    }

    // Delete items 1 and 3
    board.remove_items(&[1, 3]);

    assert_eq!(board.items.len(), 3);
    assert!(board.get_item(0).is_some());
    assert!(board.get_item(1).is_none());
    assert!(board.get_item(2).is_some());
    assert!(board.get_item(3).is_none());
    assert!(board.get_item(4).is_some());
}

#[test]
fn test_zoom_operations() {
    let mut board = Board::new_for_test();
    let center = point(px(500.0), px(500.0));

    assert_eq!(board.zoom, 1.0);

    board.zoom_in(center);
    assert!(board.zoom > 1.0);

    board.zoom_reset();
    assert_eq!(board.zoom, 1.0);

    board.zoom_out(center);
    assert!(board.zoom < 1.0);
}

#[test]
fn test_action_sequence() {
    let mut board = Board::new_for_test();

    // Add
    board.add_item(point(px(0.0), px(0.0)), ItemContent::Text("A".to_string()));
    board.push_history(); // Save baseline

    // Edit
    board.items[0].content = ItemContent::Text("A Modified".to_string());
    board.push_history();

    // Undo edit
    board.undo();
    if let ItemContent::Text(t) = &board.items[0].content {
        assert_eq!(t, "A");
    }

    // Redo edit
    board.redo();
    if let ItemContent::Text(t) = &board.items[0].content {
        assert_eq!(t, "A Modified");
    }
}
