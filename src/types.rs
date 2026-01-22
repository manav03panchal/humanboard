//! Core types for the Humanboard canvas system.
//!
//! This module defines the fundamental data structures used throughout the application,
//! including canvas items, content types, and helper functions for content detection.

use crate::pdf_thumbnail::generate_pdf_thumbnail;
use image::GenericImageView;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// ============================================================================
// Data Visualization Types
// ============================================================================

/// A data source that can be shared between tables and charts.
/// Stored in the Board's data_sources HashMap, referenced by ID.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataSource {
    /// Unique identifier
    pub id: u64,
    /// Human-readable name (from filename or user-defined)
    pub name: String,
    /// Column definitions
    pub columns: Vec<DataColumn>,
    /// Data rows
    pub rows: Vec<DataRow>,
    /// Where this data came from (for refresh capability)
    pub origin: DataOrigin,
    /// Whether this data has unsaved changes (not serialized)
    #[serde(skip)]
    pub dirty: bool,
}

impl DataSource {
    /// Create an empty data source for manual entry
    pub fn new_empty(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            columns: vec![
                DataColumn::new("Column A", DataType::Text),
                DataColumn::new("Column B", DataType::Text),
                DataColumn::new("Column C", DataType::Text),
            ],
            rows: vec![
                DataRow::new(vec![
                    DataCell::Empty,
                    DataCell::Empty,
                    DataCell::Empty,
                ]),
                DataRow::new(vec![
                    DataCell::Empty,
                    DataCell::Empty,
                    DataCell::Empty,
                ]),
                DataRow::new(vec![
                    DataCell::Empty,
                    DataCell::Empty,
                    DataCell::Empty,
                ]),
            ],
            origin: DataOrigin::Manual,
            dirty: false,
        }
    }

    /// Check if this data source has a file origin (can be saved back)
    pub fn has_file_origin(&self) -> bool {
        matches!(
            &self.origin,
            DataOrigin::File { .. } | DataOrigin::Json { path: Some(_) }
        )
    }

    /// Get the file path if this data source has a file origin
    pub fn file_path(&self) -> Option<&std::path::Path> {
        match &self.origin {
            DataOrigin::File { path, .. } => Some(path),
            DataOrigin::Json { path: Some(p) } => Some(p),
            _ => None,
        }
    }

    /// Mark this data source as dirty (has unsaved changes)
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// Mark this data source as clean (changes saved)
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    /// Check if this data source has unsaved changes
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Get the number of rows
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Get the number of columns
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// Add a new row at the end
    pub fn add_row(&mut self) {
        let cells = (0..self.columns.len())
            .map(|_| DataCell::Empty)
            .collect();
        self.rows.push(DataRow::new(cells));
    }

    /// Add a new column at the end
    pub fn add_column(&mut self, name: String, data_type: DataType) {
        self.columns.push(DataColumn::new(&name, data_type));
        for row in &mut self.rows {
            row.cells.push(DataCell::Empty);
        }
    }
}

/// Column metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataColumn {
    /// Column name/header
    pub name: String,
    /// Data type for this column
    pub data_type: DataType,
    /// Optional width for rendering (in pixels at zoom 1.0)
    pub width: Option<f32>,
}

impl DataColumn {
    pub fn new(name: &str, data_type: DataType) -> Self {
        Self {
            name: name.to_string(),
            data_type,
            width: None,
        }
    }
}

/// Supported data types for cells
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataType {
    Text,
    Number,
    Boolean,
    Date,
}

impl Default for DataType {
    fn default() -> Self {
        Self::Text
    }
}

/// A row of data cells
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataRow {
    pub cells: Vec<DataCell>,
}

impl DataRow {
    pub fn new(cells: Vec<DataCell>) -> Self {
        Self { cells }
    }
}

/// A single cell value
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DataCell {
    Text(String),
    Number(f64),
    Boolean(bool),
    Date(String), // ISO 8601 format
    Empty,
}

impl DataCell {
    /// Convert cell to string representation
    pub fn to_string(&self) -> String {
        match self {
            DataCell::Text(s) => s.clone(),
            DataCell::Number(n) => {
                // Format nicely: no trailing zeros for whole numbers
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            DataCell::Boolean(b) => if *b { "true" } else { "false" }.to_string(),
            DataCell::Date(d) => d.clone(),
            DataCell::Empty => String::new(),
        }
    }

    /// Try to convert cell to f64 (for charts)
    pub fn to_f64(&self) -> f64 {
        match self {
            DataCell::Number(n) => *n,
            DataCell::Text(s) => s.parse().unwrap_or(0.0),
            DataCell::Boolean(b) => if *b { 1.0 } else { 0.0 },
            _ => 0.0,
        }
    }

    /// Parse a string into a DataCell, trying to preserve type
    pub fn parse(value: &str, hint: &DataType) -> Self {
        if value.is_empty() {
            return DataCell::Empty;
        }

        match hint {
            DataType::Number => value
                .parse::<f64>()
                .map(DataCell::Number)
                .unwrap_or(DataCell::Text(value.to_string())),
            DataType::Boolean => match value.to_lowercase().as_str() {
                "true" | "yes" | "1" => DataCell::Boolean(true),
                "false" | "no" | "0" => DataCell::Boolean(false),
                _ => DataCell::Text(value.to_string()),
            },
            DataType::Date => DataCell::Date(value.to_string()),
            DataType::Text => DataCell::Text(value.to_string()),
        }
    }
}

/// Origin of a data source (for refresh capability)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DataOrigin {
    /// Manually entered data
    Manual,
    /// Imported from CSV/TSV file
    File {
        path: PathBuf,
        delimiter: char,
    },
    /// Loaded from JSON file
    Json {
        path: Option<PathBuf>,
    },
    /// Fetched from API URL
    Api {
        url: String,
        last_fetched: Option<u64>,
    },
}

