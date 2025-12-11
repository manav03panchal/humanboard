//! Canvas rendering - item backgrounds, content, and the infinite canvas
//!
//! This module handles all canvas-related rendering including:
//! - The infinite canvas background with grid
//! - Item background shapes (painted via GPU)
//! - Individual item content rendering
//! - Item selection and resize handles

use crate::markdown_card::render_collapsed_markdown;
use crate::types::{CanvasItem, ItemContent};
use crate::youtube_webview::YouTubeWebView;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{h_flex, v_flex};
use std::collections::HashMap;

/// Render the main canvas with item backgrounds
pub fn render_canvas(
    canvas_offset: Point<Pixels>,
    zoom: f32,
    items: Vec<CanvasItem>,
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

/// Paint item background shapes directly to GPU
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
            ItemContent::YouTube(_) => hsla(0.0, 0.8, 0.4, 0.9),
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
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(8.0 * zoom))
            .child(
                img(thumb_path.clone())
                    .w(px(150.0 * zoom))
                    .h(px(200.0 * zoom))
                    .object_fit(ObjectFit::Contain)
                    .rounded(px(4.0 * zoom)),
            )
            .child(
                div()
                    .text_size(px(12.0 * zoom))
                    .text_color(hsla(0.0, 0.0, 1.0, 0.9))
                    .font_weight(FontWeight::MEDIUM)
                    .child("PDF Document"),
            ),

        ItemContent::Pdf { path, .. } => div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(8.0 * zoom))
            .child(
                div()
                    .w(px(80.0 * zoom))
                    .h(px(100.0 * zoom))
                    .bg(hsla(0.0, 0.0, 0.2, 1.0))
                    .rounded(px(4.0 * zoom))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_size(px(24.0 * zoom))
                            .text_color(hsla(0.0, 0.0, 1.0, 0.6))
                            .child("PDF"),
                    ),
            )
            .child(
                div()
                    .text_size(px(10.0 * zoom))
                    .text_color(hsla(0.0, 0.0, 1.0, 0.7))
                    .max_w(px(200.0 * zoom))
                    .overflow_hidden()
                    .child(
                        path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("document.pdf")
                            .to_string(),
                    ),
            ),

        ItemContent::Video(path) => div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child(
                v_flex()
                    .items_center()
                    .gap(px(8.0 * zoom))
                    .child(
                        div()
                            .text_size(px(48.0 * zoom))
                            .text_color(hsla(0.0, 0.0, 1.0, 0.9))
                            .child("🎬"),
                    )
                    .child(
                        div()
                            .text_size(px(12.0 * zoom))
                            .text_color(hsla(0.0, 0.0, 1.0, 0.7))
                            .max_w(px(180.0 * zoom))
                            .overflow_hidden()
                            .child(
                                path.file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("video")
                                    .to_string(),
                            ),
                    ),
            ),

        ItemContent::Text(text) => div()
            .size_full()
            .p(px(12.0 * zoom))
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .text_size(px(14.0 * zoom))
                    .text_color(hsla(0.0, 0.0, 1.0, 0.9))
                    .font_weight(FontWeight::MEDIUM)
                    .child(text.clone()),
            ),

        ItemContent::Link(url) => div()
            .size_full()
            .p(px(12.0 * zoom))
            .flex()
            .flex_col()
            .gap(px(8.0 * zoom))
            .child(
                h_flex()
                    .gap(px(8.0 * zoom))
                    .child(
                        div()
                            .text_size(px(24.0 * zoom))
                            .text_color(hsla(0.0, 0.0, 1.0, 0.9))
                            .child("🔗"),
                    )
                    .child(
                        div()
                            .text_size(px(12.0 * zoom))
                            .text_color(hsla(0.0, 0.0, 1.0, 0.9))
                            .font_weight(FontWeight::BOLD)
                            .child("Link"),
                    ),
            )
            .child(
                div()
                    .text_size(px(10.0 * zoom))
                    .text_color(hsla(0.0, 0.0, 1.0, 0.7))
                    .overflow_hidden()
                    .child(url.clone()),
            ),

        ItemContent::YouTube(video_id) => {
            // Render YouTube WebView if available, otherwise placeholder
            if let Some(webview) = youtube_webviews.get(&item.id) {
                div()
                    .size_full()
                    .overflow_hidden()
                    .rounded(corner_radius)
                    .border_4()
                    .border_color(rgb(0xFF0000))
                    .child(webview.webview().clone())
            } else {
                div()
                    .size_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .bg(rgb(0x282828))
                    .rounded(corner_radius)
                    .child(
                        v_flex()
                            .items_center()
                            .gap(px(8.0 * zoom))
                            .child(div().text_size(px(48.0 * zoom)).child("▶️"))
                            .child(
                                div()
                                    .text_size(px(12.0 * zoom))
                                    .text_color(rgb(0xaaaaaa))
                                    .child(format!("YouTube: {}", video_id)),
                            ),
                    )
            }
        }

        ItemContent::Markdown { title, content, .. } => {
            render_collapsed_markdown(title, content, zoom)
        }
    }
}

/// Render all canvas items with positioning and selection
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

/// Render the canvas area container
pub fn render_canvas_area(
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
