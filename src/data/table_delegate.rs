//! TableDelegate implementation for DataSource.
//!
//! Bridges our DataSource type to gpui-component's Table.

use crate::focus::FocusContext;
use crate::types::{DataCell, DataSource};
use gpui::*;
use gpui_component::input::{Input, InputState};
use gpui_component::table::{Column, ColumnSort, TableDelegate, TableState};
use gpui_component::ActiveTheme;
use std::sync::Arc;

/// Delegate for rendering DataSource in a gpui-component Table.
pub struct DataSourceDelegate {
    /// The original data source (immutable reference)
    data_source: Arc<DataSource>,
    /// Sorted row indices (for virtual sorting without modifying original)
    sorted_indices: Vec<usize>,
    /// Columns derived from data source
    columns: Vec<Column>,
    /// Current sort column and direction
    current_sort: Option<(usize, ColumnSort)>,
    /// Currently editing cell (row, col)
    pub editing_cell: Option<(usize, usize)>,
    /// Input state for editing
    pub edit_input: Option<Entity<InputState>>,
    /// Callback when cell value changes
    pub on_cell_changed: Option<Box<dyn Fn(usize, usize, String) + 'static>>,
    /// Container width for column sizing
    container_width: f32,
}

impl DataSourceDelegate {
    /// Create a new delegate with a default container width
    pub fn new(data_source: Arc<DataSource>) -> Self {
        Self::with_width(data_source, 400.0) // Default width
    }

    /// Create a new delegate with a specific container width
    pub fn with_width(data_source: Arc<DataSource>, container_width: f32) -> Self {
        let row_count = data_source.rows.len();
        let sorted_indices: Vec<usize> = (0..row_count).collect();
        let col_count = data_source.columns.len();

        // Calculate even column widths to fill container
        // Account for: scrollbar (~12px), borders (2px), internal padding, resize handles
        // Using a more aggressive deduction to avoid extra column space
        let table_overhead = 4.0; // borders only (scrollbars hidden)
        let available_width = (container_width - table_overhead).max(100.0);
        let col_width = if col_count > 0 {
            (available_width / col_count as f32).max(60.0)
        } else {
            100.0
        };

        let columns = data_source
            .columns
            .iter()
            .enumerate()
            .map(|(i, col)| {
                Column::new(format!("col_{}", i), col.name.clone())
                    .width(px(col_width))
                    .sortable()
                    .resizable(true)
            })
            .collect();

        Self {
            data_source,
            sorted_indices,
            columns,
            current_sort: None,
            editing_cell: None,
            edit_input: None,
            on_cell_changed: None,
            container_width,
        }
    }

    /// Update container width and recalculate column widths
    pub fn set_container_width(&mut self, width: f32) {
        if (self.container_width - width).abs() < 1.0 {
            return; // No significant change
        }
        self.container_width = width;

        let col_count = self.columns.len();
        if col_count == 0 {
            return;
        }

        // Account for: scrollbar (~12px), borders (2px), internal padding
        let table_overhead = 4.0;
        let available_width = (width - table_overhead).max(100.0);
        let col_width = (available_width / col_count as f32).max(60.0);

        for col in &mut self.columns {
            col.width = px(col_width);
        }
    }

    /// Update the data source
    pub fn set_data_source(&mut self, data_source: Arc<DataSource>) {
        let row_count = data_source.rows.len();
        self.sorted_indices = (0..row_count).collect();
        self.current_sort = None;

        let col_count = data_source.columns.len();
        let table_overhead = 4.0;
        let available_width = (self.container_width - table_overhead).max(100.0);
        let col_width = if col_count > 0 {
            (available_width / col_count as f32).max(60.0)
        } else {
            100.0
        };

        self.columns = data_source
            .columns
            .iter()
            .enumerate()
            .map(|(i, col)| {
                Column::new(format!("col_{}", i), col.name.clone())
                    .width(px(col_width))
                    .sortable()
                    .resizable(true)
            })
            .collect();
        self.data_source = data_source;
    }

