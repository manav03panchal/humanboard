//! Command palette components - legacy full-screen version.

use crate::actions::{CloseCommandPalette, CmdPaletteDown, CmdPaletteUp};
use crate::app::Humanboard;
use crate::focus::FocusContext;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::input::{Input, InputState, MoveDown, MoveUp};
use gpui_component::{h_flex, v_flex, ActiveTheme as _, Icon, IconName};

use super::modal_base::render_kbd;

/// Render the command palette popup (legacy full-screen version)
pub fn render_command_palette(
    input: &Entity<InputState>,
    search_results: &[(u64, String)],
    selected_result: usize,
    opacity: f32,
    cx: &mut Context<Humanboard>,
) -> impl IntoElement {
    let current_text = input.read(cx).text().to_string();
    let has_results = !search_results.is_empty();
    let show_md_hint = current_text.is_empty()
        || "md".starts_with(&current_text.to_lowercase())
        || current_text.to_lowercase().starts_with("md");

    let bg = cx.theme().popover;
    let border = cx.theme().border;
    let fg = cx.theme().foreground;
    let muted_fg = cx.theme().muted_foreground;
    let primary = cx.theme().primary;
    let list_active = cx.theme().list_active;
    let list_hover = cx.theme().list_hover;
    let success = cx.theme().success;

    deferred(
        v_flex()
            .absolute()
            .inset_0()
            .bg(hsla(0.0, 0.0, 0.0, 0.6 * opacity))
            .items_center()
            .pt(px(120.0))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| this.hide_command_palette(window, cx)),
            )
            .on_scroll_wheel(|_, _, _| {})
            .child(
                v_flex()
                    .w(px(500.0))
                    .max_h(px(400.0))
                    .flex_shrink_0()
                    .bg(bg.opacity(opacity))
                    .border_1()
                    .border_color(border.opacity(opacity))
                    .rounded(px(12.0))
                    .shadow_lg()
                    .overflow_hidden()
                    .key_context(FocusContext::KEY_COMMAND_PALETTE)
                    .on_action(cx.listener(|this, _: &MoveUp, _, cx| {
                        this.select_prev_result(cx);
                    }))
                    .on_action(cx.listener(|this, _: &MoveDown, _, cx| {
                        this.select_next_result(cx);
                    }))
                    .on_action(cx.listener(|this, _: &CmdPaletteUp, _, cx| {
                        this.select_prev_result(cx);
                    }))
                    .on_action(cx.listener(|this, _: &CmdPaletteDown, _, cx| {
                        this.select_next_result(cx);
                    }))
                    .on_action(cx.listener(|this, _: &CloseCommandPalette, window, cx| {
                        this.hide_command_palette(window, cx);
                    }))
                    .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                        if this.command_palette.is_some() {
                            match &event.keystroke.key {
                                key if key == "up" => this.select_prev_result(cx),
                                key if key == "down" => this.select_next_result(cx),
                                _ => {}
                            }
                        }
                    }))
                    .on_mouse_down(MouseButton::Left, |_, _, _| {})
                    .on_scroll_wheel(|_, _, _| {})
                    // Search input
                    .child(render_palette_search_input(input, &current_text, border, muted_fg, fg))
                    // Results
                    .child(render_palette_results(
                        search_results,
                        selected_result,
                        has_results,
                        show_md_hint,
                        &current_text,
                        fg,
                        muted_fg,
                        primary,
                        list_active,
                        list_hover,
                        success,
                        cx,
                    ))
                    // Footer
                    .child(render_palette_footer(border, muted_fg)),
            ),
    )
    .with_priority(2000)
}

fn render_palette_search_input(
    input: &Entity<InputState>,
    current_text: &str,
    border: Hsla,
    muted_fg: Hsla,
    fg: Hsla,
) -> Div {
    h_flex()
        .px_4()
        .py_3()
        .gap_3()
        .border_b_1()
        .border_color(border)
        .child(
            Icon::new(IconName::Search)
                .size(px(18.0))
                .text_color(muted_fg),
        )
        .child(
            div()
                .flex_1()
                .relative()
                .child(
                    Input::new(input)
                        .w_full()
                        .appearance(false)
                        .cleanable(false),
                )
                .when(current_text.is_empty(), |d| {
                    d.child(
                        div()
                            .absolute()
                            .top_0()
                            .left_0()
                            .h_full()
                            .flex()
                            .items_center()
                            .text_sm()
                            .text_color(fg.opacity(0.5))
                            .child("Type to search or use commands..."),
                    )
                }),
        )
        .child(
            div()
                .text_xs()
                .text_color(muted_fg)
                .child("click outside to close"),
        )
}

