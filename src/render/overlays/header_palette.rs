//! Command palette dropdown component for header bar.

use crate::actions::{CloseCommandPalette, CmdPaletteDown, CmdPaletteUp};
use crate::app::Humanboard;
use crate::focus::FocusContext;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::input::{Input, InputState, MoveDown, MoveUp};
use gpui_component::{h_flex, v_flex, ActiveTheme as _, Icon, IconName};

/// Render the center command palette section of the header
#[allow(clippy::too_many_arguments)]
pub fn render_header_center(
    command_palette: Option<&Entity<InputState>>,
    search_results: &[(u64, String)],
    selected_result: usize,
    scroll_handle: &ScrollHandle,
    _palette_mode: crate::app::CmdPaletteMode,
    palette_focus: &FocusHandle,
    is_open: bool,
    has_results: bool,
    is_theme_mode: bool,
    cx: &mut Context<Humanboard>,
) -> impl IntoElement {
    let border = cx.theme().border;
    let muted = cx.theme().muted;
    let muted_fg = cx.theme().muted_foreground;
    let input_bg = cx.theme().secondary;
    let popover_bg = cx.theme().popover;
    let primary = cx.theme().primary;

    v_flex()
        .id("cmd-palette-container")
        .w(px(400.0))
        .relative()
        .track_focus(palette_focus)
        .key_context(FocusContext::KEY_COMMAND_PALETTE)
        .on_action(cx.listener(|this, _: &MoveUp, _, cx| {
            this.select_prev_result(cx);
        }))
        .on_action(cx.listener(|this, _: &MoveDown, _, cx| {
            this.select_next_result(cx);
        }))
        .on_action(cx.listener(|this, _: &CmdPaletteUp, _, cx| {
            if this.command_palette.is_some() {
                this.select_prev_result(cx);
            }
        }))
        .on_action(cx.listener(|this, _: &CmdPaletteDown, _, cx| {
            if this.command_palette.is_some() {
                this.select_next_result(cx);
            }
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
        .child(render_palette_trigger(command_palette, is_open, primary, border, input_bg, muted, muted_fg, cx))
        .when(is_open, |d| {
            d.child(render_palette_dropdown(
                search_results,
                selected_result,
                scroll_handle,
                has_results,
                is_theme_mode,
                popover_bg,
                border,
                muted,
                muted_fg,
                primary,
                cx,
            ))
        })
}

#[allow(clippy::too_many_arguments)]
fn render_palette_trigger(
    command_palette: Option<&Entity<InputState>>,
    is_open: bool,
    primary: Hsla,
    border: Hsla,
    input_bg: Hsla,
    muted: Hsla,
    muted_fg: Hsla,
    cx: &mut Context<Humanboard>,
) -> impl IntoElement {
    div()
        .id("cmd-palette-trigger")
        .w_full()
        .h(px(28.0))
        .bg(input_bg)
        .border_1()
        .border_color(if is_open { primary } else { border })
        .rounded(px(6.0))
        .px_3()
        .flex()
        .items_center()
        .gap_2()
        .child(Icon::new(IconName::Search).size(px(14.0)).text_color(muted_fg))
        .when(is_open, |d| {
            if let Some(input) = command_palette {
                d.child(Input::new(input).w_full().appearance(false).cleanable(false))
            } else {
                d
            }
        })
        .when(!is_open, |d| {
            d.cursor(CursorStyle::PointingHand)
                .hover(|s| s.border_color(muted_fg.opacity(0.4)))
                .active(|s| s.bg(muted.opacity(0.3)))
                .on_click(cx.listener(|this, _, window, cx| {
                    this.show_command_palette(window, cx);
                }))
                .child(div().text_sm().text_color(muted_fg).child("Search items or type command..."))
                .child(div().ml_auto().text_xs().text_color(muted_fg).child("Cmd+K"))
        })
}

#[allow(clippy::too_many_arguments)]
fn render_palette_dropdown(
    search_results: &[(u64, String)],
    selected_result: usize,
    scroll_handle: &ScrollHandle,
    has_results: bool,
    is_theme_mode: bool,
    popover_bg: Hsla,
    border: Hsla,
    muted: Hsla,
    muted_fg: Hsla,
    primary: Hsla,
    cx: &mut Context<Humanboard>,
) -> Div {
    let fg = cx.theme().foreground;
    let list_active = cx.theme().list_active;
    let list_hover = cx.theme().list_hover;
    let title_bar = cx.theme().title_bar;

    v_flex()
        .absolute()
        .top(px(36.0))
        .left_0()
        .w_full()
        .max_h(px(280.0))
        .bg(popover_bg)
        .border_1()
        .border_color(border)
        .rounded(px(6.0))
        .shadow_lg()
        .overflow_hidden()
        .when(has_results, |d| {
            d.child(
                div()
                    .px_2()
                    .pt_2()
                    .pb_1()
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(muted_fg)
                    .child(if is_theme_mode { "Themes" } else { "Items" }),
            )
            .child(render_dropdown_results(
                search_results, selected_result, scroll_handle, is_theme_mode,
                fg, muted, muted_fg, primary, list_active, list_hover, cx,
            ))
        })
        .child(render_dropdown_commands(border, muted_fg, primary, list_hover, cx))
        .child(render_dropdown_footer(border, title_bar, muted, muted_fg))
}

#[allow(clippy::too_many_arguments)]
fn render_dropdown_results(
    search_results: &[(u64, String)],
    selected_result: usize,
    scroll_handle: &ScrollHandle,
    is_theme_mode: bool,
    fg: Hsla,
    muted: Hsla,
    muted_fg: Hsla,
    primary: Hsla,
    list_active: Hsla,
    list_hover: Hsla,
    cx: &mut Context<Humanboard>,
) -> impl IntoElement {
    div()
        .id("cmd-dropdown-results")
        .max_h(px(200.0))
        .overflow_y_scroll()
        .track_scroll(scroll_handle)
        .px_1()
        .pb_1()
        .flex()
        .flex_col()
        .gap(px(2.0))
        .children(search_results.iter().enumerate().map(|(idx, (item_id, name))| {
            let is_selected = idx == selected_result;
            let item_id = *item_id;
            let name_clone = name.clone();

            h_flex()
                .id(ElementId::Integer(idx as u64))
                .pl(px(6.0))
                .pr_2()
                .py_1()
                .gap_2()
                .rounded(px(4.0))
                .cursor(CursorStyle::PointingHand)
                .when(is_selected, |d| d.bg(list_active).border_l_2().border_color(primary))
                .when(!is_selected, |d| d.hover(|s| s.bg(list_hover)))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _, _, cx| {
                        if this.cmd_palette_mode == crate::app::CmdPaletteMode::Themes {
                            this.pending_command = Some(format!("__theme:{}", name_clone));
                        } else {
                            this.pending_command = Some(format!("__jump:{}", item_id));
                        }
                        this.command_palette = None;
                        this.search_results.clear();
                        this.cmd_palette_mode = crate::app::CmdPaletteMode::Items;
                        cx.notify();
                    }),
                )
                .child(
                    Icon::new(if is_theme_mode { IconName::Palette } else { IconName::File })
                        .size(px(12.0))
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
                    d.child(
                        div()
                            .px_1()
                            .py(px(2.0))
                            .bg(muted)
                            .rounded(px(3.0))
                            .text_xs()
                            .text_color(muted_fg)
                            .child("↵"),
                    )
                })
        }))
}

