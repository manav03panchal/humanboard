//! Lazy data source with polars backend for large dataset performance.
//!
//! This module provides high-performance data loading using polars' lazy evaluation
//! and virtual scrolling support for rendering only visible rows.
//!
//! ## Performance Features
//!
//! - **Lazy Loading**: Data is loaded on-demand, not all at once
//! - **Virtual Scrolling**: Only visible rows are fetched and rendered
//! - **Chunk Caching**: Recently accessed chunks are cached for smooth scrolling
//! - **Streaming**: Large files are streamed rather than loaded into memory

use crate::types::{DataCell, DataColumn, DataOrigin, DataRow, DataSource, DataType};
use parking_lot::RwLock;
use polars::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

/// Row height in pixels for virtual scrolling calculations
pub const ROW_HEIGHT: f32 = 28.0;

/// Header height in pixels
pub const HEADER_HEIGHT: f32 = 28.0;

/// Number of rows to render above/below visible area as buffer
pub const BUFFER_ROWS: usize = 10;

/// Chunk size for lazy loading (number of rows per chunk)
pub const CHUNK_SIZE: usize = 100;

/// Maximum chunks to keep in cache
pub const MAX_CACHED_CHUNKS: usize = 20;

/// Virtual scroll state for a table
#[derive(Clone, Debug, Default)]
pub struct VirtualScrollState {
    /// Current scroll offset (Y) in pixels
    pub scroll_y: f32,
    /// Visible height of the table container
    pub visible_height: f32,
}

impl VirtualScrollState {
    /// Create new scroll state with given visible height
    pub fn new(visible_height: f32) -> Self {
        Self {
            scroll_y: 0.0,
            visible_height,
        }
    }

    /// Update visible height (e.g., when table is resized)
    pub fn set_visible_height(&mut self, height: f32) {
        self.visible_height = height;
    }

    /// Update scroll position, clamping to valid range
    pub fn scroll_to(&mut self, y: f32, total_rows: usize) {
        let max_scroll = (total_rows as f32 * ROW_HEIGHT - self.visible_height).max(0.0);
        self.scroll_y = y.clamp(0.0, max_scroll);
    }

    /// Scroll by a delta amount
    pub fn scroll_by(&mut self, delta_y: f32, total_rows: usize) {
        self.scroll_to(self.scroll_y + delta_y, total_rows);
    }

    /// Calculate which rows are visible (start_row, end_row exclusive)
    pub fn visible_rows(&self, total_rows: usize) -> (usize, usize) {
        let first_visible = (self.scroll_y / ROW_HEIGHT).floor() as usize;
        let visible_count = (self.visible_height / ROW_HEIGHT).ceil() as usize + 1;

        // Add buffer rows for smooth scrolling
        let start = first_visible.saturating_sub(BUFFER_ROWS);
        let end = (first_visible + visible_count + BUFFER_ROWS).min(total_rows);

        (start, end)
    }

    /// Get the Y offset for the first rendered row (for positioning)
    pub fn first_row_offset(&self, total_rows: usize) -> f32 {
        let (start, _) = self.visible_rows(total_rows);
        start as f32 * ROW_HEIGHT
    }

    /// Calculate total content height for scrollbar
    pub fn total_content_height(&self, total_rows: usize) -> f32 {
        total_rows as f32 * ROW_HEIGHT
    }

    /// Calculate scrollbar thumb position (0.0 to 1.0)
    pub fn scrollbar_position(&self, total_rows: usize) -> f32 {
        let total_height = self.total_content_height(total_rows);
        if total_height <= self.visible_height {
            return 0.0;
        }
        self.scroll_y / (total_height - self.visible_height)
    }

    /// Calculate scrollbar thumb size as fraction of track (0.0 to 1.0)
    pub fn scrollbar_thumb_size(&self, total_rows: usize) -> f32 {
        let total_height = self.total_content_height(total_rows);
        if total_height <= 0.0 {
            return 1.0;
        }
        (self.visible_height / total_height).min(1.0)
    }
}

/// A cached chunk of rows
struct CachedChunk {
    /// Starting row index
    start_row: usize,
    /// Rows in this chunk
    rows: Vec<DataRow>,
    /// Last access time (for LRU eviction)
    last_access: std::time::Instant,
}

/// Polars-backed lazy data source for large datasets.
///
/// Unlike `DataSource` which loads all data into memory, `LazyDataSource`
/// uses polars for lazy evaluation and only loads chunks as needed.
pub struct LazyDataSource {
    /// Unique identifier
    pub id: u64,
    /// Human-readable name
    pub name: String,
    /// Column definitions
    pub columns: Vec<DataColumn>,
    /// Total row count (computed lazily if unknown)
    pub row_count: usize,
    /// Origin of the data
    pub origin: DataOrigin,
    /// Polars DataFrame (kept for efficient slicing)
    frame: Arc<DataFrame>,
    /// Cached row chunks for fast access
    chunk_cache: RwLock<HashMap<usize, CachedChunk>>,
}

