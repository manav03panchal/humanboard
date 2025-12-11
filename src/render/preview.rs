//! Preview panel rendering - tabs, content, splitter
//!
//! This module handles the preview panel for PDFs and Markdown files including:
//! - Tab bar with multiple file support
//! - PDF viewer with navigation
//! - Markdown preview and editing
//! - Resizable splitter

use crate::app::{Humanboard, PreviewTab, SplitDirection};
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::input::Input;
use gpui_component::{Icon, IconName, Sizable, h_flex, v_flex};
use std::path::PathBuf;

/// Render the tab bar for the preview panel
pub fn render_tab_bar(
    tabs: &Vec<PreviewTab>,
    active_tab: usize,
    cx: &mut Context<Humanboard>,
) -> Div {
    h_flex()
        .h(px(36.0))
        .w_full()
        .bg(rgb(0x000000))
        .border_b_1()
        .border_color(rgb(0x333333))
        .items_center()
        .overflow_x_hidden()
        .children(tabs.iter().enumerate().map(|(index, tab)| {
            let is_active = index == active_tab;
            let filename = tab.title();
            let is_markdown = matches!(tab, PreviewTab::Markdown { .. });

            let display_name = if filename.len() > 20 {
                format!("{}...", &filename[..17])
            } else {
                filename
            };

            let tab_index = index;
            let tab_index_close = index;

            h_flex()
                .id(ElementId::Name(format!("tab-{}", index).into()))
                .gap_2()
                .px_3()
                .py_1()
                .bg(if is_active {
                    rgb(0x1a1a1a)
                } else {
                    rgb(0x000000)
                })
                .border_r_1()
                .border_color(rgb(0x333333))
                .hover(|style| style.bg(rgb(0x2a2a2a)))
                .cursor(CursorStyle::PointingHand)
                .on_click(cx.listener(move |this, _event, _window, cx| {
                    this.switch_tab(tab_index, cx);
                }))
                .child(if is_markdown {
                    Icon::new(IconName::File).xsmall().text_color(rgb(0x8888ff))
                } else {
                    Icon::new(IconName::File).xsmall().text_color(rgb(0xff6666))
                })
                .child(
                    div()
                        .text_xs()
                        .text_color(if is_active {
                            rgb(0xffffff)
                        } else {
                            rgb(0x888888)
                        })
                        .child(display_name),
                )
                .child(
                    div()
                        .w(px(14.0))
                        .h(px(14.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded(px(2.0))
                        .text_xs()
                        .text_color(rgb(0x666666))
                        .hover(|style| style.bg(rgb(0x444444)).text_color(rgb(0xffffff)))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _event, _window, cx| {
                                this.close_tab(tab_index_close, cx);
                            }),
                        )
                        .child("×"),
                )
        }))
}

/// Render the content area for a preview tab
pub fn render_tab_content(
    tab: &PreviewTab,
    _is_active: bool,
    _tab_index: usize,
    cx: &mut Context<Humanboard>,
) -> Div {
    match tab {
        PreviewTab::Pdf { webview, path: _ } => div()
            .size_full()
            .when_some(webview.as_ref(), |d, wv| d.child(wv.webview())),
        PreviewTab::Markdown {
            content,
            path: _,
            editing,
            editor,
        } => {
            let is_editing = *editing;

            v_flex()
                .size_full()
                .bg(rgb(0x000000))
                .child(
                    // Content area
                    div()
                        .id("md-content-scroll")
                        .flex_1()
                        .overflow_y_scroll()
                        .bg(rgb(0x000000))
                        .when(!is_editing, |d| {
                            // Preview mode - show rendered markdown (scrollable)
                            d.child(crate::markdown_card::render_markdown_content(content, 1.0))
                        })
                        .when(is_editing, |d| {
                            // Edit mode - code editor with markdown syntax highlighting
                            if let Some(ed) = editor {
                                d.child(Input::new(ed).h_full().appearance(false))
                            } else {
                                d.child(
                                    div()
                                        .p_4()
                                        .text_color(rgb(0x666666))
                                        .child("Loading editor..."),
                                )
                            }
                        }),
                )
                .child(
                    // Footer with action buttons
                    h_flex()
                        .h(px(40.0))
                        .bg(rgb(0x0a0a0a))
                        .border_t_1()
                        .border_color(rgb(0x2a2a2a))
                        .items_center()
                        .justify_between()
                        .px_3()
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgb(0x666666))
                                .child(format!("{} chars", content.len())),
                        )
                        .child(
                            h_flex()
                                .gap_2()
                                .when(is_editing, |d| {
                                    d.child(
                                        Button::new("save-markdown")
                                            .primary()
                                            .small()
                                            .label("Save")
                                            .on_click(cx.listener(|this, _, _window, cx| {
                                                this.save_markdown(cx);
                                            })),
                                    )
                                })
                                .child(if is_editing {
                                    Button::new("toggle-edit")
                                        .ghost()
                                        .small()
                                        .label("Cancel")
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.toggle_markdown_edit(window, cx);
                                        }))
                                } else {
                                    Button::new("toggle-edit")
                                        .primary()
                                        .small()
                                        .label("Edit")
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.toggle_markdown_edit(window, cx);
                                        }))
                                }),
                        ),
                )
        }
    }
}

