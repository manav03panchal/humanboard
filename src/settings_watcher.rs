//! Settings file watcher for hot-reload.
//!
//! Watches the settings file and notifies when changes are detected,
//! allowing the application to reload settings without restart.

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher, Event, EventKind};
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::time::{Duration, Instant};
use tracing::{debug, error, info};

/// Debounce duration for file change events
const DEBOUNCE_MS: u64 = 100;

/// Events that can be received from the settings watcher.
#[derive(Debug, Clone)]
pub enum SettingsEvent {
    /// Settings file was modified
    Modified,
    /// Settings file was created
    Created,
    /// Settings file was deleted
    Deleted,
    /// An error occurred while watching
    Error(String),
}

/// Watches settings files for changes.
pub struct SettingsWatcher {
    /// The file system watcher
    _watcher: RecommendedWatcher,
    /// Receiver for file events
    event_rx: Receiver<SettingsEvent>,
    /// Last event time for debouncing
    last_event: Option<Instant>,
}

impl SettingsWatcher {
    /// Create a new settings watcher for the given path.
    ///
    /// The path can be a file or directory. If a directory, all files
    /// within it will be watched.
    pub fn new(path: PathBuf) -> Result<Self, notify::Error> {
        let (tx, rx) = mpsc::channel();
        let (event_tx, event_rx) = mpsc::channel();

        // Create a debouncing wrapper
        let debounce_tx = event_tx.clone();
        std::thread::spawn(move || {
            let mut last_time: Option<Instant> = None;

            while let Ok(event) = rx.recv() {
                let now = Instant::now();

                // Debounce: ignore events within DEBOUNCE_MS of each other
                let should_send = match last_time {
                    Some(t) => now.duration_since(t) > Duration::from_millis(DEBOUNCE_MS),
                    None => true,
                };

                if should_send {
                    last_time = Some(now);
                    if debounce_tx.send(event).is_err() {
                        break;
                    }
                }
            }
        });

        // Create the file watcher
        let event_tx_clone = event_tx.clone();
        let mut watcher = RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                match res {
                    Ok(event) => {
                        let settings_event = match event.kind {
                            EventKind::Modify(_) => Some(SettingsEvent::Modified),
                            EventKind::Create(_) => Some(SettingsEvent::Created),
                            EventKind::Remove(_) => Some(SettingsEvent::Deleted),
                            _ => None,
                        };

                        if let Some(e) = settings_event {
                            debug!("Settings file event: {:?}", e);
                            let _ = tx.send(e);
                        }
                    }
                    Err(e) => {
                        error!("Watch error: {:?}", e);
                        let _ = event_tx_clone.send(SettingsEvent::Error(e.to_string()));
                    }
                }
            },
            Config::default(),
        )?;

        // Start watching
        let watch_path = if path.is_file() {
            path.parent().unwrap_or(&path).to_path_buf()
        } else {
            path.clone()
        };

        watcher.watch(&watch_path, RecursiveMode::NonRecursive)?;
        info!("Watching settings at: {:?}", watch_path);

        Ok(Self {
            _watcher: watcher,
            event_rx,
            last_event: None,
        })
    }

    /// Check for pending settings events without blocking.
    ///
    /// Returns the most recent event if any are pending.
    pub fn poll(&mut self) -> Option<SettingsEvent> {
        let mut latest = None;

        // Drain all pending events, keeping the latest
        while let Ok(event) = self.event_rx.try_recv() {
            latest = Some(event);
        }

        if latest.is_some() {
            self.last_event = Some(Instant::now());
        }

        latest
    }

    /// Check if settings were recently changed (within the debounce window).
    pub fn recently_changed(&self) -> bool {
        self.last_event
            .map(|t| t.elapsed() < Duration::from_millis(DEBOUNCE_MS * 2))
            .unwrap_or(false)
    }
}

/// Get the default settings file path.
pub fn default_settings_path() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join("humanboard").join("settings.json"))
}

/// Get the themes directory path.
pub fn themes_dir_path() -> Option<PathBuf> {
    // Try local themes first
    let local = std::env::current_dir().ok().map(|p| p.join("themes"));
    if let Some(ref p) = local {
        if p.exists() {
            return local;
        }
    }

    // Fall back to config dir
    dirs::config_dir().map(|p| p.join("humanboard").join("themes"))
}
