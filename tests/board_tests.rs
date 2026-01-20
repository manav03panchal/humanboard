//! Board module tests.
//!
//! Tests for the Board struct including item management, undo/redo, and history.

use humanboard::board::{Board, UndoOperation};
use humanboard::types::{CanvasItem, ItemContent};
use gpui::{point, px};
use std::collections::HashMap;

// Match the constant from board.rs for delta-based history
const MAX_HISTORY_OPERATIONS: usize = 100;

#[test]
fn test_board_new_empty() {
    let board = Board::new_for_test();
    assert_eq!(board.zoom, 1.0);
    assert!(board.items.is_empty());
    assert_eq!(board.next_item_id, 0);
}

#[test]
fn test_add_item() {
    let mut board = Board::new_for_test();
    let pos = point(px(100.0), px(200.0));
    board.add_item(pos, ItemContent::Text("Test".to_string()));

    assert_eq!(board.items.len(), 1);
    assert_eq!(board.items[0].id, 0);
    assert_eq!(board.next_item_id, 1);
}

#[test]
fn test_get_item_by_id() {
    let mut board = Board::new_for_test();
    board.add_item(
        point(px(0.0), px(0.0)),
        ItemContent::Text("Test".to_string()),
    );

    let item = board.get_item(0);
    assert!(item.is_some());
    assert_eq!(item.unwrap().id, 0);

    let missing = board.get_item(999);
    assert!(missing.is_none());
}

#[test]
fn test_add_multiple_items() {
    let mut board = Board::new_for_test();

    board.add_item(
        point(px(0.0), px(0.0)),
        ItemContent::Text("First".to_string()),
    );
    board.add_item(
        point(px(100.0), px(100.0)),
        ItemContent::Text("Second".to_string()),
    );
    board.add_item(
        point(px(200.0), px(200.0)),
        ItemContent::Text("Third".to_string()),
    );

    assert_eq!(board.items.len(), 3);
    assert_eq!(board.next_item_id, 3);
    assert_eq!(board.items[0].id, 0);
    assert_eq!(board.items[1].id, 1);
    assert_eq!(board.items[2].id, 2);
}

#[test]
fn test_remove_item() {
    let mut board = Board::new_for_test();
    board.add_item(
        point(px(0.0), px(0.0)),
        ItemContent::Text("First".to_string()),
    );
    board.add_item(
        point(px(100.0), px(100.0)),
        ItemContent::Text("Second".to_string()),
    );

    assert!(board.remove_item(0));
    assert_eq!(board.items.len(), 1);
    assert!(board.get_item(0).is_none());
    assert!(board.get_item(1).is_some());
}

#[test]
fn test_undo_empty() {
    let mut board = Board::new_for_test();
    assert!(!board.undo());
}

#[test]
fn test_redo_empty() {
    let mut board = Board::new_for_test();
    assert!(!board.redo());
}

#[test]
fn test_undo_after_add() {
    let mut board = Board::new_for_test();
    board.add_item(
        point(px(0.0), px(0.0)),
        ItemContent::Text("Test".to_string()),
    );

    assert_eq!(board.items.len(), 1);
    assert!(board.undo());
    assert_eq!(board.items.len(), 0);
}

#[test]
fn test_redo_after_undo() {
    let mut board = Board::new_for_test();
    board.add_item(
        point(px(0.0), px(0.0)),
        ItemContent::Text("Test".to_string()),
    );

    board.undo();
    assert_eq!(board.items.len(), 0);

    assert!(board.redo());
    assert_eq!(board.items.len(), 1);
}

#[test]
fn test_undo_redo_multiple() {
    let mut board = Board::new_for_test();

    board.add_item(
        point(px(0.0), px(0.0)),
        ItemContent::Text("First".to_string()),
    );
    board.add_item(
        point(px(100.0), px(100.0)),
        ItemContent::Text("Second".to_string()),
    );

    assert_eq!(board.items.len(), 2);

    board.undo();
    assert_eq!(board.items.len(), 1);

    board.undo();
    assert_eq!(board.items.len(), 0);

    board.redo();
    assert_eq!(board.items.len(), 1);

    board.redo();
    assert_eq!(board.items.len(), 2);
}

#[test]
fn test_undo_then_add_clears_redo() {
    let mut board = Board::new_for_test();

    board.add_item(
        point(px(0.0), px(0.0)),
        ItemContent::Text("First".to_string()),
    );
    board.add_item(
        point(px(100.0), px(100.0)),
        ItemContent::Text("Second".to_string()),
    );

    board.undo();

    board.add_item(
        point(px(200.0), px(200.0)),
        ItemContent::Text("Third".to_string()),
    );

    assert!(!board.redo());
}

