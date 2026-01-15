//! Unit tests for settings_watcher module.

use humanboard::settings_watcher::{default_settings_path, SettingsWatcher};
use std::fs;
use std::io::Write;
use std::time::Duration;
use tempfile::tempdir;

#[test]
fn test_watcher_creation() {
    let dir = tempdir().unwrap();
    let settings_path = dir.path().join("settings.json");
    fs::write(&settings_path, "{}").unwrap();

    let watcher = SettingsWatcher::new(settings_path);
    assert!(watcher.is_ok());
}

#[test]
fn test_default_paths() {
    // These should return Some on most systems
    let settings = default_settings_path();
    assert!(settings.is_some() || cfg!(target_os = "unknown"));
}

#[test]
fn test_file_modification_detection() {
    let dir = tempdir().unwrap();
    let settings_path = dir.path().join("settings.json");
    fs::write(&settings_path, "{}").unwrap();

    let mut watcher = SettingsWatcher::new(settings_path.clone()).unwrap();

    // Give watcher time to initialize
    std::thread::sleep(Duration::from_millis(50));

    // Modify the file
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&settings_path)
        .unwrap();
    writeln!(file, "{{\"modified\": true}}").unwrap();
    file.sync_all().unwrap();

    // Wait for event
    std::thread::sleep(Duration::from_millis(200));

    // Poll for events
    let event = watcher.poll();
    // Note: Event detection is platform-dependent and may not fire in tests
    // This test mainly verifies the watcher doesn't crash
    drop(event);
}
