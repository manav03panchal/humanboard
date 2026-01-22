//! DataTable component with Excel/Sheets-like UX.
//!
//! Features:
//! - Collapse/expand toggle with row count badge
//! - Pagination controls (first/prev/next/last + page size selector)
//! - Sticky header row
//! - Alternating row colors
//!
//! ## Usage
//!
//! ```rust,ignore
//! let table_state = DataTableState::new(data.len());
//! render_data_table(
//!     "my-table",
//!     "Users",
//!     &["Name", "Email", "Status"],
//!     &data,
//!     &table_state,
//!     |row, col_idx| match col_idx {
//!         0 => row.name.clone(),
//!         1 => row.email.clone(),
//!         2 => row.status.clone(),
//!         _ => String::new(),
//!     },
//!     theme_colors,
//! )
//! ```

use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{Icon, IconName, h_flex, v_flex};

/// Page size options for pagination
pub const PAGE_SIZE_OPTIONS: &[usize] = &[10, 25, 50, 100];

/// State for a DataTable instance
#[derive(Clone, Debug)]
pub struct DataTableState {
    /// Whether the table is collapsed (header-only view)
    pub collapsed: bool,
    /// Current page (0-indexed)
    pub current_page: usize,
    /// Rows per page
    pub page_size: usize,
    /// Total number of rows
    pub total_rows: usize,
    /// Whether the page size dropdown is open
    pub page_size_dropdown_open: bool,
    /// Currently hovered row index (for hover states)
    pub hovered_row: Option<usize>,
}

impl DataTableState {
    pub fn new(total_rows: usize) -> Self {
        Self {
            collapsed: false,
            current_page: 0,
            page_size: 25,
            total_rows,
            page_size_dropdown_open: false,
            hovered_row: None,
        }
    }

    pub fn total_pages(&self) -> usize {
        if self.total_rows == 0 {
            1
        } else {
            (self.total_rows + self.page_size - 1) / self.page_size
        }
    }

    pub fn can_go_prev(&self) -> bool {
        self.current_page > 0
    }

    pub fn can_go_next(&self) -> bool {
        self.current_page < self.total_pages().saturating_sub(1)
    }

    pub fn go_first(&mut self) {
        self.current_page = 0;
    }

    pub fn go_prev(&mut self) {
        if self.can_go_prev() {
            self.current_page -= 1;
        }
    }

    pub fn go_next(&mut self) {
        if self.can_go_next() {
            self.current_page += 1;
        }
    }

    pub fn go_last(&mut self) {
        self.current_page = self.total_pages().saturating_sub(1);
    }

    pub fn set_page_size(&mut self, size: usize) {
        self.page_size = size;
        // Reset to first page when changing page size
        self.current_page = 0;
    }

    pub fn toggle_collapsed(&mut self) {
        self.collapsed = !self.collapsed;
    }

    pub fn toggle_page_size_dropdown(&mut self) {
        self.page_size_dropdown_open = !self.page_size_dropdown_open;
    }

    pub fn close_page_size_dropdown(&mut self) {
        self.page_size_dropdown_open = false;
    }

    pub fn set_hovered_row(&mut self, row: Option<usize>) {
        self.hovered_row = row;
    }

    /// Get the range of rows to display for current page
    pub fn visible_range(&self) -> std::ops::Range<usize> {
        let start = self.current_page * self.page_size;
        let end = (start + self.page_size).min(self.total_rows);
        start..end
    }
}

/// Theme colors for the data table
#[derive(Clone, Copy)]
pub struct DataTableColors {
    pub background: Hsla,
    pub header_bg: Hsla,
    pub alt_row_bg: Hsla,
    pub border: Hsla,
    pub text: Hsla,
    pub text_muted: Hsla,
    pub primary: Hsla,
    pub title_bar: Hsla,
}

impl DataTableColors {
    /// Create colors from a standard theme
    pub fn from_theme(
        background: Hsla,
        muted: Hsla,
        border: Hsla,
        foreground: Hsla,
        muted_foreground: Hsla,
        primary: Hsla,
        title_bar: Hsla,
    ) -> Self {
        Self {
            background,
            header_bg: muted,
            alt_row_bg: background.opacity(0.5),
            border,
            text: foreground,
            text_muted: muted_foreground,
            primary,
            title_bar,
        }
    }
}

