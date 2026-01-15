//! Core types for the Humanboard canvas system.
//!
//! This module defines the fundamental data structures used throughout the application,
//! including canvas items, content types, and helper functions for content detection.

use crate::pdf_thumbnail::generate_pdf_thumbnail;
use image::GenericImageView;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// An item placed on the infinite canvas.
///
/// Each canvas item has a unique ID, position, size, and content type.
/// Items can be images, videos, PDFs, text boxes, shapes, arrows, and more.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CanvasItem {
    /// Unique identifier for this item
    pub id: u64,
    /// Position on the canvas in canvas coordinates (x, y)
    pub position: (f32, f32),
    /// Size of the item in canvas units (width, height)
    pub size: (f32, f32),
    /// The content this item displays
    pub content: ItemContent,
}

/// Tool types for the Miro-style tool dock
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToolType {
    #[default]
    Select,
    Text,
    Arrow,
    Shape,
}

/// Shape types for the Shape tool
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShapeType {
    #[default]
    Rectangle,
    RoundedRect,
    Ellipse,
}

/// Arrow head styles
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArrowHead {
    None,
    #[default]
    Arrow,
    Diamond,
    Circle,
}

/// The content type of a canvas item.
///
/// Determines how the item is rendered and what interactions are available.
/// Each variant represents a different type of media or element.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ItemContent {
    /// An image file (PNG, JPEG, GIF, WebP, etc.)
    Image(PathBuf),
    /// Plain text content
    Text(String),
    /// A video file (MP4, MOV, WebM, etc.)
    Video(PathBuf),
    /// An audio file (MP3, WAV, OGG, etc.)
    Audio(PathBuf),
    /// A PDF document with optional thumbnail
    Pdf {
        /// Path to the PDF file
        path: PathBuf,
        /// Path to generated thumbnail image
        thumbnail: Option<PathBuf>,
    },
    /// A web link/URL
    Link(String),
    /// An embedded YouTube video (stores video ID)
    YouTube(String),
    /// A markdown document
    Markdown {
        /// Path to the markdown file
        path: PathBuf,
        /// Document title (extracted from first heading or filename)
        title: String,
        /// Full markdown content for preview
        content: String,
    },
    /// A source code file with syntax highlighting
    Code {
        /// Path to the code file
        path: PathBuf,
        /// Language identifier for syntax highlighting (e.g., "rust", "python")
        language: String,
    },
    /// Editable text box (Miro-style)
    TextBox {
        /// The text content
        text: String,
        /// Font size in points
        font_size: f32,
        /// Text color as hex string (e.g., "#ffffff")
        color: String,
    },
    /// Arrow/line connecting points
    Arrow {
        /// Relative offset from item position to end point
        end_offset: (f32, f32),
        /// Arrow color as hex string
        color: String,
        /// Line thickness in pixels
        thickness: f32,
        /// Style of the arrow head
        head_style: ArrowHead,
    },
    /// Shape with optional fill and border
    Shape {
        /// The type of shape to render
        shape_type: ShapeType,
        /// Optional fill color as hex string
        fill_color: Option<String>,
        /// Border color as hex string
        border_color: String,
        /// Border width in pixels
        border_width: f32,
    },
}

