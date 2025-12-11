use crate::actions::{
    ClosePreview, CloseTab, CommandPalette, DeleteSelected, GoHome, NewBoard, NextPage, NextTab,
    OpenFile, Paste, PdfZoomIn, PdfZoomOut, PdfZoomReset, PrevPage, PrevTab, Redo, ShowShortcuts,
    ToggleSplit, Undo, ZoomIn, ZoomOut, ZoomReset,
};
use crate::app::{AppView, Humanboard, PreviewTab, SplitDirection};
use crate::landing::render_landing_page;
use crate::markdown_card::render_collapsed_markdown;
use crate::types::{CanvasItem, ItemContent};
use crate::youtube_webview::YouTubeWebView;
use gpui::DefiniteLength::Fraction;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::input::{Input, InputState};
use gpui_component::{Icon, IconName, Sizable, h_flex, v_flex};
use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use std::collections::HashMap;

const UI_FONT: &str = "Iosevka Nerd Font";

/// Render markdown content as styled divs (legacy - now in markdown_card module)
#[allow(dead_code)]
fn render_markdown_content(content: &str) -> Div {
    let parser = Parser::new(content);
    let mut container = div().flex().flex_col().gap_2();
    let mut current_text = String::new();
    let mut _in_heading = false;
    let mut heading_level = 0u8;
    let mut _in_bold = false;
    let mut _in_italic = false;
    let mut _in_code = false;
    let mut _in_list = false;

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                _in_heading = true;
                heading_level = level as u8;
            }
            Event::End(TagEnd::Heading(_)) => {
                let text = std::mem::take(&mut current_text);
                let heading = match heading_level {
                    1 => div()
                        .text_2xl()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xffffff))
                        .pb_2()
                        .child(text),
                    2 => div()
                        .text_xl()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0xeeeeee))
                        .pb_1()
                        .child(text),
                    _ => div()
                        .text_lg()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(0xdddddd))
                        .child(text),
                };
                container = container.child(heading);
                _in_heading = false;
            }
            Event::Start(Tag::Paragraph) => {}
            Event::End(TagEnd::Paragraph) => {
                let text = std::mem::take(&mut current_text);
                if !text.is_empty() {
                    container = container
                        .child(div().text_sm().text_color(rgb(0xcccccc)).pb_2().child(text));
                }
            }
            Event::Start(Tag::List(_)) => {
                _in_list = true;
            }
            Event::End(TagEnd::List(_)) => {
                _in_list = false;
            }
            Event::Start(Tag::Item) => {
                current_text.push_str("• ");
            }
            Event::End(TagEnd::Item) => {
                let text = std::mem::take(&mut current_text);
                container =
                    container.child(div().text_sm().text_color(rgb(0xcccccc)).pl_4().child(text));
            }
            Event::Start(Tag::Strong) => {
                _in_bold = true;
            }
            Event::End(TagEnd::Strong) => {
                _in_bold = false;
            }
            Event::Start(Tag::Emphasis) => {
                _in_italic = true;
            }
            Event::End(TagEnd::Emphasis) => {
                _in_italic = false;
            }
            Event::Start(Tag::CodeBlock(_)) => {
                _in_code = true;
            }
            Event::End(TagEnd::CodeBlock) => {
                let text = std::mem::take(&mut current_text);
                container = container.child(
                    div()
                        .bg(rgb(0x0a0a0a))
                        .rounded(px(4.0))
                        .p_3()
                        .text_sm()
                        .text_color(rgb(0x88ff88))
                        .child(text),
                );
                _in_code = false;
            }
            Event::Code(code) => {
                current_text.push_str(&format!("`{}`", code));
            }
            Event::Text(text) => {
                current_text.push_str(&text);
            }
            Event::SoftBreak | Event::HardBreak => {
                current_text.push('\n');
            }
            _ => {}
        }
    }

    // Handle any remaining text
    if !current_text.is_empty() {
        container = container.child(
            div()
                .text_sm()
                .text_color(rgb(0xcccccc))
                .child(current_text),
        );
    }

    container
}

