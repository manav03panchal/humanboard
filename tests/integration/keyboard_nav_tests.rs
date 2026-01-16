//! Keyboard Navigation Integration Tests

use humanboard::board::Board;
use humanboard::selection::SelectionManager;
use humanboard::types::ItemContent;
use gpui::{point, px};

#[test]
fn test_arrow_key_nudge() {
    let mut board = Board::new_for_test();

    board.add_item(point(px(100.0), px(100.0)), ItemContent::Text("Test".to_string()));
    let original_pos = board.items[0].position;

    // Nudge up (Arrow Up)
    board.items[0].position.1 -= 10.0;
    assert_eq!(board.items[0].position.1, original_pos.1 - 10.0);

    // Nudge down (Arrow Down)
    board.items[0].position.1 += 10.0;
    assert_eq!(board.items[0].position.1, original_pos.1);

    // Nudge left (Arrow Left)
    board.items[0].position.0 -= 10.0;
    assert_eq!(board.items[0].position.0, original_pos.0 - 10.0);

    // Nudge right (Arrow Right)
    board.items[0].position.0 += 10.0;
    assert_eq!(board.items[0].position.0, original_pos.0);
}

#[test]
fn test_multi_item_nudge() {
    let mut board = Board::new_for_test();
    let mut selection = SelectionManager::new();

    // Add items
    board.add_item(point(px(100.0), px(100.0)), ItemContent::Text("A".to_string()));
    board.add_item(point(px(200.0), px(100.0)), ItemContent::Text("B".to_string()));
    board.add_item(point(px(300.0), px(100.0)), ItemContent::Text("C".to_string()));

    // Select all
    selection.select_all(board.items.iter().map(|i| i.id));

    // Nudge all selected items down
    for item in &mut board.items {
        if selection.is_selected(item.id) {
            item.position.1 += 10.0;
        }
    }

    assert_eq!(board.items[0].position.1, 110.0);
    assert_eq!(board.items[1].position.1, 110.0);
    assert_eq!(board.items[2].position.1, 110.0);
}

#[test]
fn test_undo_redo_keyboard() {
    let mut board = Board::new_for_test();

    board.add_item(point(px(0.0), px(0.0)), ItemContent::Text("First".to_string()));
    board.add_item(point(px(100.0), px(0.0)), ItemContent::Text("Second".to_string()));
    assert_eq!(board.items.len(), 2);

    // Cmd+Z (undo)
    board.undo();
    assert_eq!(board.items.len(), 1);

    // Cmd+Shift+Z (redo)
    board.redo();
    assert_eq!(board.items.len(), 2);
}

#[test]
fn test_escape_deselect() {
    let mut selection = SelectionManager::new();

    selection.select(0);
    selection.toggle(1);
    selection.toggle(2);
    assert_eq!(selection.count(), 3);

    // Escape clears selection
    selection.clear();
    assert_eq!(selection.count(), 0);
}

#[test]
fn test_select_all_shortcut() {
    let mut board = Board::new_for_test();
    let mut selection = SelectionManager::new();

    for i in 0..10 {
        board.add_item(point(px(i as f32 * 50.0), px(0.0)), ItemContent::Text(format!("Item {}", i)));
    }

    // Cmd+A (select all)
    selection.select_all(board.items.iter().map(|i| i.id));

    assert_eq!(selection.count(), 10);
}

#[test]
fn test_delete_shortcut() {
    let mut board = Board::new_for_test();
    let mut selection = SelectionManager::new();

    board.add_item(point(px(0.0), px(0.0)), ItemContent::Text("Delete me".to_string()));
    board.add_item(point(px(100.0), px(0.0)), ItemContent::Text("Keep me".to_string()));

    selection.select(0);

    // Backspace/Delete removes selected
    let to_remove: Vec<u64> = selection.selected().iter().copied().collect();
    for id in to_remove {
        board.remove_item(id);
    }
    selection.clear();

    assert_eq!(board.items.len(), 1);
    assert!(board.get_item(1).is_some());
}

#[test]
fn test_zoom_shortcuts() {
    let mut board = Board::new_for_test();
    let center = point(px(500.0), px(500.0));

    // Cmd+= (zoom in)
    board.zoom_in(center);
    let zoomed_in = board.zoom;
    assert!(zoomed_in > 1.0);

    // Cmd+- (zoom out)
    board.zoom_out(center);
    assert!(board.zoom < zoomed_in);

    // Cmd+0 (reset zoom)
    board.zoom_reset();
    assert_eq!(board.zoom, 1.0);
}