/// Get the language identifier for syntax highlighting from file extension
pub fn language_from_extension(ext: &str) -> Option<&'static str> {
    match ext.to_lowercase().as_str() {
        "rs" => Some("rust"),
        "py" => Some("python"),
        "js" => Some("javascript"),
        "ts" => Some("typescript"),
        "jsx" => Some("javascript"),
        "tsx" => Some("typescript"),
        "go" => Some("go"),
        "c" => Some("c"),
        "h" => Some("c"),
        "cpp" | "cc" | "cxx" => Some("cpp"),
        "hpp" | "hxx" => Some("cpp"),
        "java" => Some("java"),
        "kt" | "kts" => Some("kotlin"),
        "swift" => Some("swift"),
        "rb" => Some("ruby"),
        "php" => Some("php"),
        "cs" => Some("csharp"),
        "fs" | "fsx" => Some("fsharp"),
        "scala" => Some("scala"),
        "lua" => Some("lua"),
        "sh" | "bash" | "zsh" => Some("bash"),
        "ps1" => Some("powershell"),
        "sql" => Some("sql"),
        "html" | "htm" => Some("html"),
        "css" => Some("css"),
        "scss" | "sass" => Some("scss"),
        "less" => Some("less"),
        "json" => Some("json"),
        "yaml" | "yml" => Some("yaml"),
        "toml" => Some("toml"),
        "xml" => Some("xml"),
        "vue" => Some("vue"),
        "svelte" => Some("svelte"),
        "zig" => Some("zig"),
        "nim" => Some("nim"),
        "ex" | "exs" => Some("elixir"),
        "erl" | "hrl" => Some("erlang"),
        "hs" => Some("haskell"),
        "ml" | "mli" => Some("ocaml"),
        "clj" | "cljs" => Some("clojure"),
        "lisp" | "cl" => Some("lisp"),
        "r" => Some("r"),
        "jl" => Some("julia"),
        "dart" => Some("dart"),
        "v" => Some("v"),
        "asm" | "s" => Some("asm"),
        "dockerfile" => Some("dockerfile"),
        "makefile" | "mk" => Some("makefile"),
        _ => None,
    }
}

/// Extract YouTube video ID from various URL formats
pub fn extract_youtube_id(url: &str) -> Option<String> {
    // Handle youtu.be/VIDEO_ID
    if url.contains("youtu.be/") {
        return url
            .split("youtu.be/")
            .nth(1)
            .and_then(|s| s.split(['?', '&', '#']).next())
            .map(|s| s.to_string());
    }

    // Handle youtube.com/watch?v=VIDEO_ID
    if url.contains("youtube.com/watch") {
        return url
            .split("v=")
            .nth(1)
            .and_then(|s| s.split(['&', '#']).next())
            .map(|s| s.to_string());
    }

    // Handle youtube.com/embed/VIDEO_ID
    if url.contains("youtube.com/embed/") {
        return url
            .split("youtube.com/embed/")
            .nth(1)
            .and_then(|s| s.split(['?', '&', '#']).next())
            .map(|s| s.to_string());
    }

    None
}

impl ItemContent {
    pub fn default_size(&self) -> (f32, f32) {
        match self {
            ItemContent::Image(path) => {
                // Try to load the image and get its actual dimensions, scaled to max 800px
                if let Ok(img) = image::open(path) {
                    let (width, height) = img.dimensions();
                    let max_dimension = 800.0;

                    let aspect_ratio = width as f32 / height as f32;

                    if width > height {
                        if width as f32 > max_dimension {
                            (max_dimension, max_dimension / aspect_ratio)
                        } else {
                            (width as f32, height as f32)
                        }
                    } else {
                        if height as f32 > max_dimension {
                            (max_dimension * aspect_ratio, max_dimension)
                        } else {
                            (width as f32, height as f32)
                        }
                    }
                } else {
                    (800.0, 600.0)
                }
            }
            ItemContent::Text(_) => (300.0, 100.0),
            ItemContent::Video(_) => (400.0, 300.0),
            ItemContent::Audio(_) => (320.0, 160.0), // Compact audio player
            ItemContent::Pdf { .. } => (180.0, 240.0),
            ItemContent::Link(_) => (300.0, 150.0),
            ItemContent::YouTube(_) => (560.0, 315.0), // 16:9 aspect ratio
            ItemContent::Markdown { .. } => (200.0, 36.0), // Simple filename button
            ItemContent::Code { .. } => (200.0, 36.0), // Simple filename button like markdown
            ItemContent::TextBox { .. } => (200.0, 100.0), // Default text box size
            ItemContent::Arrow { end_offset, .. } => {
                // Size based on arrow length
                let w = end_offset.0.abs().max(50.0);
                let h = end_offset.1.abs().max(20.0);
                (w, h)
            }
            ItemContent::Shape { .. } => (150.0, 100.0), // Default shape size
        }
    }