/// Render the collapse/expand toggle header with row count badge
pub fn render_table_header(
    title: &str,
    state: &DataTableState,
    colors: &DataTableColors,
) -> Div {
    let chevron_icon = if state.collapsed {
        IconName::ChevronRight
    } else {
        IconName::ChevronDown
    };

    let row_count_text = format_row_count(state.total_rows);

    h_flex()
        .w_full()
        .h(px(40.0))
        .px_3()
        .bg(colors.background)
        .border_b_1()
        .border_color(colors.border)
        .items_center()
        .justify_between()
        .child(
            // Left side: collapse toggle + title + row count badge
            h_flex()
                .items_center()
                .gap_2()
                .cursor_pointer()
                .child(
                    Icon::new(chevron_icon)
                        .size(px(16.0))
                        .text_color(colors.text_muted),
                )
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(colors.text)
                        .child(title.to_string()),
                )
                .child(
                    // Row count badge
                    div()
                        .px_2()
                        .py(px(2.0))
                        .rounded(px(10.0))
                        .bg(colors.primary.opacity(0.15))
                        .text_xs()
                        .text_color(colors.primary)
                        .child(row_count_text),
                ),
        )
}

/// Render pagination controls (stateless - caller handles state)
pub fn render_pagination(
    state: &DataTableState,
    colors: &DataTableColors,
) -> Div {
    let current_page = state.current_page + 1; // 1-indexed for display
    let total_pages = state.total_pages();
    let range = state.visible_range();
    let showing_start = if state.total_rows == 0 { 0 } else { range.start + 1 };
    let showing_end = range.end;

    h_flex()
        .w_full()
        .h(px(36.0))
        .px_3()
        .bg(colors.title_bar)
        .border_t_1()
        .border_color(colors.border)
        .items_center()
        .justify_between()
        .child(
            // Left: showing X-Y of Z + page size selector
            h_flex()
                .items_center()
                .gap_3()
                .child(
                    div()
                        .text_xs()
                        .text_color(colors.text_muted)
                        .child(format!(
                            "Showing {}-{} of {}",
                            showing_start, showing_end, state.total_rows
                        )),
                )
                .child(render_page_size_selector(state, colors)),
        )
        .child(
            // Right: pagination buttons
            h_flex()
                .items_center()
                .gap_1()
                .child(
                    // First page (double left)
                    h_flex()
                        .id("pagination-first")
                        .items_center()
                        .px_1()
                        .py(px(2.0))
                        .rounded(px(4.0))
                        .cursor_pointer()
                        .when(!state.can_go_prev(), |d| d.opacity(0.3).cursor_default())
                        .when(state.can_go_prev(), |d| d.hover(|s| s.bg(colors.alt_row_bg)))
                        .child(Icon::new(IconName::ChevronLeft).size(px(10.0)).text_color(colors.text_muted))
                        .child(Icon::new(IconName::ChevronLeft).size(px(10.0)).text_color(colors.text_muted).ml(px(-4.0))),
                )
                .child(
                    // Previous page
                    h_flex()
                        .id("pagination-prev")
                        .items_center()
                        .px_1()
                        .py(px(2.0))
                        .rounded(px(4.0))
                        .cursor_pointer()
                        .when(!state.can_go_prev(), |d| d.opacity(0.3).cursor_default())
                        .when(state.can_go_prev(), |d| d.hover(|s| s.bg(colors.alt_row_bg)))
                        .child(Icon::new(IconName::ChevronLeft).size(px(14.0)).text_color(colors.text_muted)),
                )
                .child(
                    // Page indicator
                    div()
                        .px_2()
                        .text_xs()
                        .text_color(colors.text_muted)
                        .child(format!("{} / {}", current_page, total_pages)),
                )
                .child(
                    // Next page
                    h_flex()
                        .id("pagination-next")
                        .items_center()
                        .px_1()
                        .py(px(2.0))
                        .rounded(px(4.0))
                        .cursor_pointer()
                        .when(!state.can_go_next(), |d| d.opacity(0.3).cursor_default())
                        .when(state.can_go_next(), |d| d.hover(|s| s.bg(colors.alt_row_bg)))
                        .child(Icon::new(IconName::ChevronRight).size(px(14.0)).text_color(colors.text_muted)),
                )
                .child(
                    // Last page (double right)
                    h_flex()
                        .id("pagination-last")
                        .items_center()
                        .px_1()
                        .py(px(2.0))
                        .rounded(px(4.0))
                        .cursor_pointer()
                        .when(!state.can_go_next(), |d| d.opacity(0.3).cursor_default())
                        .when(state.can_go_next(), |d| d.hover(|s| s.bg(colors.alt_row_bg)))
                        .child(Icon::new(IconName::ChevronRight).size(px(10.0)).text_color(colors.text_muted))
                        .child(Icon::new(IconName::ChevronRight).size(px(10.0)).text_color(colors.text_muted).ml(px(-4.0))),
                ),
        )
}