#[allow(clippy::too_many_arguments)]
fn render_palette_results(
    search_results: &[(u64, String)],
    selected_result: usize,
    has_results: bool,
    show_md_hint: bool,
    current_text: &str,
    fg: Hsla,
    muted_fg: Hsla,
    primary: Hsla,
    list_active: Hsla,
    list_hover: Hsla,
    success: Hsla,
    cx: &mut Context<Humanboard>,
) -> impl IntoElement {
    div()
        .id("command-palette-results")
        .flex_1()
        .overflow_y_scroll()
        .on_scroll_wheel(|_, _, _| {})
        .when(has_results, |d| {
            d.child(v_flex().py_2().children(
                search_results.iter().enumerate().map(|(idx, (item_id, name))| {
                    let is_selected = idx == selected_result;
                    let item_id = *item_id;

                    h_flex()
                        .id(ElementId::Name(format!("result-{}", item_id).into()))
                        .pl(px(12.0))
                        .pr_4()
                        .py_2()
                        .gap_3()
                        .cursor(CursorStyle::PointingHand)
                        .when(is_selected, |d| {
                            d.bg(list_active).border_l_2().border_color(primary)
                        })
                        .when(!is_selected, |d| d.hover(|s| s.bg(list_hover)))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.pending_command = Some(format!("__jump:{}", item_id));
                            this.command_palette = None;
                            this.search_results.clear();
                            cx.notify();
                        }))
                        .child(
                            Icon::new(IconName::File)
                                .size(px(16.0))
                                .text_color(if is_selected { primary } else { muted_fg }),
                        )
                        .child(
                            div()
                                .flex_1()
                                .text_sm()
                                .text_color(if is_selected { fg } else { muted_fg })
                                .overflow_hidden()
                                .whitespace_nowrap()
                                .child(name.clone()),
                        )
                        .when(is_selected, |d| {
                            d.child(div().text_xs().text_color(muted_fg).child("↵ jump"))
                        })
                }),
            ))
        })
        .when(!has_results && show_md_hint, |d| {
            d.child(
                v_flex()
                    .py_2()
                    .child(
                        div()
                            .px_4()
                            .py_1()
                            .text_xs()
                            .text_color(muted_fg)
                            .child("COMMANDS"),
                    )
                    .child(
                        h_flex()
                            .px_4()
                            .py_2()
                            .gap_3()
                            .cursor(CursorStyle::PointingHand)
                            .hover(|s| s.bg(list_hover))
                            .child(
                                Icon::new(IconName::File)
                                    .size(px(16.0))
                                    .text_color(success),
                            )
                            .child(
                                h_flex()
                                    .flex_1()
                                    .gap_2()
                                    .child(render_kbd("md", cx))
                                    .child(div().text_sm().text_color(muted_fg).child("<name>")),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(muted_fg)
                                    .child("create markdown note"),
                            ),
                    ),
            )
        })
        .when(!has_results && !show_md_hint && !current_text.is_empty(), |d| {
            d.child(
                v_flex()
                    .py_6()
                    .items_center()
                    .gap_2()
                    .child(
                        Icon::new(IconName::Search)
                            .size(px(32.0))
                            .text_color(muted_fg),
                    )
                    .child(div().text_sm().text_color(muted_fg).child("No items found")),
            )
        })
}

fn render_palette_footer(border: Hsla, muted_fg: Hsla) -> Div {
    h_flex()
        .px_4()
        .py_2()
        .gap_4()
        .border_t_1()
        .border_color(border)
        .text_xs()
        .text_color(muted_fg)
        .child(h_flex().gap_1().child("↑↓").child("navigate"))
        .child(h_flex().gap_1().child("↵").child("select"))
}
