//! UI Overlays - header bar, footer, shortcuts modal, command palette, settings
//!
//! This module contains all overlay UI elements that appear on top of the canvas:
//! - Header bar with navigation and command palette
//! - Footer bar with status info
//! - Keyboard shortcuts modal
//! - Command palette popup
//! - Settings modal

use crate::actions::{CmdPaletteDown, CmdPaletteSelect, CmdPaletteUp};
use crate::app::{Humanboard, SettingsTab};
use crate::settings::Settings;
use crate::spotify_auth;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::input::{Enter, Input, InputState, MoveDown, MoveUp};
use gpui_component::{ActiveTheme as _, Icon, IconName, h_flex, v_flex};

/// Render the header bar with navigation and integrated command palette
pub fn render_header_bar(
    board_name: Option<String>,
    command_palette: Option<&Entity<InputState>>,
    search_results: &[(u64, String)],
    selected_result: usize,
    scroll_handle: &ScrollHandle,
    palette_mode: crate::app::CmdPaletteMode,
    cx: &mut Context<Humanboard>,
) -> Div {
    let has_results = !search_results.is_empty();
    let is_open = command_palette.is_some();
    let is_theme_mode = palette_mode == crate::app::CmdPaletteMode::Themes;

    // Get theme colors
    let bg = cx.theme().title_bar;
    let border = cx.theme().border;
    let fg = cx.theme().foreground;
    let muted = cx.theme().muted;
    let muted_fg = cx.theme().muted_foreground;
    let input_bg = cx.theme().input;
    let popover_bg = cx.theme().popover;
    let primary = cx.theme().primary;
    let list_active = cx.theme().list_active;
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
        .child(
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
                        .hover(|s| s.bg(list_hover))
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
                ),
        )
        // Center - command palette
        .child(
            v_flex()
                .id("cmd-palette-container")
                .w(px(400.0))
                .relative()
                .key_context("CommandPalette")
                // Intercept Input's MoveUp/MoveDown/Enter actions to navigate results
                .on_action(cx.listener(|this, _: &MoveUp, _, cx| {
                    this.select_prev_result(cx);
                }))
                .on_action(cx.listener(|this, _: &MoveDown, _, cx| {
                    this.select_next_result(cx);
                }))
                .on_action(cx.listener(|this, _: &Enter, _, cx| {
                    this.execute_command_from_action(cx);
                }))
                .on_action(cx.listener(|this, _: &CmdPaletteUp, _, cx| {
                    this.select_prev_result(cx);
                }))
                .on_action(cx.listener(|this, _: &CmdPaletteDown, _, cx| {
                    this.select_next_result(cx);
                }))
                .on_action(cx.listener(|this, _: &CmdPaletteSelect, _, cx| {
                    this.execute_command_from_action(cx);
                }))
                // Search trigger button / input
                .child(
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
                        .child(
                            Icon::new(IconName::Search)
                                .size(px(14.0))
                                .text_color(muted_fg),
                        )
                        .when(is_open, |d| {
                            if let Some(input) = command_palette {
                                d.child(
                                    Input::new(input)
                                        .w_full()
                                        .appearance(false)
                                        .cleanable(false),
                                )
                            } else {
                                d
                            }
                        })
                        .when(!is_open, |d| {
                            d.cursor(CursorStyle::PointingHand)
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.show_command_palette(window, cx);
                                }))
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(muted_fg)
                                        .child("Search items or type command..."),
                                )
                                .child(
                                    div()
                                        .ml_auto()
                                        .text_xs()
                                        .text_color(muted_fg)
                                        .child("Cmd+K"),
                                )
                        }),
                )
                // Dropdown results
                .when(is_open, |d| {
                    d.child(
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
                            // Items/Themes section
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
                                .child(
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
                                        .children(search_results.iter().enumerate().map(
                                            |(idx, (item_id, name))| {
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
                                                    .when(is_selected, |d| {
                                                        d.bg(list_active)
                                                            .border_l_2()
                                                            .border_color(primary)
                                                    })
                                                    .when(!is_selected, |d| {
                                                        d.hover(|s| s.bg(list_hover))
                                                    })
                                                    .on_mouse_down(
                                                        MouseButton::Left,
                                                        cx.listener(move |this, _, _, cx| {
                                                            if this.cmd_palette_mode == crate::app::CmdPaletteMode::Themes {
                                                                this.pending_command =
                                                                    Some(format!("__theme:{}", name_clone));
                                                            } else {
                                                                this.pending_command =
                                                                    Some(format!("__jump:{}", item_id));
                                                            }
                                                            this.command_palette = None;
                                                            this.search_results.clear();
                                                            this.cmd_palette_mode = crate::app::CmdPaletteMode::Items;
                                                            cx.notify();
                                                        }),
                                                    )
                                                    .child(
                                                        Icon::new(if is_theme_mode {
                                                            IconName::Palette
                                                        } else {
                                                            IconName::File
                                                        })
                                                        .size(px(12.0))
                                                        .text_color(if is_selected {
                                                            primary
                                                        } else {
                                                            muted_fg
                                                        }),
                                                    )
                                                    .child(
                                                        div()
                                                            .flex_1()
                                                            .text_sm()
                                                            .text_color(if is_selected {
                                                                fg
                                                            } else {
                                                                fg
                                                            })
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
                                            },
                                        )),
                                )
                            })
                            // Commands section (when no results or showing hint)
                            .child(
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
                                                    .bg(cx.theme().success.opacity(0.15))
                                                    .rounded(px(3.0))
                                                    .text_xs()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(cx.theme().success)
                                                    .child("md"),
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(muted_fg)
                                                    .child("<name>"),
                                            )
                                            .child(
                                                div()
                                                    .ml_auto()
                                                    .text_xs()
                                                    .text_color(muted_fg)
                                                    .child("Create markdown note"),
                                            ),
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
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(|this, _, _, cx| {
                                                    this.enter_theme_mode(cx);
                                                }),
                                            )
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
                                            .child(
                                                div()
                                                    .ml_auto()
                                                    .text_xs()
                                                    .text_color(muted_fg)
                                                    .child("Change theme"),
                                            ),
                                    )
                                    .child(
                                        h_flex()
                                            .px_2()
                                            .py_1()
                                            .mx_1()
                                            .mb_1()
                                            .gap_2()
                                            .rounded(px(4.0))
                                            .hover(|s| s.bg(list_hover))
                                            .cursor(CursorStyle::PointingHand)
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(|this, _, window, cx| {
                                                    this.add_spotify_webview(window, cx);
                                                    this.hide_command_palette(cx);
                                                }),
                                            )
                                            .child(
                                                div()
                                                    .px(px(6.0))
                                                    .py(px(2.0))
                                                    .bg(Hsla::from(gpui::rgb(0x1DB954)).opacity(0.15))
                                                    .rounded(px(3.0))
                                                    .text_xs()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(Hsla::from(gpui::rgb(0x1DB954)))
                                                    .child("spotify"),
                                            )
                                            .child(
                                                div()
                                                    .ml_auto()
                                                    .text_xs()
                                                    .text_color(muted_fg)
                                                    .child("Open Spotify"),
                                            ),
                                    ),
                            )
                            // Footer
                            .child(
                                h_flex()
                                    .px_2()
                                    .py_1()
                                    .gap_3()
                                    .border_t_1()
                                    .border_color(border)
                                    .bg(cx.theme().title_bar)
                                    .text_xs()
                                    .text_color(muted_fg)
                                    .child(
                                        h_flex()
                                            .gap_1()
                                            .child(
                                                div()
                                                    .px(px(4.0))
                                                    .py(px(1.0))
                                                    .bg(muted)
                                                    .rounded(px(2.0))
                                                    .child("↑↓"),
                                            )
                                            .child("navigate"),
                                    )
                                    .child(
                                        h_flex()
                                            .gap_1()
                                            .child(
                                                div()
                                                    .px(px(4.0))
                                                    .py(px(1.0))
                                                    .bg(muted)
                                                    .rounded(px(2.0))
                                                    .child("↵"),
                                            )
                                            .child("select"),
                                    )
                                    .child(
                                        h_flex()
                                            .gap_1()
                                            .child(
                                                div()
                                                    .px(px(4.0))
                                                    .py(px(1.0))
                                                    .bg(muted)
                                                    .rounded(px(2.0))
                                                    .child("esc"),
                                            )
                                            .child("close"),
                                    ),
                            ),
                    )
                }),
        )
        // Right side - add button, settings, and help
        .child(
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
                                .child(
                                    Icon::new(IconName::Plus)
                                        .size(px(14.0))
                                        .text_color(muted_fg),
                                )
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
                        .child(
                            Icon::new(IconName::Settings)
                                .size(px(14.0))
                                .text_color(muted_fg),
                        )
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.toggle_settings(cx);
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
                ),
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
    cx: &Context<Humanboard>,
) -> Div {
    let bg = cx.theme().title_bar;
    let border = cx.theme().border;
    let fg = cx.theme().foreground;
    let muted_fg = cx.theme().muted_foreground;
    let success = cx.theme().success;

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
                // Save state indicator
                .child(if is_dirty {
                    div().text_color(muted_fg).child("Saving...")
                } else {
                    div().text_color(success).child("Saved")
                }),
        )
        .when_some(selected_item_name, |d, name| {
            d.child(div().text_color(fg).child(name))
        })
}

