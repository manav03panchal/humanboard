//! State Management Integration Tests

use humanboard::board::{Board, BoardState};
use humanboard::board_index::{BoardIndex, BoardMetadata};
use humanboard::notifications::ToastManager;
use humanboard::types::{CanvasItem, ItemContent};
use gpui::{point, px};
use std::collections::HashMap;

#[test]
fn test_board_state_serialization() {
    let state = BoardState {
        canvas_offset: (100.0, 200.0),
        zoom: 1.5,
        items: vec![
            CanvasItem {
                id: 0,
                position: (50.0, 50.0),
                size: (200.0, 150.0),
                content: ItemContent::Text("Test".to_string()),
            },
        ],
        next_item_id: 1,
        data_sources: HashMap::new(),
        next_data_source_id: 0,
    };

    let json = serde_json::to_string(&state).unwrap();
    let restored: BoardState = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.canvas_offset, (100.0, 200.0));
    assert_eq!(restored.zoom, 1.5);
    assert_eq!(restored.items.len(), 1);
}

#[test]
fn test_board_index_state() {
    let mut index = BoardIndex::default();

    index.boards.push(BoardMetadata::new("Board A".to_string()));
    index.boards.push(BoardMetadata::new("Board B".to_string()));

    assert_eq!(index.boards.len(), 2);
    assert_ne!(index.boards[0].id, index.boards[1].id);
}

#[test]
fn test_toast_manager_state() {
    use humanboard::notifications::Toast;

    let mut manager = ToastManager::new();

    assert!(manager.is_empty());
    manager.push(Toast::success("First"));
    manager.push(Toast::info("Second"));
    assert_eq!(manager.count(), 2);

    manager.clear();
    assert!(manager.is_empty());
}

#[test]
fn test_toast_removal() {
    use humanboard::notifications::Toast;

    let mut manager = ToastManager::new();

    manager.push(Toast::success("Toast 1"));
    manager.push(Toast::info("Toast 2"));
    manager.push(Toast::warning("Toast 3"));

    let toast_id = manager.toasts()[1].id;
    manager.remove(toast_id);

    assert_eq!(manager.count(), 2);
}

#[test]
fn test_history_state_management() {
    let mut board = Board::new_for_test();

    // Empty board starts with no history
    assert_eq!(board.history_len(), 0);
    assert_eq!(board.current_history_index(), 0);

    // Adding an item creates one history entry (the operation)
    board.add_item(point(px(0.0), px(0.0)), ItemContent::Text("A".to_string()));
    assert_eq!(board.history_len(), 1);
    assert_eq!(board.current_history_index(), 1);

    // Undo removes the item and moves index back
    board.undo();
    assert_eq!(board.current_history_index(), 0);
}

#[test]
fn test_canvas_state_persistence() {
    let mut board = Board::new_for_test();

    board.canvas_offset = point(px(150.0), px(250.0));
    board.zoom = 2.0;
    board.add_item(point(px(0.0), px(0.0)), ItemContent::Text("Test".to_string()));

    let state = BoardState {
        canvas_offset: (f32::from(board.canvas_offset.x), f32::from(board.canvas_offset.y)),
        zoom: board.zoom,
        items: board.items.iter().map(|i| CanvasItem {
            id: i.id, position: i.position, size: i.size, content: i.content.clone(),
        }).collect(),
        next_item_id: board.next_item_id,
        data_sources: board.data_sources.clone(),
        next_data_source_id: board.next_data_source_id,
    };

    assert_eq!(state.canvas_offset, (150.0, 250.0));
    assert_eq!(state.zoom, 2.0);
}

#[test]
fn test_item_content_variants() {
    let text = ItemContent::Text("Hello".to_string());
    let image = ItemContent::Image("/path/image.png".into());
    let pdf = ItemContent::Pdf { path: "/doc.pdf".into(), thumbnail: None };
    let video = ItemContent::Video("/video.mp4".into());
    let audio = ItemContent::Audio("/audio.mp3".into());
    let youtube = ItemContent::YouTube("123".to_string());
    let markdown = ItemContent::Markdown { path: "/notes.md".into(), title: "Notes".to_string(), content: "# Title".to_string() };
    let code = ItemContent::Code { path: "/main.rs".into(), language: "rust".to_string() };

    let items = vec![text, image, pdf, video, audio, youtube, markdown, code];
    for item in items {
        let json = serde_json::to_string(&item).unwrap();
        let _restored: ItemContent = serde_json::from_str(&json).unwrap();
    }
}

#[test]
fn test_board_metadata_touch() {
    let mut meta = BoardMetadata::new("Test".to_string());
    let original_updated = meta.updated_at;

    std::thread::sleep(std::time::Duration::from_millis(10));
    meta.touch();

    assert!(meta.updated_at >= original_updated);
}

#[test]
fn test_board_state_with_all_content_types() {
    let state = BoardState {
        canvas_offset: (0.0, 0.0),
        zoom: 1.0,
        items: vec![
            CanvasItem { id: 0, position: (0.0, 0.0), size: (200.0, 100.0), content: ItemContent::Text("Text".to_string()) },
            CanvasItem { id: 1, position: (250.0, 0.0), size: (200.0, 200.0), content: ItemContent::Image("/img.png".into()) },
            CanvasItem { id: 2, position: (500.0, 0.0), size: (200.0, 300.0), content: ItemContent::Pdf { path: "/doc.pdf".into(), thumbnail: None } },
            CanvasItem { id: 3, position: (0.0, 350.0), size: (320.0, 180.0), content: ItemContent::Video("/vid.mp4".into()) },
            CanvasItem { id: 4, position: (350.0, 350.0), size: (200.0, 50.0), content: ItemContent::Audio("/audio.mp3".into()) },
        ],
        next_item_id: 5,
        data_sources: HashMap::new(),
        next_data_source_id: 0,
    };

    let json = serde_json::to_string_pretty(&state).unwrap();
    let restored: BoardState = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.items.len(), 5);
}
