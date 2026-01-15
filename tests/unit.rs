//! Unit tests for Humanboard.

#[path = "unit/background_tests.rs"]
mod background_tests;

#[path = "unit/board_index_tests.rs"]
mod board_index_tests;

#[path = "unit/command_registry_tests.rs"]
mod command_registry_tests;

#[path = "unit/focus_tests.rs"]
mod focus_tests;

#[path = "unit/hit_testing_tests.rs"]
mod hit_testing_tests;

#[path = "unit/loading_tests.rs"]
mod loading_tests;

#[path = "unit/notifications_tests.rs"]
mod notifications_tests;

#[path = "unit/perf_tests.rs"]
mod perf_tests;

#[path = "unit/selection_tests.rs"]
mod selection_tests;

#[path = "unit/settings_watcher_tests.rs"]
mod settings_watcher_tests;

#[path = "unit/types_tests.rs"]
mod types_tests;

#[path = "unit/validation_tests.rs"]
mod validation_tests;
