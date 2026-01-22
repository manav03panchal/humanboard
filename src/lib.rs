//! Humanboard - A Miro-style infinite canvas moodboard application
//!
//! This crate provides the core functionality for the Humanboard application,
//! an infinite canvas tool for organizing images, videos, PDFs, markdown,
//! code files, and other media.
//!
//! ## Architecture
//!
//! The application follows patterns inspired by Zed editor:
//! - **GPUI Framework**: GPU-accelerated UI with reactive state management
//! - **Focus Management**: Priority-based focus contexts for keyboard handling
//! - **Settings System**: Layered configuration with hot-reloading
//! - **Action System**: Type-safe commands bound to keyboard shortcuts

#![recursion_limit = "512"]

// Re-export common error handling types
pub use anyhow::{Context, Result};

pub mod actions;
pub mod animations;
pub mod app;
pub mod background;
pub mod constants;
pub mod data_table;
pub mod audio_webview;
pub mod board;
pub mod data;
pub mod board_index;
pub mod command_palette;
pub mod command_registry;
pub mod error;
pub mod focus;
pub mod focus_ring;
pub mod hit_testing;
pub mod home;
pub mod input;
pub mod landing;
pub mod loading;
pub mod markdown_card;
pub mod notifications;
pub mod onboarding;
pub mod pdf_thumbnail;
pub mod pdf_webview;
pub mod perf;
pub mod preview;
pub mod render;
pub mod selection;
pub mod settings;
pub mod spatial_index;
pub mod settings_watcher;
pub mod types;
pub mod validation;
pub mod video_webview;
pub mod youtube_webview;