/// Render the page size selector trigger (the dropdown is rendered separately)
pub fn render_page_size_selector(
    state: &DataTableState,
    colors: &DataTableColors,
) -> Stateful<Div> {
    h_flex()
        .id("page-size-selector")
        .items_center()
        .gap_1()
        .child(
            div()
                .text_xs()
                .text_color(colors.text_muted)
                .child("Rows:"),
        )
        .child(
            h_flex()
                .id("page-size-trigger")
                .items_center()
                .gap_1()
                .px_2()
                .py(px(2.0))
                .bg(colors.background)
                .border_1()
                .border_color(colors.border)
                .rounded(px(4.0))
                .cursor_pointer()
                .hover(|s| s.border_color(colors.primary.opacity(0.5)))
                .child(
                    div()
                        .text_xs()
                        .text_color(colors.text)
                        .child(state.page_size.to_string()),
                )
                .child(
                    Icon::new(if state.page_size_dropdown_open {
                        IconName::ChevronUp
                    } else {
                        IconName::ChevronDown
                    })
                    .size(px(10.0))
                    .text_color(colors.text_muted),
                ),
        )
}

/// Render the page size dropdown menu (positioned absolutely, caller adds click handlers)
pub fn render_page_size_dropdown_menu(
    state: &DataTableState,
    colors: &DataTableColors,
) -> Stateful<Div> {
    let mut menu = div()
        .id("page-size-menu")
        .absolute()
        .bottom(px(40.0))
        .left(px(150.0))
        .w(px(80.0))
        .bg(colors.background)
        .border_1()
        .border_color(colors.border)
        .rounded(px(6.0))
        .shadow_lg()
        .py_1();

    for &size in PAGE_SIZE_OPTIONS {
        let is_current = size == state.page_size;
        menu = menu.child(
            h_flex()
                .id(ElementId::Name(format!("page-size-{}", size).into()))
                .w_full()
                .px_3()
                .py_1()
                .cursor_pointer()
                .when(is_current, |d| d.bg(colors.primary.opacity(0.15)))
                .when(!is_current, |d| d.hover(|s| s.bg(colors.alt_row_bg)))
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_xs()
                        .text_color(if is_current { colors.primary } else { colors.text })
                        .child(size.to_string()),
                )
                .when(is_current, |d| {
                    d.child(
                        Icon::new(IconName::Check)
                            .size(px(12.0))
                            .text_color(colors.primary),
                    )
                }),
        );
    }

    menu
}

/// Render column headers row
pub fn render_column_headers(
    columns: &[&str],
    colors: &DataTableColors,
) -> Div {
    let mut header_row = h_flex()
        .w_full()
        .bg(colors.header_bg)
        .border_b_1()
        .border_color(colors.border);

    for col in columns {
        header_row = header_row.child(
            div()
                .flex_1()
                .px_3()
                .py_2()
                .text_xs()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(colors.text)
                .border_r_1()
                .border_color(colors.border)
                .child(col.to_string()),
        );
    }

    header_row
}

/// Render a single data row with hover states
pub fn render_data_row(
    cells: Vec<String>,
    row_idx: usize,
    is_hovered: bool,
    colors: &DataTableColors,
) -> Stateful<Div> {
    let is_alt = row_idx % 2 == 1;

    // Hover background is slightly more prominent than alt row
    let hover_bg = colors.primary.opacity(0.08);

    let mut row = h_flex()
        .id(ElementId::Name(format!("row-{}", row_idx).into()))
        .w_full()
        .when(is_hovered, |d| d.bg(hover_bg))
        .when(!is_hovered && is_alt, |d| d.bg(colors.alt_row_bg))
        .hover(|s| s.bg(hover_bg))
        .border_b_1()
        .border_color(colors.border.opacity(0.5))
        .cursor_default();

    for cell_value in cells {
        row = row.child(
            div()
                .flex_1()
                .px_3()
                .py_2()
                .text_xs()
                .text_color(colors.text_muted)
                .border_r_1()
                .border_color(colors.border.opacity(0.3))
                .overflow_hidden()
                .child(cell_value),
        );
    }

    row
}