/// Render a keyboard key badge
fn render_kbd(key: &str, cx: &Context<Humanboard>) -> Div {
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
fn render_shortcut_row(key: &str, description: &str, cx: &Context<Humanboard>) -> Div {
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
fn render_shortcut_section(
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

/// Render the keyboard shortcuts overlay modal
pub fn render_shortcuts_overlay(cx: &mut Context<Humanboard>) -> impl IntoElement {
    let bg = cx.theme().popover;
    let border = cx.theme().border;
    let fg = cx.theme().foreground;

    deferred(
        div()
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .bg(hsla(0.0, 0.0, 0.0, 0.8))
            .flex()
            .items_center()
            .justify_center()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.show_shortcuts = false;
                    cx.notify();
                }),
            )
            .child(
                v_flex()
                    .w(px(420.0))
                    .bg(bg)
                    .border_1()
                    .border_color(border)
                    .rounded(px(16.0))
                    .overflow_hidden()
                    .shadow_lg()
                    .on_mouse_down(MouseButton::Left, |_, _, _| {})
                    // Header
                    .child(
                        h_flex()
                            .px_5()
                            .py_4()
                            .border_b_1()
                            .border_color(border)
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(fg)
                                    .child("Keyboard Shortcuts"),
                            )
                            .child(render_kbd("Cmd+/", cx)),
                    )
                    // Content
                    .child(
                        v_flex()
                            .p_5()
                            .gap_5()
                            .child(render_shortcut_section(
                                "General",
                                vec![
                                    ("Cmd+K", "Command palette"),
                                    ("Cmd+N", "New board"),
                                    ("Cmd+H", "Go home"),
                                    ("Cmd+O", "Open file"),
                                    ("Cmd+,", "Settings"),
                                    ("Cmd+Q", "Quit"),
                                ],
                                cx,
                            ))
                            .child(render_shortcut_section(
                                "Canvas",
                                vec![
                                    ("Cmd+=", "Zoom in"),
                                    ("Cmd+-", "Zoom out"),
                                    ("Cmd+0", "Reset zoom"),
                                    ("Cmd+D", "Duplicate selected"),
                                    ("Del", "Delete selected"),
                                    ("Cmd+Z", "Undo"),
                                    ("Cmd+Shift+Z", "Redo"),
                                    ("Esc", "Close palette/preview"),
                                ],
                                cx,
                            ))
                            .child(render_shortcut_section(
                                "PDF Preview",
                                vec![
                                    ("T", "Toggle split"),
                                    ("←  →", "Prev / Next page"),
                                    ("+ - 0", "Zoom PDF"),
                                    ("Cmd+]  [", "Next / Prev tab"),
                                    ("Cmd+W", "Close tab"),
                                    ("Esc", "Close preview"),
                                ],
                                cx,
                            )),
                    ),
            ),
    )
    .with_priority(1000)
}

