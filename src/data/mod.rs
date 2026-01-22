//! Data parsing and handling module
//!
//! This module provides parsers for various data formats that can be
//! used to populate tables and charts on the canvas.
//!
//! ## Performance
//!
//! For large datasets (10k+ rows), use `LazyDataSource` with polars backend
//! which provides:
//! - Lazy evaluation (only loads what's needed)
//! - Virtual scrolling support
//! - Chunk caching for smooth scroll performance

mod csv_parser;
mod json_parser;
mod lazy_source;
mod table_delegate;

pub use csv_parser::*;
pub use json_parser::*;
pub use lazy_source::*;
pub use table_delegate::*;
