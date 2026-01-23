//! JSON data parsing
//!
//! Parses JSON arrays of objects into DataSource structs.

use crate::types::{DataCell, DataColumn, DataOrigin, DataRow, DataSource, DataType};
use serde_json::Value;
use std::path::PathBuf;

/// Parse a JSON file into a DataSource
pub fn parse_json_file(path: &PathBuf) -> Result<DataSource, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let mut ds = parse_json_content(&content)?;

    // Set name from filename
    ds.name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Data")
        .to_string();

    ds.origin = DataOrigin::Json {
        path: Some(path.clone()),
    };

    Ok(ds)
}

/// Parse JSON content from a string
pub fn parse_json_content(json: &str) -> Result<DataSource, String> {
    let value: Value =
        serde_json::from_str(json).map_err(|e| format!("Invalid JSON: {}", e))?;

    let array = extract_array(&value)?;

    if array.is_empty() {
        return Ok(DataSource {
            id: 0,
            name: "Data".to_string(),
            columns: vec![],
            rows: vec![],
            origin: DataOrigin::Json { path: None },
            dirty: false,
        });
    }

    // Extract columns from first object
    let first_obj = array[0]
        .as_object()
        .ok_or("Array elements must be objects")?;

    let column_names: Vec<String> = first_obj.keys().cloned().collect();

    let columns: Vec<DataColumn> = column_names
        .iter()
        .map(|name| DataColumn {
            name: name.clone(),
            data_type: infer_json_column_type(array, name),
            width: None,
        })
        .collect();

    // Convert to rows
    let rows: Vec<DataRow> = array
        .iter()
        .filter_map(|v| {
            let obj = v.as_object()?;
            let cells: Vec<DataCell> = columns
                .iter()
                .map(|col| {
                    obj.get(&col.name)
                        .map(|v| json_value_to_cell(v, &col.data_type))
                        .unwrap_or(DataCell::Empty)
                })
                .collect();
            Some(DataRow::new(cells))
        })
        .collect();

    Ok(DataSource {
        id: 0,
        name: "Data".to_string(),
        columns,
        rows,
        origin: DataOrigin::Json { path: None },
        dirty: false,
    })
}

/// Extract the array from JSON value, handling common wrapper patterns
fn extract_array(value: &Value) -> Result<&Vec<Value>, String> {
    match value {
        Value::Array(arr) => Ok(arr),
        Value::Object(obj) => {
            // Try common wrapper patterns: data, rows, items, records, results
            let wrapper_keys = ["data", "rows", "items", "records", "results"];
            for key in wrapper_keys {
                if let Some(Value::Array(arr)) = obj.get(key) {
                    return Ok(arr);
                }
            }
            Err("JSON must be an array or have a data/rows/items/records/results array".to_string())
        }
        _ => Err("JSON must be an array of objects".to_string()),
    }
}

/// Infer the data type for a column from JSON values
fn infer_json_column_type(array: &[Value], key: &str) -> DataType {
    for item in array.iter().take(100) {
        if let Some(obj) = item.as_object() {
            if let Some(value) = obj.get(key) {
                match value {
                    Value::Number(_) => return DataType::Number,
                    Value::Bool(_) => return DataType::Boolean,
                    Value::String(s) => {
                        // Check if it looks like a date
                        if looks_like_date(s) {
                            return DataType::Date;
                        }
                        // Check if it's a number string
                        if s.parse::<f64>().is_ok() {
                            return DataType::Number;
                        }
                        return DataType::Text;
                    }
                    Value::Null => continue,
                    _ => return DataType::Text,
                }
            }
        }
    }
    DataType::Text
}

/// Check if a string looks like a date
fn looks_like_date(s: &str) -> bool {
    // ISO date format: YYYY-MM-DD
    if s.len() == 10 && s.chars().filter(|&c| c == '-').count() == 2 {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() == 3
            && parts[0].len() == 4
            && parts[1].len() == 2
            && parts[2].len() == 2
        {
            return parts[0].parse::<u32>().is_ok()
                && parts[1].parse::<u32>().is_ok()
                && parts[2].parse::<u32>().is_ok();
        }
    }
    // ISO datetime format: starts with YYYY-MM-DD
    if s.len() > 10 && s.contains('T') {
        return looks_like_date(&s[..10]);
    }
    false
}

/// Convert a JSON value to a DataCell
fn json_value_to_cell(value: &Value, expected_type: &DataType) -> DataCell {
    match value {
        Value::Null => DataCell::Empty,
        Value::Bool(b) => DataCell::Boolean(*b),
        Value::Number(n) => DataCell::Number(n.as_f64().unwrap_or(0.0)),
        Value::String(s) => {
            if s.is_empty() {
                return DataCell::Empty;
            }
            match expected_type {
                DataType::Number => s
                    .parse::<f64>()
                    .map(DataCell::Number)
                    .unwrap_or(DataCell::Text(s.clone())),
                DataType::Boolean => match s.to_lowercase().as_str() {
                    "true" | "yes" | "1" => DataCell::Boolean(true),
                    "false" | "no" | "0" => DataCell::Boolean(false),
                    _ => DataCell::Text(s.clone()),
                },
                DataType::Date => DataCell::Date(s.clone()),
                DataType::Text => DataCell::Text(s.clone()),
            }
        }
        Value::Array(arr) => {
            // Convert arrays to string representation
            DataCell::Text(
                arr.iter()
                    .map(|v| format!("{}", v))
                    .collect::<Vec<_>>()
                    .join(", "),
            )
        }
        Value::Object(_) => {
            // Convert objects to JSON string
            DataCell::Text(value.to_string())
        }
    }
}

