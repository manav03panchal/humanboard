//! UI Overlays - header bar, footer, shortcuts modal, command palette, settings
//!
//! This module contains all overlay UI elements that appear on top of the canvas:
//! - Header bar with navigation and command palette
//! - Footer bar with status info
//! - Keyboard shortcuts modal
//! - Command palette popup
//! - Settings modal
//! - Create board modal
//! - Chart configuration modal

mod chart_config;
mod command_palette;
mod create_board;
mod header;
mod header_palette;
mod modal_base;
mod settings;
mod settings_dropdowns;
mod shortcuts;

// Re-export all public items
pub use chart_config::render_chart_config_modal;
pub use command_palette::render_command_palette;
pub use create_board::render_create_board_modal;
pub use header::{render_footer_bar, render_header_bar};
pub use modal_base::{FontDropdownOpen, SettingsDropdown, ThemeDropdownOpen};
pub use settings::render_settings_modal;
pub use shortcuts::render_shortcuts_overlay;
