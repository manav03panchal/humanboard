//! History module with optimized undo/redo using VecDeque.
//!
//! This module provides efficient history management with O(1) operations
//! for both push and pop from either end, using VecDeque instead of Vec.

use std::collections::VecDeque;
use std::sync::Arc;

use crate::persistence::BoardState;
use crate::types::CanvasItem;

/// Maximum number of history states to keep
const MAX_HISTORY_SIZE: usize = 50;

/// A snapshot of the board state for undo/redo.
/// Uses Arc for efficient cloning of the items vector.
#[derive(Clone, Debug)]
pub struct HistorySnapshot {
    pub canvas_offset: (f32, f32),
    pub zoom: f32,
    /// Shared reference to items - only cloned when modified (copy-on-write)
    pub items: Arc<Vec<CanvasItem>>,
    pub next_item_id: u64,
}

impl HistorySnapshot {
    /// Create a new snapshot from current state
    pub fn new(
        canvas_offset: (f32, f32),
        zoom: f32,
        items: Vec<CanvasItem>,
        next_item_id: u64,
    ) -> Self {
        Self {
            canvas_offset,
            zoom,
            items: Arc::new(items),
            next_item_id,
        }
    }

    /// Create from board state (for loading from disk)
    pub fn from_board_state(state: &BoardState) -> Self {
        Self {
            canvas_offset: state.canvas_offset,
            zoom: state.zoom,
            items: Arc::new(state.items.clone()),
            next_item_id: state.next_item_id,
        }
    }

    /// Convert to board state (for saving to disk)
    pub fn to_board_state(&self) -> BoardState {
        BoardState {
            canvas_offset: self.canvas_offset,
            zoom: self.zoom,
            items: (*self.items).clone(),
            next_item_id: self.next_item_id,
        }
    }

    /// Get items, cloning only if we need to modify
    pub fn items_mut(&mut self) -> &mut Vec<CanvasItem> {
        Arc::make_mut(&mut self.items)
    }

    /// Get read-only reference to items (zero-cost)
    pub fn items(&self) -> &[CanvasItem] {
        &self.items
    }
}

/// History manager with efficient O(1) push/pop using VecDeque
#[derive(Debug)]
pub struct HistoryManager {
    /// Past states (for undo)
    undo_stack: VecDeque<HistorySnapshot>,
    /// Future states (for redo)
    redo_stack: VecDeque<HistorySnapshot>,
    /// Current state
    current: HistorySnapshot,
}

impl HistoryManager {
    /// Create a new history manager with an initial state
    pub fn new(initial: HistorySnapshot) -> Self {
        Self {
            undo_stack: VecDeque::with_capacity(MAX_HISTORY_SIZE),
            redo_stack: VecDeque::new(),
            current: initial,
        }
    }

    /// Create with default empty state
    pub fn new_empty() -> Self {
        Self::new(HistorySnapshot::new((0.0, 0.0), 1.0, Vec::new(), 0))
    }

    /// Get the current state
    pub fn current(&self) -> &HistorySnapshot {
        &self.current
    }

    /// Get mutable reference to current state
    pub fn current_mut(&mut self) -> &mut HistorySnapshot {
        &mut self.current
    }

    /// Push current state to history and set new current state.
    /// This is called BEFORE making a change, to save the current state.
    pub fn push(&mut self, new_state: HistorySnapshot) {
        // Push current to undo stack
        self.undo_stack.push_back(self.current.clone());

        // Clear redo stack (new branch in history)
        self.redo_stack.clear();

        // Set new current state
        self.current = new_state;

        // Enforce history limit (O(1) with VecDeque)
        while self.undo_stack.len() > MAX_HISTORY_SIZE {
            self.undo_stack.pop_front();
        }
    }

    /// Record current state before a modification.
    /// Call this before making changes to save the pre-change state.
    pub fn record(&mut self) {
        // Clone current state to undo stack
        self.undo_stack.push_back(self.current.clone());

        // Clear redo stack
        self.redo_stack.clear();

        // Enforce limit
        while self.undo_stack.len() > MAX_HISTORY_SIZE {
            self.undo_stack.pop_front();
        }
    }

    /// Undo the last action. Returns true if undo was performed.
    pub fn undo(&mut self) -> bool {
        if let Some(prev_state) = self.undo_stack.pop_back() {
            // Push current to redo stack
            self.redo_stack.push_back(self.current.clone());

            // Restore previous state
            self.current = prev_state;
            true
        } else {
            false
        }
    }