#[test]
fn test_board_state_serialization() {
    use humanboard::board::BoardState;

    let state = BoardState {
        canvas_offset: (10.0, 20.0),
        zoom: 1.5,
        items: vec![CanvasItem {
            id: 1,
            position: (100.0, 200.0),
            size: (300.0, 400.0),
            content: ItemContent::Text("Test".to_string()),
        }],
        next_item_id: 2,
        data_sources: HashMap::new(),
        next_data_source_id: 0,
    };

    let json = serde_json::to_string(&state).unwrap();
    let restored: BoardState = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.canvas_offset, (10.0, 20.0));
    assert_eq!(restored.zoom, 1.5);
    assert_eq!(restored.items.len(), 1);
    assert_eq!(restored.next_item_id, 2);
}

#[test]
fn test_history_limit() {
    let mut board = Board::new_for_test();

    for i in 0..60 {
        board.add_item(
            point(px(i as f32 * 10.0), px(0.0)),
            ItemContent::Text(format!("Item {}", i)),
        );
    }

    assert!(board.history_len() <= MAX_HISTORY_OPERATIONS + 1);
}

#[test]
fn test_screen_to_canvas_conversion() {
    let mut board = Board::new_for_test();
    board.canvas_offset = point(px(100.0), px(50.0));
    board.zoom = 2.0;

    let screen_pos = point(px(300.0), px(150.0));
    let canvas_pos = board.screen_to_canvas(screen_pos);

    // (300 - 44 - 100) / 2 = 78, (150 - 40 - 50) / 2 = 30
    // (accounts for dock width 44px and header height 40px)
    assert_eq!(f32::from(canvas_pos.x), 78.0);
    assert_eq!(f32::from(canvas_pos.y), 30.0);
}

#[test]
fn test_find_items_by_name() {
    let mut board = Board::new_for_test();
    board.add_item(
        point(px(0.0), px(0.0)),
        ItemContent::Text("Hello World".to_string()),
    );
    board.add_item(
        point(px(100.0), px(0.0)),
        ItemContent::Text("Goodbye World".to_string()),
    );
    board.add_item(
        point(px(200.0), px(0.0)),
        ItemContent::Text("Hello Again".to_string()),
    );

    let results = board.find_items("Hello");
    assert_eq!(results.len(), 2);

    let results = board.find_items("World");
    assert_eq!(results.len(), 2);

    let results = board.find_items("Goodbye");
    assert_eq!(results.len(), 1);

    let results = board.find_items("NotFound");
    assert_eq!(results.len(), 0);
}

#[test]
fn test_find_items_case_insensitive() {
    let mut board = Board::new_for_test();
    board.add_item(
        point(px(0.0), px(0.0)),
        ItemContent::Text("Hello World".to_string()),
    );

    let results = board.find_items("hello");
    assert_eq!(results.len(), 1);

    let results = board.find_items("HELLO");
    assert_eq!(results.len(), 1);
}

#[test]
fn test_canvas_to_screen_conversion() {
    let mut board = Board::new_for_test();
    board.canvas_offset = point(px(100.0), px(50.0));
    board.zoom = 2.0;

    let canvas_pos = point(px(100.0), px(50.0));
    let screen_pos = board.canvas_to_screen(canvas_pos);

    // 100 * 2 + 100 = 300, 50 * 2 + 50 = 150
    assert_eq!(f32::from(screen_pos.x), 300.0);
    assert_eq!(f32::from(screen_pos.y), 150.0);
}

#[test]
fn test_zoom_bounds() {
    let mut board = Board::new_for_test();
    let center = point(px(500.0), px(500.0));

    // Zoom in many times - should clamp at max
    for _ in 0..50 {
        board.zoom_in(center);
    }
    assert!(board.zoom <= 10.0);

    // Zoom out many times - should clamp at min
    for _ in 0..100 {
        board.zoom_out(center);
    }
    assert!(board.zoom >= 0.1);
}

// ========================================================================
// Undo/Redo State Tests
// ========================================================================

