//! Humanboard - A high-performance moodboard/canvas application.
//!
//! This crate provides a fast, modular canvas application with:
//! - Debounced persistence for minimal disk I/O
//! - Spatial indexing for O(1) item lookup
//! - Efficient history management with Arc-based copy-on-write
//! - Modular rendering architecture
//!
//! ## Architecture
//!
//! The codebase is organized into focused modules:
//!
//! - [`app`]: Application state and main Humanboard struct
//! - [`board`]: Canvas board state management
//! - [`types`]: Core data types (CanvasItem, ItemContent)
//! - [`render`]: Modular UI rendering components
//! - [`input`]: Mouse and keyboard event handling
//! - [`actions`]: Action definitions and handlers
//! - [`persistence`]: Debounced saving system
//! - [`spatial`]: Grid-based spatial indexing
//! - [`history`]: Undo/redo with VecDeque
//!
//! ## Performance Features
//!
//! - **Debounced Saving**: Changes are batched and saved after 500ms of inactivity
//! - **Spatial Index**: Grid-based spatial index for O(1) average item lookup
//! - **Arc History**: Copy-on-write semantics for efficient undo/redo
//! - **VecDeque**: O(1) operations for history and frame time tracking

pub mod actions;
pub mod app;
pub mod board;
pub mod history;
pub mod input;
pub mod pdf;
pub mod pdf_thumbnail;
pub mod pdf_webview;
pub mod persistence;
pub mod render;
pub mod spatial;
pub mod types;
