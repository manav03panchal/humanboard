use crate::board_index::BoardIndex;
use crate::types::{CanvasItem, ItemContent};
use gpui::{Pixels, Point, Size, point, px};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// Save debounce delay - saves are batched within this window
const SAVE_DEBOUNCE_MS: u64 = 500;

/// Maximum history states to keep
const MAX_HISTORY_STATES: usize = 50;

#[derive(Serialize, Deserialize, Clone)]
pub struct BoardState {
    pub canvas_offset: (f32, f32),
    pub zoom: f32,
    pub items: Vec<CanvasItem>,
    pub next_item_id: u64,
}

impl BoardState {
    pub fn save_to_path(&self, path: &PathBuf) {
        if let Ok(json) = serde_json::to_string_pretty(&self) {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = fs::write(path, json);
        }
    }

    pub fn load_from_path(path: &PathBuf) -> Option<Self> {
        if let Ok(json) = fs::read_to_string(path) {
            serde_json::from_str(&json).ok()
        } else {
            None
        }
    }
}

pub struct Board {
    pub id: String,
    pub canvas_offset: Point<Pixels>,
    pub zoom: f32,

    // Items stored in Vec for ordered rendering, with HashMap index for O(1) lookups
    pub items: Vec<CanvasItem>,
    items_index: HashMap<u64, usize>, // id -> index in items vec

    pub next_item_id: u64,

    // History using VecDeque for O(1) front removal
    history: VecDeque<BoardState>,
    history_index: usize,

    // Debounced save tracking
    dirty: bool,
    last_change: Instant,
}

impl Board {
    /// Load a board by ID, or create a new empty one
    pub fn load(id: String) -> Self {
        let board_path = BoardIndex::board_path(&id);

        if let Some(state) = BoardState::load_from_path(&board_path) {
            let items_index = Self::build_items_index(&state.items);
            let initial_state = state.clone();
            Self {
                id,
                canvas_offset: point(px(state.canvas_offset.0), px(state.canvas_offset.1)),
                zoom: state.zoom,
                items: state.items,
                items_index,
                next_item_id: state.next_item_id,
                history: VecDeque::from([initial_state]),
                history_index: 0,
                dirty: false,
                last_change: Instant::now(),
            }
        } else {
            Self::new_empty(id)
        }
    }

    /// Create a new empty board with the given ID
    pub fn new_empty(id: String) -> Self {
        let initial_state = BoardState {
            canvas_offset: (0.0, 0.0),
            zoom: 1.0,
            items: Vec::new(),
            next_item_id: 0,
        };
        Self {
            id,
            canvas_offset: point(px(0.0), px(0.0)),
            zoom: 1.0,
            items: Vec::new(),
            items_index: HashMap::new(),
            next_item_id: 0,
            history: VecDeque::from([initial_state]),
            history_index: 0,
            dirty: false,
            last_change: Instant::now(),
        }
    }

    /// Build the items index from a Vec of items
    fn build_items_index(items: &[CanvasItem]) -> HashMap<u64, usize> {
        items
            .iter()
            .enumerate()
            .map(|(idx, item)| (item.id, idx))
            .collect()
    }

    /// Rebuild the index after items vec changes
    fn rebuild_index(&mut self) {
        self.items_index = Self::build_items_index(&self.items);
    }

    /// Get item by ID in O(1)
    pub fn get_item(&self, id: u64) -> Option<&CanvasItem> {
        self.items_index
            .get(&id)
            .and_then(|&idx| self.items.get(idx))
    }

    /// Get mutable item by ID in O(1)
    pub fn get_item_mut(&mut self, id: u64) -> Option<&mut CanvasItem> {
        self.items_index
            .get(&id)
            .and_then(|&idx| self.items.get_mut(idx))
    }

    /// Add a single item (still triggers history + save for single operations)
    pub fn add_item(&mut self, position: Point<Pixels>, content: ItemContent) -> u64 {
        let id = self.add_item_internal(position, content);
        self.push_history();
        self.mark_dirty();
        id
    }

    /// Internal add without history/save - used for batch operations
    fn add_item_internal(&mut self, position: Point<Pixels>, content: ItemContent) -> u64 {
        let size = content.default_size();
        let id = self.next_item_id;

        self.items.push(CanvasItem {
            id,
            position: (f32::from(position.x), f32::from(position.y)),
            size,
            content,
        });
        self.items_index.insert(id, self.items.len() - 1);
        self.next_item_id += 1;
        id
    }