// Render helper functions
pub fn render_canvas(
    canvas_offset: Point<Pixels>,
    zoom: f32,
    items: Vec<CanvasItem>, // Must own items for 'static closure capture
) -> impl IntoElement {
    canvas(
        move |_bounds, _window, _cx| (),
        move |bounds, _data, window, _cx| {
            render_item_backgrounds(bounds, window, &items, canvas_offset, zoom);
        },
    )
    .absolute()
    .size_full()
}

fn render_item_backgrounds(
    bounds: Bounds<Pixels>,
    window: &mut Window,
    items: &[CanvasItem],
    canvas_offset: Point<Pixels>,
    zoom: f32,
) {
    for item in items {
        // Skip items that render themselves (images, markdown cards)
        if matches!(
            &item.content,
            ItemContent::Image(_) | ItemContent::Markdown { .. }
        ) {
            continue;
        }

        let item_bounds = Bounds {
            origin: point(
                bounds.origin.x + px(item.position.0 * zoom) + canvas_offset.x,
                bounds.origin.y + px(item.position.1 * zoom) + canvas_offset.y,
            ),
            size: size(px(item.size.0 * zoom), px(item.size.1 * zoom)),
        };

        let bg_color = match &item.content {
            ItemContent::Video(_) => hsla(0.15, 0.7, 0.5, 0.9),
            ItemContent::Text(_) => hsla(0.6, 0.7, 0.5, 0.9),
            ItemContent::Pdf { .. } => hsla(0.0, 0.7, 0.5, 0.9),
            ItemContent::Link(_) => hsla(0.35, 0.7, 0.5, 0.9),
            ItemContent::YouTube(_) => hsla(0.0, 0.8, 0.4, 0.9), // Red for YouTube
            _ => hsla(0.0, 0.0, 0.5, 0.9),
        };

        window.paint_quad(quad(
            item_bounds,
            px(8.0 * zoom),
            bg_color,
            px(2.0 * zoom),
            hsla(0.0, 0.0, 1.0, 0.3),
            Default::default(),
        ));
    }
}

/// Render a single canvas item based on its content type
fn render_item_content(
    item: &CanvasItem,
    zoom: f32,
    youtube_webviews: &HashMap<u64, YouTubeWebView>,
) -> Div {
    let corner_radius = px(8.0 * zoom);

    match &item.content {
        ItemContent::Image(path) => div()
            .size_full()
            .overflow_hidden()
            .rounded(corner_radius)
            .child(img(path.clone()).size_full().object_fit(ObjectFit::Contain)),

        ItemContent::Pdf {
            thumbnail: Some(thumb_path),
            ..
        } => div()
            .size_full()
            .overflow_hidden()
            .rounded(corner_radius)
            .child(
                img(thumb_path.clone())
                    .size_full()
                    .object_fit(ObjectFit::Contain),
            ),

        ItemContent::Pdf {
            thumbnail: None, ..
        } => div()
            .size_full()
            .rounded(corner_radius)
            .flex()
            .items_center()
            .justify_center()
            .child(
                Icon::new(IconName::File)
                    .size(px(48.0 * zoom))
                    .text_color(rgb(0xffffff)),
            ),

        ItemContent::YouTube(_) => {
            let padding = 8.0 * zoom;
            div()
                .size_full()
                .p(px(padding))
                .bg(rgb(0x222222))
                .rounded(px(4.0 * zoom))
                .child(
                    div()
                        .size_full()
                        .overflow_hidden()
                        .when_some(youtube_webviews.get(&item.id), |d, yt_webview| {
                            d.child(yt_webview.webview())
                        }),
                )
        }

        ItemContent::Markdown { title, content, .. } => {
            render_collapsed_markdown(title, content, zoom)
        }

        ItemContent::Text(text) => div()
            .size_full()
            .rounded(corner_radius)
            .p(px(12.0 * zoom))
            .child(
                div()
                    .text_size(px(14.0 * zoom))
                    .text_color(rgb(0xffffff))
                    .child(text.clone()),
            ),

        ItemContent::Video(path) => div()
            .size_full()
            .rounded(corner_radius)
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(8.0 * zoom))
            .child(
                Icon::new(IconName::File)
                    .size(px(32.0 * zoom))
                    .text_color(rgb(0xffffff)),
            )
            .child(
                div()
                    .text_size(px(12.0 * zoom))
                    .text_color(rgb(0xcccccc))
                    .child(
                        path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("Video")
                            .to_string(),
                    ),
            ),

        ItemContent::Link(url) => div()
            .size_full()
            .rounded(corner_radius)
            .p(px(12.0 * zoom))
            .flex()
            .flex_col()
            .gap(px(4.0 * zoom))
            .child(
                Icon::new(IconName::Globe)
                    .size(px(24.0 * zoom))
                    .text_color(rgb(0x88ccff)),
            )
            .child(
                div()
                    .text_size(px(11.0 * zoom))
                    .text_color(rgb(0xaaaaaa))
                    .overflow_hidden()
                    .child(url.clone()),
            ),
    }
}