/// Render the legacy preview panel (for PDF pages)
pub fn render_preview_panel(
    file_name: String,
    page_image_path: Option<PathBuf>,
    current_page: usize,
    page_count: usize,
    zoom: f32,
) -> Div {
    // Truncate filename if too long
    let display_name = if file_name.len() > 25 {
        format!("{}...", &file_name[..22])
    } else {
        file_name
    };

    div()
        .flex()
        .flex_col()
        .size_full()
        .bg(rgb(0x000000))
        .child(
            // Header bar - compact layout
            div()
                .flex()
                .items_center()
                .gap_2()
                .px_3()
                .py_2()
                .bg(rgb(0x000000))
                .border_b_1()
                .border_color(rgb(0x505050))
                // PDF badge
                .child(
                    div()
                        .px_2()
                        .py(px(2.0))
                        .bg(rgb(0xff6b6b))
                        .rounded(px(4.0))
                        .text_xs()
                        .text_color(rgb(0xffffff))
                        .flex_shrink_0()
                        .child("PDF"),
                )
                // Filename (truncated)
                .child(
                    div()
                        .flex_1()
                        .min_w(px(0.0))
                        .overflow_hidden()
                        .text_sm()
                        .text_color(rgb(0xffffff))
                        .child(display_name),
                )
                // Page indicator
                .child(
                    div()
                        .flex_shrink_0()
                        .text_sm()
                        .text_color(rgb(0xaaaaaa))
                        .child(format!("{}/{}", current_page, page_count)),
                )
                // Zoom indicator
                .child(
                    div()
                        .flex_shrink_0()
                        .text_sm()
                        .text_color(rgb(0x88aaff))
                        .child(format!("{:.0}%", zoom * 100.0)),
                )
                // Keyboard hints
                .child(
                    div()
                        .flex_shrink_0()
                        .text_xs()
                        .text_color(rgb(0x666666))
                        .child("Scroll=Pan • ⌘+Scroll=Zoom • T=Split"),
                ),
        )
        .child(
            // PDF content area
            div()
                .flex_1()
                .flex()
                .items_center()
                .justify_center()
                .overflow_hidden()
                .bg(rgb(0x000000))
                .when_some(page_image_path.clone(), |d, path| {
                    d.child(
                        img(path)
                            .max_w_full()
                            .max_h_full()
                            .object_fit(ObjectFit::Contain),
                    )
                })
                .when(page_image_path.is_none(), |d| {
                    if page_count == 0 {
                        d.child(
                            div()
                                .text_color(rgb(0xff6b6b))
                                .text_sm()
                                .child("Failed to load PDF"),
                        )
                    } else {
                        d.child(
                            div()
                                .text_color(rgb(0x888888))
                                .text_sm()
                                .child("Loading..."),
                        )
                    }
                }),
        )
}

/// Render the resizable splitter between canvas and preview
pub fn render_splitter(direction: SplitDirection, cx: &mut Context<Humanboard>) -> Div {
    match direction {
        SplitDirection::Vertical => div()
            .w(px(8.0))
            .h_full()
            .bg(rgb(0x0a0a0a))
            .hover(|s| s.bg(rgb(0x1a1a1a)))
            .cursor(CursorStyle::ResizeLeftRight)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event: &MouseDownEvent, _, cx| {
                    this.dragging_splitter = true;
                    this.splitter_drag_start = Some(event.position);
                    cx.notify();
                }),
            )
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .w(px(2.0))
                    .h(px(40.0))
                    .bg(rgb(0x333333))
                    .rounded(px(1.0)),
            ),
        SplitDirection::Horizontal => div()
            .h(px(8.0))
            .w_full()
            .bg(rgb(0x0a0a0a))
            .hover(|s| s.bg(rgb(0x1a1a1a)))
            .cursor(CursorStyle::ResizeUpDown)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event: &MouseDownEvent, _, cx| {
                    this.dragging_splitter = true;
                    this.splitter_drag_start = Some(event.position);
                    cx.notify();
                }),
            )
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .h(px(2.0))
                    .w(px(40.0))
                    .bg(rgb(0x333333))
                    .rounded(px(1.0)),
            ),
    }
}

/// Render selected item label (placeholder)
pub fn render_selected_item_label(_name: String) -> Div {
    div().size_0()
}