/// Render a complete data table with header, body, and pagination.
///
/// This is a stateless render function. The caller is responsible for:
/// - Storing the `DataTableState`
/// - Handling click events on the header (to toggle collapse)
/// - Handling click events on pagination buttons
/// - Handling click events on page size selector
///
/// Returns a Stateful<Div> that can have event handlers attached.
pub fn render_data_table<T, F>(
    id: impl Into<ElementId>,
    title: &str,
    columns: &[&str],
    rows: &[T],
    state: &DataTableState,
    render_cell: F,
    colors: &DataTableColors,
) -> Stateful<Div>
where
    F: Fn(&T, usize) -> String,
{
    let mut container = v_flex()
        .id(id)
        .w_full()
        .border_1()
        .border_color(colors.border)
        .rounded(px(6.0))
        .overflow_hidden()
        .bg(colors.background)
        .relative();

    // Header with collapse toggle
    container = container.child(render_table_header(title, state, colors));

    // Table body (only when not collapsed)
    if !state.collapsed {
        // Column headers
        container = container.child(render_column_headers(columns, colors));

        // Data rows (paginated)
        let range = state.visible_range();
        let visible_rows: Vec<_> = rows
            .iter()
            .skip(range.start)
            .take(range.end - range.start)
            .collect();

        let mut body = v_flex().w_full();
        for (row_idx, row_data) in visible_rows.iter().enumerate() {
            let cells: Vec<String> = (0..columns.len())
                .map(|col_idx| render_cell(row_data, col_idx))
                .collect();
            let is_hovered = state.hovered_row == Some(row_idx);
            body = body.child(render_data_row(cells, row_idx, is_hovered, colors));
        }
        container = container.child(body);

        // Pagination footer (with relative positioning for dropdown)
        container = container.child(
            div()
                .relative()
                .child(render_pagination(state, colors))
                .when(state.page_size_dropdown_open, |d| {
                    d.child(render_page_size_dropdown_menu(state, colors))
                }),
        );
    }

    container
}

/// Format row count with thousands separator (e.g., "1,234 rows")
fn format_row_count(count: usize) -> String {
    let formatted = if count >= 1_000_000 {
        format!("{:.1}M", count as f64 / 1_000_000.0)
    } else if count >= 1_000 {
        format!("{:.1}K", count as f64 / 1_000.0)
    } else {
        count.to_string()
    };

    if count == 1 {
        format!("{} row", formatted)
    } else {
        format!("{} rows", formatted)
    }
}

#[cfg(test)]
mod tests {
    use super::{DataTableState, format_row_count};

    #[test]
    fn test_pagination_state() {
        let mut state = DataTableState::new(100);
        assert_eq!(state.total_pages(), 4); // 100 / 25 = 4
        assert!(!state.can_go_prev());
        assert!(state.can_go_next());

        state.go_next();
        assert_eq!(state.current_page, 1);
        assert!(state.can_go_prev());

        state.go_last();
        assert_eq!(state.current_page, 3);
        assert!(!state.can_go_next());

        state.go_first();
        assert_eq!(state.current_page, 0);
    }

    #[test]
    fn test_visible_range() {
        let state = DataTableState::new(100);
        assert_eq!(state.visible_range(), 0..25);

        let mut state = DataTableState::new(100);
        state.current_page = 3;
        assert_eq!(state.visible_range(), 75..100);

        // Edge case: partial last page
        let mut state = DataTableState::new(30);
        state.current_page = 1;
        assert_eq!(state.visible_range(), 25..30);
    }

    #[test]
    fn test_format_row_count() {
        assert_eq!(format_row_count(0), "0 rows");
        assert_eq!(format_row_count(1), "1 row");
        assert_eq!(format_row_count(999), "999 rows");
        assert_eq!(format_row_count(1000), "1.0K rows");
        assert_eq!(format_row_count(1234), "1.2K rows");
        assert_eq!(format_row_count(1000000), "1.0M rows");
    }
}