    /// Handle file drop - batched operation (single history push + save)
    pub fn handle_file_drop(&mut self, position: Point<Pixels>, paths: Vec<PathBuf>) {
        if paths.is_empty() {
            return;
        }

        // Stagger offset for multiple files so they don't overlap
        const STAGGER_X: f32 = 30.0;
        const STAGGER_Y: f32 = 30.0;

        for (i, path) in paths.iter().enumerate() {
            let content = ItemContent::from_path(path);
            let base_pos = self.screen_to_canvas(position);
            let staggered_pos = point(
                px(f32::from(base_pos.x) + (i as f32 * STAGGER_X)),
                px(f32::from(base_pos.y) + (i as f32 * STAGGER_Y)),
            );
            self.add_item_internal(staggered_pos, content);
        }

        // Single history push and save for the entire batch
        self.push_history();
        self.mark_dirty();
    }

    /// Add URL (YouTube or generic link)
    pub fn add_url(&mut self, url: &str, position: Point<Pixels>) {
        use crate::types::extract_youtube_id;

        let content = if let Some(video_id) = extract_youtube_id(url) {
            ItemContent::YouTube(video_id)
        } else {
            ItemContent::Link(url.to_string())
        };

        let canvas_pos = self.screen_to_canvas(position);
        self.add_item(canvas_pos, content);
    }

    /// Remove an item by ID
    pub fn remove_item(&mut self, id: u64) -> bool {
        if let Some(&idx) = self.items_index.get(&id) {
            self.items.remove(idx);
            self.rebuild_index();
            self.push_history();
            self.mark_dirty();
            true
        } else {
            false
        }
    }

    /// Convert screen position to canvas position
    #[inline]
    pub fn screen_to_canvas(&self, screen_pos: Point<Pixels>) -> Point<Pixels> {
        // Account for dock width (44px) and header (40px)
        const DOCK_WIDTH: f32 = 44.0;
        const HEADER_HEIGHT: f32 = 40.0;
        point(
            px(
                (f32::from(screen_pos.x) - DOCK_WIDTH - f32::from(self.canvas_offset.x))
                    / self.zoom,
            ),
            px(
                (f32::from(screen_pos.y) - HEADER_HEIGHT - f32::from(self.canvas_offset.y))
                    / self.zoom,
            ),
        )
    }

    /// Convert canvas position to screen position
    #[inline]
    pub fn canvas_to_screen(&self, canvas_pos: Point<Pixels>) -> Point<Pixels> {
        point(
            px(f32::from(canvas_pos.x) * self.zoom + f32::from(self.canvas_offset.x)),
            px(f32::from(canvas_pos.y) * self.zoom + f32::from(self.canvas_offset.y)),
        )
    }

    /// Zoom by a factor, keeping the given screen position fixed
    /// Returns true if zoom changed
    pub fn zoom_around(&mut self, factor: f32, center: Point<Pixels>) -> bool {
        let old_zoom = self.zoom;
        self.zoom = (self.zoom * factor).clamp(0.1, 10.0);

        if (self.zoom - old_zoom).abs() < 0.0001 {
            return false;
        }

        let zoom_factor = self.zoom / old_zoom;
        let mouse_canvas_x = center.x - self.canvas_offset.x;
        let mouse_canvas_y = center.y - self.canvas_offset.y;

        self.canvas_offset.x = center.x - mouse_canvas_x * zoom_factor;
        self.canvas_offset.y = center.y - mouse_canvas_y * zoom_factor;

        self.mark_dirty();
        true
    }

    /// Zoom in by standard step (1.2x)
    pub fn zoom_in(&mut self, center: Point<Pixels>) -> bool {
        self.zoom_around(1.2, center)
    }

    /// Zoom out by standard step (1/1.2x)
    pub fn zoom_out(&mut self, center: Point<Pixels>) -> bool {
        self.zoom_around(1.0 / 1.2, center)
    }

    /// Reset zoom to 1.0
    pub fn zoom_reset(&mut self) {
        self.zoom = 1.0;
        self.mark_dirty();
    }

