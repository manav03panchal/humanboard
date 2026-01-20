//! Data parsing and handling module
//!
//! This module provides parsers for various data formats that can be
//! used to populate tables and charts on the canvas.

mod csv_parser;
mod json_parser;

pub use csv_parser::*;
pub use json_parser::*;
