//! UI Overlays - header bar, footer, shortcuts modal, command palette, settings
//!
//! This module contains all overlay UI elements that appear on top of the canvas:
//! - Header bar with navigation and command palette
//! - Footer bar with status info
//! - Keyboard shortcuts modal
//! - Command palette popup
//! - Settings modal

use crate::actions::{CloseCommandPalette, CmdPaletteDown, CmdPaletteUp, ModalFocusTrap, OpenSettings};
use crate::app::{Humanboard, SettingsTab};
use crate::focus::FocusContext;
use crate::settings::Settings;

use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::input::{Escape, Input, InputState, MoveDown, MoveUp};
use gpui_component::{ActiveTheme as _, Icon, IconName, h_flex, v_flex};

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
                ),
        )
        // Center - command palette
        .child(
            v_flex()
                .id("cmd-palette-container")
                .w(px(400.0))
                .relative()
                .track_focus(palette_focus)
                .key_context(FocusContext::KEY_COMMAND_PALETTE)
                // Intercept Input's MoveUp/MoveDown actions to navigate results
                // Note: Enter is handled by Input's PressEnter subscription - don't duplicate!
                .on_action(cx.listener(|this, _: &MoveUp, _, cx| {
                    println!("[DEBUG] MoveUp action received!");
                    this.select_prev_result(cx);
                }))
                .on_action(cx.listener(|this, _: &MoveDown, _, cx| {
                    println!("[DEBUG] MoveDown action received!");
                    this.select_next_result(cx);
                }))
                .on_action(cx.listener(|this, _: &CmdPaletteUp, _, cx| {
                    if this.command_palette.is_some() {
                        println!("[DEBUG] CmdPaletteUp action - palette is open, navigating");
                        this.select_prev_result(cx);
                    }
                }))
                .on_action(cx.listener(|this, _: &CmdPaletteDown, _, cx| {
                    if this.command_palette.is_some() {
                        println!("[DEBUG] CmdPaletteDown action - palette is open, navigating");
                        this.select_next_result(cx);
                    }
                }))
                .on_action(cx.listener(|this, _: &CloseCommandPalette, window, cx| {
                    println!("[DEBUG] CloseCommandPalette action received!");
                    this.hide_command_palette(window, cx);
                }))
                // Direct key interception for arrow navigation (bypasses keybinding context issues)
                .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                    println!("[DEBUG] on_key_down on palette container: key={}", event.keystroke.key);
                    if this.command_palette.is_some() {
                        match &event.keystroke.key {
                            key if key == "up" => {
                                println!("[DEBUG] Handling UP key");
                                this.select_prev_result(cx);
                            }
                            key if key == "down" => {
                                println!("[DEBUG] Handling DOWN key");
                                this.select_next_result(cx);
                            }
                            _ => {}
                        }
                    }
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
                                .hover(|s| s.border_color(muted_fg.opacity(0.4)))
                                .active(|s| s.bg(muted.opacity(0.3)))
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
                                                                muted_fg
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
                cx.listener(|this, _, window, cx| this.hide_command_palette(window, cx)),
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
                    .key_context(FocusContext::KEY_COMMAND_PALETTE)
                    // Intercept Input's MoveUp/MoveDown actions to navigate results
                    // Note: Enter is handled by Input's PressEnter subscription - don't duplicate!
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
                    // Direct key interception for arrow navigation (bypasses keybinding context issues)
                    .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                        if this.command_palette.is_some() {
                            match &event.keystroke.key {
                                key if key == "up" => {
                                    this.select_prev_result(cx);
                                }
                                key if key == "down" => {
                                    this.select_next_result(cx);
                                }
                                _ => {}
                            }
                        }
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
    current_font: &str,
    _theme_index: usize,
    _theme_scroll: &ScrollHandle,
    active_tab: SettingsTab,
    modal_focus: &FocusHandle,
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
    let _primary = cx.theme().primary;
    let list_active = cx.theme().list_active;
    let list_hover = cx.theme().list_hover;
    let input_bg = cx.theme().input;

    deferred(
        div()
            .id("settings-backdrop")
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
                        // Force focus back to canvas
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
                    .bg(bg)
                    .border_1()
                    .border_color(border)
                    .rounded(px(10.0))
                    .overflow_hidden()
                    .shadow_lg()
                    .on_scroll_wheel(|_, _, _| {})
                    .on_action(cx.listener(|this, _: &OpenSettings, window, cx| {
                        this.toggle_settings(window, cx);
                    }))
                    // Focus trap: consume Tab/Shift+Tab to prevent focus escaping modal (accessibility)
                    .on_action(cx.listener(|_, _: &ModalFocusTrap, _, _| {
                        // No-op: consuming the event prevents focus from leaving the modal
                    }))
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
                                                    .text_color(
                                                        if active_tab == SettingsTab::Appearance {
                                                            fg
                                                        } else {
                                                            muted_fg
                                                        },
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(
                                                        if active_tab == SettingsTab::Appearance {
                                                            fg
                                                        } else {
                                                            muted_fg
                                                        },
                                                    )
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
                                                    .text_color(
                                                        if active_tab == SettingsTab::Integrations {
                                                            fg
                                                        } else {
                                                            muted_fg
                                                        },
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(
                                                        if active_tab == SettingsTab::Integrations {
                                                            fg
                                                        } else {
                                                            muted_fg
                                                        },
                                                    )
                                                    .child("Integrations"),
                                            ),
                                    ),
                            ),
                    )
                    // Right content area
                    .child(
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
                                        // Section: Theme
                                        .child(render_section_header("Theme", cx))
                                        .child(render_setting_row(
                                            "Theme",
                                            "Choose a color theme for the interface",
                                            // Theme dropdown
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
                                        // Section: Font
                                        .child(render_section_header("Font", cx))
                                        .child(render_setting_row(
                                            "Font Family",
                                            "Choose a font for the interface",
                                            // Font dropdown
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
                                                            .child(current_font_display.clone()),
                                                    )
                                                    .child(
                                                        Icon::new(IconName::ChevronDown)
                                                            .size(px(12.0))
                                                            .text_color(muted_fg),
                                                    ),
                                            ),
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
                            // Theme dropdown menu - rendered last so it appears on top
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
                            })
                            // Font dropdown menu - rendered last so it appears on top
                            .when(cx.try_global::<FontDropdownOpen>().is_some(), |d| {
                                d.child(
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
                                            let is_current = *font_name == current_font_display;
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
                                        })),
                                )
                            }),
                    ),
            ),
    )
    .with_priority(1500) // Settings modal z-index
}

