//! Settings modal component.

use crate::actions::{ModalFocusNext, ModalFocusPrev, OpenSettings};
use crate::app::{Humanboard, SettingsTab};
use crate::focus::FocusContext;
use crate::settings::Settings;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{h_flex, v_flex, ActiveTheme as _, Icon, IconName};

use super::modal_base::{
    render_section_header, render_setting_row, FontDropdownOpen, ThemeDropdownOpen,
};
use super::settings_dropdowns::{
    render_font_dropdown, render_font_dropdown_menu, render_theme_dropdown,
    render_theme_dropdown_menu,
};

/// Render the settings modal
pub fn render_settings_modal(
    current_theme: &str,
    current_font: &str,
    _theme_index: usize,
    _theme_scroll: &ScrollHandle,
    active_tab: SettingsTab,
    modal_focus: &FocusHandle,
    opacity: f32,
    cx: &mut Context<Humanboard>,
) -> impl IntoElement {
    let themes = Settings::available_themes(cx);
    let fonts = Settings::available_fonts();
    let current_theme_display = current_theme.to_string();
    let current_font_display = current_font.to_string();

    let bg = cx.theme().background;
    let border = cx.theme().border;
    let fg = cx.theme().foreground;
    let muted_fg = cx.theme().muted_foreground;
    let title_bar = cx.theme().title_bar;
    let list_active = cx.theme().list_active;
    let list_hover = cx.theme().list_hover;
    let input_bg = cx.theme().secondary;

    deferred(
        div()
            .id("settings-backdrop")
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .bg(hsla(0.0, 0.0, 0.0, 0.6 * opacity))
            .flex()
            .items_center()
            .justify_center()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.settings_backdrop_clicked = true;
                    cx.notify();
                }),
            )
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| {
                    if this.settings_backdrop_clicked {
                        this.show_settings = false;
                        this.settings_backdrop_clicked = false;
                        this.focus.force_canvas_focus(window);
                    }
                    cx.notify();
                }),
            )
            .on_scroll_wheel(cx.listener(|_, _, _, _| {}))
            .child(
                h_flex()
                    .id("settings-modal")
                    .track_focus(modal_focus)
                    .key_context(FocusContext::KEY_MODAL)
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, _| {
                            this.settings_backdrop_clicked = false;
                        }),
                    )
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(|this, _, _, _| {
                            this.settings_backdrop_clicked = false;
                        }),
                    )
                    .w(px(680.0))
                    .h(px(480.0))
                    .bg(bg.opacity(opacity))
                    .border_1()
                    .border_color(border.opacity(opacity))
                    .rounded(px(10.0))
                    .overflow_hidden()
                    .shadow_lg()
                    .on_scroll_wheel(|_, _, _| {})
                    .on_action(cx.listener(|this, _: &OpenSettings, window, cx| {
                        this.toggle_settings(window, cx);
                    }))
                    .on_action(cx.listener(|this, _: &ModalFocusNext, window, cx| {
                        this.modal_focus_next(window, cx);
                    }))
                    .on_action(cx.listener(|this, _: &ModalFocusPrev, window, cx| {
                        this.modal_focus_prev(window, cx);
                    }))
                    // Left sidebar
                    .child(render_settings_sidebar(active_tab, title_bar, border, fg, muted_fg, list_active, list_hover, cx))
                    // Right content area
                    .child(render_settings_content(
                        active_tab,
                        &current_theme_display,
                        &current_font_display,
                        &themes,
                        &fonts,
                        bg,
                        border,
                        fg,
                        muted_fg,
                        input_bg,
                        list_active,
                        list_hover,
                        cx,
                    )),
            ),
    )
    .with_priority(1500)
}

