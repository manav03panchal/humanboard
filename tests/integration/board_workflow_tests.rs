//! Board Workflow Integration Tests

use humanboard::board::{Board, BoardState};
use humanboard::board_index::{BoardIndex, BoardMetadata};
use humanboard::types::{CanvasItem, ItemContent};
use gpui::{point, px};
use std::collections::HashMap;

#[test]
fn test_new_board_workflow() {
    let board = Board::new_for_test();
    assert!(board.items.is_empty());
    assert_eq!(board.zoom, 1.0);
    assert_eq!(board.next_item_id, 0);
}

#[test]
fn test_board_with_items_workflow() {
    let mut board = Board::new_for_test();

    board.add_item(point(px(100.0), px(100.0)), ItemContent::Text("Note 1".to_string()));
    board.add_item(point(px(300.0), px(100.0)), ItemContent::Text("Note 2".to_string()));
    board.add_item(point(px(100.0), px(300.0)), ItemContent::Markdown {
        path: "/test/readme.md".into(),
        title: "README".to_string(),
        content: "# Test".to_string(),
    });

    assert_eq!(board.items.len(), 3);
    assert_eq!(board.next_item_id, 3);
}

#[test]
fn test_board_state_round_trip() {
    let mut board = Board::new_for_test();
    board.canvas_offset = point(px(50.0), px(75.0));
    board.zoom = 1.5;

    board.add_item(point(px(100.0), px(200.0)), ItemContent::Text("Test Item".to_string()));
    board.add_item(point(px(400.0), px(300.0)), ItemContent::Code {
        path: "/test/main.rs".into(),
        language: "rust".to_string(),
    });

    let state = BoardState {
        canvas_offset: (f32::from(board.canvas_offset.x), f32::from(board.canvas_offset.y)),
        zoom: board.zoom,
        items: board.items.iter().map(|item| CanvasItem {
            id: item.id,
            position: item.position,
            size: item.size,
            content: item.content.clone(),
        }).collect(),
        next_item_id: board.next_item_id,
        data_sources: board.data_sources.clone(),
        next_data_source_id: board.next_data_source_id,
    };

    let json = serde_json::to_string_pretty(&state).unwrap();
    let restored: BoardState = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.canvas_offset, (50.0, 75.0));
    assert_eq!(restored.zoom, 1.5);
    assert_eq!(restored.items.len(), 2);
}

#[test]
fn test_board_index_workflow() {
    let mut index = BoardIndex::default();
    assert!(index.boards.is_empty());

    let meta1 = BoardMetadata::new("Project Alpha".to_string());
    let meta2 = BoardMetadata::new("Project Beta".to_string());
    let id1 = meta1.id.clone();
    let id2 = meta2.id.clone();

    index.boards.push(meta1);
    index.boards.push(meta2);

    assert_eq!(index.boards.len(), 2);
    assert_ne!(id1, id2);
}

#[test]
fn test_item_modification_workflow() {
    let mut board = Board::new_for_test();

    board.add_item(point(px(100.0), px(100.0)), ItemContent::Text("Original".to_string()));
    board.push_history(); // Save baseline

    board.items[0].content = ItemContent::Text("Modified".to_string());
    board.items[0].position = (200.0, 200.0);
    board.push_history();

    if let ItemContent::Text(text) = &board.items[0].content {
        assert_eq!(text, "Modified");
    }

    board.undo();
    if let ItemContent::Text(text) = &board.items[0].content {
        assert_eq!(text, "Original");
    }
}

#[test]
fn test_zoom_workflow() {
    let mut board = Board::new_for_test();
    let center = point(px(500.0), px(500.0));

    assert_eq!(board.zoom, 1.0);
    board.zoom_in(center);
    assert!(board.zoom > 1.0);

    board.zoom_reset();
    assert_eq!(board.zoom, 1.0);
}

#[test]
fn test_search_workflow() {
    let mut board = Board::new_for_test();

    board.add_item(point(px(0.0), px(0.0)), ItemContent::Text("Meeting Notes".to_string()));
    board.add_item(point(px(100.0), px(0.0)), ItemContent::Text("Project Plan".to_string()));
    board.add_item(point(px(200.0), px(0.0)), ItemContent::Markdown {
        path: "/notes.md".into(),
        title: "Meeting Summary".to_string(),
        content: "# Meeting Summary\nDiscussed project timeline.".to_string(),
    });

    let results = board.find_items("meeting");
    assert_eq!(results.len(), 2);

    let results = board.find_items("nonexistent");
    assert_eq!(results.len(), 0);
}

#[test]
fn test_complete_board_lifecycle() {
    let mut board = Board::new_for_test();

    board.add_item(point(px(100.0), px(100.0)), ItemContent::Text("Task 1".to_string()));
    board.add_item(point(px(300.0), px(100.0)), ItemContent::Text("Task 2".to_string()));

    board.canvas_offset = point(px(50.0), px(50.0));
    board.zoom = 1.5;

    board.items[0].content = ItemContent::Text("Task 1 - Completed".to_string());
    board.push_history();

    let state = BoardState {
        canvas_offset: (f32::from(board.canvas_offset.x), f32::from(board.canvas_offset.y)),
        zoom: board.zoom,
        items: board.items.iter().map(|item| CanvasItem {
            id: item.id, position: item.position, size: item.size, content: item.content.clone(),
        }).collect(),
        next_item_id: board.next_item_id,
        data_sources: board.data_sources.clone(),
        next_data_source_id: board.next_data_source_id,
    };

    let json = serde_json::to_string(&state).unwrap();
    let restored: BoardState = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.items.len(), 2);
}

#[test]
fn test_board_with_mixed_content_types() {
    let mut board = Board::new_for_test();

    board.add_item(point(px(0.0), px(0.0)), ItemContent::Text("Plain text".to_string()));
    board.add_item(point(px(200.0), px(0.0)), ItemContent::Image("/path/to/image.png".into()));
    board.add_item(point(px(400.0), px(0.0)), ItemContent::Pdf { path: "/path/to/doc.pdf".into(), thumbnail: None });
    board.add_item(point(px(0.0), px(200.0)), ItemContent::Video("/path/to/video.mp4".into()));
    board.add_item(point(px(200.0), px(200.0)), ItemContent::Audio("/path/to/audio.mp3".into()));
    board.add_item(point(px(400.0), px(200.0)), ItemContent::YouTube("abc123".to_string()));
    board.add_item(point(px(0.0), px(400.0)), ItemContent::Markdown { path: "/notes.md".into(), title: "Notes".to_string(), content: "# Notes".to_string() });
    board.add_item(point(px(200.0), px(400.0)), ItemContent::Code { path: "/main.rs".into(), language: "rust".to_string() });

    assert_eq!(board.items.len(), 8);

    let state = BoardState {
        canvas_offset: (0.0, 0.0),
        zoom: 1.0,
        items: board.items.iter().map(|item| CanvasItem {
            id: item.id, position: item.position, size: item.size, content: item.content.clone(),
        }).collect(),
        next_item_id: board.next_item_id,
        data_sources: board.data_sources.clone(),
        next_data_source_id: board.next_data_source_id,
    };

    let json = serde_json::to_string(&state).unwrap();
    let restored: BoardState = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.items.len(), 8);
}
