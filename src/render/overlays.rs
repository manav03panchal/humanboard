//! UI Overlays - header bar, footer, shortcuts modal, command palette
//!
//! This module contains all overlay UI elements that appear on top of the canvas:
//! - Header bar with navigation and command palette
//! - Footer bar with status info
//! - Keyboard shortcuts modal
//! - Command palette popup

use crate::app::Humanboard;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::input::{Input, InputState};
use gpui_component::{Icon, IconName, h_flex, v_flex};

/// Render the header bar with navigation and integrated command palette
pub fn render_header_bar(
    board_name: Option<String>,
    command_palette: Option<&Entity<InputState>>,
    search_results: &[(u64, String)],
    selected_result: usize,
    cx: &mut Context<Humanboard>,
) -> Div {
    let has_results = !search_results.is_empty();
    let is_open = command_palette.is_some();

    h_flex()
        .absolute()
        .top_0()
        .left_0()
        .right_0()
        .h(px(40.0))
        .bg(rgb(0x0a0a0a))
        .border_b_1()
        .border_color(rgb(0x222222))
        .items_center()
        .justify_between()
        .px_4()
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
                        .hover(|s| s.bg(rgb(0x333333)))
                        .text_sm()
                        .text_color(rgb(0xaaaaaa))
                        .child("←")
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.go_home(cx);
                        })),
                )
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0xffffff))
                        .child(board_name.unwrap_or_else(|| "Humanboard".to_string())),
                ),
        )
        // Center - command palette
        .child(
            v_flex()
                .id("cmd-palette-container")
                .w(px(400.0))
                .relative()
                .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                    if this.command_palette.is_some() {
                        match event.keystroke.key.as_str() {
                            "down" => this.select_next_result(cx),
                            "up" => this.select_prev_result(cx),
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
                        .bg(rgb(0x1a1a1a))
                        .border_1()
                        .border_color(if is_open {
                            rgb(0x444488)
                        } else {
                            rgb(0x2a2a2a)
                        })
                        .rounded(px(6.0))
                        .px_3()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            Icon::new(IconName::Search)
                                .size(px(14.0))
                                .text_color(rgb(0x666666)),
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
                                        .text_color(rgb(0x666666))
                                        .child("Search items or type command..."),
                                )
                                .child(
                                    div()
                                        .ml_auto()
                                        .text_xs()
                                        .text_color(rgb(0x555555))
                                        .child(":"),
                                )
                        }),
                )
                // Dropdown results
                .when(is_open, |d| {
                    d.child(
                        v_flex()
                            .absolute()
                            .top(px(32.0))
                            .left_0()
                            .w_full()
                            .max_h(px(300.0))
                            .bg(rgb(0x1a1a1a))
                            .border_1()
                            .border_color(rgb(0x333333))
                            .rounded(px(8.0))
                            .shadow_lg()
                            .overflow_hidden()
                            .child(
                                div()
                                    .id("cmd-dropdown-results")
                                    .max_h(px(250.0))
                                    .overflow_y_scroll()
                                    // Search results
                                    .when(has_results, |d| {
                                        d.child(v_flex().py_1().children(
                                            search_results.iter().enumerate().map(
                                                |(idx, (item_id, name))| {
                                                    let is_selected = idx == selected_result;
                                                    let item_id = *item_id;

                                                    h_flex()
                                                        .id(ElementId::Name(
                                                            format!("hdr-result-{}", item_id)
                                                                .into(),
                                                        ))
                                                        .px_3()
                                                        .py_1p5()
                                                        .gap_2()
                                                        .cursor(CursorStyle::PointingHand)
                                                        .when(is_selected, |d| d.bg(rgb(0x2a2a4a)))
                                                        .hover(|s| s.bg(rgb(0x252535)))
                                                        .on_click(cx.listener(
                                                            move |this, _, _, cx| {
                                                                this.pending_command = Some(
                                                                    format!("__jump:{}", item_id),
                                                                );
                                                                this.command_palette = None;
                                                                this.search_results.clear();
                                                                cx.notify();
                                                            },
                                                        ))
                                                        .child(
                                                            Icon::new(IconName::File)
                                                                .size(px(14.0))
                                                                .text_color(if is_selected {
                                                                    rgb(0x88aaff)
                                                                } else {
                                                                    rgb(0x666666)
                                                                }),
                                                        )
                                                        .child(
                                                            div()
                                                                .flex_1()
                                                                .text_sm()
                                                                .text_color(if is_selected {
                                                                    rgb(0xffffff)
                                                                } else {
                                                                    rgb(0xcccccc)
                                                                })
                                                                .overflow_hidden()
                                                                .whitespace_nowrap()
                                                                .child(name.clone()),
                                                        )
                                                        .when(is_selected, |d| {
                                                            d.child(
                                                                div()
                                                                    .text_xs()
                                                                    .text_color(rgb(0x888888))
                                                                    .child("↵"),
                                                            )
                                                        })
                                                },
                                            ),
                                        ))
                                    })
                                    // Command hint
                                    .when(!has_results, |d| {
                                        d.child(
                                            v_flex()
                                                .py_2()
                                                .child(
                                                    div()
                                                        .px_3()
                                                        .py_1()
                                                        .text_xs()
                                                        .text_color(rgb(0x555555))
                                                        .child("COMMANDS"),
                                                )
                                                .child(
                                                    h_flex()
                                                        .px_3()
                                                        .py_1p5()
                                                        .gap_2()
                                                        .child(
                                                            Icon::new(IconName::File)
                                                                .size(px(14.0))
                                                                .text_color(rgb(0x88ff88)),
                                                        )
                                                        .child(
                                                            h_flex().gap_1().child(
                                                                div()
                                                                    .text_sm()
                                                                    .text_color(rgb(0xcccccc))
                                                                    .child("md <name>"),
                                                            ),
                                                        )
                                                        .child(
                                                            div()
                                                                .ml_auto()
                                                                .text_xs()
                                                                .text_color(rgb(0x666666))
                                                                .child("create note"),
                                                        ),
                                                ),
                                        )
                                    }),
                            )
                            // Footer hints
                            .child(
                                h_flex()
                                    .px_3()
                                    .py_1p5()
                                    .gap_3()
                                    .border_t_1()
                                    .border_color(rgb(0x2a2a2a))
                                    .text_xs()
                                    .text_color(rgb(0x555555))
                                    .child(h_flex().gap_1().child("↑↓").child("nav"))
                                    .child(h_flex().gap_1().child("↵").child("go")),
                            ),
                    )
                }),
        )
        // Right side - shortcuts button
        .child(
            h_flex().gap_2().child(
                div()
                    .id("show-shortcuts-btn")
                    .px_2()
                    .py_1()
                    .rounded(px(4.0))
                    .cursor(CursorStyle::PointingHand)
                    .hover(|s| s.bg(rgb(0x333333)))
                    .text_sm()
                    .text_color(rgb(0xaaaaaa))
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
) -> Div {
    h_flex()
        .absolute()
        .bottom_0()
        .left_0()
        .right_0()
        .h(px(28.0))
        .bg(hsla(0.0, 0.0, 0.0, 0.95))
        .border_t_1()
        .border_color(hsla(0.0, 0.0, 0.3, 1.0))
        .items_center()
        .justify_between()
        .px_4()
        .gap_6()
        .text_xs()
        .text_color(rgb(0xaaaaaa))
        .child(
            h_flex()
                .gap_6()
                .child(
                    div()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xffffff))
                        .child(board_name.unwrap_or_else(|| "Humanboard".to_string())),
                )
                .child(div().child(format!("Items: {}", item_count)))
                .child(div().child(format!("Zoom: {:.2}x", zoom)))
                .child(div().child(format!(
                    "X: {:.0} Y: {:.0}",
                    f32::from(canvas_offset.x),
                    f32::from(canvas_offset.y)
                ))),
        )
        .when_some(selected_item_name, |d, name| {
            d.child(div().text_color(rgb(0xffffff)).child(name))
        })
}

