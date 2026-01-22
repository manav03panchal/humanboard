//! Table cell editing and state management methods

use crate::app::Humanboard;
use crate::data::DataSourceDelegate;
use crate::types::{DataCell, ItemContent};
use gpui::*;
use gpui_component::input::InputState;
use gpui_component::table::TableState;
use std::sync::Arc;

impl Humanboard {
    /// Start editing a table cell
    pub fn start_table_cell_editing(
        &mut self,
        table_id: u64,
        row: usize,
        col: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Get current cell value
        let current_value = if let Some(ref board) = self.board {
            if let Some(item) = board.items.iter().find(|i| i.id == table_id) {
                if let ItemContent::Table { data_source_id, .. } = &item.content {
                    if let Some(ds) = board.data_sources.get(data_source_id) {
                        ds.rows.get(row)
                            .and_then(|r| r.cells.get(col))
                            .map(|c| c.to_string())
                            .unwrap_or_default()
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        // Create input state with current value
        let input_state = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(current_value.clone())
        });

        self.editing_table_cell = Some((table_id, row, col));
        self.table_cell_input = Some(input_state.clone());

        // Focus the input after it's mounted
        let input_clone = input_state.clone();
        window.defer(cx, move |window, cx| {
            input_clone.update(cx, |state, cx| {
                // Focus the input so user can type immediately
                state.focus(window, cx);
            });
        });

        // Subscribe to input events
        cx.subscribe(
            &input_state,
            |this, _input, event: &gpui_component::input::InputEvent, cx| {
                match event {
                    gpui_component::input::InputEvent::PressEnter { .. } => {
                        // Save on Enter
                        this.finish_table_cell_editing(cx);
                    }
                    gpui_component::input::InputEvent::Blur => {
                        // Save on blur (click outside)
                        this.finish_table_cell_editing(cx);
                    }
                    _ => {}
                }
            },
        )
        .detach();

        cx.notify();
    }

    /// Finish editing a table cell and save the value
    pub fn finish_table_cell_editing(&mut self, cx: &mut Context<Self>) {
        let Some((table_id, row, col)) = self.editing_table_cell.take() else {
            return;
        };

        let Some(input) = self.table_cell_input.take() else {
            return;
        };

        // Get the new value from the input
        let new_value = input.read(cx).text().to_string();

        // Update the data source
        if let Some(ref mut board) = self.board {
            // Find the data source ID from the table item
            let data_source_id = board.items.iter()
                .find(|i| i.id == table_id)
                .and_then(|item| {
                    if let ItemContent::Table { data_source_id, .. } = &item.content {
                        Some(*data_source_id)
                    } else {
                        None
                    }
                });

            if let Some(ds_id) = data_source_id {
                if let Some(ds) = board.data_sources.get_mut(&ds_id) {
                    // Update the cell
                    if let Some(data_row) = ds.rows.get_mut(row) {
                        if let Some(cell) = data_row.cells.get_mut(col) {
                            // Try to parse as number, otherwise keep as text
                            *cell = if let Ok(n) = new_value.parse::<f64>() {
                                DataCell::Number(n)
                            } else if new_value.is_empty() {
                                DataCell::Empty
                            } else {
                                DataCell::Text(new_value)
                            };
                        }
                    }
                    // Mark data source as dirty (has unsaved changes to file)
                    ds.mark_dirty();
                }

                // Mark as modified
                board.push_history();
                let _ = board.flush_save();
            }
        }

        cx.notify();
    }

    /// Cancel table cell editing without saving
    pub fn cancel_table_cell_editing(&mut self, cx: &mut Context<Self>) {
        self.editing_table_cell = None;
        self.table_cell_input = None;
        cx.notify();
    }

    /// Ensure TableState entities exist for all table items.
    /// Call this before rendering to ensure all tables have their state initialized.
    /// Takes zoom to calculate actual pixel widths for columns.
    pub fn ensure_table_states(&mut self, zoom: f32, window: &mut Window, cx: &mut Context<Self>) {
        let Some(ref board) = self.board else {
            return;
        };

        // Collect table items that need states (with their size for column width calculation)
        // Use actual pixel width (canvas size * zoom)
        let tables_needing_state: Vec<(u64, u64, f32)> = board
            .items
            .iter()
            .filter_map(|item| {
                if let ItemContent::Table { data_source_id, .. } = &item.content {
                    if !self.table_states.contains_key(&item.id) {
                        Some((item.id, *data_source_id, item.size.0 * zoom))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        // Create states for tables that need them
        for (table_id, data_source_id, pixel_width) in tables_needing_state {
            if let Some(ds) = board.data_sources.get(&data_source_id) {
                let delegate = DataSourceDelegate::with_width(Arc::new(ds.clone()), pixel_width);
                let state = cx.new(|cx| TableState::new(delegate, window, cx));
                self.table_states.insert(table_id, state);
            }
        }

        // Update column widths for existing tables if their size or zoom changed
        let existing_tables: Vec<(u64, f32)> = board
            .items
            .iter()
            .filter_map(|item| {
                if matches!(&item.content, ItemContent::Table { .. }) {
                    Some((item.id, item.size.0 * zoom))
                } else {
                    None
                }
            })
            .collect();

        for (table_id, pixel_width) in &existing_tables {
            if let Some(state) = self.table_states.get(table_id) {
                state.update(cx, |table_state, _cx| {
                    table_state.delegate_mut().set_container_width(*pixel_width);
                });
            }
        }

        // Clean up states for tables that no longer exist
        let existing_table_ids: std::collections::HashSet<u64> = existing_tables.iter().map(|(id, _)| *id).collect();
        self.table_states.retain(|id, _| existing_table_ids.contains(id));
    }

    /// Update the data source for a specific table's TableState.
    /// Call this when the underlying data changes.
    pub fn update_table_data(&mut self, table_id: u64, cx: &mut Context<Self>) {
        let Some(ref board) = self.board else {
            return;
        };

        // Find the table item and its data source
        let data_source = board.items.iter().find_map(|item| {
            if item.id == table_id {
                if let ItemContent::Table { data_source_id, .. } = &item.content {
                    board.data_sources.get(data_source_id).cloned()
                } else {
                    None
                }
            } else {
                None
            }
        });

        if let Some(ds) = data_source {
            if let Some(state) = self.table_states.get(&table_id) {
                state.update(cx, |table_state, _cx| {
                    table_state.delegate_mut().set_data_source(Arc::new(ds));
                });
            }
        }
    }
}
