//! Board Module - Canvas state and item management
//!
//! This module provides the core data structures for managing the infinite canvas,
//! including items, undo/redo history, and debounced saving.
//!
//! ## Performance Notes
//!
//! Key performance considerations:
//! - Item lookup: O(1) via HashMap index
//! - Undo/redo: O(k) where k is items affected by operation
//! - Save: Debounced to avoid frequent disk I/O
//! - History: Uses VecDeque for O(1) front/back operations
//!
//! Enable profiling with `cargo build --features profiling` to see timing.

use crate::board_index::BoardIndex;
use crate::constants::{DOCK_WIDTH, HEADER_HEIGHT};
use crate::error::BoardError;
use crate::profile_scope;
use crate::spatial_index::SpatialIndex;
use crate::data::{is_data_file, parse_csv_file, parse_json_file, write_csv_file, write_json_file};
use crate::types::DataOrigin;
use crate::types::{CanvasItem, DataSource, ItemContent};
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

/// Maximum history operations to keep
const MAX_HISTORY_OPERATIONS: usize = 100;

/// How often to create full snapshots (every N operations)
const SNAPSHOT_INTERVAL: usize = 20;

#[derive(Serialize, Deserialize, Clone)]
pub struct BoardState {
    pub canvas_offset: (f32, f32),
    pub zoom: f32,
    pub items: Vec<CanvasItem>,
    pub next_item_id: u64,
    /// Shared data sources for tables and charts
    #[serde(default)]
    pub data_sources: HashMap<u64, DataSource>,
    #[serde(default)]
    pub next_data_source_id: u64,
}

/// A single undoable operation (delta-based)
#[derive(Clone, Debug)]
pub enum UndoOperation {
    /// Add an item to the canvas
    AddItem(CanvasItem),
    /// Remove an item from the canvas (stores the removed item for undo)
    RemoveItem(CanvasItem),
    /// Move an item from old position to new position
    MoveItem {
        id: u64,
        old_pos: (f32, f32),
        new_pos: (f32, f32),
    },
    /// Resize an item
    ResizeItem {
        id: u64,
        old_size: (f32, f32),
        new_size: (f32, f32),
    },
    /// Move and resize combined (for drag operations that do both)
    TransformItem {
        id: u64,
        old_pos: (f32, f32),
        new_pos: (f32, f32),
        old_size: (f32, f32),
        new_size: (f32, f32),
    },
    /// Modify item content (stores old and new item states)
    ModifyItem {
        old_item: CanvasItem,
        new_item: CanvasItem,
    },
    /// Batch of operations (for multi-item actions like file drop)
    Batch(Vec<UndoOperation>),
}

impl UndoOperation {
    /// Apply this operation to the board (for redo)
    pub fn apply(&self, items: &mut Vec<CanvasItem>, items_index: &mut HashMap<u64, usize>) {
        match self {
            UndoOperation::AddItem(item) => {
                items_index.insert(item.id, items.len());
                items.push(item.clone());
            }
            UndoOperation::RemoveItem(item) => {
                if let Some(&idx) = items_index.get(&item.id) {
                    items.remove(idx);
                    items_index.remove(&item.id);
                    // Rebuild index for items after the removed one
                    for (i, it) in items.iter().enumerate().skip(idx) {
                        items_index.insert(it.id, i);
                    }
                }
            }
            UndoOperation::MoveItem { id, new_pos, .. } => {
                if let Some(&idx) = items_index.get(id) {
                    if let Some(item) = items.get_mut(idx) {
                        item.position = *new_pos;
                    }
                }
            }
            UndoOperation::ResizeItem { id, new_size, .. } => {
                if let Some(&idx) = items_index.get(id) {
                    if let Some(item) = items.get_mut(idx) {
                        item.size = *new_size;
                    }
                }
            }
            UndoOperation::TransformItem {
                id,
                new_pos,
                new_size,
                ..
            } => {
                if let Some(&idx) = items_index.get(id) {
                    if let Some(item) = items.get_mut(idx) {
                        item.position = *new_pos;
                        item.size = *new_size;
                    }
                }
            }
            UndoOperation::ModifyItem { new_item, .. } => {
                if let Some(&idx) = items_index.get(&new_item.id) {
                    items[idx] = new_item.clone();
                }
            }
            UndoOperation::Batch(ops) => {
                for op in ops {
                    op.apply(items, items_index);
                }
            }
        }
    }