    /// Redo the last undone action. Returns true if redo was performed.
    pub fn redo(&mut self) -> bool {
        if let Some(next_state) = self.redo_stack.pop_back() {
            // Push current to undo stack
            self.undo_stack.push_back(self.current.clone());

            // Restore next state
            self.current = next_state;
            true
        } else {
            false
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Get the number of undo steps available
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get the number of redo steps available
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }

    /// Clear all history (but keep current state)
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Update current state directly without recording history.
    /// Use for continuous updates like dragging.
    pub fn update_current(&mut self, canvas_offset: (f32, f32), zoom: f32) {
        self.current.canvas_offset = canvas_offset;
        self.current.zoom = zoom;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ItemContent;

    fn make_item(id: u64) -> CanvasItem {
        CanvasItem {
            id,
            position: (id as f32 * 10.0, id as f32 * 10.0),
            size: (100.0, 100.0),
            content: ItemContent::Text(format!("Item {}", id)),
        }
    }

    #[test]
    fn test_new_empty() {
        let history = HistoryManager::new_empty();
        assert!(!history.can_undo());
        assert!(!history.can_redo());
        assert!(history.current().items().is_empty());
    }

    #[test]
    fn test_push_and_undo() {
        let mut history = HistoryManager::new_empty();

        // Add first item
        let state1 = HistorySnapshot::new((0.0, 0.0), 1.0, vec![make_item(1)], 2);
        history.push(state1);

        assert_eq!(history.current().items().len(), 1);
        assert!(history.can_undo());
        assert!(!history.can_redo());

        // Undo
        assert!(history.undo());
        assert!(history.current().items().is_empty());
        assert!(!history.can_undo());
        assert!(history.can_redo());
    }

    #[test]
    fn test_undo_redo_cycle() {
        let mut history = HistoryManager::new_empty();

        // Create states
        let state1 = HistorySnapshot::new((0.0, 0.0), 1.0, vec![make_item(1)], 2);
        let state2 = HistorySnapshot::new((0.0, 0.0), 1.0, vec![make_item(1), make_item(2)], 3);

        history.push(state1);
        history.push(state2);

        assert_eq!(history.current().items().len(), 2);

        // Undo twice
        history.undo();
        assert_eq!(history.current().items().len(), 1);

        history.undo();
        assert_eq!(history.current().items().len(), 0);

        // Redo twice
        history.redo();
        assert_eq!(history.current().items().len(), 1);

        history.redo();
        assert_eq!(history.current().items().len(), 2);
    }

    #[test]
    fn test_new_action_clears_redo() {
        let mut history = HistoryManager::new_empty();

        let state1 = HistorySnapshot::new((0.0, 0.0), 1.0, vec![make_item(1)], 2);
        let state2 = HistorySnapshot::new((0.0, 0.0), 1.0, vec![make_item(1), make_item(2)], 3);

        history.push(state1);
        history.push(state2);

        // Undo once
        history.undo();
        assert!(history.can_redo());

        // Push new state - should clear redo
        let state3 = HistorySnapshot::new((0.0, 0.0), 1.0, vec![make_item(1), make_item(3)], 4);
        history.push(state3);

        assert!(!history.can_redo());
    }

    #[test]
    fn test_history_limit() {
        let mut history = HistoryManager::new_empty();

        // Push more than MAX_HISTORY_SIZE states
        for i in 0..(MAX_HISTORY_SIZE + 10) {
            let state =
                HistorySnapshot::new((0.0, 0.0), 1.0, vec![make_item(i as u64)], (i + 1) as u64);
            history.push(state);
        }

        // Should be capped at MAX_HISTORY_SIZE
        assert!(history.undo_count() <= MAX_HISTORY_SIZE);
    }

    #[test]
    fn test_arc_sharing() {
        let items = vec![make_item(1), make_item(2)];
        let snapshot1 = HistorySnapshot::new((0.0, 0.0), 1.0, items, 3);

        // Clone should share the Arc
        let snapshot2 = snapshot1.clone();

        // Both should point to the same Arc
        assert!(Arc::ptr_eq(&snapshot1.items, &snapshot2.items));
    }

    #[test]
    fn test_record_for_modifications() {
        let mut history = HistoryManager::new_empty();

        // Set up initial state
        history.current = HistorySnapshot::new((0.0, 0.0), 1.0, vec![make_item(1)], 2);

        // Record before modification
        history.record();

        // Modify current
        history.current_mut().items_mut().push(make_item(2));

        assert_eq!(history.current().items().len(), 2);
        assert!(history.can_undo());

        // Undo should restore the state before record()
        history.undo();
        assert_eq!(history.current().items().len(), 1);
    }

    #[test]
    fn test_update_current_no_history() {
        let mut history = HistoryManager::new_empty();

        history.update_current((100.0, 200.0), 1.5);

        assert_eq!(history.current().canvas_offset, (100.0, 200.0));
        assert_eq!(history.current().zoom, 1.5);
        assert!(!history.can_undo()); // Should not record
    }
}
