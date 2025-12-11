//! Board module - central state management for the canvas.
//!
//! This module coordinates between the spatial index, history manager,
//! and persistence system for optimal performance.

use gpui::{Pixels, Point, point, px};

use crate::history::{HistoryManager, HistorySnapshot};
use crate::persistence::{BoardState, PersistenceHandle};
use crate::spatial::SpatialIndex;
use crate::types::{CanvasItem, ItemContent};

/// The main board state container
pub struct Board {
    /// Canvas offset in screen pixels
    pub canvas_offset: Point<Pixels>,
    /// Current zoom level
    pub zoom: f32,
    /// All items on the canvas
    pub items: Vec<CanvasItem>,
    /// Next ID to assign to a new item
    pub next_item_id: u64,
    /// Spatial index for fast item lookup
    spatial_index: SpatialIndex,
    /// History manager for undo/redo
    history: HistoryManager,
    /// Debounced persistence handle
    persistence: PersistenceHandle,
    /// Flag to track if we have unsaved changes
    dirty: bool,
}

impl Board {
    /// Create a new board, loading from disk if available
    pub fn new() -> Self {
        if let Some(state) = PersistenceHandle::load() {
            let snapshot = HistorySnapshot::from_board_state(&state);
            let mut board = Self {
                canvas_offset: point(px(state.canvas_offset.0), px(state.canvas_offset.1)),
                zoom: state.zoom,
                items: state.items,
                next_item_id: state.next_item_id,
                spatial_index: SpatialIndex::new(),
                history: HistoryManager::new(snapshot),
                persistence: PersistenceHandle::new(),
                dirty: false,
            };
            board.rebuild_spatial_index();
            board
        } else {
            let snapshot = HistorySnapshot::new((0.0, 0.0), 1.0, Vec::new(), 0);
            Self {
                canvas_offset: point(px(0.0), px(0.0)),
                zoom: 1.0,
                items: Vec::new(),
                next_item_id: 0,
                spatial_index: SpatialIndex::new(),
                history: HistoryManager::new(snapshot),
                persistence: PersistenceHandle::new(),
                dirty: false,
            }
        }
    }

    /// Create a fresh board for testing (doesn't load from disk)
    #[cfg(test)]
    pub fn new_empty() -> Self {
        let snapshot = HistorySnapshot::new((0.0, 0.0), 1.0, Vec::new(), 0);
        Self {
            canvas_offset: point(px(0.0), px(0.0)),
            zoom: 1.0,
            items: Vec::new(),
            next_item_id: 0,
            spatial_index: SpatialIndex::new(),
            history: HistoryManager::new(snapshot),
            persistence: PersistenceHandle::new(),
            dirty: false,
        }
    }

    /// Rebuild the spatial index from current items
    fn rebuild_spatial_index(&mut self) {
        self.spatial_index
            .rebuild(self.items.iter().enumerate().map(|(idx, item)| {
                (
                    item.id,
                    item.position.0,
                    item.position.1,
                    item.size.0,
                    item.size.1,
                    idx,
                )
            }));
    }

    /// Add a new item to the board
    pub fn add_item(&mut self, position: Point<Pixels>, content: ItemContent) {
        // Record history before modification
        self.push_history();

        let size = content.default_size();
        let id = self.next_item_id;

        let item = CanvasItem {
            id,
            position: (f32::from(position.x), f32::from(position.y)),
            size,
            content,
        };

        // Add to spatial index
        self.spatial_index.insert(
            id,
            item.position.0,
            item.position.1,
            item.size.0,
            item.size.1,
            self.items.len(),
        );

        self.items.push(item);
        self.next_item_id += 1;

        // Sync history with new state
        self.sync_history_current();
        self.mark_dirty();
    }