    /// Reverse this operation (for undo)
    pub fn reverse(&self, items: &mut Vec<CanvasItem>, items_index: &mut HashMap<u64, usize>) {
        match self {
            UndoOperation::AddItem(item) => {
                // Undo add = remove
                if let Some(&idx) = items_index.get(&item.id) {
                    items.remove(idx);
                    items_index.remove(&item.id);
                    for (i, it) in items.iter().enumerate().skip(idx) {
                        items_index.insert(it.id, i);
                    }
                }
            }
            UndoOperation::RemoveItem(item) => {
                // Undo remove = add back
                items_index.insert(item.id, items.len());
                items.push(item.clone());
            }
            UndoOperation::MoveItem { id, old_pos, .. } => {
                if let Some(&idx) = items_index.get(id) {
                    if let Some(item) = items.get_mut(idx) {
                        item.position = *old_pos;
                    }
                }
            }
            UndoOperation::ResizeItem { id, old_size, .. } => {
                if let Some(&idx) = items_index.get(id) {
                    if let Some(item) = items.get_mut(idx) {
                        item.size = *old_size;
                    }
                }
            }
            UndoOperation::TransformItem {
                id,
                old_pos,
                old_size,
                ..
            } => {
                if let Some(&idx) = items_index.get(id) {
                    if let Some(item) = items.get_mut(idx) {
                        item.position = *old_pos;
                        item.size = *old_size;
                    }
                }
            }
            UndoOperation::ModifyItem { old_item, .. } => {
                if let Some(&idx) = items_index.get(&old_item.id) {
                    items[idx] = old_item.clone();
                }
            }
            UndoOperation::Batch(ops) => {
                // Reverse in opposite order
                for op in ops.iter().rev() {
                    op.reverse(items, items_index);
                }
            }
        }
    }
}

/// A history entry - either an operation or a full snapshot
#[derive(Clone)]
enum HistoryEntry {
    /// A delta operation
    Operation(UndoOperation),
    /// A full state snapshot (created periodically for efficiency)
    Snapshot(BoardState),
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

    // Spatial index for O(log n) hit testing queries
    spatial_index: SpatialIndex,

    pub next_item_id: u64,

    /// Shared data sources for tables and charts
    pub data_sources: HashMap<u64, DataSource>,
    pub next_data_source_id: u64,

