//! PDF preview panel rendering module.
//!
//! Handles rendering of the PDF preview panel including tabs and WebView containers.

use gpui::prelude::FluentBuilder;
use gpui::*;
use std::path::PathBuf;

use crate::app::{Humanboard, PdfTab};

/// Render the preview panel container
pub fn render_preview_container(
    width: Option<DefiniteLength>,
    height: Option<DefiniteLength>,
    tabs: &[PdfTab],
    active_tab: usize,
    cx: &mut Context<Humanboard>,
) -> Div {
    let mut container = div()
        .flex_shrink_0()
        .bg(rgb(0x000000))
        .flex()
        .flex_col()
        .overflow_hidden();

    // Apply width or height based on split direction
    if let Some(w) = width {
        container = container.w(w).h_full();
    }
    if let Some(h) = height {
        container = container.h(h).w_full();
    }

    container
        .child(render_tab_bar(tabs, active_tab, cx))
        .child(render_webview_container(tabs, active_tab))
}

/// Render the tab bar for the preview panel
pub fn render_tab_bar(tabs: &[PdfTab], active_tab: usize, cx: &mut Context<Humanboard>) -> Div {
    div()
        .h(px(36.0))
        .w_full()
        .bg(rgb(0x000000))
        .border_b_1()
        .border_color(rgb(0x333333))
        .flex()
        .items_center()
        .overflow_x_hidden()
        .children(
            tabs.iter()
                .enumerate()
                .map(|(index, tab)| render_tab(tab, index, index == active_tab, cx)),
        )
}

/// Render a single tab
fn render_tab(tab: &PdfTab, index: usize, is_active: bool, cx: &mut Context<Humanboard>) -> Div {
    let display_name = get_truncated_filename(&tab.path, 20);

    div()
        .flex()
        .items_center()
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
        .group("tab")
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _event, _window, cx| {
                this.switch_tab(index, cx);
            }),
        )
        .child(render_tab_label(&display_name, is_active))
        .child(render_tab_close_button(index, is_active, cx))
}

/// Render the tab label
fn render_tab_label(name: &str, is_active: bool) -> Div {
    div()
        .text_xs()
        .text_color(if is_active {
            rgb(0xffffff)
        } else {
            rgb(0x888888)
        })
        .child(name.to_string())
}

/// Render the tab close button
fn render_tab_close_button(index: usize, is_active: bool, cx: &mut Context<Humanboard>) -> Div {
    div()
        .w(px(14.0))
        .h(px(14.0))
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(2.0))
        .text_color(if is_active {
            rgb(0x1a1a1a)
        } else {
            rgb(0x000000)
        })
        .text_xs()
        .hover(|style| style.bg(rgb(0x444444)).text_color(rgb(0xffffff)))
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _event, _window, cx| {
                this.close_tab(index, cx);
            }),
        )
        .child("×")
}

/// Render the WebView container with all tabs
fn render_webview_container(tabs: &[PdfTab], active_tab: usize) -> Div {
    div()
        .flex_1()
        .relative()
        .overflow_hidden()
        .children(tabs.iter().enumerate().map(|(index, tab)| {
            let is_active = index == active_tab;
            div()
                .absolute()
                .when(is_active, |d| d.size_full())
                .when(!is_active, |d| d.size_0())
                .when_some(tab.webview.as_ref().map(|wv| wv.webview()), |d, wv| {
                    d.child(wv)
                })
        }))
}

/// Get a truncated filename for display
fn get_truncated_filename(path: &PathBuf, max_len: usize) -> String {
    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Unknown")
        .to_string();

    if filename.len() > max_len {
        format!("{}...", &filename[..max_len.saturating_sub(3)])
    } else {
        filename
    }
}

/// Render a standalone preview panel (for header-based preview)
pub fn render_preview_panel(
    file_name: String,
    page_image_path: Option<PathBuf>,
    current_page: usize,
    page_count: usize,
    zoom: f32,
) -> Div {
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
        .child(render_preview_header(
            &display_name,
            current_page,
            page_count,
            zoom,
        ))
        .child(render_preview_content(page_image_path, page_count))
}

/// Render the preview header bar
fn render_preview_header(
    display_name: &str,
    current_page: usize,
    page_count: usize,
    zoom: f32,
) -> Div {
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
        // Filename
        .child(
            div()
                .flex_1()
                .min_w(px(0.0))
                .overflow_hidden()
                .text_sm()
                .text_color(rgb(0xffffff))
                .child(display_name.to_string()),
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
                .child("Scroll=Pan • Cmd+Scroll=Zoom • T=Split"),
        )
}

/// Render the preview content area
fn render_preview_content(page_image_path: Option<PathBuf>, page_count: usize) -> Div {
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
        })
}