    /// Handle file drop onto the canvas
    pub fn handle_file_drop(&mut self, position: Point<Pixels>, paths: Vec<std::path::PathBuf>) {
        for path in paths {
            let content = ItemContent::from_path(&path);
            // Convert screen position to canvas position accounting for offset and zoom
            let canvas_pos = point(
                px((f32::from(position.x) - f32::from(self.canvas_offset.x)) / self.zoom),
                px((f32::from(position.y) - f32::from(self.canvas_offset.y)) / self.zoom),
            );
            self.add_item(canvas_pos, content);
        }
    }

    /// Find the topmost item at a screen position
    /// Returns (item_id, vec_index) for the topmost item, if any
    pub fn item_at_screen_pos(&self, screen_x: f32, screen_y: f32) -> Option<u64> {
        // Convert screen position to canvas position
        let canvas_x = (screen_x - f32::from(self.canvas_offset.x)) / self.zoom;
        let canvas_y = (screen_y - f32::from(self.canvas_offset.y)) / self.zoom;

        // Query spatial index (returns items sorted by z-order, top first)
        let candidates = self.spatial_index.query_point(canvas_x, canvas_y);
        candidates.into_iter().next()
    }

    /// Find item at screen position with full bounds check
    /// This is more accurate than item_at_screen_pos for edge cases
    pub fn item_at_screen_pos_precise(&self, screen_x: f32, screen_y: f32) -> Option<&CanvasItem> {
        // First try fast path through spatial index
        if let Some(id) = self.item_at_screen_pos(screen_x, screen_y) {
            return self.items.iter().find(|item| item.id == id);
        }

        // Fallback: linear search (shouldn't normally happen if spatial index is synced)
        self.items.iter().rev().find(|item| {
            let scaled_x = item.position.0 * self.zoom + f32::from(self.canvas_offset.x);
            let scaled_y = item.position.1 * self.zoom + f32::from(self.canvas_offset.y);
            let scaled_width = item.size.0 * self.zoom;
            let scaled_height = item.size.1 * self.zoom;

            screen_x >= scaled_x
                && screen_x <= scaled_x + scaled_width
                && screen_y >= scaled_y
                && screen_y <= scaled_y + scaled_height
        })
    }

    /// Get item by ID
    pub fn get_item(&self, id: u64) -> Option<&CanvasItem> {
        self.items.iter().find(|item| item.id == id)
    }

    /// Get mutable item by ID
    pub fn get_item_mut(&mut self, id: u64) -> Option<&mut CanvasItem> {
        self.items.iter_mut().find(|item| item.id == id)
    }

    /// Update an item's position (for dragging)
    pub fn update_item_position(&mut self, id: u64, new_x: f32, new_y: f32) {
        // Find index first to avoid borrow issues
        let item_data = self.items.iter().enumerate().find_map(|(idx, item)| {
            if item.id == id {
                Some((idx, item.size))
            } else {
                None
            }
        });

        if let Some((idx, size)) = item_data {
            // Update the item
            self.items[idx].position = (new_x, new_y);

            // Update spatial index
            self.spatial_index
                .update(id, new_x, new_y, size.0, size.1, idx);

            self.mark_dirty();
        }
    }

    /// Update an item's size (for resizing)
    pub fn update_item_size(&mut self, id: u64, new_width: f32, new_height: f32) {
        // Find index first to avoid borrow issues
        let item_data = self.items.iter().enumerate().find_map(|(idx, item)| {
            if item.id == id {
                Some((idx, item.position))
            } else {
                None
            }
        });

        if let Some((idx, position)) = item_data {
            // Update the item
            self.items[idx].size = (new_width, new_height);

            // Update spatial index
            self.spatial_index
                .update(id, position.0, position.1, new_width, new_height, idx);

            self.mark_dirty();
        }
    }

    /// Delete an item by ID
    pub fn delete_item(&mut self, id: u64) {
        // Record history before deletion
        self.push_history();

        // Remove from spatial index
        self.spatial_index.remove(id);

        // Remove from items
        self.items.retain(|item| item.id != id);

        // Rebuild spatial index to fix vec_indices
        self.rebuild_spatial_index();

        // Sync history with new state
        self.sync_history_current();
        self.mark_dirty();
    }