    /// Get the actual row index after sorting
    fn actual_row_index(&self, display_row: usize) -> usize {
        self.sorted_indices.get(display_row).copied().unwrap_or(display_row)
    }

    /// Sort rows by the given column
    fn sort_rows(&mut self, col_ix: usize, sort: ColumnSort) {
        let row_count = self.data_source.rows.len();
        self.sorted_indices = (0..row_count).collect();

        if matches!(sort, ColumnSort::Default) {
            self.current_sort = None;
            return;
        }

        self.current_sort = Some((col_ix, sort));

        // Sort indices based on cell values
        let data_source = &self.data_source;
        self.sorted_indices.sort_by(|&a, &b| {
            let cell_a = data_source.rows.get(a)
                .and_then(|r| r.cells.get(col_ix));
            let cell_b = data_source.rows.get(b)
                .and_then(|r| r.cells.get(col_ix));

            let cmp = match (cell_a, cell_b) {
                (Some(DataCell::Number(na)), Some(DataCell::Number(nb))) => {
                    na.partial_cmp(nb).unwrap_or(std::cmp::Ordering::Equal)
                }
                (Some(DataCell::Text(ta)), Some(DataCell::Text(tb))) => {
                    ta.to_lowercase().cmp(&tb.to_lowercase())
                }
                (Some(DataCell::Boolean(ba)), Some(DataCell::Boolean(bb))) => ba.cmp(bb),
                (Some(a), Some(b)) => a.to_string().cmp(&b.to_string()),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            };

            match sort {
                ColumnSort::Ascending => cmp,
                ColumnSort::Descending => cmp.reverse(),
                ColumnSort::Default => std::cmp::Ordering::Equal,
            }
        });
    }

    /// Get the data source reference
    pub fn data_source(&self) -> &Arc<DataSource> {
        &self.data_source
    }

    /// Check if the data source has unsaved changes
    pub fn is_dirty(&self) -> bool {
        self.data_source.is_dirty()
    }

    /// Set callback for cell changes
    pub fn on_cell_changed<F: Fn(usize, usize, String) + 'static>(mut self, f: F) -> Self {
        self.on_cell_changed = Some(Box::new(f));
        self
    }

    /// Start editing a cell
    pub fn start_editing(
        &mut self,
        display_row: usize,
        col: usize,
        window: &mut Window,
        cx: &mut Context<TableState<Self>>,
    ) {
        let actual_row = self.actual_row_index(display_row);
        let current_value = self
            .data_source
            .rows
            .get(actual_row)
            .and_then(|r| r.cells.get(col))
            .map(|c| c.to_string())
            .unwrap_or_default();

        let input = cx.new(|cx| InputState::new(window, cx).default_value(current_value));

        // Subscribe to input events
        cx.subscribe(&input, |state, _input, event: &gpui_component::input::InputEvent, cx| {
            match event {
                gpui_component::input::InputEvent::PressEnter { .. } => {
                    state.delegate_mut().finish_editing(cx);
                    cx.notify();
                }
                gpui_component::input::InputEvent::Blur => {
                    state.delegate_mut().finish_editing(cx);
                    cx.notify();
                }
                _ => {}
            }
        }).detach();

        // Focus the input
        let input_clone = input.clone();
        window.defer(cx, move |window, cx| {
            input_clone.update(cx, |state, cx| {
                state.focus(window, cx);
            });
        });

        self.editing_cell = Some((display_row, col));
        self.edit_input = Some(input);
    }

    /// Finish editing and save value
    pub fn finish_editing(&mut self, cx: &mut Context<TableState<Self>>) {
        if let (Some((display_row, col)), Some(input)) = (self.editing_cell.take(), self.edit_input.take())
        {
            let actual_row = self.actual_row_index(display_row);
            let new_value = input.read(cx).text().to_string();

            // Update the local data source copy
            let mut ds = (*self.data_source).clone();
            if let Some(row) = ds.rows.get_mut(actual_row) {
                if let Some(cell) = row.cells.get_mut(col) {
                    // Parse as number if possible, otherwise text
                    *cell = if let Ok(n) = new_value.parse::<f64>() {
                        DataCell::Number(n)
                    } else if new_value.is_empty() {
                        DataCell::Empty
                    } else {
                        DataCell::Text(new_value.clone())
                    };
                }
            }
            ds.mark_dirty();
            self.data_source = Arc::new(ds);

            // Call the callback to sync to board
            if let Some(ref callback) = self.on_cell_changed {
                callback(actual_row, col, new_value);
            }
        }
    }

    /// Cancel editing
    pub fn cancel_editing(&mut self) {
        self.editing_cell = None;
        self.edit_input = None;
    }
}