#[test]
fn test_undo_does_not_affect_canvas_offset() {
    let mut board = Board::new_for_test();

    // Add item at initial offset
    board.add_item(
        point(px(0.0), px(0.0)),
        ItemContent::Text("First".to_string()),
    );

    // Change canvas offset (view state, independent of items)
    board.canvas_offset = point(px(100.0), px(200.0));
    board.add_item(
        point(px(50.0), px(50.0)),
        ItemContent::Text("Second".to_string()),
    );

    // Undo removes item but does NOT change canvas offset (delta-based undo only affects items)
    board.undo();
    assert_eq!(board.items.len(), 1); // Second item removed
    // Canvas offset is view state, unchanged by item undo
    assert_eq!(f32::from(board.canvas_offset.x), 100.0);
    assert_eq!(f32::from(board.canvas_offset.y), 200.0);
}

#[test]
fn test_undo_does_not_affect_zoom() {
    let mut board = Board::new_for_test();

    // Add item at default zoom
    board.add_item(
        point(px(0.0), px(0.0)),
        ItemContent::Text("First".to_string()),
    );

    // Change zoom (view state, independent of items)
    board.zoom = 2.5;
    board.add_item(
        point(px(50.0), px(50.0)),
        ItemContent::Text("Second".to_string()),
    );

    // Undo removes item but does NOT change zoom (delta-based undo only affects items)
    board.undo();
    assert_eq!(board.items.len(), 1); // Second item removed
    // Zoom is view state, unchanged by item undo
    assert_eq!(board.zoom, 2.5);
}

#[test]
fn test_undo_keeps_next_item_id_monotonic() {
    let mut board = Board::new_for_test();

    board.add_item(
        point(px(0.0), px(0.0)),
        ItemContent::Text("First".to_string()),
    );
    assert_eq!(board.next_item_id, 1);

    board.add_item(
        point(px(50.0), px(50.0)),
        ItemContent::Text("Second".to_string()),
    );
    assert_eq!(board.next_item_id, 2);

    // Undo removes item but next_item_id stays monotonic (prevents ID collisions)
    board.undo();
    assert_eq!(board.items.len(), 1);
    assert_eq!(board.next_item_id, 2); // Stays at 2 to prevent ID reuse
}

#[test]
fn test_undo_rebuilds_items_index() {
    let mut board = Board::new_for_test();

    board.add_item(
        point(px(0.0), px(0.0)),
        ItemContent::Text("First".to_string()),
    );
    board.add_item(
        point(px(100.0), px(0.0)),
        ItemContent::Text("Second".to_string()),
    );

    // Verify items are accessible by ID
    assert!(board.get_item(0).is_some());
    assert!(board.get_item(1).is_some());

    // Undo removes second item
    board.undo();

    // Items index should be rebuilt - first item still accessible, second not
    assert!(board.get_item(0).is_some());
    assert!(board.get_item(1).is_none());
}

#[test]
fn test_redo_rebuilds_items_index() {
    let mut board = Board::new_for_test();

    board.add_item(
        point(px(0.0), px(0.0)),
        ItemContent::Text("First".to_string()),
    );
    board.add_item(
        point(px(100.0), px(0.0)),
        ItemContent::Text("Second".to_string()),
    );

    board.undo();
    assert!(board.get_item(1).is_none());

    // Redo should restore the item and rebuild index
    board.redo();
    assert!(board.get_item(0).is_some());
    assert!(board.get_item(1).is_some());
}

#[test]
fn test_undo_at_first_state_returns_false() {
    let mut board = Board::new_for_test();

    // First undo should fail (at initial state)
    assert!(!board.undo());

    // Add an item and undo back to initial state
    board.add_item(
        point(px(0.0), px(0.0)),
        ItemContent::Text("Test".to_string()),
    );
    assert!(board.undo());

    // Second undo should fail (back at initial state)
    assert!(!board.undo());
}

#[test]
fn test_redo_at_last_state_returns_false() {
    let mut board = Board::new_for_test();

    // Redo without any undo should fail
    assert!(!board.redo());

    // Add items
    board.add_item(
        point(px(0.0), px(0.0)),
        ItemContent::Text("First".to_string()),
    );
    board.add_item(
        point(px(100.0), px(0.0)),
        ItemContent::Text("Second".to_string()),
    );

    // Redo should still fail (at latest state)
    assert!(!board.redo());
}

#[test]
fn test_multiple_consecutive_undo_at_boundary() {
    let mut board = Board::new_for_test();

    board.add_item(
        point(px(0.0), px(0.0)),
        ItemContent::Text("Test".to_string()),
    );

    // Undo to initial state
    assert!(board.undo());
    assert_eq!(board.items.len(), 0);

    // Multiple undos at boundary should all return false
    assert!(!board.undo());
    assert!(!board.undo());
    assert!(!board.undo());

    // State should remain at initial
    assert_eq!(board.items.len(), 0);
}