    /// Update canvas offset (for panning)
    pub fn update_offset(&mut self, new_offset: Point<Pixels>) {
        self.canvas_offset = new_offset;
        self.mark_dirty();
    }

    /// Update zoom level
    pub fn update_zoom(&mut self, new_zoom: f32) {
        self.zoom = new_zoom.clamp(0.1, 10.0);
        self.mark_dirty();
    }

    /// Record current state to history (call before modifications)
    /// This saves the current state to undo stack before any changes are made.
    pub fn push_history(&mut self) {
        // First sync history.current with board state (in case they diverged)
        self.sync_history_current();
        // Then record to undo stack
        self.history.record();
    }

    /// Sync the history manager's current state with the board's state
    /// Call this after modifications to keep history in sync.
    fn sync_history_current(&mut self) {
        let current = self.history.current_mut();
        current.canvas_offset = (
            f32::from(self.canvas_offset.x),
            f32::from(self.canvas_offset.y),
        );
        current.zoom = self.zoom;
        *current.items_mut() = self.items.clone();
        current.next_item_id = self.next_item_id;
    }

    /// Undo the last action
    pub fn undo(&mut self) -> bool {
        if self.history.undo() {
            self.restore_from_history();
            true
        } else {
            false
        }
    }

    /// Redo the last undone action
    pub fn redo(&mut self) -> bool {
        if self.history.redo() {
            self.restore_from_history();
            true
        } else {
            false
        }
    }

    /// Restore state from the current history snapshot
    fn restore_from_history(&mut self) {
        let current = self.history.current();
        self.canvas_offset = point(px(current.canvas_offset.0), px(current.canvas_offset.1));
        self.zoom = current.zoom;
        self.items = current.items().to_vec();
        self.next_item_id = current.next_item_id;
        self.rebuild_spatial_index();
        self.mark_dirty();
    }

    /// Mark the board as having unsaved changes
    fn mark_dirty(&mut self) {
        self.dirty = true;
        self.queue_save();
    }

    /// Queue a debounced save operation
    fn queue_save(&mut self) {
        let state = BoardState {
            canvas_offset: (
                f32::from(self.canvas_offset.x),
                f32::from(self.canvas_offset.y),
            ),
            zoom: self.zoom,
            items: self.items.clone(),
            next_item_id: self.next_item_id,
        };
        self.persistence.save(&state);
    }

    /// Force an immediate save (use sparingly)
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
        PersistenceHandle::save_immediate(&state);
    }

    /// Legacy save method for compatibility (now uses debounced saving)
    pub fn save(&mut self) {
        self.mark_dirty();
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        self.history.can_undo()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        self.history.can_redo()
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
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
    fn test_spatial_index_query() {
        let mut board = Board::new_empty();

        // Add item at (100, 100) with default text size (300x100)
        board.add_item(
            point(px(100.0), px(100.0)),
            ItemContent::Text("Test".to_string()),
        );

        // Query inside the item bounds
        let result = board.item_at_screen_pos(200.0, 150.0);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 0);

        // Query outside the item bounds
        let result = board.item_at_screen_pos(0.0, 0.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_delete_item() {
        let mut board = Board::new_empty();

        board.add_item(
            point(px(0.0), px(0.0)),
            ItemContent::Text("Test".to_string()),
        );
        assert_eq!(board.items.len(), 1);

        board.delete_item(0);
        assert_eq!(board.items.len(), 0);

        // Should be able to undo deletion
        assert!(board.undo());
        assert_eq!(board.items.len(), 1);
    }

    #[test]
    fn test_update_item_position() {
        let mut board = Board::new_empty();

        board.add_item(
            point(px(0.0), px(0.0)),
            ItemContent::Text("Test".to_string()),
        );

        board.update_item_position(0, 500.0, 500.0);

        assert_eq!(board.items[0].position, (500.0, 500.0));

        // Spatial index should reflect the new position
        let result = board.item_at_screen_pos(550.0, 550.0);
        assert!(result.is_some());
    }
}