impl TableDelegate for DataSourceDelegate {
    fn columns_count(&self, _cx: &App) -> usize {
        self.data_source.columns.len()
    }

    fn rows_count(&self, _cx: &App) -> usize {
        self.data_source.rows.len()
    }

    fn column(&self, col_ix: usize, _cx: &App) -> &Column {
        &self.columns[col_ix]
    }

    fn perform_sort(
        &mut self,
        col_ix: usize,
        sort: ColumnSort,
        _window: &mut Window,
        _cx: &mut Context<TableState<Self>>,
    ) {
        // Update column sort state visually
        for (ix, col) in self.columns.iter_mut().enumerate() {
            if ix == col_ix {
                col.sort = Some(sort);
            } else {
                col.sort = Some(ColumnSort::Default);
            }
        }

        // Actually sort the data
        self.sort_rows(col_ix, sort);
    }

    fn render_td(
        &mut self,
        row_ix: usize,
        col_ix: usize,
        _window: &mut Window,
        cx: &mut Context<TableState<Self>>,
    ) -> impl IntoElement {
        let is_editing = self
            .editing_cell
            .map(|(r, c)| r == row_ix && c == col_ix)
            .unwrap_or(false);

        // Use sorted index to get the actual row
        let actual_row = self.actual_row_index(row_ix);
        let cell_value = self
            .data_source
            .rows
            .get(actual_row)
            .and_then(|r| r.cells.get(col_ix))
            .map(|c| c.to_string())
            .unwrap_or_default();

        let fg = cx.theme().foreground;
        let muted = cx.theme().muted_foreground;
        let selection_bg = cx.theme().selection;

        if is_editing {
            if let Some(input) = &self.edit_input {
                return div()
                    .size_full()
                    .key_context(FocusContext::KEY_PREVIEW)
                    .child(
                        Input::new(input)
                            .appearance(false)
                            .size_full()
                    )
                    .into_any_element();
            }
        }

        // Regular cell display with double-click to edit
        let is_empty = cell_value.is_empty();
        let display_value = if is_empty {
            "-".to_string()
        } else {
            cell_value
        };

        div()
            .id(ElementId::Name(format!("cell-{}-{}", row_ix, col_ix).into()))
            .size_full()
            .px_2()
            .flex()
            .items_center()
            .text_color(if is_empty { muted } else { fg })
            .overflow_x_hidden()
            .text_ellipsis()
            .hover(|s| s.bg(selection_bg.opacity(0.3)))
            .on_mouse_down(MouseButton::Left, cx.listener(move |state, event: &MouseDownEvent, window, cx| {
                if event.click_count == 2 {
                    // Double-click: start editing
                    state.delegate_mut().start_editing(row_ix, col_ix, window, cx);
                    cx.notify();
                }
            }))
            .child(display_value)
            .into_any_element()
    }

    fn render_tr(
        &mut self,
        row_ix: usize,
        _window: &mut Window,
        _cx: &mut Context<TableState<Self>>,
    ) -> Stateful<Div> {
        div().id(("row", row_ix))
    }

    fn render_empty(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<TableState<Self>>,
    ) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .text_color(cx.theme().muted_foreground)
            .child("No data")
            .into_any_element()
    }

    /// Override to fill remaining space instead of leaving empty column
    fn render_last_empty_col(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<TableState<Self>>,
    ) -> impl IntoElement {
        // Fill remaining space with flex-grow
        div().flex_1().h_full()
    }
}