/// Render the stats overlay (wrapper around footer_bar)
pub fn render_stats_overlay(
    fps: f32,
    frame_count: u64,
    item_count: usize,
    zoom: f32,
    canvas_offset: Point<Pixels>,
) -> Div {
    render_footer_bar(
        fps,
        frame_count,
        item_count,
        zoom,
        canvas_offset,
        None,
        None,
    )
}

/// Render a keyboard key badge
fn render_kbd(key: &str) -> Div {
    div()
        .px(px(8.0))
        .py(px(4.0))
        .bg(rgb(0x2a2a2a))
        .border_1()
        .border_color(rgb(0x3a3a3a))
        .rounded(px(6.0))
        .text_xs()
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgb(0x999999))
        .child(key.to_string())
}

/// Render a shortcut row with key and description
fn render_shortcut_row(key: &str, description: &str) -> Div {
    h_flex()
        .h(px(28.0))
        .items_center()
        .justify_between()
        .child(
            div()
                .text_sm()
                .text_color(rgb(0xcccccc))
                .child(description.to_string()),
        )
        .child(render_kbd(key))
}

/// Render a section of shortcuts with title
fn render_shortcut_section(title: &str, shortcuts: Vec<(&str, &str)>) -> Div {
    let mut section = v_flex().gap_1().child(
        div()
            .text_xs()
            .font_weight(FontWeight::BOLD)
            .text_color(rgb(0x666666))
            .mb_1()
            .child(title.to_string().to_uppercase()),
    );

    for (key, desc) in shortcuts {
        section = section.child(render_shortcut_row(key, desc));
    }

    section
}