    /// Center the viewport on an item by its ID
    /// screen_size is the visible canvas area size
    pub fn center_on_item(&mut self, item_id: u64, screen_size: Size<Pixels>) {
        if let Some(item) = self.items.iter().find(|i| i.id == item_id) {
            // Calculate the center of the item in canvas coordinates
            let item_center_x = item.position.0 + item.size.0 / 2.0;
            let item_center_y = item.position.1 + item.size.1 / 2.0;

            // Calculate offset to center item on screen
            let screen_center_x = f32::from(screen_size.width) / 2.0;
            let screen_center_y = f32::from(screen_size.height) / 2.0;

            self.canvas_offset = point(
                px(screen_center_x - item_center_x * self.zoom),
                px(screen_center_y - item_center_y * self.zoom),
            );

            self.mark_dirty();
        }
    }

    /// Find items matching a search query (searches display names)
    pub fn find_items(&self, query: &str) -> Vec<(u64, String)> {
        let query_lower = query.to_lowercase();
        self.items
            .iter()
            .filter(|item| {
                item.content.is_searchable()
                    && item
                        .content
                        .display_name()
                        .to_lowercase()
                        .contains(&query_lower)
            })
            .map(|item| (item.id, item.content.display_name()))
            .collect()
    }

    /// Mark the board as dirty (needing save)
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
        self.last_change = Instant::now();
    }

    /// Check if the board has unsaved changes
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Check if enough time has passed to save (debouncing)
    pub fn should_save(&self) -> bool {
        self.dirty && self.last_change.elapsed() >= Duration::from_millis(SAVE_DEBOUNCE_MS)
    }

    /// Flush save if dirty (call this periodically or on app idle)
    pub fn flush_save(&mut self) {
        if self.dirty {
            self.save_immediate();
            self.dirty = false;
        }
    }

    /// Force immediate save (used when leaving board)
    pub fn save_immediate(&self) {
        let state = BoardState {
            canvas_offset: (
                f32::from(self.canvas_offset.x),
                f32::from(self.canvas_offset.y),
            ),
            zoom: self.zoom,
            items: self.items.clone(),
            next_item_id: self.next_item_id,
        };
        let board_path = BoardIndex::board_path(&self.id);
        state.save_to_path(&board_path);
    }

    /// Legacy save method - now marks dirty for debounced save
    pub fn save(&mut self) {
        self.mark_dirty();
    }

    pub fn push_history(&mut self) {
        // Remove any states after current index (for redo branch pruning)
        while self.history.len() > self.history_index + 1 {
            self.history.pop_back();
        }

        let state = BoardState {
            canvas_offset: (
                f32::from(self.canvas_offset.x),
                f32::from(self.canvas_offset.y),
            ),
            zoom: self.zoom,
            items: self.items.clone(),
            next_item_id: self.next_item_id,
        };

        self.history.push_back(state);
        self.history_index = self.history.len() - 1;

        // Limit history - O(1) removal from front with VecDeque
        while self.history.len() > MAX_HISTORY_STATES {
            self.history.pop_front();
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
        // Clone the state to avoid borrow issues
        let state = self.history.get(self.history_index).cloned();
        if let Some(state) = state {
            self.canvas_offset = point(px(state.canvas_offset.0), px(state.canvas_offset.1));
            self.zoom = state.zoom;
            self.items = state.items.clone();
            self.next_item_id = state.next_item_id;
            self.rebuild_index();
            self.mark_dirty();
        }
    }

    /// Create a fresh board for testing (doesn't load from disk)
    #[cfg(test)]
    pub fn new_for_test() -> Self {
        Self::new_empty("test-board".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let mut board = Board::new_for_test();

        for i in 0..60 {
            board.add_item(
                point(px(i as f32 * 10.0), px(0.0)),
                ItemContent::Text(format!("Item {}", i)),
            );
        }

        assert!(board.history.len() <= MAX_HISTORY_STATES + 1);
    }

    #[test]
    fn test_screen_to_canvas_conversion() {
        let mut board = Board::new_for_test();
        board.canvas_offset = point(px(100.0), px(50.0));
        board.zoom = 2.0;

        let screen_pos = point(px(300.0), px(150.0));
        let canvas_pos = board.screen_to_canvas(screen_pos);

        // (300 - 100) / 2 = 100, (150 - 50) / 2 = 50
        assert_eq!(f32::from(canvas_pos.x), 100.0);
        assert_eq!(f32::from(canvas_pos.y), 50.0);
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
}
