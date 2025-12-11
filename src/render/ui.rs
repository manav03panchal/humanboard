//! UI components module.
//!
//! Contains reusable UI components like footer bar, splitter, etc.

use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::app::SplitDirection;

/// Render the footer status bar
pub fn render_footer_bar(
    _fps: f32,
    _frame_count: u64,
    item_count: usize,
    zoom: f32,
    canvas_offset: Point<Pixels>,
    selected_item_name: Option<String>,
) -> Div {
    div()
        .absolute()
        .bottom_0()
        .left_0()
        .right_0()
        .h(px(28.0))
        .bg(hsla(0.0, 0.0, 0.0, 0.95))
        .border_t_1()
        .border_color(hsla(0.0, 0.0, 0.3, 1.0))
        .flex()
        .items_center()
        .justify_between()
        .px_4()
        .gap_6()
        .text_xs()
        .text_color(rgb(0xaaaaaa))
        .child(render_footer_left_section(item_count, zoom, canvas_offset))
        .when_some(selected_item_name, |d, name| {
            d.child(render_selected_item_indicator(name))
        })
}

/// Render the left section of the footer
fn render_footer_left_section(item_count: usize, zoom: f32, canvas_offset: Point<Pixels>) -> Div {
    div()
        .flex()
        .gap_6()
        .child(render_app_title())
        .child(render_stat("Items", &item_count.to_string()))
        .child(render_stat("Zoom", &format!("{:.2}x", zoom)))
        .child(render_stat(
            "Position",
            &format!(
                "X: {:.0} Y: {:.0}",
                f32::from(canvas_offset.x),
                f32::from(canvas_offset.y)
            ),
        ))
}

/// Render the application title
fn render_app_title() -> Div {
    div()
        .font_weight(FontWeight::BOLD)
        .text_color(rgb(0xffffff))
        .child("Humanboard")
}

/// Render a stat item
fn render_stat(label: &str, value: &str) -> Div {
    div().child(format!("{}: {}", label, value))
}

/// Render the selected item indicator
fn render_selected_item_indicator(name: String) -> Div {
    div().text_color(rgb(0xffffff)).child(name)
}

/// Render a splitter bar between panels
pub fn render_splitter(direction: SplitDirection) -> Div {
    match direction {
        SplitDirection::Vertical => render_vertical_splitter(),
        SplitDirection::Horizontal => render_horizontal_splitter(),
    }
}

/// Render a vertical splitter (for left/right split)
fn render_vertical_splitter() -> Div {
    div()
        .w(px(16.0))
        .h_full()
        .bg(rgb(0x000000))
        .hover(|s| s.bg(rgb(0x1a1a1a)))
        .cursor(CursorStyle::ResizeLeftRight)
        .flex()
        .items_center()
        .justify_center()
        .child(render_splitter_handle_vertical())
}

/// Render a horizontal splitter (for top/bottom split)
fn render_horizontal_splitter() -> Div {
    div()
        .h(px(16.0))
        .w_full()
        .bg(rgb(0x000000))
        .hover(|s| s.bg(rgb(0x1a1a1a)))
        .cursor(CursorStyle::ResizeUpDown)
        .flex()
        .items_center()
        .justify_center()
        .child(render_splitter_handle_horizontal())
}

/// Render the visual handle for vertical splitter
fn render_splitter_handle_vertical() -> Div {
    div()
        .w(px(1.0))
        .h(px(60.0))
        .bg(rgb(0x333333))
        .rounded(px(1.0))
}

/// Render the visual handle for horizontal splitter
fn render_splitter_handle_horizontal() -> Div {
    div()
        .h(px(1.0))
        .w(px(60.0))
        .bg(rgb(0x333333))
        .rounded(px(1.0))
}

/// Render a stats overlay (deprecated, use footer_bar instead)
pub fn render_stats_overlay(
    fps: f32,
    frame_count: u64,
    item_count: usize,
    zoom: f32,
    canvas_offset: Point<Pixels>,
) -> Div {
    render_footer_bar(fps, frame_count, item_count, zoom, canvas_offset, None)
}

/// Render a label for selected item (deprecated)
pub fn render_selected_item_label(_name: String) -> Div {
    div().size_0()
}