    // Delta-based history using VecDeque for O(1) front removal
    history: VecDeque<HistoryEntry>,
    history_index: usize,
    /// Counter for operations since last snapshot
    ops_since_snapshot: usize,

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
            let spatial_index = SpatialIndex::from_items(
                state.items.iter().map(|item| (item.id, item.position, item.size))
            );
            Self {
                id,
                canvas_offset: point(px(state.canvas_offset.0), px(state.canvas_offset.1)),
                zoom: state.zoom,
                items: state.items,
                items_index,
                spatial_index,
                next_item_id: state.next_item_id,
                data_sources: state.data_sources,
                next_data_source_id: state.next_data_source_id,
                history: VecDeque::new(),
                history_index: 0,
                ops_since_snapshot: 0,
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
        Self {
            id,
            canvas_offset: point(px(0.0), px(0.0)),
            zoom: 1.0,
            items: Vec::new(),
            items_index: HashMap::new(),
            spatial_index: SpatialIndex::new(),
            next_item_id: 0,
            data_sources: HashMap::new(),
            next_data_source_id: 0,
            history: VecDeque::new(),
            history_index: 0,
            ops_since_snapshot: 0,
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
        self.spatial_index.rebuild(
            self.items.iter().map(|item| (item.id, item.position, item.size))
        );
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

    /// Query the spatial index for items at a point in canvas coordinates.
    /// Returns item IDs that contain the point. O(log n) performance.
    pub fn query_items_at_point(&self, x: f32, y: f32) -> Vec<u64> {
        self.spatial_index.query_point(x, y)
    }

    /// Query the spatial index for items in a rectangle (for marquee selection).
    /// Returns item IDs that intersect the rectangle. O(log n + k) where k is results.
    pub fn query_items_in_rect(&self, min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Vec<u64> {
        self.spatial_index.query_rect(min_x, min_y, max_x, max_y)
    }

    /// Update an item's position in the spatial index.
    /// Call this after modifying an item's position directly.
    pub fn update_spatial_index(&mut self, id: u64) {
        if let Some(item) = self.get_item(id) {
            self.spatial_index.update(id, item.position, item.size);
        }
    }

    /// Add a single item (still triggers history + save for single operations)
    pub fn add_item(&mut self, position: Point<Pixels>, content: ItemContent) -> u64 {
        let id = self.add_item_internal(position, content);
        // Get the just-added item for the undo operation
        if let Some(item) = self.get_item(id).cloned() {
            self.push_operation(UndoOperation::AddItem(item));
        }
        self.mark_dirty();
        id
    }

    /// Internal add without history/save - used for batch operations
    fn add_item_internal(&mut self, position: Point<Pixels>, content: ItemContent) -> u64 {
        let size = content.default_size();
        let id = self.next_item_id;
        let pos = (f32::from(position.x), f32::from(position.y));

        self.items.push(CanvasItem {
            id,
            position: pos,
            size,
            content,
        });
        self.items_index.insert(id, self.items.len() - 1);
        self.spatial_index.insert(id, pos, size);
        self.next_item_id += 1;
        id
    }

    /// Handle file drop - batched operation (single history push + save)
    /// For iCloud boards, files are copied to the board's files/ directory
    /// so they sync across devices.
    ///
    /// Returns a list of error messages for any files that failed to copy.
    /// The caller should display these to the user via toast notifications.
    ///
    /// ## Performance
    /// This can be slow for many files or large files due to:
    /// - File I/O for copying (iCloud boards)
    /// - Content type detection
    pub fn handle_file_drop(&mut self, position: Point<Pixels>, paths: Vec<PathBuf>) -> Vec<String> {
        profile_scope!("handle_file_drop");

        let mut errors = Vec::new();

        if paths.is_empty() {
            return errors;
        }

        // Stagger offset for multiple files so they don't overlap
        const STAGGER_X: f32 = 30.0;
        const STAGGER_Y: f32 = 30.0;

        let mut added_ids = Vec::new();

        for (i, path) in paths.iter().enumerate() {
            // For iCloud boards, copy the file to the board's files directory
            let actual_path = if self.should_copy_files() {
                match self.copy_file_to_board(path) {
                    Ok(copied_path) => copied_path,
                    Err(e) => {
                        let filename = path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("file");
                        errors.push(format!("Failed to copy '{}': {}", filename, e));
                        warn!("Failed to copy file to board storage: {}", e);
                        path.clone()
                    }
                }
            } else {
                path.clone()
            };

            let base_pos = self.screen_to_canvas(position);
            let staggered_pos = point(
                px(f32::from(base_pos.x) + (i as f32 * STAGGER_X)),
                px(f32::from(base_pos.y) + (i as f32 * STAGGER_Y)),
            );

            // Check if this is a data file (CSV/TSV/JSON)
            if is_data_file(&actual_path) {
                let filename = actual_path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("data");

                // Parse the data file
                let parse_result = if actual_path.extension()
                    .and_then(|e| e.to_str())
                    .map(|e| e.to_lowercase() == "json")
                    .unwrap_or(false)
                {
                    parse_json_file(&actual_path)
                } else {
                    parse_csv_file(&actual_path)
                };

                match parse_result {
                    Ok(mut data_source) => {
                        // Assign ID and store the data source
                        data_source.id = self.next_data_source_id;
                        self.next_data_source_id += 1;
                        let ds_id = data_source.id;
                        self.data_sources.insert(ds_id, data_source);

                        // Create a table item referencing this data source
                        let content = ItemContent::Table {
                            data_source_id: ds_id,
                            show_headers: true,
                            stripe: true,
                        };
                        let id = self.add_item_internal(staggered_pos, content);
                        added_ids.push(id);
                        info!("Created table from data file: {}", filename);
                    }
                    Err(e) => {
                        errors.push(format!("Failed to parse '{}': {}", filename, e));
                        warn!("Failed to parse data file '{}': {}", filename, e);
                    }
                }
            } else {
                // Handle as regular file (image, video, etc.)
                let content = ItemContent::from_path(&actual_path);
                let id = self.add_item_internal(staggered_pos, content);
                added_ids.push(id);
            }
        }

        // Create batch operation for all added items
        let ops: Vec<UndoOperation> = added_ids
            .iter()
            .filter_map(|&id| self.get_item(id).cloned())
            .map(UndoOperation::AddItem)
            .collect();

        if !ops.is_empty() {
            if ops.len() == 1 {
                self.push_operation(ops.into_iter().next().unwrap());
            } else {
                self.push_operation(UndoOperation::Batch(ops));
            }
        }
        self.mark_dirty();

        errors
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
            // Clone the item before removing for undo operation
            let item = self.items[idx].clone();
            self.items.remove(idx);
            self.rebuild_index();
            self.push_operation(UndoOperation::RemoveItem(item));
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
            .filter_map(|item| {
                if !item.content.is_searchable() {
                    return None;
                }

                // Get display name - for tables, use the data source name
                let display_name = match &item.content {
                    ItemContent::Table { data_source_id, .. } => {
                        self.data_sources
                            .get(data_source_id)
                            .map(|ds| ds.name.clone())
                            .unwrap_or_else(|| "Table".to_string())
                    }
                    _ => item.content.display_name(),
                };

                if display_name.to_lowercase().contains(&query_lower) {
                    Some((item.id, display_name))
                } else {
                    None
                }
            })
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
        profile_scope!("board_save");

        let state = BoardState {
            canvas_offset: (
                f32::from(self.canvas_offset.x),
                f32::from(self.canvas_offset.y),
            ),
            zoom: self.zoom,
            items: self.items.clone(),
            next_item_id: self.next_item_id,
            data_sources: self.data_sources.clone(),
            next_data_source_id: self.next_data_source_id,
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

    /// Push a delta operation to history (memory-efficient)
    pub fn push_operation(&mut self, op: UndoOperation) {
        // Remove any operations after current index (for redo branch pruning)
        while self.history.len() > self.history_index {
            self.history.pop_back();
        }

        self.history.push_back(HistoryEntry::Operation(op));
        self.history_index = self.history.len();
        self.ops_since_snapshot += 1;

        // Create periodic snapshot for efficient reconstruction
        if self.ops_since_snapshot >= SNAPSHOT_INTERVAL {
            self.create_snapshot();
        }

        // Limit history - O(1) removal from front with VecDeque
        while self.history.len() > MAX_HISTORY_OPERATIONS {
            self.history.pop_front();
            if self.history_index > 0 {
                self.history_index -= 1;
            }
        }
    }

    /// Create a full snapshot in history (for periodic checkpoints)
    fn create_snapshot(&mut self) {
        let state = BoardState {
            canvas_offset: (
                f32::from(self.canvas_offset.x),
                f32::from(self.canvas_offset.y),
            ),
            zoom: self.zoom,
            items: self.items.clone(),
            next_item_id: self.next_item_id,
            data_sources: self.data_sources.clone(),
            next_data_source_id: self.next_data_source_id,
        };
        self.history.push_back(HistoryEntry::Snapshot(state));
        self.history_index = self.history.len();
        self.ops_since_snapshot = 0;
    }

    /// Legacy push_history - creates a snapshot (backward compatibility)
    /// Prefer push_operation for new code
    pub fn push_history(&mut self) {
        // Remove any operations after current index (for redo branch pruning)
        while self.history.len() > self.history_index {
            self.history.pop_back();
        }

        self.create_snapshot();

        // Limit history
        while self.history.len() > MAX_HISTORY_OPERATIONS {
            self.history.pop_front();
            if self.history_index > 0 {
                self.history_index -= 1;
            }
        }
    }

    pub fn undo(&mut self) -> bool {
        profile_scope!("board_undo");

        if self.history_index == 0 {
            return false;
        }

        self.history_index -= 1;
        let entry = self.history.get(self.history_index).cloned();

        match entry {
            Some(HistoryEntry::Operation(op)) => {
                // Reverse the operation
                op.reverse(&mut self.items, &mut self.items_index);
                // Rebuild spatial index after operation
                self.spatial_index.rebuild(
                    self.items.iter().map(|item| (item.id, item.position, item.size))
                );
                self.mark_dirty();
                true
            }
            Some(HistoryEntry::Snapshot(_)) => {
                // For snapshots, restore the PREVIOUS state (the one before this change)
                // history[n] contains state AFTER change n, so we need history[n-1]
                if self.history_index > 0 {
                    if let Some(HistoryEntry::Snapshot(prev_state)) = self.history.get(self.history_index - 1).cloned() {
                        self.restore_from_snapshot(&prev_state);
                        return true;
                    }
                }
                // Can't undo if there's no previous snapshot
                self.history_index += 1; // Restore index since we can't actually undo
                false
            }
            None => false,
        }
    }

    pub fn redo(&mut self) -> bool {
        profile_scope!("board_redo");

        if self.history_index >= self.history.len() {
            return false;
        }

        let entry = self.history.get(self.history_index).cloned();
        self.history_index += 1;

        match entry {
            Some(HistoryEntry::Operation(op)) => {
                // Apply the operation
                op.apply(&mut self.items, &mut self.items_index);
                // Rebuild spatial index after operation
                self.spatial_index.rebuild(
                    self.items.iter().map(|item| (item.id, item.position, item.size))
                );
                self.mark_dirty();
                true
            }
            Some(HistoryEntry::Snapshot(state)) => {
                // For snapshots on redo, we need to apply operations until next snapshot
                // For simplicity, just restore the snapshot
                self.restore_from_snapshot(&state);
                true
            }
            None => false,
        }
    }

    fn restore_from_snapshot(&mut self, state: &BoardState) {
        self.canvas_offset = point(px(state.canvas_offset.0), px(state.canvas_offset.1));
        self.zoom = state.zoom;
        self.items = state.items.clone();
        self.next_item_id = state.next_item_id;
        self.data_sources = state.data_sources.clone();
        self.next_data_source_id = state.next_data_source_id;
        self.rebuild_index();
        self.mark_dirty();
    }

    // =========================================================================
    // Data Source File Operations
    // =========================================================================

    /// Save a data source back to its original file.
    ///
    /// Only works for data sources with file origins (CSV/JSON).
    /// Returns Ok with the path saved to, or Err with error message.
    pub fn save_data_source_to_file(&mut self, data_source_id: u64) -> Result<PathBuf, String> {
        let ds = self.data_sources.get(&data_source_id)
            .ok_or_else(|| "Data source not found".to_string())?;

        let result = match &ds.origin {
            DataOrigin::File { .. } => write_csv_file(ds),
            DataOrigin::Json { path: Some(_) } => write_json_file(ds),
            DataOrigin::Json { path: None } => {
                Err("JSON data source has no file path".to_string())
            }
            DataOrigin::Manual => {
                Err("Cannot save manually-created data to file".to_string())
            }
            DataOrigin::Api { .. } => {
                Err("Cannot save API data source to file".to_string())
            }
        };

        // If successful, mark the data source as clean
        if result.is_ok() {
            if let Some(ds) = self.data_sources.get_mut(&data_source_id) {
                ds.mark_clean();
            }
        }

        result
    }

    /// Reload a data source from its original file.
    ///
    /// Replaces the current data with fresh data from the file.
    /// Returns Ok on success, or Err with error message.
    pub fn reload_data_source_from_file(&mut self, data_source_id: u64) -> Result<(), String> {
        let ds = self.data_sources.get(&data_source_id)
            .ok_or_else(|| "Data source not found".to_string())?;

        let new_data = match &ds.origin {
            DataOrigin::File { path, .. } => parse_csv_file(path),
            DataOrigin::Json { path: Some(p) } => parse_json_file(p),
            DataOrigin::Json { path: None } => {
                Err("JSON data source has no file path".to_string())
            }
            DataOrigin::Manual => {
                Err("Cannot reload manually-created data".to_string())
            }
            DataOrigin::Api { .. } => {
                Err("API reload not implemented".to_string())
            }
        }?;

        // Update the existing data source with new data
        if let Some(ds) = self.data_sources.get_mut(&data_source_id) {
            ds.columns = new_data.columns;
            ds.rows = new_data.rows;
            ds.mark_clean();
        }

        self.mark_dirty();
        Ok(())
    }

    /// Check if a data source has unsaved changes
    pub fn is_data_source_dirty(&self, data_source_id: u64) -> bool {
        self.data_sources.get(&data_source_id)
            .map(|ds| ds.is_dirty())
            .unwrap_or(false)
    }

    /// Check if a data source can be saved to file
    pub fn can_save_data_source(&self, data_source_id: u64) -> bool {
        self.data_sources.get(&data_source_id)
            .map(|ds| ds.has_file_origin())
            .unwrap_or(false)
    }

    /// Create a fresh board for testing (doesn't load from disk)
    pub fn new_for_test() -> Self {
        Self::new_empty("test-board".to_string())
    }

    /// Get the current history length (for testing)
    pub fn history_len(&self) -> usize {
        self.history.len()
    }

    /// Get the current history index (for testing)
    pub fn current_history_index(&self) -> usize {
        self.history_index
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