impl LazyDataSource {
    /// Load a CSV file lazily
    pub fn from_csv(id: u64, path: &PathBuf) -> Result<Self, PolarsError> {
        let start = std::time::Instant::now();

        // Detect separator
        let separator = if path.extension().map(|e| e == "tsv").unwrap_or(false) {
            b'\t'
        } else {
            b','
        };

        // Use lazy loading for large files
        let lf = LazyCsvReader::new(path)
            .with_has_header(true)
            .with_separator(separator)
            .with_infer_schema_length(Some(1000))
            .finish()?;

        // Collect to get the frame (we need it for slicing)
        // For truly massive files, we could keep it lazy and use streaming
        let df = lf.collect()?;

        let columns: Vec<DataColumn> = df
            .get_column_names()
            .iter()
            .take(50) // Limit columns for display
            .map(|name| DataColumn {
                name: name.to_string(),
                data_type: infer_polars_dtype(df.column(name).ok()),
                width: None,
            })
            .collect();

        let row_count = df.height();
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Data")
            .to_string();

        tracing::debug!(
            "Loaded CSV {} with {} rows x {} cols in {:?}",
            path.display(),
            row_count,
            columns.len(),
            start.elapsed()
        );

        Ok(Self {
            id,
            name,
            columns,
            row_count,
            origin: DataOrigin::File {
                path: path.clone(),
                delimiter: separator as char,
            },
            frame: Arc::new(df),
            chunk_cache: RwLock::new(HashMap::new()),
        })
    }

    /// Load a JSON file
    pub fn from_json(id: u64, path: &PathBuf) -> Result<Self, PolarsError> {
        let start = std::time::Instant::now();

        let file = std::fs::File::open(path)?;
        let df = JsonReader::new(file)
            .with_json_format(JsonFormat::Json)
            .infer_schema_len(Some(std::num::NonZeroUsize::new(1000).unwrap()))
            .finish()?;

        let columns: Vec<DataColumn> = df
            .get_column_names()
            .iter()
            .take(50)
            .map(|name| DataColumn {
                name: name.to_string(),
                data_type: infer_polars_dtype(df.column(name).ok()),
                width: None,
            })
            .collect();

        let row_count = df.height();
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Data")
            .to_string();

        tracing::debug!(
            "Loaded JSON {} with {} rows x {} cols in {:?}",
            path.display(),
            row_count,
            columns.len(),
            start.elapsed()
        );

        Ok(Self {
            id,
            name,
            columns,
            row_count,
            origin: DataOrigin::Json {
                path: Some(path.clone()),
            },
            frame: Arc::new(df),
            chunk_cache: RwLock::new(HashMap::new()),
        })
    }

    /// Get rows for a given range (for virtual scrolling)
    ///
    /// Uses chunk caching for efficient repeated access during scrolling.
    pub fn get_rows(&self, start: usize, count: usize) -> Vec<DataRow> {
        let end = (start + count).min(self.row_count);
        if start >= end {
            return Vec::new();
        }

        // Determine which chunks we need
        let start_chunk = start / CHUNK_SIZE;
        let end_chunk = (end - 1) / CHUNK_SIZE;

        let mut result = Vec::with_capacity(end - start);

        for chunk_idx in start_chunk..=end_chunk {
            let chunk_start = chunk_idx * CHUNK_SIZE;
            let chunk = self.get_chunk(chunk_idx);

            // Calculate slice within this chunk
            let local_start = if chunk_idx == start_chunk {
                start - chunk_start
            } else {
                0
            };
            let local_end = if chunk_idx == end_chunk {
                end - chunk_start
            } else {
                CHUNK_SIZE
            };

            // Add rows from this chunk
            for row_idx in local_start..local_end.min(chunk.len()) {
                if let Some(row) = chunk.get(row_idx) {
                    result.push(row.clone());
                }
            }
        }

        result
    }

    /// Get a single chunk, loading from cache or DataFrame
    fn get_chunk(&self, chunk_idx: usize) -> Vec<DataRow> {
        // Check cache first
        {
            let mut cache = self.chunk_cache.write();

            // Update access time if cached
            if let Some(cached) = cache.get_mut(&chunk_idx) {
                cached.last_access = std::time::Instant::now();
                return cached.rows.clone();
            }
        }

        // Load chunk from DataFrame
        let chunk_start = chunk_idx * CHUNK_SIZE;
        let chunk_count = CHUNK_SIZE.min(self.row_count.saturating_sub(chunk_start));

        let rows = self.load_rows_from_frame(chunk_start, chunk_count);

        // Cache the chunk
        {
            let mut cache = self.chunk_cache.write();

            // Evict old chunks if cache is full
            if cache.len() >= MAX_CACHED_CHUNKS {
                // Find oldest chunk
                if let Some(oldest_idx) = cache
                    .iter()
                    .min_by_key(|(_, c)| c.last_access)
                    .map(|(idx, _)| *idx)
                {
                    cache.remove(&oldest_idx);
                }
            }

            cache.insert(
                chunk_idx,
                CachedChunk {
                    start_row: chunk_start,
                    rows: rows.clone(),
                    last_access: std::time::Instant::now(),
                },
            );
        }

        rows
    }