fn render_dropdown_commands(
    border: Hsla,
    muted_fg: Hsla,
    primary: Hsla,
    list_hover: Hsla,
    cx: &mut Context<Humanboard>,
) -> Div {
    let success = cx.theme().success;

    v_flex()
        .border_t_1()
        .border_color(border)
        .child(
            div()
                .px_2()
                .pt_2()
                .pb_1()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(muted_fg)
                .child("Commands"),
        )
        .child(
            h_flex()
                .px_2()
                .py_1()
                .mx_1()
                .gap_2()
                .rounded(px(4.0))
                .hover(|s| s.bg(list_hover))
                .cursor(CursorStyle::PointingHand)
                .child(
                    div()
                        .px(px(6.0))
                        .py(px(2.0))
                        .bg(success.opacity(0.15))
                        .rounded(px(3.0))
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(success)
                        .child("md"),
                )
                .child(div().text_sm().text_color(muted_fg).child("<name>"))
                .child(div().ml_auto().text_xs().text_color(muted_fg).child("Create markdown note")),
        )
        .child(
            h_flex()
                .px_2()
                .py_1()
                .mx_1()
                .gap_2()
                .rounded(px(4.0))
                .hover(|s| s.bg(list_hover))
                .cursor(CursorStyle::PointingHand)
                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                    this.enter_theme_mode(cx);
                }))
                .child(
                    div()
                        .px(px(6.0))
                        .py(px(2.0))
                        .bg(primary.opacity(0.15))
                        .rounded(px(3.0))
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(primary)
                        .child("theme"),
                )
                .child(div().ml_auto().text_xs().text_color(muted_fg).child("Change theme")),
        )
}

fn render_dropdown_footer(border: Hsla, title_bar: Hsla, muted: Hsla, muted_fg: Hsla) -> Div {
    h_flex()
        .px_2()
        .py_1()
        .gap_3()
        .border_t_1()
        .border_color(border)
        .bg(title_bar)
        .text_xs()
        .text_color(muted_fg)
        .child(
            h_flex()
                .gap_1()
                .child(div().px(px(4.0)).py(px(1.0)).bg(muted).rounded(px(2.0)).child("↑↓"))
                .child("navigate"),
        )
        .child(
            h_flex()
                .gap_1()
                .child(div().px(px(4.0)).py(px(1.0)).bg(muted).rounded(px(2.0)).child("↵"))
                .child("select"),
        )
        .child(
            h_flex()
                .gap_1()
                .child(div().px(px(4.0)).py(px(1.0)).bg(muted).rounded(px(2.0)).child("esc"))
                .child("close"),
        )
}