pub fn render_items(
    items: &[CanvasItem],
    canvas_offset: Point<Pixels>,
    zoom: f32,
    selected_item_id: Option<u64>,
    youtube_webviews: &HashMap<u64, YouTubeWebView>,
) -> Vec<Div> {
    let offset_x = f32::from(canvas_offset.x);
    let offset_y = f32::from(canvas_offset.y);
    let selection_size = 20.0 * zoom;

    items
        .iter()
        .map(|item| {
            let x = item.position.0 * zoom + offset_x;
            let y = item.position.1 * zoom + offset_y;
            let w = item.size.0 * zoom;
            let h = item.size.1 * zoom;
            let is_selected = selected_item_id == Some(item.id);

            div()
                .absolute()
                .left(px(x))
                .top(px(y))
                .w(px(w))
                .h(px(h))
                .child(render_item_content(item, zoom, youtube_webviews))
                .when(is_selected, |d| {
                    d.child(
                        // Selection resize handle
                        div()
                            .absolute()
                            .right(px(0.0))
                            .bottom(px(0.0))
                            .w(px(selection_size))
                            .h(px(selection_size))
                            .bg(hsla(0.0, 0.0, 1.0, 0.7))
                            .rounded_tl(px(4.0 * zoom))
                            .border_2()
                            .border_color(hsla(0.0, 0.0, 1.0, 1.0))
                            .cursor(CursorStyle::ResizeUpLeftDownRight),
                    )
                })
        })
        .collect()
}

/// Render the header bar with integrated command palette dropdown
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
                    Button::new("go-home")
                        .ghost()
                        .xsmall()
                        .icon(Icon::new(IconName::ArrowLeft))
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
                Button::new("show-shortcuts")
                    .ghost()
                    .xsmall()
                    .icon(Icon::new(IconName::Info))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.toggle_shortcuts(cx);
                    })),
            ),
        )
}

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
    .with_priority(2000) // Above shortcuts overlay (1000)
}

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

pub fn render_selected_item_label(_name: String) -> Div {
    div().size_0()
}

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

