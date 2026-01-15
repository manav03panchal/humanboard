//! Error types for Humanboard
//!
//! This module defines application-specific error types using thiserror,
//! following Zed's pattern of structured error handling.

use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during board operations
#[derive(Error, Debug)]
pub enum BoardError {
    #[error("Board not found: {0}")]
    NotFound(String),

    #[error("Failed to load board from {path}: {source}")]
    LoadFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to save board to {path}: {source}")]
    SaveFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Invalid board data: {0}")]
    InvalidData(String),

    #[error("Failed to parse board JSON: {0}")]
    ParseError(#[from] serde_json::Error),
}

/// Errors that can occur during settings operations
#[derive(Error, Debug)]
pub enum SettingsError {
    #[error("Failed to load settings from {path}: {source}")]
    LoadFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to save settings to {path}: {source}")]
    SaveFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to parse settings in {path}: {message}")]
    ParseFailed { path: PathBuf, message: String },

    #[error("Settings file not found at {0}")]
    NotFound(PathBuf),

    #[error("Invalid settings value: {0}")]
    InvalidValue(String),

    #[error("Invalid settings source: {0}")]
    InvalidSource(String),

    #[error("Settings lock poisoned: {0}")]
    LockPoisoned(String),
}

/// Errors that can occur with media operations
#[derive(Error, Debug)]
pub enum MediaError {
    #[error("Unsupported file type: {0}")]
    UnsupportedType(String),

    #[error("Failed to read file {path}: {source}")]
    ReadFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to process image: {0}")]
    ImageError(#[from] image::ImageError),

    #[error("Failed to create webview: {0}")]
    WebviewError(String),

    #[error("PDF rendering failed: {0}")]
    PdfError(String),
}

/// Errors that can occur during UI operations
#[derive(Error, Debug)]
pub enum UiError {
    #[error("Failed to open window: {0}")]
    WindowError(String),

    #[error("Focus error: {0}")]
    FocusError(String),

    #[error("Render error: {0}")]
    RenderError(String),
}

/// A Result type alias using anyhow::Error for convenience
pub type HumanboardResult<T> = anyhow::Result<T>;

/// Extension trait for adding context to Results
pub trait ResultExt<T, E> {
    /// Add context to an error, converting it to anyhow::Error
    fn with_context_str(self, context: &str) -> anyhow::Result<T>;
}

impl<T, E: std::error::Error + Send + Sync + 'static> ResultExt<T, E> for std::result::Result<T, E> {
    fn with_context_str(self, context: &str) -> anyhow::Result<T> {
        self.map_err(|e| anyhow::anyhow!("{}: {}", context, e))
    }
}
