//! Settings dropdown components - theme and font selectors.

use crate::app::Humanboard;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{h_flex, ActiveTheme as _, Icon, IconName};

/// Render the theme selector dropdown trigger
pub fn render_theme_dropdown(
    current_theme: &str,
    fg: Hsla,
    muted_fg: Hsla,
    input_bg: Hsla,
    border: Hsla,
    cx: &mut Context<Humanboard>,
) -> Stateful<Div> {
    div().id("theme-dropdown").relative().child(
        div()
            .id("theme-dropdown-trigger")
            .w(px(160.0))
            .h(px(28.0))
            .px_3()
            .bg(input_bg)
            .border_1()
            .border_color(border)
            .rounded(px(6.0))
            .cursor(CursorStyle::PointingHand)
            .flex()
            .items_center()
            .justify_between()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.toggle_theme_dropdown(cx);
                }),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(fg)
                    .overflow_hidden()
                    .whitespace_nowrap()
                    .child(current_theme.to_string()),
            )
            .child(
                Icon::new(IconName::ChevronDown)
                    .size(px(12.0))
                    .text_color(muted_fg),
            ),
    )
}

/// Render the font selector dropdown trigger
pub fn render_font_dropdown(
    current_font: &str,
    fg: Hsla,
    muted_fg: Hsla,
    input_bg: Hsla,
    border: Hsla,
    cx: &mut Context<Humanboard>,
) -> Stateful<Div> {
    div().id("font-dropdown").relative().child(
        div()
            .id("font-dropdown-trigger")
            .w(px(160.0))
            .h(px(28.0))
            .px_3()
            .bg(input_bg)
            .border_1()
            .border_color(border)
            .rounded(px(6.0))
            .cursor(CursorStyle::PointingHand)
            .flex()
            .items_center()
            .justify_between()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.toggle_font_dropdown(cx);
                }),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(fg)
                    .overflow_hidden()
                    .whitespace_nowrap()
                    .child(current_font.to_string()),
            )
            .child(
                Icon::new(IconName::ChevronDown)
                    .size(px(12.0))
                    .text_color(muted_fg),
            ),
    )
}

/// Render the theme dropdown menu
#[allow(clippy::too_many_arguments)]
pub fn render_theme_dropdown_menu(
    themes: &[String],
    current_theme: &str,
    bg: Hsla,
    border: Hsla,
    fg: Hsla,
    muted_fg: Hsla,
    list_active: Hsla,
    list_hover: Hsla,
    cx: &mut Context<Humanboard>,
) -> impl IntoElement {
    div()
        .id("theme-dropdown-menu")
        .absolute()
        .top(px(120.0))
        .right(px(24.0))
        .w(px(200.0))
        .max_h(px(280.0))
        .bg(bg)
        .border_1()
        .border_color(border)
        .rounded(px(6.0))
        .shadow_lg()
        .overflow_y_scroll()
        .py_1()
        .children(themes.iter().map(|theme_name| {
            let is_current = theme_name == current_theme;
            let theme_clone = theme_name.clone();

            div()
                .id(ElementId::Name(theme_name.clone().into()))
                .w_full()
                .px_3()
                .py_1p5()
                .cursor(CursorStyle::PointingHand)
                .when(is_current, |d| d.bg(list_active))
                .when(!is_current, |d| d.hover(|s| s.bg(list_hover)))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _, _, cx| {
                        this.set_theme(theme_clone.clone(), cx);
                        this.close_theme_dropdown(cx);
                    }),
                )
                .child(
                    h_flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_sm()
                                .text_color(if is_current { fg } else { muted_fg })
                                .child(theme_name.clone()),
                        )
                        .when(is_current, |d| {
                            d.child(
                                Icon::new(IconName::Check)
                                    .size(px(14.0))
                                    .text_color(cx.theme().primary),
                            )
                        }),
                )
        }))
}

/// Render the font dropdown menu
#[allow(clippy::too_many_arguments)]
pub fn render_font_dropdown_menu(
    fonts: &[String],
    current_font: &str,
    bg: Hsla,
    border: Hsla,
    fg: Hsla,
    muted_fg: Hsla,
    list_active: Hsla,
    list_hover: Hsla,
    cx: &mut Context<Humanboard>,
) -> impl IntoElement {
    div()
        .id("font-dropdown-menu")
        .absolute()
        .top(px(200.0))
        .right(px(24.0))
        .w(px(220.0))
        .max_h(px(280.0))
        .bg(bg)
        .border_1()
        .border_color(border)
        .rounded(px(6.0))
        .shadow_lg()
        .overflow_y_scroll()
        .py_1()
        .children(fonts.iter().map(|font_name| {
            let is_current = font_name == current_font;
            let font_clone = font_name.to_string();

            div()
                .id(ElementId::Name(format!("font-{}", font_name).into()))
                .w_full()
                .px_3()
                .py_1p5()
                .cursor(CursorStyle::PointingHand)
                .font_family(font_name.to_string())
                .when(is_current, |d| d.bg(list_active))
                .when(!is_current, |d| d.hover(|s| s.bg(list_hover)))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _, _, cx| {
                        this.set_font(font_clone.clone(), cx);
                        this.close_font_dropdown(cx);
                    }),
                )
                .child(
                    h_flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_sm()
                                .text_color(if is_current { fg } else { muted_fg })
                                .child(font_name.to_string()),
                        )
                        .when(is_current, |d| {
                            d.child(
                                Icon::new(IconName::Check)
                                    .size(px(14.0))
                                    .text_color(cx.theme().primary),
                            )
                        }),
                )
        }))
}
