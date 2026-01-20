//! Data visualization methods - creating charts from tables

use crate::app::Humanboard;
use crate::types::{ChartConfig, ChartType, ItemContent};
use gpui::*;

impl Humanboard {
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