pub fn render_preview_panel(
    file_name: String,
    page_image_path: Option<std::path::PathBuf>,
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

// Render the canvas area (moodboard content)
// Note: items must be cloned because render_canvas needs ownership for 'static closure
fn render_canvas_area(
    canvas_offset: Point<Pixels>,
    zoom: f32,
    items: &[CanvasItem],
    selected_item_id: Option<u64>,
    youtube_webviews: &HashMap<u64, YouTubeWebView>,
) -> Div {
    div()
        .size_full()
        .bg(rgb(0x000000))
        .overflow_hidden()
        .relative()
        .child(render_canvas(canvas_offset, zoom, items.to_vec()))
        .children(render_items(
            items,
            canvas_offset,
            zoom,
            selected_item_id,
            youtube_webviews,
        ))
}

// Render implementation for Humanboard
impl Render for Humanboard {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.update_fps();

        // Process any pending command from Enter key press
        self.process_pending_command(window, cx);

        // Route based on current view
        let content = match &self.view {
            AppView::Landing => self.render_landing_view(cx),
            AppView::Board(_) => self.render_board_view(window, cx),
        };

        // Wrap everything in a container with overlays on top
        div()
            .size_full()
            .font_family(UI_FONT)
            .relative()
            .child(content)
            .when(self.show_shortcuts, |d| {
                d.child(render_shortcuts_overlay(cx))
            })
        // Command palette is now in the header bar, not a floating overlay
    }
}

