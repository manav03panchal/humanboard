//! Table cell editing methods

use crate::app::Humanboard;
use crate::types::{DataCell, ItemContent};
use gpui::*;
use gpui_component::input::InputState;

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
            |this, input, event: &gpui_component::input::InputEvent, cx| {
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
}