#[test]
fn test_multiple_consecutive_redo_at_boundary() {
    let mut board = Board::new_for_test();

    board.add_item(
        point(px(0.0), px(0.0)),
        ItemContent::Text("First".to_string()),
    );
    board.add_item(
        point(px(100.0), px(0.0)),
        ItemContent::Text("Second".to_string()),
    );

    // Undo once
    board.undo();
    assert_eq!(board.items.len(), 1);

    // Redo back to latest
    assert!(board.redo());
    assert_eq!(board.items.len(), 2);

    // Multiple redos at boundary should all return false
    assert!(!board.redo());
    assert!(!board.redo());
    assert!(!board.redo());

    // State should remain at latest
    assert_eq!(board.items.len(), 2);
}

#[test]
fn test_undo_all_then_redo_all() {
    let mut board = Board::new_for_test();

    // Add 5 items
    for i in 0..5 {
        board.add_item(
            point(px(i as f32 * 100.0), px(0.0)),
            ItemContent::Text(format!("Item {}", i)),
        );
    }
    assert_eq!(board.items.len(), 5);

    // Undo all the way back
    for expected in (0..5).rev() {
        assert!(board.undo());
        assert_eq!(board.items.len(), expected);
    }
    assert!(!board.undo()); // Can't undo further

    // Redo all the way forward
    for expected in 1..=5 {
        assert!(board.redo());
        assert_eq!(board.items.len(), expected);
    }
    assert!(!board.redo()); // Can't redo further
}

#[test]
fn test_branch_pruning_clears_all_redo_states() {
    let mut board = Board::new_for_test();

    // Create history: A -> B -> C -> D
    board.add_item(
        point(px(0.0), px(0.0)),
        ItemContent::Text("A".to_string()),
    );
    board.add_item(
        point(px(100.0), px(0.0)),
        ItemContent::Text("B".to_string()),
    );
    board.add_item(
        point(px(200.0), px(0.0)),
        ItemContent::Text("C".to_string()),
    );
    board.add_item(
        point(px(300.0), px(0.0)),
        ItemContent::Text("D".to_string()),
    );

    // Undo twice: now at state B (2 items)
    board.undo();
    board.undo();
    assert_eq!(board.items.len(), 2);

    // Add new item E - should prune C and D from redo
    board.add_item(
        point(px(400.0), px(0.0)),
        ItemContent::Text("E".to_string()),
    );
    assert_eq!(board.items.len(), 3);

    // Redo should fail - branch was pruned
    assert!(!board.redo());
}

#[test]
fn test_history_limit_removes_oldest_state() {
    let mut board = Board::new_for_test();

    // Add more items than MAX_HISTORY_OPERATIONS
    for i in 0..(MAX_HISTORY_OPERATIONS + 10) {
        board.add_item(
            point(px(i as f32 * 10.0), px(0.0)),
            ItemContent::Text(format!("Item {}", i)),
        );
    }

    // History should be limited
    assert!(board.history_len() <= MAX_HISTORY_OPERATIONS + 1);

    // We can't undo all the way back to empty state
    let mut undo_count = 0;
    while board.undo() {
        undo_count += 1;
    }

    // Should be able to undo MAX_HISTORY_OPERATIONS times at most
    assert!(undo_count <= MAX_HISTORY_OPERATIONS);
    // Items should not be empty because oldest states were pruned
    assert!(board.items.len() > 0);
}

#[test]
fn test_undo_redo_with_item_position_changes() {
    let mut board = Board::new_for_test();

    board.add_item(
        point(px(0.0), px(0.0)),
        ItemContent::Text("Test".to_string()),
    );
    let original_pos = board.items[0].position;

    // Modify item position using delta operation
    let old_pos = board.items[0].position;
    let new_pos = (100.0, 200.0);
    board.items[0].position = new_pos;
    board.push_operation(UndoOperation::MoveItem {
        id: 0,
        old_pos,
        new_pos,
    });

    // Verify new position
    assert_eq!(board.items[0].position, (100.0, 200.0));

    // Undo should restore original position
    board.undo();
    assert_eq!(board.items[0].position, original_pos);

    // Redo should restore modified position
    board.redo();
    assert_eq!(board.items[0].position, (100.0, 200.0));
}