/// Global marker for which settings dropdown is open
#[derive(Clone, PartialEq)]
pub enum SettingsDropdown {
    Theme,
    Font,
}

impl gpui::Global for SettingsDropdown {}

/// Legacy markers - kept for compatibility but using SettingsDropdown internally
#[derive(Clone)]
pub struct ThemeDropdownOpen;

impl gpui::Global for ThemeDropdownOpen {}

#[derive(Clone)]
pub struct FontDropdownOpen;

impl gpui::Global for FontDropdownOpen {}

/// Render the create board modal with name input and storage location picker
pub fn render_create_board_modal(
    input: &Entity<gpui_component::input::InputState>,
    current_location: &crate::app::StorageLocation,
    modal_focus: &FocusHandle,
    cx: &mut Context<Humanboard>,
) -> impl IntoElement {
    let bg = cx.theme().background;
    let border = cx.theme().border;
    let fg = cx.theme().foreground;
    let muted_fg = cx.theme().muted_foreground;
    let primary = cx.theme().primary;
    let list_hover = cx.theme().list_hover;
    let list_active = cx.theme().list_active;

    let location = current_location.clone();
    let is_icloud_available = crate::app::StorageLocation::ICloud.is_available();

    deferred(
        div()
            .id("create-board-backdrop")
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
                    this.create_board_backdrop_clicked = true;
                    cx.notify();
                }),
            )
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| {
                    if this.create_board_backdrop_clicked {
                        this.close_create_board_modal(window, cx);
                    }
                    this.create_board_backdrop_clicked = false;
                }),
            )
            .on_action(cx.listener(|this, _: &Escape, window, cx| {
                this.close_create_board_modal(window, cx);
            }))
            .child(
                v_flex()
                    .id("create-board-modal")
                    .track_focus(modal_focus)
                    .key_context(FocusContext::KEY_MODAL)
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, _| {
                            this.create_board_backdrop_clicked = false;
                        }),
                    )
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(|this, _, _, _| {
                            this.create_board_backdrop_clicked = false;
                        }),
                    )
                    .w(px(420.0))
                    .bg(bg)
                    .border_1()
                    .border_color(border)
                    .rounded(px(12.0))
                    .overflow_hidden()
                    .shadow_lg()
                    // Focus trap: consume Tab/Shift+Tab to prevent focus escaping modal (accessibility)
                    .on_action(cx.listener(|_, _: &ModalFocusTrap, _, _| {
                        // No-op: consuming the event prevents focus from leaving the modal
                    }))
                    // Header
                    .child(
                        h_flex()
                            .w_full()
                            .px(px(20.0))
                            .py(px(16.0))
                            .border_b_1()
                            .border_color(border)
                            .justify_between()
                            .child(
                                div()
                                    .text_size(px(16.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(fg)
                                    .child("Create New Board"),
                            )
                            .child(
                                div()
                                    .id("close-modal")
                                    .cursor_pointer()
                                    .p(px(4.0))
                                    .rounded(px(4.0))
                                    .hover(|s| s.bg(list_hover))
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.close_create_board_modal(window, cx);
                                    }))
                                    .child(
                                        Icon::new(IconName::Close)
                                            .size(px(16.0))
                                            .text_color(muted_fg),
                                    ),
                            ),
                    )
                    // Content
                    .child(
                        v_flex()
                            .w_full()
                            .p(px(20.0))
                            .gap(px(16.0))
                            // Board name input
                            .child(
                                v_flex()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(fg)
                                            .child("Board Name"),
                                    )
                                    .child(Input::new(input).w_full().cleanable(true)),
                            )
                            // Storage location picker
                            .child(
                                v_flex()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(fg)
                                            .child("Storage Location"),
                                    )
                                    .child({
                                        let is_default_selected = location == crate::app::StorageLocation::Default;
                                        let is_icloud_selected = location == crate::app::StorageLocation::ICloud;

                                        v_flex()
                                            .gap(px(4.0))
                                            // Default location option
                                            .child(
                                                h_flex()
                                                    .id("loc-local")
                                                    .w_full()
                                                    .px(px(12.0))
                                                    .py(px(10.0))
                                                    .rounded(px(6.0))
                                                    .bg(if is_default_selected { list_active } else { gpui::transparent_black() })
                                                    .cursor_pointer()
                                                    .hover(|s| s.bg(list_hover))
                                                    .on_click(cx.listener(|this, _, _, cx| {
                                                        this.set_create_board_location(
                                                            crate::app::StorageLocation::Default,
                                                            cx,
                                                        );
                                                    }))
                                                    .gap(px(12.0))
                                                    .child(
                                                        Icon::new(IconName::Folder)
                                                            .size(px(20.0))
                                                            .text_color(if is_default_selected { primary } else { muted_fg }),
                                                    )
                                                    .child(
                                                        v_flex()
                                                            .flex_1()
                                                            .gap(px(2.0))
                                                            .child(
                                                                div()
                                                                    .text_size(px(13.0))
                                                                    .font_weight(if is_default_selected { FontWeight::MEDIUM } else { FontWeight::NORMAL })
                                                                    .text_color(fg)
                                                                    .child("Local Storage"),
                                                            )
                                                            .child(
                                                                div()
                                                                    .text_size(px(11.0))
                                                                    .text_color(muted_fg)
                                                                    .child("~/Library/Application Support/humanboard"),
                                                            ),
                                                    )
                                                    .when(is_default_selected, |d| {
                                                        d.child(
                                                            Icon::new(IconName::Check)
                                                                .size(px(16.0))
                                                                .text_color(primary),
                                                        )
                                                    }),
                                            )
                                            // iCloud option
                                            .child(
                                                h_flex()
                                                    .id("loc-icloud")
                                                    .w_full()
                                                    .px(px(12.0))
                                                    .py(px(10.0))
                                                    .rounded(px(6.0))
                                                    .bg(if is_icloud_selected { list_active } else { gpui::transparent_black() })
                                                    .opacity(if is_icloud_available { 1.0 } else { 0.5 })
                                                    .when(is_icloud_available, |d| {
                                                        d.cursor_pointer()
                                                            .hover(|s| s.bg(list_hover))
                                                            .on_click(cx.listener(|this, _, _, cx| {
                                                                this.set_create_board_location(
                                                                    crate::app::StorageLocation::ICloud,
                                                                    cx,
                                                                );
                                                            }))
                                                    })
                                                    .when(!is_icloud_available, |d| {
                                                        d.cursor(CursorStyle::OperationNotAllowed)
                                                    })
                                                    .gap(px(12.0))
                                                    .child(
                                                        Icon::new(IconName::Globe)
                                                            .size(px(20.0))
                                                            .text_color(if is_icloud_selected { primary } else { muted_fg }),
                                                    )
                                                    .child(
                                                        v_flex()
                                                            .flex_1()
                                                            .gap(px(2.0))
                                                            .child(
                                                                div()
                                                                    .text_size(px(13.0))
                                                                    .font_weight(if is_icloud_selected { FontWeight::MEDIUM } else { FontWeight::NORMAL })
                                                                    .text_color(fg)
                                                                    .child("iCloud Drive"),
                                                            )
                                                            .child(
                                                                div()
                                                                    .text_size(px(11.0))
                                                                    .text_color(muted_fg)
                                                                    .child(if is_icloud_available { "Sync across devices" } else { "iCloud Drive not available" }),
                                                            ),
                                                    )
                                                    .when(is_icloud_selected, |d| {
                                                        d.child(
                                                            Icon::new(IconName::Check)
                                                                .size(px(16.0))
                                                                .text_color(primary),
                                                        )
                                                    }),
                                            )
                                    }),
                            ),
                    )
                    // Footer with buttons
                    .child(
                        h_flex()
                            .w_full()
                            .px(px(20.0))
                            .py(px(16.0))
                            .border_t_1()
                            .border_color(border)
                            .justify_end()
                            .gap(px(12.0))
                            .child(
                                Button::new("cancel")
                                    .label("Cancel")
                                    .tooltip("Cancel board creation")
                                    .ghost()
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.close_create_board_modal(window, cx);
                                    })),
                            )
                            .child(
                                Button::new("create")
                                    .label("Create Board")
                                    .tooltip("Create new board with these settings")
                                    .primary()
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.confirm_create_board(window, cx);
                                    })),
                            ),
                    ),
            ),
    )
    .with_priority(1600) // Create Board modal z-index (above Settings modal at 1500)
}