/// Render the command palette popup (legacy full-screen version)
pub fn render_command_palette(
    input: &Entity<InputState>,
    search_results: &[(u64, String)],
    selected_result: usize,
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
            .bg(hsla(0.0, 0.0, 0.0, 0.6))
            .items_center()
            .pt(px(120.0))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| this.hide_command_palette(cx)),
            )
            .on_scroll_wheel(|_, _, _| {})
            .child(
                v_flex()
                    .w(px(500.0))
                    .max_h(px(400.0))
                    .flex_shrink_0()
                    .bg(bg)
                    .border_1()
                    .border_color(border)
                    .rounded(px(12.0))
                    .shadow_lg()
                    .overflow_hidden()
                    .key_context("CommandPalette")
                    // Intercept Input's MoveUp/MoveDown/Enter actions to navigate results
                    .on_action(cx.listener(|this, _: &MoveUp, _, cx| {
                        this.select_prev_result(cx);
                    }))
                    .on_action(cx.listener(|this, _: &MoveDown, _, cx| {
                        this.select_next_result(cx);
                    }))
                    .on_action(cx.listener(|this, _: &Enter, _, cx| {
                        this.execute_command_from_action(cx);
                    }))
                    .on_action(cx.listener(|this, _: &CmdPaletteUp, _, cx| {
                        this.select_prev_result(cx);
                    }))
                    .on_action(cx.listener(|this, _: &CmdPaletteDown, _, cx| {
                        this.select_next_result(cx);
                    }))
                    .on_action(cx.listener(|this, _: &CmdPaletteSelect, _, cx| {
                        this.execute_command_from_action(cx);
                    }))
                    .on_mouse_down(MouseButton::Left, |_, _, _| {})
                    .on_scroll_wheel(|_, _, _| {})
                    .child(
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
                            ),
                    )
                    .child(
                        div()
                            .id("command-palette-results")
                            .flex_1()
                            .overflow_y_scroll()
                            .on_scroll_wheel(|_, _, _| {})
                            .when(has_results, |d| {
                                d.child(v_flex().py_2().children(
                                    search_results.iter().enumerate().map(
                                        |(idx, (item_id, name))| {
                                            let is_selected = idx == selected_result;
                                            let item_id = *item_id;

                                            h_flex()
                                                .id(ElementId::Name(
                                                    format!("result-{}", item_id).into(),
                                                ))
                                                .pl(px(12.0))
                                                .pr_4()
                                                .py_2()
                                                .gap_3()
                                                .cursor(CursorStyle::PointingHand)
                                                .when(is_selected, |d| {
                                                    d.bg(list_active)
                                                        .border_l_2()
                                                        .border_color(primary)
                                                })
                                                .when(!is_selected, |d| {
                                                    d.hover(|s| s.bg(list_hover))
                                                })
                                                .on_click(cx.listener(move |this, _, _, cx| {
                                                    this.pending_command =
                                                        Some(format!("__jump:{}", item_id));
                                                    this.command_palette = None;
                                                    this.search_results.clear();
                                                    cx.notify();
                                                }))
                                                .child(
                                                    Icon::new(IconName::File)
                                                        .size(px(16.0))
                                                        .text_color(if is_selected {
                                                            primary
                                                        } else {
                                                            muted_fg
                                                        }),
                                                )
                                                .child(
                                                    div()
                                                        .flex_1()
                                                        .text_sm()
                                                        .text_color(if is_selected {
                                                            fg
                                                        } else {
                                                            muted_fg
                                                        })
                                                        .overflow_hidden()
                                                        .whitespace_nowrap()
                                                        .child(name.clone()),
                                                )
                                                .when(is_selected, |d| {
                                                    d.child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(muted_fg)
                                                            .child("↵ jump"),
                                                    )
                                                })
                                        },
                                    ),
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
                                                        .child(
                                                            div()
                                                                .text_sm()
                                                                .text_color(muted_fg)
                                                                .child("<name>"),
                                                        ),
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
                            .when(
                                !has_results && !show_md_hint && !current_text.is_empty(),
                                |d| {
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
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(muted_fg)
                                                    .child("No items found"),
                                            ),
                                    )
                                },
                            ),
                    )
                    .child(
                        h_flex()
                            .px_4()
                            .py_2()
                            .gap_4()
                            .border_t_1()
                            .border_color(border)
                            .text_xs()
                            .text_color(muted_fg)
                            .child(h_flex().gap_1().child("↑↓").child("navigate"))
                            .child(h_flex().gap_1().child("↵").child("select")),
                    ),
            ),
    )
    .with_priority(2000)
}