    pub fn display_name(&self) -> String {
        match self {
            ItemContent::Image(path) | ItemContent::Video(path) | ItemContent::Audio(path) => path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string(),
            ItemContent::Pdf { path, .. } => path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string(),
            ItemContent::Text(text) => text.clone(),
            ItemContent::Link(url) => url.clone(),
            ItemContent::YouTube(id) => format!("YouTube: {}", id),
            ItemContent::Markdown { title, .. } => title.clone(),
            ItemContent::Code { path, .. } => path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string(),
            ItemContent::TextBox { .. } => "TextBox".to_string(),
            ItemContent::Arrow { .. } => "Arrow".to_string(),
            ItemContent::Shape { shape_type, .. } => match shape_type {
                ShapeType::Rectangle => "Rectangle".to_string(),
                ShapeType::RoundedRect => "Rounded Rect".to_string(),
                ShapeType::Ellipse => "Ellipse".to_string(),
            },
        }
    }

    /// Returns true if this item should appear in search results
    pub fn is_searchable(&self) -> bool {
        !matches!(
            self,
            ItemContent::TextBox { .. } | ItemContent::Arrow { .. } | ItemContent::Shape { .. }
        )
    }

    pub fn type_label(&self) -> &str {
        match self {
            ItemContent::Image(_) => "IMAGE",
            ItemContent::Video(_) => "VIDEO",
            ItemContent::Audio(_) => "AUDIO",
            ItemContent::Pdf { .. } => "PDF",
            ItemContent::Text(_) => "TEXT",
            ItemContent::Link(_) => "LINK",
            ItemContent::YouTube(_) => "YOUTUBE",
            ItemContent::Markdown { .. } => "MARKDOWN",
            ItemContent::Code { language, .. } => match language.as_str() {
                "rust" => "RUST",
                "python" => "PYTHON",
                "javascript" | "typescript" => "JS/TS",
                "go" => "GO",
                "c" | "cpp" => "C/C++",
                "java" => "JAVA",
                "swift" => "SWIFT",
                "ruby" => "RUBY",
                "php" => "PHP",
                "html" => "HTML",
                "css" | "scss" => "CSS",
                "json" => "JSON",
                "yaml" => "YAML",
                "toml" => "TOML",
                "bash" => "SHELL",
                _ => "CODE",
            },
            ItemContent::TextBox { .. } => "TEXT",
            ItemContent::Arrow { .. } => "ARROW",
            ItemContent::Shape { shape_type, .. } => match shape_type {
                ShapeType::Rectangle => "RECT",
                ShapeType::RoundedRect => "RRECT",
                ShapeType::Ellipse => "ELLIPSE",
            },
        }
    }

    pub fn from_path(path: &PathBuf) -> Self {
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            match ext.to_lowercase().as_str() {
                "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "svg" => {
                    ItemContent::Image(path.clone())
                }
                "mp4" | "mov" | "avi" | "webm" | "mkv" => ItemContent::Video(path.clone()),
                "mp3" | "wav" | "ogg" | "m4a" | "aac" | "flac" => ItemContent::Audio(path.clone()),
                "pdf" => {
                    let thumbnail = generate_pdf_thumbnail(path);
                    ItemContent::Pdf {
                        path: path.clone(),
                        thumbnail,
                    }
                }
                "md" => {
                    let title = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("Untitled")
                        .to_string();
                    let content = std::fs::read_to_string(path).unwrap_or_default();
                    ItemContent::Markdown {
                        path: path.clone(),
                        title,
                        content,
                    }
                }
                ext if language_from_extension(ext).is_some() => {
                    // Safe to use unwrap_or here since we already checked is_some()
                    let language = language_from_extension(ext)
                        .unwrap_or("text")
                        .to_string();
                    ItemContent::Code {
                        path: path.clone(),
                        language,
                    }
                }
                "txt" => {
                    let name = path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| "Untitled".to_string());
                    ItemContent::Text(name)
                }
                _ => {
                    let name = path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| "Untitled".to_string());
                    ItemContent::Text(name)
                }
            }
        } else {
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "Untitled".to_string());
            ItemContent::Text(name)
        }
    }
}