// ============================================================================
// Chart Types
// ============================================================================

/// Chart configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChartConfig {
    /// Type of chart to render
    pub chart_type: ChartType,
    /// Column index for X axis (labels)
    pub x_column: Option<usize>,
    /// Column indices for Y axis (values)
    pub y_columns: Vec<usize>,
    /// Optional chart title
    pub title: Option<String>,
    /// Whether to show legend
    pub show_legend: bool,
    /// How to aggregate Y values when X has duplicates
    pub aggregation: AggregationType,
    /// Sort order for the chart data
    pub sort_order: SortOrder,
}

impl Default for ChartConfig {
    fn default() -> Self {
        Self {
            chart_type: ChartType::Bar,
            x_column: Some(0),
            y_columns: vec![1],
            title: None,
            show_legend: true,
            aggregation: AggregationType::default(),
            sort_order: SortOrder::default(),
        }
    }
}

impl ChartConfig {
    pub fn new(chart_type: ChartType) -> Self {
        Self {
            chart_type,
            ..Default::default()
        }
    }

    pub fn with_columns(mut self, x: usize, y: Vec<usize>) -> Self {
        self.x_column = Some(x);
        self.y_columns = y;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_aggregation(mut self, aggregation: AggregationType) -> Self {
        self.aggregation = aggregation;
        self
    }

    pub fn with_sort_order(mut self, sort_order: SortOrder) -> Self {
        self.sort_order = sort_order;
        self
    }
}

/// Types of charts available
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChartType {
    Line,
    #[default]
    Bar,
    Area,
    Pie,
    Scatter,
}

impl ChartType {
    pub fn label(&self) -> &'static str {
        match self {
            ChartType::Line => "Line",
            ChartType::Bar => "Bar",
            ChartType::Area => "Area",
            ChartType::Pie => "Pie",
            ChartType::Scatter => "Scatter",
        }
    }

    pub fn all() -> &'static [ChartType] {
        &[
            ChartType::Bar,
            ChartType::Line,
            ChartType::Area,
            ChartType::Pie,
            ChartType::Scatter,
        ]
    }
}

/// Aggregation method for grouping duplicate X values
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum AggregationType {
    /// No aggregation - show raw values (may have duplicates)
    None,
    /// Sum values for each group
    #[default]
    Sum,
    /// Average values for each group
    Average,
    /// Count occurrences in each group
    Count,
    /// Minimum value in each group
    Min,
    /// Maximum value in each group
    Max,
}

impl AggregationType {
    pub fn label(&self) -> &'static str {
        match self {
            AggregationType::None => "None",
            AggregationType::Sum => "Sum",
            AggregationType::Average => "Average",
            AggregationType::Count => "Count",
            AggregationType::Min => "Min",
            AggregationType::Max => "Max",
        }
    }

    pub fn all() -> &'static [AggregationType] {
        &[
            AggregationType::Sum,
            AggregationType::Average,
            AggregationType::Count,
            AggregationType::Min,
            AggregationType::Max,
            AggregationType::None,
        ]
    }
}

/// Sort order for chart data
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortOrder {
    /// No sorting - keep original order
    #[default]
    None,
    /// Sort by X axis (label) ascending (A-Z, 0-9)
    LabelAsc,
    /// Sort by X axis (label) descending (Z-A, 9-0)
    LabelDesc,
    /// Sort by Y axis (value) ascending (low to high)
    ValueAsc,
    /// Sort by Y axis (value) descending (high to low)
    ValueDesc,
}

impl SortOrder {
    pub fn label(&self) -> &'static str {
        match self {
            SortOrder::None => "Original",
            SortOrder::LabelAsc => "Label A→Z",
            SortOrder::LabelDesc => "Label Z→A",
            SortOrder::ValueAsc => "Value ↑",
            SortOrder::ValueDesc => "Value ↓",
        }
    }

    pub fn all() -> &'static [SortOrder] {
        &[
            SortOrder::None,
            SortOrder::LabelAsc,
            SortOrder::LabelDesc,
            SortOrder::ValueAsc,
            SortOrder::ValueDesc,
        ]
    }
}

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
    Table,
    Chart,
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
    /// A data table with editable cells
    Table {
        /// Reference to the shared data source
        data_source_id: u64,
        /// Whether to show column headers
        show_headers: bool,
        /// Whether to stripe alternating rows
        stripe: bool,
    },
    /// A chart visualizing data from a data source
    Chart {
        /// Reference to the shared data source
        data_source_id: u64,
        /// ID of the source table item (for drawing connection lines)
        source_item_id: Option<u64>,
        /// Chart configuration (type, columns, styling)
        config: ChartConfig,
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
            ItemContent::Table { .. } => (400.0, 300.0), // Default table size
            ItemContent::Chart { .. } => (400.0, 300.0), // Default chart size
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
            ItemContent::Table { .. } => "Table".to_string(),
            ItemContent::Chart { config, .. } => format!("{} Chart", config.chart_type.label()),
        }
    }

    /// Returns true if this item should appear in search results
    pub fn is_searchable(&self) -> bool {
        !matches!(
            self,
            ItemContent::TextBox { .. }
                | ItemContent::Arrow { .. }
                | ItemContent::Shape { .. }
                | ItemContent::Chart { .. }
        )
        // Note: Table is now searchable - name comes from data source
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
            ItemContent::Table { .. } => "TABLE",
            ItemContent::Chart { config, .. } => match config.chart_type {
                ChartType::Line => "LINE",
                ChartType::Bar => "BAR",
                ChartType::Area => "AREA",
                ChartType::Pie => "PIE",
                ChartType::Scatter => "SCATTER",
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
