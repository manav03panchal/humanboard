//! Preview panel rendering - tabs, content, splitter
//!
//! This module handles the preview panel for PDFs and Markdown files including:
//! - Tab bar with multiple file support
//! - PDF viewer with navigation
//! - Markdown preview and editing
//! - Resizable splitter

use crate::app::{Humanboard, PreviewTab, SplitDirection};
use crate::loading::render_loading_spinner;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::input::Input;
use gpui_component::{ActiveTheme as _, Icon, IconName, Sizable, h_flex, v_flex};
use std::path::PathBuf;

/// Render the tab bar for the preview panel
pub fn render_tab_bar(
    tabs: &Vec<PreviewTab>,
    active_tab: usize,
    scroll_handle: &ScrollHandle,
    cx: &mut Context<Humanboard>,
) -> Stateful<Div> {
    let bg = cx.theme().title_bar;
    let border = cx.theme().border;
    let fg = cx.theme().foreground;
    let muted_fg = cx.theme().muted_foreground;
    let list_active = cx.theme().list_active;
    let list_hover = cx.theme().list_hover;
    let primary = cx.theme().primary;
    let danger = cx.theme().danger;

    div()
        .id("preview-tab-bar")
        .h(px(36.0))
        .w_full()
        .bg(bg)
        .border_b_1()
        .border_color(border)
        .flex()
        .items_center()
        .overflow_x_scroll()
        .track_scroll(scroll_handle)
        .child(
            h_flex()
                .flex_shrink_0()
                .children(tabs.iter().enumerate().map(|(index, tab)| {
                    let is_active = index == active_tab;
                    let filename = tab.title();
                    let is_markdown = matches!(tab, PreviewTab::Markdown { .. });
                    let is_code = matches!(tab, PreviewTab::Code { .. });
                    let is_dirty = tab.is_dirty();

                    let display_name = if filename.len() > 20 {
                        format!("{}...", &filename[..17])
                    } else {
                        filename
                    };

                    let tab_index = index;
                    let tab_index_close = index;

                    h_flex()
                        .id(ElementId::Name(format!("tab-{}", index).into()))
                        .flex_shrink_0()
                        .gap_2()
                        .px_3()
                        .py_1()
                        .bg(if is_active { list_active } else { bg })
                        .border_r_1()
                        .border_color(border)
                        .hover(|style| style.bg(list_hover))
                        .cursor(CursorStyle::PointingHand)
                        .on_click(cx.listener(move |this, _event, _window, cx| {
                            this.switch_tab(tab_index, cx);
                        }))
                        .child(if is_code {
                            Icon::new(IconName::SquareTerminal)
                                .xsmall()
                                .text_color(hsla(40.0 / 360.0, 0.8, 0.6, 1.0)) // Orange for code
                        } else if is_markdown {
                            Icon::new(IconName::File).xsmall().text_color(primary)
                        } else {
                            Icon::new(IconName::File).xsmall().text_color(danger) // PDF
                        })
                        .child(
                            div()
                                .text_xs()
                                .whitespace_nowrap()
                                .text_color(if is_active { fg } else { muted_fg })
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
                                .when(is_dirty, |d| {
                                    // Show dot indicator when dirty
                                    d.child(div().w(px(8.0)).h(px(8.0)).rounded_full().bg(primary))
                                })
                                .when(!is_dirty, |d| {
                                    // Show close button when not dirty
                                    d.text_xs()
                                        .text_color(muted_fg)
                                        .hover(|style| style.bg(list_hover).text_color(fg))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _event, _window, cx| {
                                                this.close_tab(tab_index_close, cx);
                                            }),
                                        )
                                        .child("×")
                                }),
                        )
                })),
        )
}