#[test]
fn test_undo_redo_with_item_size_changes() {
    let mut board = Board::new_for_test();

    board.add_item(
        point(px(0.0), px(0.0)),
        ItemContent::Text("Test".to_string()),
    );
    let original_size = board.items[0].size;

    // Modify item size using delta operation
    let old_size = board.items[0].size;
    let new_size = (500.0, 300.0);
    board.items[0].size = new_size;
    board.push_operation(UndoOperation::ResizeItem {
        id: 0,
        old_size,
        new_size,
    });

    // Undo should restore original size
    board.undo();
    assert_eq!(board.items[0].size, original_size);

    // Redo should restore modified size
    board.redo();
    assert_eq!(board.items[0].size, (500.0, 300.0));
}

#[test]
fn test_history_index_consistency() {
    let mut board = Board::new_for_test();

    // Initial state: with delta-based history, we start empty
    assert_eq!(board.current_history_index(), 0);
    assert_eq!(board.history_len(), 0);

    // Add items and verify history_index grows
    board.add_item(
        point(px(0.0), px(0.0)),
        ItemContent::Text("First".to_string()),
    );
    assert_eq!(board.current_history_index(), 1);
    assert_eq!(board.history_len(), 1);

    board.add_item(
        point(px(100.0), px(0.0)),
        ItemContent::Text("Second".to_string()),
    );
    assert_eq!(board.current_history_index(), 2);
    assert_eq!(board.history_len(), 2);

    // Undo and verify history_index decreases
    board.undo();
    assert_eq!(board.current_history_index(), 1);

    board.undo();
    assert_eq!(board.current_history_index(), 0);

    // Redo and verify history_index increases
    board.redo();
    assert_eq!(board.current_history_index(), 1);
}

#[test]
fn test_remove_item_with_undo() {
    let mut board = Board::new_for_test();

    board.add_item(
        point(px(0.0), px(0.0)),
        ItemContent::Text("First".to_string()),
    );
    board.add_item(
        point(px(100.0), px(0.0)),
        ItemContent::Text("Second".to_string()),
    );

    // Remove an item (remove_item internally calls push_history)
    board.remove_item(0);
    assert_eq!(board.items.len(), 1);
    assert!(board.get_item(0).is_none());
    assert!(board.get_item(1).is_some());

    // Undo should restore the removed item
    board.undo();
    assert_eq!(board.items.len(), 2);
    assert!(board.get_item(0).is_some());
    assert!(board.get_item(1).is_some());
}

#[test]
fn test_undo_redo_state_isolation() {
    let mut board = Board::new_for_test();

    board.add_item(
        point(px(0.0), px(0.0)),
        ItemContent::Text("Test".to_string()),
    );

    // Capture the original item
    let old_item = board.items[0].clone();

    // Modify the item content using delta operation
    let mut new_item = old_item.clone();
    new_item.content = ItemContent::Text("Modified".to_string());
    board.items[0] = new_item.clone();
    board.push_operation(UndoOperation::ModifyItem {
        old_item: old_item.clone(),
        new_item,
    });

    // Verify modification
    if let ItemContent::Text(t) = &board.items[0].content {
        assert_eq!(t, "Modified");
    } else {
        panic!("Expected Text content");
    }

    // Undo should restore original content
    board.undo();
    if let ItemContent::Text(t) = &board.items[0].content {
        assert_eq!(t, "Test");
    } else {
        panic!("Expected Text content after undo");
    }
}

#[test]
fn test_push_history_at_capacity() {
    let mut board = Board::new_for_test();

    // Fill history to capacity
    for i in 0..MAX_HISTORY_OPERATIONS {
        board.add_item(
            point(px(i as f32 * 10.0), px(0.0)),
            ItemContent::Text(format!("Item {}", i)),
        );
    }

    let history_len_at_capacity = board.history_len();

    // Add one more - should stay at or below capacity
    board.add_item(
        point(px(1000.0), px(0.0)),
        ItemContent::Text("Overflow".to_string()),
    );

    assert!(board.history_len() <= history_len_at_capacity);
}

#[test]
fn test_undo_redo_preserves_item_id() {
    let mut board = Board::new_for_test();

    board.add_item(
        point(px(0.0), px(0.0)),
        ItemContent::Text("Test".to_string()),
    );

    let original_id = board.items[0].id;

    board.add_item(
        point(px(100.0), px(0.0)),
        ItemContent::Text("Second".to_string()),
    );

    // Undo and verify first item's ID is preserved
    board.undo();
    assert_eq!(board.items[0].id, original_id);

    // Redo and verify ID is still correct
    board.redo();
    assert_eq!(board.items[0].id, original_id);
}
