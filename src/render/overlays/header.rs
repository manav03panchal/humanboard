//! Header bar and footer bar components.

use crate::app::Humanboard;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::input::InputState;
use gpui_component::{h_flex, ActiveTheme as _, Icon, IconName};

use super::header_palette::render_header_center;

/// Render the header bar with navigation and integrated command palette
pub fn render_header_bar(
    board_name: Option<String>,
    command_palette: Option<&Entity<InputState>>,
    search_results: &[(u64, String)],
    selected_result: usize,
    scroll_handle: &ScrollHandle,
    palette_mode: crate::app::CmdPaletteMode,
    palette_focus: &FocusHandle,
    cx: &mut Context<Humanboard>,
) -> Div {
    let has_results = !search_results.is_empty();
    let is_open = command_palette.is_some();
    let is_theme_mode = palette_mode == crate::app::CmdPaletteMode::Themes;

    let bg = cx.theme().title_bar;
    let border = cx.theme().border;
    let fg = cx.theme().foreground;
    let muted = cx.theme().muted;
    let muted_fg = cx.theme().muted_foreground;
    let list_hover = cx.theme().list_hover;

    h_flex()
        .absolute()
        .top_0()
        .left_0()
        .right_0()
        .h(px(40.0))
        .bg(bg)
        .border_b_1()
        .border_color(border)
        .items_center()
        .justify_between()
        .pl(px(80.0))
        .pr_4()
        // Left side - board name and home button
        .child(render_header_left(board_name, fg, muted, muted_fg, cx))
        // Center - command palette
        .child(render_header_center(
            command_palette,
            search_results,
            selected_result,
            scroll_handle,
            palette_mode,
            palette_focus,
            is_open,
            has_results,
            is_theme_mode,
            cx,
        ))
        // Right side - add button, settings, and help
        .child(render_header_right(muted_fg, list_hover, cx))
}

fn render_header_left(
    board_name: Option<String>,
    fg: Hsla,
    muted: Hsla,
    muted_fg: Hsla,
    cx: &mut Context<Humanboard>,
) -> Div {
    h_flex()
        .gap_3()
        .items_center()
        .child(
            div()
                .id("go-home-btn")
                .px_2()
                .py_1()
                .rounded(px(4.0))
                .cursor(CursorStyle::PointingHand)
                .hover(|s| s.bg(muted.opacity(0.5)))
                .active(|s| s.bg(muted.opacity(0.7)))
                .text_sm()
                .text_color(muted_fg)
                .child("←")
                .on_click(cx.listener(|this, _, _, cx| {
                    this.go_home(cx);
                })),
        )
        .child(
            div()
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(fg)
                .child(board_name.unwrap_or_else(|| "Humanboard".to_string())),
        )
}

fn render_header_right(muted_fg: Hsla, list_hover: Hsla, cx: &mut Context<Humanboard>) -> Div {
    h_flex()
        .gap_2()
        // Add button
        .child(
            div()
                .id("add-item-btn")
                .px_2()
                .py_1()
                .rounded(px(4.0))
                .cursor(CursorStyle::PointingHand)
                .hover(|s| s.bg(list_hover))
                .text_sm()
                .text_color(muted_fg)
                .child(
                    h_flex()
                        .gap_1()
                        .items_center()
                        .child(Icon::new(IconName::Plus).size(px(14.0)).text_color(muted_fg))
                        .child("Add"),
                )
                .on_click(cx.listener(|this, _, window, cx| {
                    this.open_file(window, cx);
                })),
        )
        // Settings button
        .child(
            div()
                .id("settings-btn")
                .px_2()
                .py_1()
                .rounded(px(4.0))
                .cursor(CursorStyle::PointingHand)
                .hover(|s| s.bg(list_hover))
                .child(Icon::new(IconName::Settings).size(px(14.0)).text_color(muted_fg))
                .on_click(cx.listener(|this, _, window, cx| {
                    this.toggle_settings(window, cx);
                })),
        )
        // Help button
        .child(
            div()
                .id("show-shortcuts-btn")
                .px_2()
                .py_1()
                .rounded(px(4.0))
                .cursor(CursorStyle::PointingHand)
                .hover(|s| s.bg(list_hover))
                .text_sm()
                .text_color(muted_fg)
                .child("?")
                .on_click(cx.listener(|this, _, _, cx| {
                    this.toggle_shortcuts(cx);
                })),
        )
}

/// Render the footer status bar
pub fn render_footer_bar(
    _fps: f32,
    _frame_count: u64,
    item_count: usize,
    zoom: f32,
    canvas_offset: Point<Pixels>,
    selected_item_name: Option<String>,
    board_name: Option<String>,
    is_dirty: bool,
    cx: &mut Context<Humanboard>,
) -> Div {
    let bg = cx.theme().title_bar;
    let border = cx.theme().border;
    let fg = cx.theme().foreground;
    let muted_fg = cx.theme().muted_foreground;
    let success = cx.theme().success;
    let muted = cx.theme().muted;

    h_flex()
        .absolute()
        .bottom_0()
        .left_0()
        .right_0()
        .h(px(28.0))
        .bg(bg)
        .border_t_1()
        .border_color(border)
        .items_center()
        .justify_between()
        .px_4()
        .gap_6()
        .text_xs()
        .text_color(muted_fg)
        .child(
            h_flex()
                .gap_6()
                .child(
                    div()
                        .font_weight(FontWeight::BOLD)
                        .text_color(fg)
                        .child(board_name.unwrap_or_else(|| "Humanboard".to_string())),
                )
                .child(div().child(format!("Items: {}", item_count)))
                .child(div().child(format!("Zoom: {:.2}x", zoom)))
                .child(div().child(format!(
                    "X: {:.0} Y: {:.0}",
                    f32::from(canvas_offset.x),
                    f32::from(canvas_offset.y)
                )))
                .child(if is_dirty {
                    div().text_color(muted_fg).child("Saving...")
                } else {
                    div().text_color(success).child("Saved")
                }),
        )
        // Right side: selected item name and help hints
        .child(
            h_flex()
                .gap_4()
                .items_center()
                .when_some(selected_item_name, |d, name| {
                    d.child(div().text_color(fg).child(name))
                })
                // Help hint: Cmd+K for search
                .child(
                    h_flex()
                        .id("help-hint-search")
                        .gap_1()
                        .items_center()
                        .px(px(6.0))
                        .py(px(2.0))
                        .rounded(px(4.0))
                        .cursor_pointer()
                        .hover(|s| s.bg(muted))
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.show_command_palette(window, cx);
                        }))
                        .child(
                            div()
                                .px(px(4.0))
                                .py(px(1.0))
                                .bg(muted)
                                .border_1()
                                .border_color(border)
                                .rounded(px(3.0))
                                .text_xs()
                                .child("⌘K"),
                        )
                        .child(div().text_color(muted_fg).child("Search")),
                )
                // Help hint: ? for shortcuts
                .child(
                    h_flex()
                        .id("help-hint-shortcuts")
                        .gap_1()
                        .items_center()
                        .px(px(6.0))
                        .py(px(2.0))
                        .rounded(px(4.0))
                        .cursor_pointer()
                        .hover(|s| s.bg(muted))
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.show_shortcuts = !this.show_shortcuts;
                            cx.notify();
                        }))
                        .child(
                            div()
                                .px(px(6.0))
                                .py(px(1.0))
                                .bg(muted)
                                .border_1()
                                .border_color(border)
                                .rounded(px(3.0))
                                .text_xs()
                                .child("?"),
                        )
                        .child(div().text_color(muted_fg).child("Help")),
                ),
        )
}
