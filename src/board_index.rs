//! Board index and metadata management.
//!
//! This module provides persistence for board metadata, enabling multiple boards
//! with different storage locations (local, iCloud, custom paths). It handles:
//!
//! - Board creation, renaming, and deletion (soft-delete with trash)
//! - iCloud sync discovery across devices
//! - Automatic purging of old trashed boards (30+ days)
//! - Legacy board migration from single-board format
//!
//! ## Storage Locations
//!
//! Boards can be stored in three locations:
//! - **Default**: Local app data directory
//! - **iCloud**: Apple iCloud Drive (enables cross-device sync)
//! - **Custom**: User-specified directory

use crate::app::StorageLocation;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// Serializable storage location for boards
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(tag = "type", content = "path")]
pub enum StoredLocation {
    #[default]
    Default,
    ICloud,
    Custom(PathBuf),
}

impl From<StorageLocation> for StoredLocation {
    fn from(loc: StorageLocation) -> Self {
        match loc {
            StorageLocation::Default => StoredLocation::Default,
            StorageLocation::ICloud => StoredLocation::ICloud,
            StorageLocation::Custom(p) => StoredLocation::Custom(p),
        }
    }
}

impl StoredLocation {
    /// Get the base path for this storage location
    pub fn base_path(&self) -> PathBuf {
        match self {
            StoredLocation::Default => {
                dirs::data_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join("humanboard")
                    .join("boards")
            }
            StoredLocation::ICloud => {
                dirs::home_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join("Library")
                    .join("Mobile Documents")
                    .join("com~apple~CloudDocs")
                    .join("Humanboard")
            }
            StoredLocation::Custom(path) => path.clone(),
        }
    }

    /// Get the display name for UI
    pub fn display_name(&self) -> &str {
        match self {
            StoredLocation::Default => "Local",
            StoredLocation::ICloud => "iCloud",
            StoredLocation::Custom(_) => "Custom",
        }
    }
}

/// Metadata for a single board stored in the index.
///
/// Contains identification, timestamps, and storage location info.
/// Does not include the actual board content (items), which is stored
/// separately in the board's directory.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BoardMetadata {
    /// Unique identifier (UUID-like hex string)
    pub id: String,
    /// User-visible board name
    pub name: String,
    /// Unix timestamp when board was created
    pub created_at: u64,
    /// Unix timestamp of last modification
    pub updated_at: u64,
    /// Where this board's data is stored
    #[serde(default)]
    pub storage_location: StoredLocation,
    /// Timestamp when board was moved to trash (None = not deleted)
    #[serde(default)]
    pub deleted_at: Option<u64>,
}

impl BoardMetadata {
    pub fn new(name: String) -> Self {
        Self::with_location(name, StoredLocation::Default)
    }

    pub fn with_location(name: String, location: StoredLocation) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id: generate_uuid(),
            name,
            created_at: now,
            updated_at: now,
            storage_location: location,
            deleted_at: None,
        }
    }

    /// Check if this board is in the trash
    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    /// Move this board to trash
    pub fn move_to_trash(&mut self) {
        self.deleted_at = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        );
    }

    /// Restore this board from trash
    pub fn restore(&mut self) {
        self.deleted_at = None;
    }

    /// Get how long ago this was deleted (for display)
    pub fn deleted_ago(&self) -> Option<String> {
        let deleted_at = self.deleted_at?;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let diff = now.saturating_sub(deleted_at);

        Some(if diff < 60 {
            "Just now".to_string()
        } else if diff < 3600 {
            format!("{} min ago", diff / 60)
        } else if diff < 86400 {
            format!("{} hours ago", diff / 3600)
        } else {
            format!("{} days ago", diff / 86400)
        })
    }

    /// Check if board should be permanently deleted (older than 30 days)
    pub fn should_purge(&self) -> bool {
        const THIRTY_DAYS: u64 = 30 * 24 * 60 * 60;
        if let Some(deleted_at) = self.deleted_at {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            now.saturating_sub(deleted_at) > THIRTY_DAYS
        } else {
            false
        }
    }

    pub fn touch(&mut self) {
        self.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }

    /// Get the directory for this board
    pub fn board_dir(&self) -> PathBuf {
        self.storage_location.base_path().join(&self.id)
    }

    /// Get the path to the board.json file
    pub fn board_path(&self) -> PathBuf {
        self.board_dir().join("board.json")
    }

    /// Get the files directory for this board
    pub fn files_dir(&self) -> PathBuf {
        self.board_dir().join("files")
    }

    pub fn formatted_date(&self) -> String {
        // Simple date formatting - just show relative time or date
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let diff = now.saturating_sub(self.updated_at);

        if diff < 60 {
            "Just now".to_string()
        } else if diff < 3600 {
            format!("{} min ago", diff / 60)
        } else if diff < 86400 {
            format!("{} hours ago", diff / 3600)
        } else if diff < 604800 {
            format!("{} days ago", diff / 86400)
        } else {
            format!("{} weeks ago", diff / 604800)
        }
    }
}