/// Render the Spotify connect/disconnect button
fn render_spotify_button(
    connected: bool,
    connecting: bool,
    muted_fg: Hsla,
    cx: &mut Context<Humanboard>,
) -> Stateful<Div> {
    let success = cx.theme().success;
    let danger_color = Hsla::from(gpui::rgb(0xef4444));
    let spotify_green = Hsla::from(gpui::rgb(0x1DB954));
    let spotify_green_hover = Hsla::from(gpui::rgb(0x1ed760));

    div()
        .id("spotify-btn")
        .px_3()
        .py_1p5()
        .rounded(px(6.0))
        .when(connected, |d| {
            d.bg(danger_color.opacity(0.1))
                .border_1()
                .border_color(danger_color.opacity(0.3))
                .cursor(CursorStyle::PointingHand)
                .hover(|s| s.bg(danger_color.opacity(0.2)))
                .child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(
                            div()
                                .w(px(6.0))
                                .h(px(6.0))
                                .rounded_full()
                                .bg(success),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(danger_color)
                                .child("Disconnect"),
                        ),
                )
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, _, cx| {
                        this.disconnect_spotify(cx);
                    }),
                )
        })
        .when(connecting && !connected, |d| {
            d.bg(muted_fg.opacity(0.1)).child(
                div()
                    .text_sm()
                    .text_color(muted_fg)
                    .child("Connecting..."),
            )
        })
        .when(!connected && !connecting, |d| {
            d.bg(spotify_green)
                .cursor(CursorStyle::PointingHand)
                .hover(|s| s.bg(spotify_green_hover))
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(Hsla::from(gpui::rgb(0x000000)))
                        .child("Connect"),
                )
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, _, cx| {
                        this.start_spotify_connect(cx);
                    }),
                )
        })
}