/// Render the content area for a preview tab
pub fn render_tab_content(
    tab: &PreviewTab,
    _is_active: bool,
    _tab_index: usize,
    cx: &mut Context<Humanboard>,
) -> Div {
    let bg = cx.theme().background;
    let title_bar = cx.theme().title_bar;
    let border = cx.theme().border;
    let muted_fg = cx.theme().muted_foreground;

    match tab {
        PreviewTab::Pdf { webview, path: _ } => div()
            .size_full()
            .overflow_hidden()
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
                .bg(bg)
                .child(
                    // Content area
                    div()
                        .id("md-content-scroll")
                        .flex_1()
                        .overflow_y_scroll()
                        .bg(bg)
                        .when(!is_editing, |d| {
                            // Preview mode - show rendered markdown (scrollable)
                            d.child(crate::markdown_card::render_markdown_content(
                                content, 1.0, cx,
                            ))
                        })
                        .when(is_editing, |d| {
                            // Edit mode - code editor with markdown syntax highlighting
                            if let Some(ed) = editor {
                                d.child(Input::new(ed).h_full().appearance(false))
                            } else {
                                d.child(div().p_4().child(render_loading_spinner(
                                    "Loading editor...",
                                    cx.theme().primary,
                                    cx.theme().muted_foreground,
                                )))
                            }
                        }),
                )
                .child(
                    // Footer with action buttons
                    h_flex()
                        .h(px(40.0))
                        .bg(title_bar)
                        .border_t_1()
                        .border_color(border)
                        .items_center()
                        .justify_between()
                        .px_3()
                        .child(
                            div()
                                .text_xs()
                                .text_color(muted_fg)
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
        PreviewTab::Code {
            content,
            language,
            dirty,
            editor,
            ..
        } => {
            let is_dirty = *dirty;
            let lang = language.clone();
            let line_count = content.lines().count();

            v_flex()
                .size_full()
                .bg(bg)
                .child({
                    // Content area - always editable
                    let editor_entity = editor.clone();
                    let code_editor_focus = cx.focus_handle();
                    div()
                        .id("code-content-scroll")
                        .flex_1()
                        .overflow_y_scroll()
                        .bg(bg)
                        .track_focus(&code_editor_focus)
                        .key_context("CodeEditor")
                        .on_click(cx.listener(move |this, _event, window, cx| {
                            // Set focus context to CodeEditor and focus the editor
                            this.focus
                                .focus(crate::focus::FocusContext::CodeEditor, window);
                            code_editor_focus.focus(window);
                            if let Some(ref ed) = editor_entity {
                                ed.update(cx, |state, cx| {
                                    state.focus(window, cx);
                                });
                            }
                        }))
                        .child(if let Some(ed) = editor {
                            Input::new(ed).h_full().appearance(false).into_any_element()
                        } else {
                            div()
                                .p_4()
                                .child(render_loading_spinner(
                                    "Loading code...",
                                    cx.theme().primary,
                                    cx.theme().muted_foreground,
                                ))
                                .into_any_element()
                        })
                })
                .child(
                    // Footer with action buttons
                    h_flex()
                        .h(px(40.0))
                        .bg(title_bar)
                        .border_t_1()
                        .border_color(border)
                        .items_center()
                        .justify_between()
                        .px_3()
                        .child(
                            h_flex()
                                .gap_3()
                                .child(
                                    div()
                                        .px(px(6.0))
                                        .py(px(2.0))
                                        .bg(hsla(200.0 / 360.0, 0.4, 0.25, 1.0))
                                        .rounded(px(3.0))
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(hsla(200.0 / 360.0, 0.6, 0.8, 1.0))
                                        .child(lang.to_uppercase()),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(muted_fg)
                                        .child(format!("{} lines", line_count)),
                                ),
                        )
                        .when(is_dirty, |d| {
                            d.child(div().text_xs().text_color(muted_fg).child("⌘S to save"))
                        }),
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
    cx: &Context<Humanboard>,
) -> Div {
    let bg = cx.theme().background;
    let border = cx.theme().border;
    let fg = cx.theme().foreground;
    let muted_fg = cx.theme().muted_foreground;
    let danger = cx.theme().danger;
    let primary = cx.theme().primary;

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
        .bg(bg)
        .child(
            // Header bar - compact layout
            div()
                .flex()
                .items_center()
                .gap_2()
                .px_3()
                .py_2()
                .bg(bg)
                .border_b_1()
                .border_color(border)
                // PDF badge
                .child(
                    div()
                        .px_2()
                        .py(px(2.0))
                        .bg(danger)
                        .rounded(px(4.0))
                        .text_xs()
                        .text_color(fg)
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
                        .text_color(fg)
                        .child(display_name),
                )
                // Page indicator
                .child(
                    div()
                        .flex_shrink_0()
                        .text_sm()
                        .text_color(muted_fg)
                        .child(format!("{}/{}", current_page, page_count)),
                )
                // Zoom indicator
                .child(
                    div()
                        .flex_shrink_0()
                        .text_sm()
                        .text_color(primary)
                        .child(format!("{:.0}%", zoom * 100.0)),
                )
                // Keyboard hints
                .child(
                    div()
                        .flex_shrink_0()
                        .text_xs()
                        .text_color(muted_fg)
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
                .bg(bg)
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
                                .text_color(danger)
                                .text_sm()
                                .child("Failed to load PDF"),
                        )
                    } else {
                        d.child(render_loading_spinner("Loading page...", primary, muted_fg))
                    }
                }),
        )
}

/// Render the resizable splitter between canvas and preview
pub fn render_splitter(direction: SplitDirection, cx: &mut Context<Humanboard>) -> Div {
    let title_bar = cx.theme().title_bar;
    let list_hover = cx.theme().list_hover;
    let border = cx.theme().border;

    match direction {
        SplitDirection::Vertical => div()
            .w(px(8.0))
            .h_full()
            .bg(title_bar)
            .hover(|s| s.bg(list_hover))
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
            .child(div().w(px(2.0)).h(px(40.0)).bg(border).rounded(px(1.0))),
        SplitDirection::Horizontal => div()
            .h(px(8.0))
            .w_full()
            .bg(title_bar)
            .hover(|s| s.bg(list_hover))
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
            .child(div().h(px(2.0)).w(px(40.0)).bg(border).rounded(px(1.0))),
    }
}

/// Render selected item label (placeholder)
pub fn render_selected_item_label(_name: String) -> Div {
    div().size_0()
}