fn render_settings_sidebar(
    active_tab: SettingsTab,
    title_bar: Hsla,
    border: Hsla,
    fg: Hsla,
    muted_fg: Hsla,
    list_active: Hsla,
    list_hover: Hsla,
    cx: &mut Context<Humanboard>,
) -> Div {
    v_flex()
        .w(px(180.0))
        .h_full()
        .bg(title_bar)
        .border_r_1()
        .border_color(border)
        .rounded_l(px(10.0))
        .p_2()
        .gap_1()
        // Appearance tab
        .child(
            div()
                .id("tab-appearance")
                .w_full()
                .px_2()
                .py_1p5()
                .rounded(px(4.0))
                .cursor(CursorStyle::PointingHand)
                .when(active_tab == SettingsTab::Appearance, |d| d.bg(list_active))
                .when(active_tab != SettingsTab::Appearance, |d| {
                    d.hover(|s| s.bg(list_hover))
                })
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, _, cx| {
                        this.set_settings_tab(SettingsTab::Appearance, cx);
                    }),
                )
                .child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(
                            Icon::new(IconName::Palette)
                                .size(px(14.0))
                                .text_color(if active_tab == SettingsTab::Appearance { fg } else { muted_fg }),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(if active_tab == SettingsTab::Appearance { fg } else { muted_fg })
                                .child("Appearance"),
                        ),
                ),
        )
        // Integrations tab
        .child(
            div()
                .id("tab-integrations")
                .w_full()
                .px_2()
                .py_1p5()
                .rounded(px(4.0))
                .cursor(CursorStyle::PointingHand)
                .when(active_tab == SettingsTab::Integrations, |d| d.bg(list_active))
                .when(active_tab != SettingsTab::Integrations, |d| {
                    d.hover(|s| s.bg(list_hover))
                })
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, _, cx| {
                        this.set_settings_tab(SettingsTab::Integrations, cx);
                    }),
                )
                .child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(
                            Icon::new(IconName::Settings)
                                .size(px(14.0))
                                .text_color(if active_tab == SettingsTab::Integrations { fg } else { muted_fg }),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(if active_tab == SettingsTab::Integrations { fg } else { muted_fg })
                                .child("Integrations"),
                        ),
                ),
        )
}

#[allow(clippy::too_many_arguments)]
fn render_settings_content(
    active_tab: SettingsTab,
    current_theme: &str,
    current_font: &str,
    themes: &[String],
    fonts: &[&str],
    bg: Hsla,
    border: Hsla,
    fg: Hsla,
    muted_fg: Hsla,
    input_bg: Hsla,
    list_active: Hsla,
    list_hover: Hsla,
    cx: &mut Context<Humanboard>,
) -> impl IntoElement {
    let current_theme_clone = current_theme.to_string();
    let current_font_clone = current_font.to_string();
    let themes_clone = themes.to_vec();
    let fonts_clone: Vec<String> = fonts.iter().map(|s| s.to_string()).collect();

    v_flex()
        .id("settings-content")
        .flex_1()
        .h_full()
        .overflow_hidden()
        .px_6()
        .py_6()
        // Content - Appearance tab
        .when(active_tab == SettingsTab::Appearance, |d| {
            d.child(
                v_flex()
                    .gap_4()
                    .child(render_section_header("Theme", cx))
                    .child(render_setting_row(
                        "Theme",
                        "Choose a color theme for the interface",
                        render_theme_dropdown(&current_theme_clone, fg, muted_fg, input_bg, border, cx),
                        cx,
                    ))
                    .child(render_section_header("Font", cx))
                    .child(render_setting_row(
                        "Font Family",
                        "Choose a font for the interface",
                        render_font_dropdown(&current_font_clone, fg, muted_fg, input_bg, border, cx),
                        cx,
                    )),
            )
        })
        // Content - Integrations tab
        .when(active_tab == SettingsTab::Integrations, |d| {
            d.child(
                v_flex().gap_4().child(
                    div()
                        .py_8()
                        .text_color(muted_fg)
                        .text_sm()
                        .child("No integrations available yet."),
                ),
            )
        })
        // Theme dropdown menu
        .when(cx.try_global::<ThemeDropdownOpen>().is_some(), |d| {
            d.child(render_theme_dropdown_menu(&themes_clone, &current_theme_clone, bg, border, fg, muted_fg, list_active, list_hover, cx))
        })
        // Font dropdown menu
        .when(cx.try_global::<FontDropdownOpen>().is_some(), |d| {
            d.child(render_font_dropdown_menu(&fonts_clone, &current_font_clone, bg, border, fg, muted_fg, list_active, list_hover, cx))
        })
}
