//! Board Module - Canvas state and item management
//!
//! This module provides the core data structures for managing the infinite canvas,
//! including items, undo/redo history, and debounced saving.

use crate::board_index::BoardIndex;
use crate::error::BoardError;
use crate::types::{CanvasItem, ItemContent};
use crate::validation::validate_items;
use gpui::{point, px, Pixels, Point, Size};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tracing::{debug, error, info, info_span, trace, warn};

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
    /// Save board state to a file path.
    ///
    /// Returns Ok(()) on success, or a BoardError on failure.
    pub fn save_to_path(&self, path: &PathBuf) -> Result<(), BoardError> {
        let json = serde_json::to_string_pretty(&self).map_err(BoardError::ParseError)?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| BoardError::SaveFailed {
                path: path.clone(),
                source: e,
            })?;
        }

        fs::write(path, json).map_err(|e| BoardError::SaveFailed {
            path: path.clone(),
            source: e,
        })?;

        trace!("Board state saved to {:?}", path);
        Ok(())
    }

    /// Load board state from a file path.
    ///
    /// Returns the loaded state, or a BoardError if loading fails.
    pub fn load_from_path(path: &PathBuf) -> Result<Self, BoardError> {
        let json = fs::read_to_string(path).map_err(|e| BoardError::LoadFailed {
            path: path.clone(),
            source: e,
        })?;

        let state = serde_json::from_str(&json).map_err(BoardError::ParseError)?;
        trace!("Board state loaded from {:?}", path);
        Ok(state)
    }

    /// Try to load board state, returning None if the file doesn't exist
    /// or an error occurs.
    pub fn try_load(path: &PathBuf) -> Option<Self> {
        match Self::load_from_path(path) {
            Ok(state) => Some(state),
            Err(e) => {
                if !matches!(e, BoardError::LoadFailed { .. }) {
                    warn!("Failed to load board state: {}", e);
                }
                None
            }
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

    // Storage location for this board (used to determine if files should be copied)
    storage_location: crate::board_index::StoredLocation,
}

impl Board {
    /// Load a board by ID, or create a new empty one.
    ///
    /// If the board file doesn't exist or can't be loaded, a new empty
    /// board is created. Uses the board index to find the correct storage location.
    pub fn load(id: String) -> Self {
        let _span = info_span!("board_load", board_id = %id).entered();

        // Try to get path and storage location from board index
        let index = BoardIndex::load();
        let (board_path, storage_location) = index.get_board(&id)
            .map(|b| (b.board_path(), b.storage_location.clone()))
            .unwrap_or_else(|| (BoardIndex::board_path(&id), crate::board_index::StoredLocation::Default));

        if let Some(mut state) = BoardState::try_load(&board_path) {
            info!(items = state.items.len(), "Loaded board");

            // Validate and fix any invalid item properties
            let fixed_count = validate_items(&mut state.items);
            if fixed_count > 0 {
                warn!(
                    "Fixed {} items with invalid properties in board '{}'",
                    fixed_count, id
                );
            }

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
                dirty: fixed_count > 0, // Mark dirty if we fixed anything
                last_change: Instant::now(),
                storage_location,
            }
        } else {
            debug!("Creating new empty board '{}'", id);
            Self::new_empty_with_location(id, storage_location)
        }
    }

    /// Create a new empty board with the given ID and default storage location
    pub fn new_empty(id: String) -> Self {
        Self::new_empty_with_location(id, crate::board_index::StoredLocation::Default)
    }

    /// Create a new empty board with the given ID and storage location
    pub fn new_empty_with_location(id: String, storage_location: crate::board_index::StoredLocation) -> Self {
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
            storage_location,
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
    /// For iCloud boards, files are copied to the board's files/ directory
    /// so they sync across devices.
    pub fn handle_file_drop(&mut self, position: Point<Pixels>, paths: Vec<PathBuf>) {
        if paths.is_empty() {
            return;
        }

        // Stagger offset for multiple files so they don't overlap
        const STAGGER_X: f32 = 30.0;
        const STAGGER_Y: f32 = 30.0;

        for (i, path) in paths.iter().enumerate() {
            // For iCloud boards, copy the file to the board's files directory
            let actual_path = if self.should_copy_files() {
                self.copy_file_to_board(path).unwrap_or_else(|e| {
                    warn!("Failed to copy file to board storage: {}", e);
                    path.clone()
                })
            } else {
                path.clone()
            };

            let content = ItemContent::from_path(&actual_path);
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

    /// Check if files should be copied to the board's storage
    /// Returns true for iCloud boards (files need to be synced)
    fn should_copy_files(&self) -> bool {
        matches!(self.storage_location, crate::board_index::StoredLocation::ICloud)
    }

    /// Get the files directory for this board
    pub fn files_dir(&self) -> PathBuf {
        self.storage_location.base_path().join(&self.id).join("files")
    }

    /// Copy a file to the board's files directory
    /// Returns the new path to the copied file
    fn copy_file_to_board(&self, source: &PathBuf) -> Result<PathBuf, std::io::Error> {
        let files_dir = self.files_dir();
        std::fs::create_dir_all(&files_dir)?;

        // Get original filename and sanitize it to prevent path traversal attacks
        let filename = source
            .file_name()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "No filename"))?
            .to_str()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid filename encoding"))?;

        // Sanitize filename: remove path separators and reject dangerous names
        let sanitized = sanitize_filename(filename)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid filename"))?;

        // Generate unique filename if it already exists
        let mut dest = files_dir.join(&sanitized);
        if dest.exists() {
            let path = std::path::Path::new(&sanitized);
            let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
            let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis()
                % 100000;

            let new_name = if ext.is_empty() {
                format!("{}_{}", stem, timestamp)
            } else {
                format!("{}_{}.{}", stem, timestamp, ext)
            };
            dest = files_dir.join(new_name);
        }

        // Final safety check: verify destination is within files_dir
        let canonical_files_dir = files_dir.canonicalize().unwrap_or(files_dir.clone());
        let canonical_dest = dest.parent()
            .and_then(|p| p.canonicalize().ok())
            .map(|p| p.join(dest.file_name().unwrap_or_default()));

        if let Some(ref canonical) = canonical_dest {
            if !canonical.starts_with(&canonical_files_dir) {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    "Path traversal attempt detected",
                ));
            }
        }

        std::fs::copy(source, &dest)?;
        info!("Copied file to board storage: {:?} -> {:?}", source, dest);
        Ok(dest)
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

    /// Remove multiple items by their IDs
    /// This is more efficient than calling remove_item multiple times
    /// as it only rebuilds the index once.
    pub fn remove_items(&mut self, ids: &[u64]) {
        if ids.is_empty() {
            return;
        }
        let id_set: std::collections::HashSet<u64> = ids.iter().copied().collect();
        self.items.retain(|item| !id_set.contains(&item.id));
        self.rebuild_index();
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
    /// Returns Ok(true) if saved successfully, Ok(false) if not dirty, Err on failure
    pub fn flush_save(&mut self) -> Result<bool, BoardError> {
        if self.dirty {
            self.try_save()?;
            self.dirty = false;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Try to save, returning any errors
    pub fn try_save(&self) -> Result<(), BoardError> {
        let state = BoardState {
            canvas_offset: (
                f32::from(self.canvas_offset.x),
                f32::from(self.canvas_offset.y),
            ),
            zoom: self.zoom,
            items: self.items.clone(),
            next_item_id: self.next_item_id,
        };

        // Get path from board index (supports custom storage locations)
        let index = BoardIndex::load();
        let board_path = index.get_board_path(&self.id)
            .unwrap_or_else(|| BoardIndex::board_path(&self.id));

        state.save_to_path(&board_path)?;
        debug!("Board '{}' saved with {} items", self.id, self.items.len());
        Ok(())
    }

    /// Force immediate save (used when leaving board).
    ///
    /// Logs any errors but doesn't propagate them since this is
    /// typically called during cleanup.
    pub fn save_immediate(&self) {
        if let Err(e) = self.try_save() {
            error!("Failed to save board '{}': {}", self.id, e);
        }
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

/// Sanitize a filename to prevent path traversal attacks.
/// Returns None if the filename is invalid or dangerous.
fn sanitize_filename(filename: &str) -> Option<String> {
    // Reject empty filenames
    if filename.is_empty() {
        return None;
    }

    // Reject dangerous names
    if filename == "." || filename == ".." {
        return None;
    }

    // Remove any path separators (both Unix and Windows style)
    let sanitized: String = filename
        .chars()
        .filter(|&c| c != '/' && c != '\\')
        .collect();

    // Reject if sanitization resulted in empty string or dangerous name
    if sanitized.is_empty() || sanitized == "." || sanitized == ".." {
        return None;
    }

    // Reject filenames that start with a dot followed by dot (hidden traversal)
    if sanitized.starts_with("..") {
        return None;
    }

    Some(sanitized)
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

    // ========================================================================
    // Undo/Redo State Tests
    // ========================================================================

    #[test]
    fn test_undo_preserves_canvas_offset() {
        let mut board = Board::new_for_test();

        // Add item at initial offset
        board.add_item(
            point(px(0.0), px(0.0)),
            ItemContent::Text("First".to_string()),
        );

        // Change canvas offset and add another item
        board.canvas_offset = point(px(100.0), px(200.0));
        board.add_item(
            point(px(50.0), px(50.0)),
            ItemContent::Text("Second".to_string()),
        );

        // Undo should restore previous canvas offset
        board.undo();
        assert_eq!(f32::from(board.canvas_offset.x), 0.0);
        assert_eq!(f32::from(board.canvas_offset.y), 0.0);
    }

    #[test]
    fn test_undo_preserves_zoom() {
        let mut board = Board::new_for_test();

        // Add item at default zoom
        board.add_item(
            point(px(0.0), px(0.0)),
            ItemContent::Text("First".to_string()),
        );
        let initial_zoom = board.zoom;

        // Change zoom and add another item
        board.zoom = 2.5;
        board.add_item(
            point(px(50.0), px(50.0)),
            ItemContent::Text("Second".to_string()),
        );

        // Undo should restore previous zoom
        board.undo();
        assert_eq!(board.zoom, initial_zoom);
    }

    #[test]
    fn test_undo_preserves_next_item_id() {
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

        // Undo should restore previous next_item_id
        board.undo();
        assert_eq!(board.next_item_id, 1);
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

        // Add more items than MAX_HISTORY_STATES
        for i in 0..(MAX_HISTORY_STATES + 10) {
            board.add_item(
                point(px(i as f32 * 10.0), px(0.0)),
                ItemContent::Text(format!("Item {}", i)),
            );
        }

        // History should be limited
        assert!(board.history.len() <= MAX_HISTORY_STATES + 1);

        // We can't undo all the way back to empty state
        let mut undo_count = 0;
        while board.undo() {
            undo_count += 1;
        }

        // Should be able to undo MAX_HISTORY_STATES times at most
        assert!(undo_count <= MAX_HISTORY_STATES);
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

        // Modify item position and push history
        board.items[0].position = (100.0, 200.0);
        board.push_history();

        // Verify new position
        assert_eq!(board.items[0].position, (100.0, 200.0));

        // Undo should restore original position
        board.undo();
        assert_eq!(board.items[0].position, (0.0, 0.0));

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

        // Modify item size and push history
        board.items[0].size = (500.0, 300.0);
        board.push_history();

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

        // Initial state: history_index should be 0
        assert_eq!(board.history_index, 0);
        assert_eq!(board.history.len(), 1);

        // Add items and verify history_index grows
        board.add_item(
            point(px(0.0), px(0.0)),
            ItemContent::Text("First".to_string()),
        );
        assert_eq!(board.history_index, 1);

        board.add_item(
            point(px(100.0), px(0.0)),
            ItemContent::Text("Second".to_string()),
        );
        assert_eq!(board.history_index, 2);

        // Undo and verify history_index decreases
        board.undo();
        assert_eq!(board.history_index, 1);

        board.undo();
        assert_eq!(board.history_index, 0);

        // Redo and verify history_index increases
        board.redo();
        assert_eq!(board.history_index, 1);
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

        // Remove an item and push history
        board.remove_item(0);
        board.push_history();
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

        // Capture state before modification
        let original_text = if let ItemContent::Text(t) = &board.items[0].content {
            t.clone()
        } else {
            panic!("Expected Text content");
        };

        // Modify the item content directly (simulating user edit)
        board.items[0].content = ItemContent::Text("Modified".to_string());
        board.push_history();

        // Undo should restore original content
        board.undo();
        if let ItemContent::Text(t) = &board.items[0].content {
            assert_eq!(t, &original_text);
        } else {
            panic!("Expected Text content after undo");
        }
    }

    #[test]
    fn test_push_history_at_capacity() {
        let mut board = Board::new_for_test();

        // Fill history to capacity
        for i in 0..MAX_HISTORY_STATES {
            board.add_item(
                point(px(i as f32 * 10.0), px(0.0)),
                ItemContent::Text(format!("Item {}", i)),
            );
        }

        let history_len_at_capacity = board.history.len();

        // Add one more - should stay at or below capacity
        board.add_item(
            point(px(1000.0), px(0.0)),
            ItemContent::Text("Overflow".to_string()),
        );

        assert!(board.history.len() <= history_len_at_capacity);
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
}
