//! Data visualization methods - creating charts from tables

use super::state::ChartConfigModal;
use crate::app::Humanboard;
use crate::types::{AggregationType, ChartConfig, ChartType, ItemContent, SortOrder};
use gpui::*;

impl Humanboard {
    /// Show the chart configuration modal for a table
    pub fn show_chart_config_modal(&mut self, table_item_id: u64, cx: &mut Context<Self>) {
        // Get the data source ID and column names from the table
        if let Some(ref board) = self.board {
            if let Some(item) = board.items.iter().find(|i| i.id == table_item_id) {
                if let ItemContent::Table { data_source_id, .. } = &item.content {
                    if let Some(ds) = board.data_sources.get(data_source_id) {
                        let column_names: Vec<String> = ds.columns.iter()
                            .map(|c| c.name.clone())
                            .collect();

                        self.chart_config_modal = Some(ChartConfigModal::new(
                            table_item_id,
                            *data_source_id,
                            column_names,
                        ));
                        cx.notify();
                    }
                }
            }
        }
    }

    /// Close the chart configuration modal
    pub fn close_chart_config_modal(&mut self, cx: &mut Context<Self>) {
        self.chart_config_modal = None;
        cx.notify();
    }

    /// Set the chart type in the config modal
    pub fn set_chart_config_type(&mut self, chart_type: ChartType, cx: &mut Context<Self>) {
        if let Some(ref mut modal) = self.chart_config_modal {
            modal.chart_type = chart_type;
            cx.notify();
        }
    }

    /// Set the X axis column in the config modal
    pub fn set_chart_config_x_column(&mut self, column: usize, cx: &mut Context<Self>) {
        if let Some(ref mut modal) = self.chart_config_modal {
            modal.x_column = column;
            cx.notify();
        }
    }

    /// Toggle a Y axis column in the config modal
    pub fn toggle_chart_config_y_column(&mut self, column: usize, cx: &mut Context<Self>) {
        if let Some(ref mut modal) = self.chart_config_modal {
            if modal.y_columns.contains(&column) {
                modal.y_columns.retain(|&c| c != column);
            } else {
                modal.y_columns.push(column);
            }
            // Ensure at least one Y column is selected
            if modal.y_columns.is_empty() {
                modal.y_columns.push(column);
            }
            cx.notify();
        }
    }

    /// Set the aggregation type in the config modal
    pub fn set_chart_config_aggregation(&mut self, aggregation: AggregationType, cx: &mut Context<Self>) {
        if let Some(ref mut modal) = self.chart_config_modal {
            modal.aggregation = aggregation;
            cx.notify();
        }
    }

    /// Set the sort order in the config modal
    pub fn set_chart_config_sort_order(&mut self, sort_order: SortOrder, cx: &mut Context<Self>) {
        if let Some(ref mut modal) = self.chart_config_modal {
            modal.sort_order = sort_order;
            cx.notify();
        }
    }

    /// Confirm and create the chart from the modal configuration
    pub fn confirm_chart_config(&mut self, cx: &mut Context<Self>) {
        if let Some(modal) = self.chart_config_modal.take() {
            // Create the chart with the configured settings
            let config = ChartConfig::new(modal.chart_type)
                .with_columns(modal.x_column, modal.y_columns)
                .with_aggregation(modal.aggregation)
                .with_sort_order(modal.sort_order);

            self.create_chart_from_table_with_config(
                modal.table_item_id,
                modal.data_source_id,
                config,
                cx,
            );
        }
    }

    /// Create a chart from a table with specific configuration
    fn create_chart_from_table_with_config(
        &mut self,
        table_item_id: u64,
        data_source_id: u64,
        config: ChartConfig,
        cx: &mut Context<Self>,
    ) {
        let chart_type_name = config.chart_type.label().to_string();

        if let Some(ref mut board) = self.board {
            // Find the table item to get its position
            if let Some(table_item) = board.items.iter().find(|i| i.id == table_item_id) {
                let chart_pos = (
                    table_item.position.0 + table_item.size.0 + 50.0,
                    table_item.position.1,
                );

                // Create a new chart item
                let chart_id = board.add_item(
                    point(px(chart_pos.0), px(chart_pos.1)),
                    ItemContent::Chart {
                        data_source_id,
                        config,
                        source_item_id: Some(table_item_id),
                    },
                );

                // Set chart size
                if let Some(item) = board.get_item_mut(chart_id) {
                    item.size = (400.0, 300.0);
                }

                // Update spatial index
                board.update_spatial_index(chart_id);

                // Select the new chart
                self.selected_items.clear();
                self.selected_items.insert(chart_id);

                // Save
                board.push_history();
                let _ = board.flush_save();

                self.show_toast(crate::notifications::Toast::success(
                    format!("Created {} chart", chart_type_name),
                ));
            }
        }
        cx.notify();
    }

