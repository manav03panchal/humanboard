//! Shared modal utilities - keyboard badges, setting rows, section headers, dropdown markers.

use crate::app::Humanboard;
use gpui::*;
use gpui_component::{h_flex, v_flex, ActiveTheme as _};

/// Render a keyboard key badge
pub fn render_kbd(key: &str, cx: &Context<Humanboard>) -> Div {
    let muted = cx.theme().muted;
    let border = cx.theme().border;
    let muted_fg = cx.theme().muted_foreground;

    div()
        .px(px(8.0))
        .py(px(4.0))
        .bg(muted)
        .border_1()
        .border_color(border)
        .rounded(px(6.0))
        .text_xs()
        .font_weight(FontWeight::MEDIUM)
        .text_color(muted_fg)
        .child(key.to_string())
}

/// Render a shortcut row with key and description
pub fn render_shortcut_row(key: &str, description: &str, cx: &Context<Humanboard>) -> Div {
    let fg = cx.theme().foreground;

    h_flex()
        .h(px(28.0))
        .items_center()
        .justify_between()
        .child(
            div()
                .text_sm()
                .text_color(fg)
                .child(description.to_string()),
        )
        .child(render_kbd(key, cx))
}

/// Render a section of shortcuts with title
pub fn render_shortcut_section(
    title: &str,
    shortcuts: Vec<(&str, &str)>,
    cx: &Context<Humanboard>,
) -> Div {
    let muted_fg = cx.theme().muted_foreground;

    let mut section = v_flex().gap_1().child(
        div()
            .text_xs()
            .font_weight(FontWeight::BOLD)
            .text_color(muted_fg)
            .mb_1()
            .child(title.to_string().to_uppercase()),
    );

    for (key, desc) in shortcuts {
        section = section.child(render_shortcut_row(key, desc, cx));
    }

    section
}

/// Render a setting row with title, description, and control on the right
pub fn render_setting_row(
    title: &str,
    description: &str,
    control: impl IntoElement,
    cx: &Context<Humanboard>,
) -> Div {
    let fg = cx.theme().foreground;
    let muted_fg = cx.theme().muted_foreground;

    h_flex()
        .w_full()
        .py_3()
        .items_center()
        .justify_between()
        .gap_4()
        .child(
            v_flex()
                .flex_1()
                .min_w_0()
                .gap(px(2.0))
                .child(div().text_sm().text_color(fg).child(title.to_string()))
                .child(
                    div()
                        .text_xs()
                        .text_color(muted_fg)
                        .child(description.to_string()),
                ),
        )
        .child(div().flex_shrink_0().child(control))
}

/// Render a section header
pub fn render_section_header(title: &str, cx: &Context<Humanboard>) -> Div {
    let muted_fg = cx.theme().muted_foreground;
    let border = cx.theme().border;

    div()
        .w_full()
        .pb_2()
        .mb_2()
        .border_b_1()
        .border_color(border)
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(muted_fg)
                .child(title.to_string().to_uppercase()),
        )
}

/// Global marker for which settings dropdown is open
#[derive(Clone, PartialEq)]
pub enum SettingsDropdown {
    Theme,
    Font,
}

impl gpui::Global for SettingsDropdown {}

/// Marker for theme dropdown being open
#[derive(Clone)]
pub struct ThemeDropdownOpen;

impl gpui::Global for ThemeDropdownOpen {}

/// Marker for font dropdown being open
#[derive(Clone)]
pub struct FontDropdownOpen;

impl gpui::Global for FontDropdownOpen {}
