//! CSV and TSV file parsing
//!
//! Parses CSV/TSV files into DataSource structs with automatic type inference.

use crate::types::{DataCell, DataColumn, DataOrigin, DataRow, DataSource, DataType};
use std::path::PathBuf;

/// Parse a CSV or TSV file into a DataSource
///
/// Automatically detects delimiter based on file extension (.tsv uses tab)
/// or content analysis (whichever delimiter appears more frequently).
pub fn parse_csv_file(path: &PathBuf) -> Result<DataSource, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let delimiter = detect_delimiter(path, &content);
    parse_csv_content(&content, delimiter, Some(path.clone()))
}

/// Parse CSV/TSV content from a string
pub fn parse_csv_content(
    content: &str,
    delimiter: char,
    source_path: Option<PathBuf>,
) -> Result<DataSource, String> {
    let mut lines = content.lines().peekable();

    // Parse header row
    let header_line = lines.next().ok_or("Empty file")?;
    let headers: Vec<&str> = split_csv_line(header_line, delimiter);

    if headers.is_empty() {
        return Err("No columns found in header".to_string());
    }

    // Parse data rows
    let mut rows: Vec<Vec<String>> = Vec::new();
    for line in lines {
        if !line.trim().is_empty() {
            let cells: Vec<String> = split_csv_line(line, delimiter)
                .into_iter()
                .map(|s| s.to_string())
                .collect();
            rows.push(cells);
        }
    }

    // Infer column types from data
    let columns: Vec<DataColumn> = headers
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let data_type = infer_column_type(&rows, i);
            DataColumn {
                name: name.trim().to_string(),
                data_type,
                width: None,
            }
        })
        .collect();

    // Convert to typed cells
    let data_rows: Vec<DataRow> = rows
        .iter()
        .map(|row| {
            DataRow::new(
                row.iter()
                    .enumerate()
                    .map(|(i, cell)| {
                        let data_type = columns
                            .get(i)
                            .map(|c| &c.data_type)
                            .unwrap_or(&DataType::Text);
                        DataCell::parse(cell.trim(), data_type)
                    })
                    .collect(),
            )
        })
        .collect();

    let name = source_path
        .as_ref()
        .and_then(|p| p.file_stem())
        .and_then(|s| s.to_str())
        .unwrap_or("Data")
        .to_string();

    Ok(DataSource {
        id: 0, // Will be assigned by Board
        name,
        columns,
        rows: data_rows,
        origin: DataOrigin::File {
            path: source_path.unwrap_or_default(),
            delimiter,
        },
        dirty: false,
    })
}

/// Detect the delimiter to use for parsing
fn detect_delimiter(path: &PathBuf, content: &str) -> char {
    // Check file extension first
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        if ext.to_lowercase() == "tsv" {
            return '\t';
        }
    }

    // Count delimiters in first few lines to determine most likely
    let first_lines: String = content.lines().take(5).collect::<Vec<_>>().join("\n");

    let comma_count = first_lines.matches(',').count();
    let tab_count = first_lines.matches('\t').count();
    let semicolon_count = first_lines.matches(';').count();

    if tab_count > comma_count && tab_count > semicolon_count {
        '\t'
    } else if semicolon_count > comma_count {
        ';'
    } else {
        ','
    }
}

/// Split a CSV line respecting quoted fields
fn split_csv_line(line: &str, delimiter: char) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = 0;
    let mut in_quotes = false;
    let chars: Vec<char> = line.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        if c == '"' {
            in_quotes = !in_quotes;
        } else if c == delimiter && !in_quotes {
            let field = &line[start..byte_index(line, i)];
            result.push(unquote(field));
            start = byte_index(line, i + 1);
        }
    }

    // Add the last field
    if start <= line.len() {
        let field = &line[start..];
        result.push(unquote(field));
    }

    result
}

/// Get byte index for character position in string
fn byte_index(s: &str, char_index: usize) -> usize {
    s.char_indices()
        .nth(char_index)
        .map(|(i, _)| i)
        .unwrap_or(s.len())
}

/// Remove surrounding quotes from a field
fn unquote(s: &str) -> &str {
    let trimmed = s.trim();
    if trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2 {
        &trimmed[1..trimmed.len() - 1]
    } else {
        trimmed
    }
}

/// Infer the data type for a column by sampling values
fn infer_column_type(rows: &[Vec<String>], col_idx: usize) -> DataType {
    // Sample up to 100 rows for type inference
    let sample: Vec<&str> = rows
        .iter()
        .take(100)
        .filter_map(|r| r.get(col_idx).map(|s| s.as_str()))
        .filter(|s| !s.trim().is_empty())
        .collect();

    if sample.is_empty() {
        return DataType::Text;
    }

    // Check if all values are numbers
    let all_numbers = sample.iter().all(|s| {
        let trimmed = s.trim();
        trimmed.parse::<f64>().is_ok()
            || trimmed
                .replace(',', "")
                .replace('$', "")
                .replace('%', "")
                .parse::<f64>()
                .is_ok()
    });
    if all_numbers {
        return DataType::Number;
    }

    // Check if all values are booleans
    let all_bools = sample.iter().all(|s| {
        matches!(
            s.trim().to_lowercase().as_str(),
            "true" | "false" | "yes" | "no" | "1" | "0" | "y" | "n"
        )
    });
    if all_bools {
        return DataType::Boolean;
    }

    // Check if values look like dates (simple heuristic)
    let looks_like_dates = sample.iter().all(|s| {
        let trimmed = s.trim();
        // Common date patterns: YYYY-MM-DD, MM/DD/YYYY, DD/MM/YYYY
        (trimmed.contains('-') && trimmed.len() >= 8 && trimmed.len() <= 10)
            || (trimmed.contains('/') && trimmed.len() >= 8 && trimmed.len() <= 10)
    });
    if looks_like_dates {
        return DataType::Date;
    }

    DataType::Text
}