    // =========================================================================
    // Data Source File Operations (Save/Reload)
    // =========================================================================

    /// Save a table's data source back to its original file
    pub fn save_table_to_file(&mut self, table_item_id: u64, cx: &mut Context<Self>) {
        // Get the data source ID from the table
        let data_source_id = if let Some(ref board) = self.board {
            board.items.iter()
                .find(|i| i.id == table_item_id)
                .and_then(|item| {
                    if let ItemContent::Table { data_source_id, .. } = &item.content {
                        Some(*data_source_id)
                    } else {
                        None
                    }
                })
        } else {
            None
        };

        let Some(ds_id) = data_source_id else {
            self.show_toast(crate::notifications::Toast::error("Table not found"));
            return;
        };

        // Save the data source
        if let Some(ref mut board) = self.board {
            match board.save_data_source_to_file(ds_id) {
                Ok(path) => {
                    let filename = path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("file");
                    self.show_toast(crate::notifications::Toast::success(
                        format!("Saved to {}", filename)
                    ));
                }
                Err(e) => {
                    self.show_toast(crate::notifications::Toast::error(e));
                }
            }
        }

        cx.notify();
    }

    /// Reload a table's data source from its original file
    pub fn reload_table_from_file(&mut self, table_item_id: u64, cx: &mut Context<Self>) {
        // Get the data source ID from the table
        let data_source_id = if let Some(ref board) = self.board {
            board.items.iter()
                .find(|i| i.id == table_item_id)
                .and_then(|item| {
                    if let ItemContent::Table { data_source_id, .. } = &item.content {
                        Some(*data_source_id)
                    } else {
                        None
                    }
                })
        } else {
            None
        };

        let Some(ds_id) = data_source_id else {
            self.show_toast(crate::notifications::Toast::error("Table not found"));
            return;
        };

        // Reload the data source
        if let Some(ref mut board) = self.board {
            match board.reload_data_source_from_file(ds_id) {
                Ok(()) => {
                    self.show_toast(crate::notifications::Toast::success("Reloaded from file"));
                }
                Err(e) => {
                    self.show_toast(crate::notifications::Toast::error(e));
                }
            }
        }

        cx.notify();
    }

    /// Check if a table's data source is dirty (has unsaved changes)
    pub fn is_table_dirty(&self, table_item_id: u64) -> bool {
        if let Some(ref board) = self.board {
            board.items.iter()
                .find(|i| i.id == table_item_id)
                .and_then(|item| {
                    if let ItemContent::Table { data_source_id, .. } = &item.content {
                        Some(board.is_data_source_dirty(*data_source_id))
                    } else {
                        None
                    }
                })
                .unwrap_or(false)
        } else {
            false
        }
    }

    /// Check if a table's data source can be saved to file
    pub fn can_save_table(&self, table_item_id: u64) -> bool {
        if let Some(ref board) = self.board {
            board.items.iter()
                .find(|i| i.id == table_item_id)
                .and_then(|item| {
                    if let ItemContent::Table { data_source_id, .. } = &item.content {
                        Some(board.can_save_data_source(*data_source_id))
                    } else {
                        None
                    }
                })
                .unwrap_or(false)
        } else {
            false
        }
    }

    /// Create a chart from an existing table item
    /// The chart will be positioned to the right of the table
    pub fn create_chart_from_table(&mut self, table_item_id: u64, chart_type: ChartType, cx: &mut Context<Self>) {
        let Some(ref mut board) = self.board else {
            return;
        };

        // Find the table item and get its data source ID
        let table_info = board.items.iter().find(|item| item.id == table_item_id).and_then(|item| {
            if let ItemContent::Table { data_source_id, .. } = &item.content {
                Some((item.position, item.size, *data_source_id))
            } else {
                None
            }
        });

        let Some((table_pos, table_size, data_source_id)) = table_info else {
            return;
        };

        // Position the chart to the right of the table with some gap
        let chart_x = table_pos.0 + table_size.0 + 50.0;
        let chart_y = table_pos.1;

        // Create the chart item linked to the same data source
        let chart_id = board.add_item(
            point(px(chart_x), px(chart_y)),
            ItemContent::Chart {
                data_source_id,
                source_item_id: Some(table_item_id),
                config: ChartConfig::new(chart_type),
            },
        );

        // Set chart size
        if let Some(item) = board.get_item_mut(chart_id) {
            item.size = (400.0, 300.0);
        }

        // Update spatial index
        board.update_spatial_index(chart_id);

        // Select the new chart
        self.selected_items.clear();
        self.selected_items.insert(chart_id);

        // Save
        board.push_history();
        let _ = board.flush_save();

        cx.notify();
    }
}
