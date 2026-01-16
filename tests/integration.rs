//! Integration tests for Humanboard.
//!
//! These tests verify the interaction between multiple components
//! and test complete workflows end-to-end.

#[path = "integration/board_workflow_tests.rs"]
mod board_workflow_tests;

#[path = "integration/undo_redo_tests.rs"]
mod undo_redo_tests;

#[path = "integration/selection_tests.rs"]
mod selection_tests;

#[path = "integration/event_flow_tests.rs"]
mod event_flow_tests;

#[path = "integration/keyboard_nav_tests.rs"]
mod keyboard_nav_tests;

#[path = "integration/theming_tests.rs"]
mod theming_tests;

#[path = "integration/state_management_tests.rs"]
mod state_management_tests;
