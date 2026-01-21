//! Application module - the main Humanboard application state and logic.
//!
//! This module is organized into several submodules:
//! - `types` - Enums, structs, and type definitions
//! - `state` - The Humanboard struct definition
//! - `lifecycle` - Initialization and cleanup methods
//! - `board_management` - Board CRUD operations
//! - `settings_methods` - Theme, font, and settings management
//! - `command_palette_methods` - Command palette functionality
//! - `preview_core` - Core preview panel operations
//! - `preview_webviews` - YouTube, Audio, Video webview management
//! - `preview_tabs` - Tab close, drag, and history management
//! - `preview_panes` - Tab switching and pane split management
//! - `preview_search` - Find in file functionality
//! - `textbox` - Textbox editing and utility methods

mod types;
mod state;
mod lifecycle;
mod board_management;
mod settings_methods;
mod command_palette_methods;
mod preview_core;
mod preview_webviews;
mod preview_tabs;
mod preview_panes;
mod preview_search;
mod textbox;
mod error_recovery;
mod data_viz;
mod table_editing;

pub use types::*;
pub use state::{ChartConfigModal, Humanboard};
