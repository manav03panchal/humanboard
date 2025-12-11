//! Persistence module with debounced saving for optimal disk I/O.
//!
//! This module implements a smart saving strategy that batches multiple
//! state changes into a single disk write, dramatically reducing I/O
//! during interactive operations like dragging.

use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

use crate::types::CanvasItem;

/// Time to wait before actually writing to disk after the last change
const DEBOUNCE_DELAY_MS: u64 = 500;

/// Serializable board state for persistence
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BoardState {
    pub canvas_offset: (f32, f32),
    pub zoom: f32,
    pub items: Vec<CanvasItem>,
    pub next_item_id: u64,
}

impl BoardState {
    /// Compute a hash of the state for change detection
    pub fn compute_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        // Hash key fields
        self.canvas_offset.0.to_bits().hash(&mut hasher);
        self.canvas_offset.1.to_bits().hash(&mut hasher);
        self.zoom.to_bits().hash(&mut hasher);
        self.next_item_id.hash(&mut hasher);
        self.items.len().hash(&mut hasher);
        for item in &self.items {
            item.id.hash(&mut hasher);
            item.position.0.to_bits().hash(&mut hasher);
            item.position.1.to_bits().hash(&mut hasher);
            item.size.0.to_bits().hash(&mut hasher);
            item.size.1.to_bits().hash(&mut hasher);
        }
        hasher.finish()
    }
}

/// Message sent to the persistence worker thread
enum PersistMessage {
    Save(BoardState),
    Shutdown,
}

/// Handle to the debounced persistence system
pub struct PersistenceHandle {
    sender: Sender<PersistMessage>,
    shutdown_flag: Arc<AtomicBool>,
    last_hash: u64,
}

impl PersistenceHandle {
    /// Create a new persistence handle with a background worker thread
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let shutdown_flag = Arc::new(AtomicBool::new(false));
        let shutdown_clone = Arc::clone(&shutdown_flag);

        // Spawn background worker thread for debounced saving
        thread::spawn(move || {
            persistence_worker(receiver, shutdown_clone);
        });

        Self {
            sender,
            shutdown_flag,
            last_hash: 0,
        }
    }

    /// Queue a save operation (will be debounced)
    pub fn save(&mut self, state: &BoardState) {
        let new_hash = state.compute_hash();

        // Only send if state actually changed
        if new_hash != self.last_hash {
            self.last_hash = new_hash;
            let _ = self.sender.send(PersistMessage::Save(state.clone()));
        }
    }

    /// Force an immediate save (bypasses debouncing)
    pub fn save_immediate(state: &BoardState) {
        if let Ok(json) = serde_json::to_string_pretty(state) {
            let save_path = get_save_path();
            if let Some(parent) = save_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = fs::write(&save_path, json);
        }
    }

    /// Load state from disk
    pub fn load() -> Option<BoardState> {
        let save_path = get_save_path();
        if let Ok(json) = fs::read_to_string(&save_path) {
            serde_json::from_str(&json).ok()
        } else {
            None
        }
    }
}

impl Default for PersistenceHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for PersistenceHandle {
    fn drop(&mut self) {
        self.shutdown_flag.store(true, Ordering::SeqCst);
        let _ = self.sender.send(PersistMessage::Shutdown);
    }
}

/// Get the path to the save file
fn get_save_path() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("humanboard")
        .join("board.json")
}

/// Background worker that handles debounced saving
fn persistence_worker(receiver: Receiver<PersistMessage>, shutdown_flag: Arc<AtomicBool>) {
    let mut pending_state: Option<BoardState> = None;
    let mut last_save_request: Option<Instant> = None;

    loop {
        if shutdown_flag.load(Ordering::SeqCst) {
            // Save any pending state before shutting down
            if let Some(state) = pending_state.take() {
                do_save(&state);
            }
            break;
        }

        // Calculate timeout for next check
        let timeout = if pending_state.is_some() {
            Duration::from_millis(50) // Check frequently when we have pending saves
        } else {
            Duration::from_millis(100)
        };

        match receiver.recv_timeout(timeout) {
            Ok(PersistMessage::Save(state)) => {
                pending_state = Some(state);
                last_save_request = Some(Instant::now());
            }
            Ok(PersistMessage::Shutdown) => {
                if let Some(state) = pending_state.take() {
                    do_save(&state);
                }
                break;
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                // Check if we should perform debounced save
                if let (Some(state), Some(last_request)) = (&pending_state, last_save_request) {
                    if last_request.elapsed() >= Duration::from_millis(DEBOUNCE_DELAY_MS) {
                        do_save(state);
                        pending_state = None;
                        last_save_request = None;
                    }
                }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                // Channel closed, save and exit
                if let Some(state) = pending_state.take() {
                    do_save(&state);
                }
                break;
            }
        }
    }
}

/// Actually perform the save to disk
fn do_save(state: &BoardState) {
    if let Ok(json) = serde_json::to_string_pretty(state) {
        let save_path = get_save_path();
        if let Some(parent) = save_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Err(e) = fs::write(&save_path, &json) {
            eprintln!("Failed to save board state: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ItemContent;

    #[test]
    fn test_board_state_hash_changes_on_modification() {
        let state1 = BoardState {
            canvas_offset: (0.0, 0.0),
            zoom: 1.0,
            items: vec![],
            next_item_id: 0,
        };

        let state2 = BoardState {
            canvas_offset: (10.0, 0.0),
            zoom: 1.0,
            items: vec![],
            next_item_id: 0,
        };

        assert_ne!(state1.compute_hash(), state2.compute_hash());
    }

    #[test]
    fn test_board_state_hash_stable() {
        let state = BoardState {
            canvas_offset: (100.0, 200.0),
            zoom: 1.5,
            items: vec![CanvasItem {
                id: 1,
                position: (10.0, 20.0),
                size: (100.0, 100.0),
                content: ItemContent::Text("test".to_string()),
            }],
            next_item_id: 2,
        };

        let hash1 = state.compute_hash();
        let hash2 = state.compute_hash();
        assert_eq!(hash1, hash2);
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
}