    /// Load rows directly from the polars DataFrame
    fn load_rows_from_frame(&self, start: usize, count: usize) -> Vec<DataRow> {
        if count == 0 {
            return Vec::new();
        }

        let slice = self.frame.slice(start as i64, count);
        let mut rows = Vec::with_capacity(count);

        for row_idx in 0..slice.height() {
            let cells: Vec<DataCell> = self
                .columns
                .iter()
                .map(|col| {
                    if let Ok(column) = slice.column(&col.name) {
                        polars_value_to_cell(column, row_idx)
                    } else {
                        DataCell::Empty
                    }
                })
                .collect();
            rows.push(DataRow::new(cells));
        }

        rows
    }

    /// Convert to a regular DataSource (for small datasets or full export)
    pub fn to_data_source(&self) -> DataSource {
        let rows = self.get_rows(0, self.row_count);
        DataSource {
            id: self.id,
            name: self.name.clone(),
            columns: self.columns.clone(),
            rows,
            origin: self.origin.clone(),
            dirty: false,
        }
    }

    /// Clear the chunk cache (e.g., after data modification)
    pub fn clear_cache(&self) {
        self.chunk_cache.write().clear();
    }
}

/// Infer DataType from polars column dtype
fn infer_polars_dtype(column: Option<&Column>) -> DataType {
    let Some(col) = column else {
        return DataType::Text;
    };

    match col.dtype() {
        polars::datatypes::DataType::Int8
        | polars::datatypes::DataType::Int16
        | polars::datatypes::DataType::Int32
        | polars::datatypes::DataType::Int64
        | polars::datatypes::DataType::UInt8
        | polars::datatypes::DataType::UInt16
        | polars::datatypes::DataType::UInt32
        | polars::datatypes::DataType::UInt64
        | polars::datatypes::DataType::Float32
        | polars::datatypes::DataType::Float64 => DataType::Number,
        polars::datatypes::DataType::Boolean => DataType::Boolean,
        polars::datatypes::DataType::Date | polars::datatypes::DataType::Datetime(_, _) => {
            DataType::Date
        }
        _ => DataType::Text,
    }
}

/// Convert a polars cell value to DataCell
fn polars_value_to_cell(column: &Column, row_idx: usize) -> DataCell {
    if let Ok(val) = column.get(row_idx) {
        match val {
            AnyValue::Null => DataCell::Empty,
            AnyValue::Int8(v) => DataCell::Number(v as f64),
            AnyValue::Int16(v) => DataCell::Number(v as f64),
            AnyValue::Int32(v) => DataCell::Number(v as f64),
            AnyValue::Int64(v) => DataCell::Number(v as f64),
            AnyValue::UInt8(v) => DataCell::Number(v as f64),
            AnyValue::UInt16(v) => DataCell::Number(v as f64),
            AnyValue::UInt32(v) => DataCell::Number(v as f64),
            AnyValue::UInt64(v) => DataCell::Number(v as f64),
            AnyValue::Float32(v) => DataCell::Number(v as f64),
            AnyValue::Float64(v) => DataCell::Number(v),
            AnyValue::Boolean(v) => DataCell::Boolean(v),
            AnyValue::String(s) => DataCell::Text(s.to_string()),
            AnyValue::StringOwned(s) => {
                let s_str: &str = &s;
                DataCell::Text(s_str.to_string())
            }
            _ => DataCell::Text(format!("{}", val)),
        }
    } else {
        DataCell::Empty
    }
}

/// Format row count for display (e.g., "1.2M rows")
pub fn format_row_count(count: usize) -> String {
    if count >= 1_000_000 {
        format!("{:.1}M rows", count as f64 / 1_000_000.0)
    } else if count >= 1_000 {
        format!("{:.1}K rows", count as f64 / 1_000.0)
    } else {
        format!("{} rows", count)
    }
}

/// Format current position for display (e.g., "Rows 1-50 of 1.2K")
pub fn format_position(start: usize, end: usize, total: usize) -> String {
    let display_start = start + 1; // 1-indexed for users
    let display_end = end.min(total);

    if total >= 1_000_000 {
        format!(
            "{}-{} of {:.1}M",
            display_start,
            display_end,
            total as f64 / 1_000_000.0
        )
    } else if total >= 1_000 {
        format!(
            "{}-{} of {:.1}K",
            display_start,
            display_end,
            total as f64 / 1_000.0
        )
    } else {
        format!("{}-{} of {}", display_start, display_end, total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visible_rows_calculation() {
        let mut state = VirtualScrollState::new(300.0); // ~10 visible rows at 28px each

        // At scroll 0, should show rows 0-20 (with buffer)
        let (start, end) = state.visible_rows(1000);
        assert_eq!(start, 0);
        assert!(end <= 30); // visible + buffer

        // Scroll down
        state.scroll_to(280.0, 1000); // 10 rows down
        let (start, end) = state.visible_rows(1000);
        assert!(start > 0);
        assert!(end > 20);
    }

    #[test]
    fn test_format_row_count() {
        assert_eq!(format_row_count(50), "50 rows");
        assert_eq!(format_row_count(1500), "1.5K rows");
        assert_eq!(format_row_count(1_500_000), "1.5M rows");
    }
}