/// Parse JSON from a URL (for API data sources)
pub async fn fetch_json_from_url(url: &str) -> Result<DataSource, String> {
    // This is a placeholder - in practice you'd use reqwest or similar
    Err(format!("URL fetching not implemented yet: {}", url))
}

/// Write a DataSource back to a JSON file
///
/// Writes as an array of objects with pretty formatting.
/// Returns the path written to, or an error message.
pub fn write_json_file(data_source: &DataSource) -> Result<PathBuf, String> {
    let path = match &data_source.origin {
        DataOrigin::Json { path: Some(p) } => p.clone(),
        _ => return Err("Data source does not have a JSON file origin".to_string()),
    };

    let content = write_json_content(data_source);
    std::fs::write(&path, content)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(path)
}

/// Convert a DataSource to JSON string content (array of objects)
pub fn write_json_content(data_source: &DataSource) -> String {
    let mut array: Vec<serde_json::Map<String, Value>> = Vec::new();

    for row in &data_source.rows {
        let mut obj = serde_json::Map::new();
        for (col_idx, cell) in row.cells.iter().enumerate() {
            if let Some(col) = data_source.columns.get(col_idx) {
                let value = cell_to_json_value(cell);
                obj.insert(col.name.clone(), value);
            }
        }
        array.push(obj);
    }

    serde_json::to_string_pretty(&array).unwrap_or_else(|_| "[]".to_string())
}

/// Convert a DataCell to a JSON Value
fn cell_to_json_value(cell: &DataCell) -> Value {
    match cell {
        DataCell::Text(s) => Value::String(s.clone()),
        DataCell::Number(n) => {
            // Use integer if it's a whole number
            if n.fract() == 0.0 && *n >= i64::MIN as f64 && *n <= i64::MAX as f64 {
                Value::Number(serde_json::Number::from(*n as i64))
            } else {
                serde_json::Number::from_f64(*n)
                    .map(Value::Number)
                    .unwrap_or(Value::Null)
            }
        }
        DataCell::Boolean(b) => Value::Bool(*b),
        DataCell::Date(d) => Value::String(d.clone()),
        DataCell::Empty => Value::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_json_array() {
        let json = r#"[
            {"name": "Alice", "age": 30, "active": true},
            {"name": "Bob", "age": 25, "active": false}
        ]"#;

        let result = parse_json_content(json).unwrap();

        assert_eq!(result.columns.len(), 3);
        assert_eq!(result.rows.len(), 2);
    }

    #[test]
    fn test_parse_wrapped_json() {
        let json = r#"{
            "data": [
                {"id": 1, "value": 100},
                {"id": 2, "value": 200}
            ]
        }"#;

        let result = parse_json_content(json).unwrap();

        assert_eq!(result.columns.len(), 2);
        assert_eq!(result.rows.len(), 2);
    }

    #[test]
    fn test_type_inference() {
        let json = r#"[
            {"name": "Test", "count": 42, "enabled": true, "date": "2024-01-15"}
        ]"#;

        let result = parse_json_content(json).unwrap();

        // Find columns by name
        let name_col = result.columns.iter().find(|c| c.name == "name").unwrap();
        let count_col = result.columns.iter().find(|c| c.name == "count").unwrap();
        let enabled_col = result.columns.iter().find(|c| c.name == "enabled").unwrap();
        let date_col = result.columns.iter().find(|c| c.name == "date").unwrap();

        assert_eq!(name_col.data_type, DataType::Text);
        assert_eq!(count_col.data_type, DataType::Number);
        assert_eq!(enabled_col.data_type, DataType::Boolean);
        assert_eq!(date_col.data_type, DataType::Date);
    }

    #[test]
    fn test_empty_array() {
        let json = "[]";
        let result = parse_json_content(json).unwrap();

        assert_eq!(result.columns.len(), 0);
        assert_eq!(result.rows.len(), 0);
    }

    #[test]
    fn test_write_json_content() {
        let ds = DataSource {
            id: 1,
            name: "Test".to_string(),
            columns: vec![
                DataColumn::new("name", DataType::Text),
                DataColumn::new("age", DataType::Number),
                DataColumn::new("active", DataType::Boolean),
            ],
            rows: vec![
                DataRow::new(vec![
                    DataCell::Text("Alice".to_string()),
                    DataCell::Number(30.0),
                    DataCell::Boolean(true),
                ]),
            ],
            origin: DataOrigin::Manual,
            dirty: false,
        };

        let output = write_json_content(&ds);
        assert!(output.contains("\"name\": \"Alice\""));
        assert!(output.contains("\"age\": 30"));
        assert!(output.contains("\"active\": true"));
    }

    #[test]
    fn test_json_roundtrip() {
        let original = r#"[{"name": "Test", "value": 42}]"#;
        let parsed = parse_json_content(original).unwrap();
        let written = write_json_content(&parsed);
        let reparsed = parse_json_content(&written).unwrap();

        assert_eq!(parsed.columns.len(), reparsed.columns.len());
        assert_eq!(parsed.rows.len(), reparsed.rows.len());
    }
}