/// Render the keyboard shortcuts overlay modal
pub fn render_shortcuts_overlay(cx: &mut Context<Humanboard>) -> impl IntoElement {
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
                    .bg(rgb(0x141414))
                    .border_1()
                    .border_color(rgb(0x2a2a2a))
                    .rounded(px(16.0))
                    .overflow_hidden()
                    .on_mouse_down(MouseButton::Left, |_, _, _| {})
                    // Header
                    .child(
                        h_flex()
                            .px_5()
                            .py_4()
                            .border_b_1()
                            .border_color(rgb(0x2a2a2a))
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0xffffff))
                                    .child("Keyboard Shortcuts"),
                            )
                            .child(render_kbd("Cmd+/")),
                    )
                    // Content
                    .child(
                        v_flex()
                            .p_5()
                            .gap_5()
                            .child(render_shortcut_section(
                                "General",
                                vec![
                                    (":", "Command palette / Search"),
                                    ("Cmd+N", "New board"),
                                    ("Cmd+H", "Go home"),
                                    ("Cmd+O", "Open file"),
                                    ("Cmd+Q", "Quit"),
                                ],
                            ))
                            .child(render_shortcut_section(
                                "Canvas",
                                vec![
                                    ("Cmd+=", "Zoom in"),
                                    ("Cmd+-", "Zoom out"),
                                    ("Cmd+0", "Reset zoom"),
                                    ("Del", "Delete selected"),
                                    ("Cmd+Z", "Undo"),
                                    ("Cmd+Shift+Z", "Redo"),
                                ],
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

    // Show command hints when input is empty or matches command prefix
    let show_md_hint = current_text.is_empty()
        || "md".starts_with(&current_text.to_lowercase())
        || current_text.to_lowercase().starts_with("md");

    deferred(
        v_flex()
            .absolute()
            .inset_0()
            .bg(hsla(0.0, 0.0, 0.0, 0.6))
            .items_center()
            .pt(px(120.0))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.hide_command_palette(cx);
                }),
            )
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                match event.keystroke.key.as_str() {
                    "down" => this.select_next_result(cx),
                    "up" => this.select_prev_result(cx),
                    _ => {}
                }
            }))
            .child(
                v_flex()
                    .w(px(500.0))
                    .max_h(px(400.0))
                    .flex_shrink_0()
                    .bg(rgb(0x1a1a1a))
                    .border_1()
                    .border_color(rgb(0x333333))
                    .rounded(px(12.0))
                    .shadow_lg()
                    .overflow_hidden()
                    .on_mouse_down(MouseButton::Left, |_, _, _| {})
                    // Search input
                    .child(
                        h_flex()
                            .px_4()
                            .py_3()
                            .gap_3()
                            .border_b_1()
                            .border_color(rgb(0x2a2a2a))
                            .child(
                                Icon::new(IconName::Search)
                                    .size(px(18.0))
                                    .text_color(rgb(0x666666)),
                            )
                            .child(
                                Input::new(input)
                                    .w_full()
                                    .appearance(false)
                                    .cleanable(false),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x555555))
                                    .child("click outside to close"),
                            ),
                    )
                    // Results section
                    .child(
                        div()
                            .id("command-palette-results")
                            .flex_1()
                            .overflow_y_scroll()
                            // Search results
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
                                                .px_4()
                                                .py_2()
                                                .gap_3()
                                                .cursor(CursorStyle::PointingHand)
                                                .when(is_selected, |d| d.bg(rgb(0x2a2a4a)))
                                                .hover(|s| s.bg(rgb(0x252535)))
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
                                                            rgb(0x88aaff)
                                                        } else {
                                                            rgb(0x666666)
                                                        }),
                                                )
                                                .child(
                                                    div()
                                                        .flex_1()
                                                        .text_sm()
                                                        .text_color(if is_selected {
                                                            rgb(0xffffff)
                                                        } else {
                                                            rgb(0xcccccc)
                                                        })
                                                        .overflow_hidden()
                                                        .whitespace_nowrap()
                                                        .child(name.clone()),
                                                )
                                                .when(is_selected, |d| {
                                                    d.child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(0x888888))
                                                            .child("↵ jump"),
                                                    )
                                                })
                                        },
                                    ),
                                ))
                            })
                            // Command hints when no results
                            .when(!has_results && show_md_hint, |d| {
                                d.child(
                                    v_flex()
                                        .py_2()
                                        .child(
                                            div()
                                                .px_4()
                                                .py_1()
                                                .text_xs()
                                                .text_color(rgb(0x555555))
                                                .child("COMMANDS"),
                                        )
                                        .child(
                                            h_flex()
                                                .px_4()
                                                .py_2()
                                                .gap_3()
                                                .cursor(CursorStyle::PointingHand)
                                                .hover(|s| s.bg(rgb(0x252535)))
                                                .child(
                                                    Icon::new(IconName::File)
                                                        .size(px(16.0))
                                                        .text_color(rgb(0x88ff88)),
                                                )
                                                .child(
                                                    h_flex()
                                                        .flex_1()
                                                        .gap_2()
                                                        .child(render_kbd("md"))
                                                        .child(
                                                            div()
                                                                .text_sm()
                                                                .text_color(rgb(0x888888))
                                                                .child("<name>"),
                                                        ),
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(rgb(0x555555))
                                                        .child("create markdown note"),
                                                ),
                                        ),
                                )
                            })
                            // Empty state
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
                                                    .text_color(rgb(0x444444)),
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x666666))
                                                    .child("No items found"),
                                            ),
                                    )
                                },
                            ),
                    )
                    // Footer with hints
                    .child(
                        h_flex()
                            .px_4()
                            .py_2()
                            .gap_4()
                            .border_t_1()
                            .border_color(rgb(0x2a2a2a))
                            .text_xs()
                            .text_color(rgb(0x555555))
                            .child(h_flex().gap_1().child("↑↓").child("navigate"))
                            .child(h_flex().gap_1().child("↵").child("select")),
                    ),
            ),
    )
    .with_priority(2000)
}