/// Check if a file path is a data file (CSV/TSV/JSON)
pub fn is_data_file(path: &PathBuf) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|ext| matches!(ext.to_lowercase().as_str(), "csv" | "tsv" | "json"))
        .unwrap_or(false)
}

/// Write a DataSource back to a CSV file
///
/// Preserves the original delimiter from the DataOrigin.
/// Returns the path written to, or an error message.
pub fn write_csv_file(data_source: &DataSource) -> Result<PathBuf, String> {
    let (path, delimiter) = match &data_source.origin {
        DataOrigin::File { path, delimiter } => (path.clone(), *delimiter),
        _ => return Err("Data source does not have a file origin".to_string()),
    };

    let content = write_csv_content(data_source, delimiter);
    std::fs::write(&path, content)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(path)
}

/// Convert a DataSource to CSV string content
pub fn write_csv_content(data_source: &DataSource, delimiter: char) -> String {
    let mut lines = Vec::new();

    // Write header row
    let headers: Vec<String> = data_source.columns
        .iter()
        .map(|col| quote_csv_field(&col.name, delimiter))
        .collect();
    lines.push(headers.join(&delimiter.to_string()));

    // Write data rows
    for row in &data_source.rows {
        let cells: Vec<String> = row.cells
            .iter()
            .map(|cell| quote_csv_field(&cell.to_string(), delimiter))
            .collect();
        lines.push(cells.join(&delimiter.to_string()));
    }

    lines.join("\n")
}

/// Quote a CSV field if necessary (contains delimiter, quotes, or newlines)
fn quote_csv_field(value: &str, delimiter: char) -> String {
    let needs_quoting = value.contains(delimiter)
        || value.contains('"')
        || value.contains('\n')
        || value.contains('\r');

    if needs_quoting {
        // Escape internal quotes by doubling them
        let escaped = value.replace('"', "\"\"");
        format!("\"{}\"", escaped)
    } else {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_csv() {
        let content = "Name,Age,Active\nAlice,30,true\nBob,25,false";
        let result = parse_csv_content(content, ',', None).unwrap();

        assert_eq!(result.columns.len(), 3);
        assert_eq!(result.columns[0].name, "Name");
        assert_eq!(result.columns[1].name, "Age");
        assert_eq!(result.columns[2].name, "Active");

        assert_eq!(result.rows.len(), 2);
    }

    #[test]
    fn test_type_inference() {
        let content = "Name,Score,Pass\nAlice,95.5,true\nBob,87.0,false";
        let result = parse_csv_content(content, ',', None).unwrap();

        assert_eq!(result.columns[0].data_type, DataType::Text);
        assert_eq!(result.columns[1].data_type, DataType::Number);
        assert_eq!(result.columns[2].data_type, DataType::Boolean);
    }

    #[test]
    fn test_quoted_fields() {
        let content = r#"Name,Description
"John Doe","A ""quoted"" value"
"Jane, Smith","Contains comma""#;
        let result = parse_csv_content(content, ',', None).unwrap();

        assert_eq!(result.rows.len(), 2);
    }

    #[test]
    fn test_write_csv_content() {
        let ds = DataSource {
            id: 1,
            name: "Test".to_string(),
            columns: vec![
                DataColumn::new("Name", DataType::Text),
                DataColumn::new("Age", DataType::Number),
            ],
            rows: vec![
                DataRow::new(vec![
                    DataCell::Text("Alice".to_string()),
                    DataCell::Number(30.0),
                ]),
                DataRow::new(vec![
                    DataCell::Text("Bob".to_string()),
                    DataCell::Number(25.0),
                ]),
            ],
            origin: DataOrigin::Manual,
            dirty: false,
        };

        let output = write_csv_content(&ds, ',');
        assert!(output.contains("Name,Age"));
        assert!(output.contains("Alice,30"));
        assert!(output.contains("Bob,25"));
    }

    #[test]
    fn test_quote_csv_field() {
        assert_eq!(quote_csv_field("simple", ','), "simple");
        assert_eq!(quote_csv_field("with,comma", ','), "\"with,comma\"");
        assert_eq!(quote_csv_field("with\"quote", ','), "\"with\"\"quote\"");
        assert_eq!(quote_csv_field("with\nnewline", ','), "\"with\nnewline\"");
    }

    #[test]
    fn test_roundtrip() {
        let original = "Name,Score\nAlice,95.5\nBob,87.0";
        let parsed = parse_csv_content(original, ',', None).unwrap();
        let written = write_csv_content(&parsed, ',');
        let reparsed = parse_csv_content(&written, ',', None).unwrap();

        assert_eq!(parsed.columns.len(), reparsed.columns.len());
        assert_eq!(parsed.rows.len(), reparsed.rows.len());
    }
}