/// Central index of all known boards across storage locations.
///
/// The index is stored locally and tracks boards from all storage locations.
/// On load, it discovers any new boards from iCloud that may have been
/// created on other devices.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct BoardIndex {
    /// All known boards, sorted by updated_at (most recent first)
    pub boards: Vec<BoardMetadata>,
}

impl BoardIndex {
    pub fn load() -> Self {
        let index_path = Self::index_path();

        let mut index = if let Ok(json) = fs::read_to_string(&index_path) {
            serde_json::from_str(&json).unwrap_or_default()
        } else {
            // Check for legacy board.json and migrate if exists
            Self::migrate_legacy()
        };

        // Discover boards from iCloud that aren't in the local index
        let discovered = index.discover_icloud_boards();
        if discovered > 0 {
            tracing::info!("Discovered {} boards from iCloud", discovered);
            index.save();
        }

        // Auto-purge boards in trash for more than 30 days
        index.purge_old_trash();

        index
    }

    /// Discover boards stored in iCloud that aren't in the local index.
    /// This enables cross-device sync - boards created on one device
    /// will be discovered when the app runs on another device.
    fn discover_icloud_boards(&mut self) -> usize {
        let icloud_path = StoredLocation::ICloud.base_path();

        // Check if iCloud Humanboard folder exists
        if !icloud_path.exists() {
            return 0;
        }

        let mut discovered = 0;

        // Scan for board directories in iCloud
        if let Ok(entries) = fs::read_dir(&icloud_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }

                // Check if this directory contains a board.json
                let board_json = path.join("board.json");
                if !board_json.exists() {
                    continue;
                }

                // Get the board ID from the directory name
                let board_id = match path.file_name().and_then(|n| n.to_str()) {
                    Some(id) => id.to_string(),
                    None => continue,
                };

                // Skip if already in index
                if self.boards.iter().any(|b| b.id == board_id) {
                    continue;
                }

                // Try to read the board to get its name
                let name = Self::read_board_name(&board_json)
                    .unwrap_or_else(|| "Untitled Board".to_string());

                // Get file timestamps for created/updated times
                let (created_at, updated_at) = Self::get_file_timestamps(&board_json);

                let metadata = BoardMetadata {
                    id: board_id,
                    name,
                    created_at,
                    updated_at,
                    storage_location: StoredLocation::ICloud,
                    deleted_at: None,
                };

                tracing::debug!("Discovered iCloud board: {} ({})", metadata.name, metadata.id);
                self.boards.push(metadata);
                discovered += 1;
            }
        }

        // Sort by updated_at descending (most recent first)
        if discovered > 0 {
            self.boards.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        }

        discovered
    }

    /// Read the board name from a board.json file
    fn read_board_name(path: &PathBuf) -> Option<String> {
        // We don't have the full Board struct here, so just parse minimally
        #[derive(serde::Deserialize)]
        struct MinimalBoard {
            #[serde(default)]
            name: Option<String>,
        }

        let json = fs::read_to_string(path).ok()?;
        let board: MinimalBoard = serde_json::from_str(&json).ok()?;
        board.name
    }

    /// Get created and updated timestamps from file metadata
    fn get_file_timestamps(path: &PathBuf) -> (u64, u64) {
        use std::time::UNIX_EPOCH;

        let metadata = fs::metadata(path).ok();

        let created = metadata.as_ref()
            .and_then(|m| m.created().ok())
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let modified = metadata
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);

        (created, modified)
    }

    pub fn save(&self) {
        let index_path = Self::index_path();

        if let Some(parent) = index_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        // Also ensure boards directory exists
        let _ = fs::create_dir_all(Self::boards_dir());

        if let Ok(json) = serde_json::to_string_pretty(&self) {
            let _ = fs::write(&index_path, json);
        }
    }

    pub fn create_board(&mut self, name: String) -> BoardMetadata {
        self.create_board_at(name, StorageLocation::Default)
    }

    /// Create a board at a specific storage location
    pub fn create_board_at(&mut self, name: String, location: StorageLocation) -> BoardMetadata {
        let stored_location: StoredLocation = location.into();
        let metadata = BoardMetadata::with_location(name, stored_location.clone());

        // Ensure the storage directory exists
        let board_dir = stored_location.base_path().join(&metadata.id);
        let _ = fs::create_dir_all(&board_dir);
        let _ = fs::create_dir_all(board_dir.join("files"));

        self.boards.insert(0, metadata.clone()); // Add to front (most recent)
        self.save();
        metadata
    }

    pub fn rename_board(&mut self, id: &str, new_name: String) -> bool {
        if let Some(board) = self.boards.iter_mut().find(|b| b.id == id) {
            board.name = new_name;
            board.touch();
            self.save();
            true
        } else {
            false
        }
    }

    /// Soft delete - moves board to trash (can be restored)
    pub fn delete_board(&mut self, id: &str) -> bool {
        if let Some(board) = self.boards.iter_mut().find(|b| b.id == id) {
            board.move_to_trash();
            self.save();
            true
        } else {
            false
        }
    }

    /// Restore a board from trash
    pub fn restore_board(&mut self, id: &str) -> bool {
        if let Some(board) = self.boards.iter_mut().find(|b| b.id == id && b.is_deleted()) {
            board.restore();
            board.touch(); // Update timestamp so it appears at top
            self.save();
            true
        } else {
            false
        }
    }

    /// Permanently delete a board (no recovery)
    pub fn permanently_delete_board(&mut self, id: &str) -> bool {
        // Find the board first to get its storage location
        let board_dir = self.boards.iter()
            .find(|b| b.id == id)
            .map(|b| b.storage_location.base_path().join(&b.id));

        let initial_len = self.boards.len();
        self.boards.retain(|b| b.id != id);

        if self.boards.len() != initial_len {
            // Delete the entire board directory (includes board.json and files/)
            if let Some(dir) = board_dir {
                let _ = fs::remove_dir_all(dir);
            }
            self.save();
            true
        } else {
            false
        }
    }

    /// Empty trash - permanently delete all trashed boards
    pub fn empty_trash(&mut self) -> usize {
        let trashed: Vec<_> = self.boards.iter()
            .filter(|b| b.is_deleted())
            .map(|b| (b.id.clone(), b.storage_location.base_path().join(&b.id)))
            .collect();

        let count = trashed.len();

        for (_, dir) in &trashed {
            let _ = fs::remove_dir_all(dir);
        }

        self.boards.retain(|b| !b.is_deleted());

        if count > 0 {
            self.save();
        }

        count
    }

    /// Auto-purge boards deleted more than 30 days ago
    pub fn purge_old_trash(&mut self) -> usize {
        let to_purge: Vec<_> = self.boards.iter()
            .filter(|b| b.should_purge())
            .map(|b| (b.id.clone(), b.storage_location.base_path().join(&b.id)))
            .collect();

        let count = to_purge.len();

        for (_, dir) in &to_purge {
            let _ = fs::remove_dir_all(dir);
        }

        self.boards.retain(|b| !b.should_purge());

        if count > 0 {
            tracing::info!("Purged {} boards from trash (older than 30 days)", count);
            self.save();
        }

        count
    }

    /// Get all active (non-deleted) boards
    pub fn active_boards(&self) -> Vec<&BoardMetadata> {
        self.boards.iter().filter(|b| !b.is_deleted()).collect()
    }

    /// Get all trashed boards
    pub fn trashed_boards(&self) -> Vec<&BoardMetadata> {
        self.boards.iter().filter(|b| b.is_deleted()).collect()
    }

    pub fn touch_board(&mut self, id: &str) {
        if let Some(board) = self.boards.iter_mut().find(|b| b.id == id) {
            board.touch();
            // Move to front of list
            let board = board.clone();
            self.boards.retain(|b| b.id != id);
            self.boards.insert(0, board);
            self.save();
        }
    }

    pub fn get_board(&self, id: &str) -> Option<&BoardMetadata> {
        self.boards.iter().find(|b| b.id == id)
    }

    /// Get the board path for a specific board ID (uses the board's storage location)
    pub fn get_board_path(&self, id: &str) -> Option<PathBuf> {
        self.get_board(id).map(|b| b.board_path())
    }

    /// Get the files directory for a specific board ID
    pub fn get_board_files_dir(&self, id: &str) -> Option<PathBuf> {
        self.get_board(id).map(|b| b.files_dir())
    }

    fn index_path() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("humanboard")
            .join("index.json")
    }

    pub fn boards_dir() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("humanboard")
            .join("boards")
    }

    /// Get the directory for a specific board (contains board.json and files/)
    pub fn board_dir(id: &str) -> PathBuf {
        Self::boards_dir().join(id)
    }

    pub fn board_path(id: &str) -> PathBuf {
        Self::board_dir(id).join("board.json")
    }

    /// Get the files directory for a specific board (for markdown notes, etc.)
    pub fn board_files_dir(id: &str) -> PathBuf {
        Self::board_dir(id).join("files")
    }

    fn legacy_board_path() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("humanboard")
            .join("board.json")
    }

    fn migrate_legacy() -> Self {
        let legacy_path = Self::legacy_board_path();

        if legacy_path.exists() {
            // Create a new board entry for the legacy board
            let metadata = BoardMetadata::new("My Board".to_string());

            // Move the legacy file to the new location
            let new_path = Self::board_path(&metadata.id);
            if let Some(parent) = new_path.parent() {
                let _ = fs::create_dir_all(parent);
            }

            if fs::rename(&legacy_path, &new_path).is_ok() {
                let index = BoardIndex {
                    boards: vec![metadata],
                };
                index.save();
                return index;
            }
        }

        // No legacy board or migration failed - start fresh
        BoardIndex::default()
    }
}

/// Simple UUID generation without external dependency
fn generate_uuid() -> String {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};

    let state = RandomState::new();
    let mut hasher = state.build_hasher();

    // Hash current time and some random state
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    hasher.write_u128(now.as_nanos());

    let hash1 = hasher.finish();

    let state2 = RandomState::new();
    let mut hasher2 = state2.build_hasher();
    hasher2.write_u64(hash1);
    let hash2 = hasher2.finish();

    format!("{:016x}{:016x}", hash1, hash2)
}