impl Humanboard {
    fn render_landing_view(&mut self, cx: &mut Context<Self>) -> Div {
        let deleting_board = self.deleting_board_id.as_ref().and_then(|id| {
            self.board_index
                .get_board(id)
                .map(|meta| (id.as_str(), meta.name.as_str()))
        });

        let is_editing = self.editing_board_id.is_some();

        div()
            .size_full()
            .track_focus(&self.focus_handle)
            // Only steal focus when not editing (so Input can receive focus)
            .when(!is_editing, |d| {
                d.on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, window, _| {
                        this.focus_handle.focus(window);
                    }),
                )
            })
            .on_action(cx.listener(|this, _: &NewBoard, _, cx| this.create_new_board(cx)))
            .on_action(cx.listener(|this, _: &ShowShortcuts, _, cx| this.toggle_shortcuts(cx)))
            .child(render_landing_page(
                &self.board_index,
                self.editing_board_id.as_deref(),
                self.edit_input.as_ref(),
                deleting_board,
                cx,
            ))
    }

    fn render_board_view(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Div {
        // Poll for file picker results (from Cmd+O)
        if let Some(rx) = &self.file_drop_rx {
            if let Ok((pos, paths)) = rx.try_recv() {
                if let Some(ref mut board) = self.board {
                    board.handle_file_drop(pos, paths);
                }
                self.file_drop_rx = None;
                cx.notify();
            }
        }

        // Ensure WebViews are created if preview is active
        if self.preview.is_some() {
            self.ensure_pdf_webview(window, cx);
            // Markdown uses native rendering now - no WebView needed!
        }

        // Ensure YouTube WebViews are created for any YouTube items
        self.ensure_youtube_webviews(window, cx);

        // Get board data (with fallback defaults if somehow no board)
        let (canvas_offset, zoom, items, item_count) = if let Some(ref board) = self.board {
            (
                board.canvas_offset,
                board.zoom,
                board.items.clone(),
                board.items.len(),
            )
        } else {
            (point(px(0.0), px(0.0)), 1.0, Vec::new(), 0)
        };

        let fps = self.calculate_fps();
        let frame_count = self.frame_count;
        let selected_item_id = self.selected_item;
        let selected_item_name = self.selected_item.and_then(|id| {
            self.board
                .as_ref()
                .and_then(|b| b.items.iter().find(|i| i.id == id))
                .map(|i| i.content.display_name())
        });

        // Get board name from index
        let board_name = if let AppView::Board(ref id) = self.view {
            self.board_index.get_board(id).map(|m| m.name.clone())
        } else {
            None
        };

        // Extract preview info
        let preview_info = self
            .preview
            .as_ref()
            .map(|p| (p.split, p.size, &p.tabs, p.active_tab));

        let base = div()
            .size_full()
            .track_focus(&self.focus_handle)
            .key_context("Canvas")
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event: &MouseDownEvent, window, cx| {
                    // Check if we're clicking in the preview panel area (WebView handles its own focus)
                    let in_preview_area = if let Some(ref preview) = this.preview {
                        let bounds = window.bounds();
                        match preview.split {
                            SplitDirection::Vertical => {
                                let preview_start_x =
                                    f32::from(bounds.size.width) * (1.0 - preview.size);
                                f32::from(event.position.x) > preview_start_x
                            }
                            SplitDirection::Horizontal => {
                                let preview_start_y =
                                    f32::from(bounds.size.height) * (1.0 - preview.size);
                                f32::from(event.position.y) > preview_start_y
                            }
                        }
                    } else {
                        false
                    };

                    if !in_preview_area {
                        this.focus_handle.focus(window);
                        this.handle_mouse_down(event, window, cx);
                    }
                }),
            )
            .on_mouse_up(MouseButton::Left, cx.listener(Humanboard::handle_mouse_up))
            .on_mouse_move(cx.listener(Humanboard::handle_mouse_move))
            .on_scroll_wheel(cx.listener(Humanboard::handle_scroll))
            .on_action(cx.listener(|this, _: &GoHome, _, cx| this.go_home(cx)))
            .on_action(cx.listener(|this, _: &OpenFile, window, cx| this.open_file(window, cx)))
            .on_action(cx.listener(|this, _: &ZoomIn, window, cx| this.zoom_in(window, cx)))
            .on_action(cx.listener(|this, _: &ZoomOut, window, cx| this.zoom_out(window, cx)))
            .on_action(cx.listener(|this, _: &ZoomReset, _, cx| this.zoom_reset(cx)))
            .on_action(cx.listener(|this, _: &DeleteSelected, _, cx| this.delete_selected(cx)))
            .on_action(cx.listener(|this, _: &Undo, _, cx| this.undo(cx)))
            .on_action(cx.listener(|this, _: &Redo, _, cx| this.redo(cx)))
            .on_action(cx.listener(|this, _: &ClosePreview, _, cx| this.close_preview(cx)))
            .on_action(cx.listener(|this, _: &ToggleSplit, _, cx| this.toggle_split_direction(cx)))
            .on_action(cx.listener(|this, _: &NextPage, _, cx| this.next_page(cx)))
            .on_action(cx.listener(|this, _: &PrevPage, _, cx| this.prev_page(cx)))
            .on_action(cx.listener(|this, _: &PdfZoomIn, _, cx| this.pdf_zoom_in(cx)))
            .on_action(cx.listener(|this, _: &PdfZoomOut, _, cx| this.pdf_zoom_out(cx)))
            .on_action(cx.listener(|this, _: &PdfZoomReset, _, cx| this.pdf_zoom_reset(cx)))
            .on_action(cx.listener(|this, _: &NextTab, _, cx| this.next_tab(cx)))
            .on_action(cx.listener(|this, _: &PrevTab, _, cx| this.prev_tab(cx)))
            .on_action(cx.listener(|this, _: &CloseTab, _, cx| this.close_current_tab(cx)))
            .on_action(cx.listener(|this, _: &ShowShortcuts, _, cx| this.toggle_shortcuts(cx)))
            .on_action(cx.listener(|this, _: &Paste, window, cx| this.paste(window, cx)))
            .on_action(cx.listener(|this, _: &CommandPalette, window, cx| {
                this.show_command_palette(window, cx)
            }))
            .on_drop(cx.listener(|this, paths: &ExternalPaths, window, cx| {
                if let Some(first_path) = paths.paths().first() {
                    let drop_pos = if let Some(pos) = this.last_drop_pos {
                        pos
                    } else {
                        let bounds = window.bounds();
                        let window_size = bounds.size;

                        let (canvas_center_x, canvas_center_y) =
                            if let Some(ref preview) = this.preview {
                                match preview.split {
                                    SplitDirection::Vertical => {
                                        let canvas_width =
                                            f32::from(window_size.width) * (1.0 - preview.size);
                                        (canvas_width / 2.0, f32::from(window_size.height) / 2.0)
                                    }
                                    SplitDirection::Horizontal => {
                                        let canvas_height =
                                            f32::from(window_size.height) * (1.0 - preview.size);
                                        (f32::from(window_size.width) / 2.0, canvas_height / 2.0)
                                    }
                                }
                            } else {
                                (
                                    f32::from(window_size.width) / 2.0,
                                    f32::from(window_size.height) / 2.0,
                                )
                            };

                        point(px(canvas_center_x), px(canvas_center_y))
                    };

                    if let Some(ref mut board) = this.board {
                        board.handle_file_drop(drop_pos, vec![first_path.clone()]);
                    }
                    cx.notify();
                }
            }));

        let content = match preview_info {
            Some((split, size, tabs, active_tab)) => {
                let canvas_size = 1.0 - size;
                let preview_size = size;

                match split {
                    SplitDirection::Vertical => base
                        .flex()
                        .flex_row()
                        .pt(px(40.0)) // header
                        .pb(px(28.0))
                        .child(
                            div()
                                .flex_shrink_0()
                                .w(Fraction(canvas_size))
                                .h_full()
                                .child(render_canvas_area(
                                    canvas_offset,
                                    zoom,
                                    &items,
                                    selected_item_id,
                                    &self.youtube_webviews,
                                )),
                        )
                        .child(render_splitter(SplitDirection::Vertical, cx))
                        .child(
                            div()
                                .flex_shrink_0()
                                .w(Fraction(preview_size))
                                .h_full()
                                .bg(rgb(0x000000))
                                .flex()
                                .flex_col()
                                .overflow_hidden()
                                .child(render_tab_bar(tabs, active_tab, cx))
                                .child(
                                    div()
                                        .id(ElementId::Name(
                                            format!("tab-container-v-{}", active_tab).into(),
                                        ))
                                        .flex_1()
                                        .overflow_hidden()
                                        .when_some(tabs.get(active_tab), |d, tab| {
                                            d.child(render_tab_content(tab, true, active_tab, cx))
                                        }),
                                ),
                        ),
                    SplitDirection::Horizontal => {
                        base.flex()
                            .flex_col()
                            .pt(px(40.0)) // header
                            .pb(px(28.0))
                            .child(
                                div()
                                    .flex_shrink_0()
                                    .h(Fraction(canvas_size))
                                    .w_full()
                                    .child(render_canvas_area(
                                        canvas_offset,
                                        zoom,
                                        &items,
                                        selected_item_id,
                                        &self.youtube_webviews,
                                    )),
                            )
                            .child(render_splitter(SplitDirection::Horizontal, cx))
                            .child(
                                div()
                                    .flex_shrink_0()
                                    .h(Fraction(preview_size))
                                    .w_full()
                                    .bg(rgb(0x000000))
                                    .flex()
                                    .flex_col()
                                    .overflow_hidden()
                                    .child(render_tab_bar(tabs, active_tab, cx))
                                    .child(
                                        div()
                                            .id(ElementId::Name(
                                                format!("tab-container-h-{}", active_tab).into(),
                                            ))
                                            .flex_1()
                                            .overflow_hidden()
                                            .when_some(tabs.get(active_tab), |d, tab| {
                                                d.child(render_tab_content(
                                                    tab, true, active_tab, cx,
                                                ))
                                            }),
                                    ),
                            )
                    }
                }
            }
            None => base.pt(px(40.0)).pb(px(28.0)).child(render_canvas_area(
                canvas_offset,
                zoom,
                &items,
                selected_item_id,
                &self.youtube_webviews,
            )),
        }
        .child(render_footer_bar(
            fps,
            frame_count,
            item_count,
            zoom,
            canvas_offset,
            selected_item_name,
            None, // board_name shown in header now
        ))
        // Header bar with command palette dropdown
        .child(render_header_bar(
            board_name,
            self.command_palette.as_ref(),
            &self.search_results,
            self.selected_result,
            cx,
        ));

        content
    }
}
