use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BoardMetadata {
    pub id: String,
    pub name: String,
    pub created_at: u64,
    pub updated_at: u64,
}

impl BoardMetadata {
    pub fn new(name: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id: generate_uuid(),
            name,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn touch(&mut self) {
        self.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
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

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct BoardIndex {
    pub boards: Vec<BoardMetadata>,
}

impl BoardIndex {
    pub fn load() -> Self {
        let index_path = Self::index_path();

        if let Ok(json) = fs::read_to_string(&index_path) {
            serde_json::from_str(&json).unwrap_or_default()
        } else {
            // Check for legacy board.json and migrate if exists
            Self::migrate_legacy()
        }
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
        let metadata = BoardMetadata::new(name);
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

    pub fn delete_board(&mut self, id: &str) -> bool {
        let initial_len = self.boards.len();
        self.boards.retain(|b| b.id != id);

        if self.boards.len() != initial_len {
            // Also delete the board file
            let board_path = Self::board_path(id);
            let _ = fs::remove_file(board_path);
            self.save();
            true
        } else {
            false
        }
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

    pub fn board_path(id: &str) -> PathBuf {
        Self::boards_dir().join(format!("{}.json", id))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_metadata_new() {
        let meta = BoardMetadata::new("Test Board".to_string());
        assert_eq!(meta.name, "Test Board");
        assert!(!meta.id.is_empty());
        assert!(meta.created_at > 0);
        assert_eq!(meta.created_at, meta.updated_at);
    }

    #[test]
    fn test_board_metadata_touch() {
        let mut meta = BoardMetadata::new("Test".to_string());
        let original = meta.updated_at;
        std::thread::sleep(std::time::Duration::from_millis(10));
        meta.touch();
        assert!(meta.updated_at >= original);
    }

    #[test]
    fn test_formatted_date_just_now() {
        let meta = BoardMetadata::new("Test".to_string());
        assert_eq!(meta.formatted_date(), "Just now");
    }

    #[test]
    fn test_generate_uuid_unique() {
        let id1 = generate_uuid();
        let id2 = generate_uuid();
        assert_ne!(id1, id2);
        assert_eq!(id1.len(), 32);
    }

    #[test]
    fn test_board_index_default() {
        let index = BoardIndex::default();
        assert!(index.boards.is_empty());
    }
}