/// Render a setting row with title, description, and control on the right
fn render_setting_row(
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
        .items_start()
        .justify_between()
        .gap_4()
        .child(
            v_flex()
                .flex_1()
                .gap(px(2.0))
                .child(
                    div()
                        .text_sm()
                        .text_color(fg)
                        .child(title.to_string()),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(muted_fg)
                        .child(description.to_string()),
                ),
        )
        .child(control)
}

/// Render a section header
fn render_section_header(title: &str, cx: &Context<Humanboard>) -> Div {
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

/// Render the settings modal
pub fn render_settings_modal(
    current_theme: &str,
    _theme_index: usize,
    _theme_scroll: &ScrollHandle,
    active_tab: SettingsTab,
    spotify_connecting: bool,
    cx: &mut Context<Humanboard>,
) -> impl IntoElement {
    let themes = Settings::available_themes(cx);
    let current_theme_display = current_theme.to_string();
    let spotify_connected = spotify_auth::is_connected();

    let bg = cx.theme().background;
    let border = cx.theme().border;
    let fg = cx.theme().foreground;
    let muted_fg = cx.theme().muted_foreground;
    let title_bar = cx.theme().title_bar;
    let _primary = cx.theme().primary;
    let list_active = cx.theme().list_active;
    let list_hover = cx.theme().list_hover;
    let input_bg = cx.theme().input;

    deferred(
        div()
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .bg(hsla(0.0, 0.0, 0.0, 0.6))
            .flex()
            .items_center()
            .justify_center()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.show_settings = false;
                    cx.notify();
                }),
            )
            .on_scroll_wheel(cx.listener(|_, _, _, _| {}))
            .child(
                div()
                    .p_4()
                    .child(
                        h_flex()
                            .w(px(680.0))
                            .h(px(480.0))
                            .bg(bg)
                            .border_1()
                            .border_color(border)
                            .rounded(px(10.0))
                            .overflow_hidden()
                            .shadow_lg()
                            .on_mouse_down(MouseButton::Left, |_, _, _| {})
                            .on_scroll_wheel(|_, _, _| {})
                            .key_context("SettingsModal")
                            // Left sidebar
                            .child(
                                v_flex()
                                    .w(px(180.0))
                                    .h_full()
                                    .bg(title_bar)
                                    .border_r_1()
                                    .border_color(border)
                                    .p_2()
                                    .gap_1()
                                    // Search input placeholder
                                    .child(
                                        h_flex()
                                            .w_full()
                                            .h(px(28.0))
                                            .mb_2()
                                            .px_2()
                                            .bg(input_bg)
                                            .border_1()
                                            .border_color(border)
                                            .rounded(px(6.0))
                                            .items_center()
                                            .gap_2()
                                            .child(
                                                Icon::new(IconName::Search)
                                                    .size(px(12.0))
                                                    .text_color(muted_fg),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(muted_fg)
                                                    .child("Search settings..."),
                                            ),
                                    )
                                    // Appearance tab
                                    .child(
                                        div()
                                            .id("tab-appearance")
                                            .w_full()
                                            .px_2()
                                            .py_1p5()
                                            .rounded(px(4.0))
                                            .cursor(CursorStyle::PointingHand)
                                            .when(active_tab == SettingsTab::Appearance, |d| {
                                                d.bg(list_active)
                                            })
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
                                                            .text_color(if active_tab == SettingsTab::Appearance {
                                                                fg
                                                            } else {
                                                                muted_fg
                                                            }),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .text_color(if active_tab == SettingsTab::Appearance {
                                                                fg
                                                            } else {
                                                                muted_fg
                                                            })
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
                                            .when(active_tab == SettingsTab::Integrations, |d| {
                                                d.bg(list_active)
                                            })
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
                                                            .text_color(if active_tab == SettingsTab::Integrations {
                                                                fg
                                                            } else {
                                                                muted_fg
                                                            }),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .text_color(if active_tab == SettingsTab::Integrations {
                                                                fg
                                                            } else {
                                                                muted_fg
                                                            })
                                                            .child("Integrations"),
                                                    ),
                                            ),
                                    ),
                            )
                            // Right content area
                            .child(
                                div()
                                    .id("settings-content")
                                    .flex_1()
                                    .h_full()
                                    .overflow_y_scroll()
                                    .p_6()
                                    // Content - Appearance tab
                                    .when(active_tab == SettingsTab::Appearance, |d| {
                                        d.child(
                                            v_flex()
                                                .gap_4()
                                                // Section: Theme
                                                .child(render_section_header("Theme", cx))
                                                .child(render_setting_row(
                                                    "Theme",
                                                    "Choose a color theme for the interface",
                                                    // Theme dropdown
                                                    div()
                                                        .id("theme-dropdown")
                                                        .relative()
                                                        .child(
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
                                                                        .child(current_theme_display.clone()),
                                                                )
                                                                .child(
                                                                    Icon::new(IconName::ChevronDown)
                                                                        .size(px(12.0))
                                                                        .text_color(muted_fg),
                                                                ),
                                                        ),
                                                    cx,
                                                ))
                                                // Theme dropdown menu (shown when open)
                                                .when(cx.try_global::<ThemeDropdownOpen>().is_some(), |d| {
                                                    d.child(
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
                                                                let is_current = theme_name == &current_theme_display;
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
                                                            })),
                                                    )
                                                }),
                                        )
                                    })
                                    // Content - Integrations tab
                                    .when(active_tab == SettingsTab::Integrations, |d| {
                                        d.child(
                                            v_flex()
                                                .gap_4()
                                                // Section: Music
                                                .child(render_section_header("Music", cx))
                                                .child(render_setting_row(
                                                    "Spotify",
                                                    "Connect to play full tracks instead of 30-second previews. Requires Premium.",
                                                    render_spotify_button(spotify_connected, spotify_connecting, muted_fg, cx),
                                                    cx,
                                                )),
                                        )
                                    }),
                            ),
                    ),
            ),
    )
    .with_priority(1500)
}

/// Global marker for theme dropdown state
#[derive(Clone)]
pub struct ThemeDropdownOpen;

impl gpui::Global for ThemeDropdownOpen {}
