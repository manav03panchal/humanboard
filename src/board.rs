use crate::types::{CanvasItem, ItemContent};
use gpui::{Pixels, Point, point, px};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct BoardState {
    pub canvas_offset: (f32, f32),
    pub zoom: f32,
    pub items: Vec<CanvasItem>,
    pub next_item_id: u64,
}

impl BoardState {
    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self) {
            let save_path = Self::save_path();

            if let Some(parent) = save_path.parent() {
                let _ = fs::create_dir_all(parent);
            }

            let _ = fs::write(&save_path, json);
        }
    }

    pub fn load() -> Option<Self> {
        let save_path = Self::save_path();

        if let Ok(json) = fs::read_to_string(&save_path) {
            serde_json::from_str(&json).ok()
        } else {
            None
        }
    }

    fn save_path() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("humanboard")
            .join("board.json")
    }
}

pub struct Board {
    pub canvas_offset: Point<Pixels>,
    pub zoom: f32,
    pub items: Vec<CanvasItem>,
    pub next_item_id: u64,
    history: Vec<BoardState>,
    history_index: usize,
}

impl Board {
    pub fn new() -> Self {
        if let Some(state) = BoardState::load() {
            let initial_state = state.clone();
            Self {
                canvas_offset: point(px(state.canvas_offset.0), px(state.canvas_offset.1)),
                zoom: state.zoom,
                items: state.items,
                next_item_id: state.next_item_id,
                history: vec![initial_state],
                history_index: 0,
            }
        } else {
            let initial_state = BoardState {
                canvas_offset: (0.0, 0.0),
                zoom: 1.0,
                items: Vec::new(),
                next_item_id: 0,
            };
            Self {
                canvas_offset: point(px(0.0), px(0.0)),
                zoom: 1.0,
                items: Vec::new(),
                next_item_id: 0,
                history: vec![initial_state],
                history_index: 0,
            }
        }
    }

    pub fn add_item(&mut self, position: Point<Pixels>, content: ItemContent) {
        let size = content.default_size();

        self.items.push(CanvasItem {
            id: self.next_item_id,
            position: (f32::from(position.x), f32::from(position.y)),
            size,
            content,
        });
        self.next_item_id += 1;
        self.push_history();
        self.save();
    }

    pub fn handle_file_drop(&mut self, position: Point<Pixels>, paths: Vec<PathBuf>) {
        for path in paths {
            let content = ItemContent::from_path(&path);
            // Convert screen position to canvas position accounting for both offset and zoom
            let canvas_pos = point(
                px((f32::from(position.x) - f32::from(self.canvas_offset.x)) / self.zoom),
                px((f32::from(position.y) - f32::from(self.canvas_offset.y)) / self.zoom),
            );
            self.add_item(canvas_pos, content);
        }
    }

    pub fn save(&self) {
        let state = BoardState {
            canvas_offset: (
                f32::from(self.canvas_offset.x),
                f32::from(self.canvas_offset.y),
            ),
            zoom: self.zoom,
            items: self.items.clone(),
            next_item_id: self.next_item_id,
        };
        state.save();
    }

    pub fn push_history(&mut self) {
        // Remove any states after current index (for redo branch pruning)
        self.history.truncate(self.history_index + 1);

        let state = BoardState {
            canvas_offset: (
                f32::from(self.canvas_offset.x),
                f32::from(self.canvas_offset.y),
            ),
            zoom: self.zoom,
            items: self.items.clone(),
            next_item_id: self.next_item_id,
        };

        self.history.push(state);
        self.history_index = self.history.len() - 1;

        // Limit history to 50 states
        if self.history.len() > 50 {
            self.history.remove(0);
            self.history_index = self.history_index.saturating_sub(1);
        }
    }

    pub fn undo(&mut self) -> bool {
        if self.history_index > 0 {
            self.history_index -= 1;
            self.restore_from_history();
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        if self.history_index < self.history.len() - 1 {
            self.history_index += 1;
            self.restore_from_history();
            true
        } else {
            false
        }
    }

    fn restore_from_history(&mut self) {
        if let Some(state) = self.history.get(self.history_index) {
            self.canvas_offset = point(px(state.canvas_offset.0), px(state.canvas_offset.1));
            self.zoom = state.zoom;
            self.items = state.items.clone();
            self.next_item_id = state.next_item_id;
            self.save();
        }
    }

    /// Create a fresh board for testing (doesn't load from disk)
    #[cfg(test)]
    pub fn new_empty() -> Self {
        let initial_state = BoardState {
            canvas_offset: (0.0, 0.0),
            zoom: 1.0,
            items: Vec::new(),
            next_item_id: 0,
        };
        Self {
            canvas_offset: point(px(0.0), px(0.0)),
            zoom: 1.0,
            items: Vec::new(),
            next_item_id: 0,
            history: vec![initial_state],
            history_index: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_new_empty() {
        let board = Board::new_empty();
        assert_eq!(board.zoom, 1.0);
        assert!(board.items.is_empty());
        assert_eq!(board.next_item_id, 0);
    }

    #[test]
    fn test_add_item() {
        let mut board = Board::new_empty();
        let pos = point(px(100.0), px(200.0));
        board.add_item(pos, ItemContent::Text("Test".to_string()));

        assert_eq!(board.items.len(), 1);
        assert_eq!(board.items[0].id, 0);
        assert_eq!(board.next_item_id, 1);
    }

    #[test]
    fn test_add_multiple_items() {
        let mut board = Board::new_empty();

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
    fn test_undo_empty() {
        let mut board = Board::new_empty();
        assert!(!board.undo()); // Can't undo with no history
    }

    #[test]
    fn test_redo_empty() {
        let mut board = Board::new_empty();
        assert!(!board.redo()); // Can't redo with no history
    }

    #[test]
    fn test_undo_after_add() {
        let mut board = Board::new_empty();
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
        let mut board = Board::new_empty();
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
        let mut board = Board::new_empty();

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
        let mut board = Board::new_empty();

        board.add_item(
            point(px(0.0), px(0.0)),
            ItemContent::Text("First".to_string()),
        );
        board.add_item(
            point(px(100.0), px(100.0)),
            ItemContent::Text("Second".to_string()),
        );

        board.undo(); // Go back to 1 item

        // Add a new item - this should clear the redo history
        board.add_item(
            point(px(200.0), px(200.0)),
            ItemContent::Text("Third".to_string()),
        );

        // Redo should fail since we added a new item
        assert!(!board.redo());
    }

    #[test]
    fn test_board_state_serialization() {
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
        let mut board = Board::new_empty();

        // Add 60 items (exceeds 50 limit)
        for i in 0..60 {
            board.add_item(
                point(px(i as f32 * 10.0), px(0.0)),
                ItemContent::Text(format!("Item {}", i)),
            );
        }

        // History should be capped at 50
        assert!(board.history.len() <= 51); // 50 + initial state
    }
}
