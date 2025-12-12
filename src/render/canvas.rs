//! Canvas rendering - item backgrounds, content, and the infinite canvas
//!
//! This module handles all canvas-related rendering including:
//! - The infinite canvas background with grid
//! - Item background shapes (painted via GPU)
//! - Individual item content rendering
//! - Item selection and resize handles

use crate::app::Humanboard;
use crate::audio_webview::AudioWebView;
use crate::markdown_card::render_collapsed_markdown;
use crate::types::{CanvasItem, ItemContent};
use crate::video_webview::VideoWebView;
use crate::youtube_webview::YouTubeWebView;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{ActiveTheme as _, h_flex, v_flex};
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

        // Use semantic colors based on content type for better theme integration
        // Each type has a distinct hue for visual differentiation
        let bg_color = match &item.content {
            ItemContent::Video(_) => hsla(280.0 / 360.0, 0.6, 0.45, 0.85), // Purple/magenta for video
            ItemContent::Audio(_) => hsla(320.0 / 360.0, 0.6, 0.45, 0.85), // Pink/magenta for audio
            ItemContent::Text(_) => hsla(210.0 / 360.0, 0.6, 0.45, 0.85),  // Blue for text
            ItemContent::Pdf { .. } => hsla(15.0 / 360.0, 0.7, 0.5, 0.85), // Orange for documents
            ItemContent::Link(_) => hsla(180.0 / 360.0, 0.6, 0.4, 0.85),   // Cyan for links
            ItemContent::YouTube(_) => hsla(0.0 / 360.0, 0.75, 0.5, 0.85), // Red for YouTube
            _ => hsla(0.0, 0.0, 0.4, 0.85),                                // Gray for unknown
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
    audio_webviews: &HashMap<u64, AudioWebView>,
    video_webviews: &HashMap<u64, VideoWebView>,
    fg: Hsla,
    muted_fg: Hsla,
    muted_bg: Hsla,
    _danger: Hsla,
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
                    .text_color(fg)
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
                    .bg(muted_bg)
                    .rounded(px(4.0 * zoom))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_size(px(24.0 * zoom))
                            .text_color(muted_fg)
                            .child("PDF"),
                    ),
            )
            .child(
                div()
                    .text_size(px(10.0 * zoom))
                    .text_color(muted_fg)
                    .max_w(px(200.0 * zoom))
                    .overflow_hidden()
                    .child(
                        path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("document.pdf")
                            .to_string(),
                    ),
            ),

        ItemContent::Video(_path) => {
            // Render Video WebView if available
            if let Some(webview) = video_webviews.get(&item.id) {
                v_flex()
                    .size_full()
                    .rounded(corner_radius)
                    .overflow_hidden()
                    // Drag handle bar at top
                    .child(
                        div()
                            .w_full()
                            .h(px(24.0 * zoom))
                            .bg(hsla(0.0, 0.0, 0.1, 1.0))
                            .border_b_1()
                            .border_color(hsla(0.0, 0.0, 0.2, 1.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_size(px(12.0 * zoom))
                                    .text_color(hsla(0.0, 0.0, 0.4, 1.0))
                                    .child("≡"),
                            ),
                    )
                    // WebView takes remaining space
                    .child(
                        div()
                            .flex_1()
                            .w_full()
                            .overflow_hidden()
                            .child(webview.webview_entity.clone()),
                    )
            } else {
                // Placeholder while loading
                div()
                    .size_full()
                    .bg(hsla(0.0, 0.0, 0.1, 1.0))
                    .rounded(corner_radius)
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_size(px(14.0 * zoom))
                            .text_color(muted_fg)
                            .child("Loading video..."),
                    )
            }
        }

        ItemContent::Audio(_path) => {
            // Render Audio WebView if available
            if let Some(webview) = audio_webviews.get(&item.id) {
                v_flex()
                    .size_full()
                    .rounded(corner_radius)
                    .overflow_hidden()
                    // Drag handle bar at top
                    .child(
                        div()
                            .w_full()
                            .h(px(24.0 * zoom))
                            .bg(hsla(0.0, 0.0, 0.1, 1.0))
                            .border_b_1()
                            .border_color(hsla(0.0, 0.0, 0.2, 1.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_size(px(12.0 * zoom))
                                    .text_color(hsla(0.0, 0.0, 0.4, 1.0))
                                    .child("≡"),
                            ),
                    )
                    // WebView takes remaining space
                    .child(
                        div()
                            .flex_1()
                            .w_full()
                            .overflow_hidden()
                            .child(webview.webview_entity.clone()),
                    )
            } else {
                // Placeholder while loading
                div()
                    .size_full()
                    .bg(hsla(0.0, 0.0, 0.1, 1.0))
                    .rounded(corner_radius)
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_size(px(14.0 * zoom))
                            .text_color(muted_fg)
                            .child("Loading audio..."),
                    )
            }
        }

        ItemContent::Text(text) => div()
            .size_full()
            .p(px(12.0 * zoom))
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .text_size(px(14.0 * zoom))
                    .text_color(fg)
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
                    .child(div().text_size(px(24.0 * zoom)).text_color(fg).child("🔗"))
                    .child(
                        div()
                            .text_size(px(12.0 * zoom))
                            .text_color(fg)
                            .font_weight(FontWeight::BOLD)
                            .child("Link"),
                    ),
            )
            .child(
                div()
                    .text_size(px(10.0 * zoom))
                    .text_color(muted_fg)
                    .overflow_hidden()
                    .child(url.clone()),
            ),

        ItemContent::YouTube(video_id) => {
            // Render YouTube WebView if available, otherwise placeholder
            if let Some(webview) = youtube_webviews.get(&item.id) {
                // Use vertical flex with drag bar ABOVE the webview
                // (overlays don't work on webviews - they render on top layer)
                v_flex()
                    .size_full()
                    // Drag handle bar at top - OUTSIDE the webview
                    .child(
                        div()
                            .w_full()
                            .h(px(24.0 * zoom))
                            .bg(hsla(0.0, 0.0, 0.15, 1.0))
                            .border_b_1()
                            .border_color(hsla(0.0, 0.0, 0.3, 1.0))
                            .rounded_t(corner_radius)
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_size(px(14.0 * zoom))
                                    .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                                    .child("≡"),
                            ),
                    )
                    // WebView takes remaining space
                    .child(
                        div()
                            .flex_1()
                            .w_full()
                            .overflow_hidden()
                            .rounded_b(corner_radius)
                            .child(webview.webview().clone()),
                    )
            } else {
                div()
                    .size_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .bg(muted_bg)
                    .rounded(corner_radius)
                    .child(
                        v_flex()
                            .items_center()
                            .gap(px(8.0 * zoom))
                            .child(div().text_size(px(48.0 * zoom)).child("▶️"))
                            .child(
                                div()
                                    .text_size(px(12.0 * zoom))
                                    .text_color(muted_fg)
                                    .child(format!("YouTube: {}", video_id)),
                            ),
                    )
            }
        }

        ItemContent::Markdown { title, content, .. } => {
            // Use theme colors for markdown cards
            let popover_bg = hsla(220.0 / 360.0, 0.15, 0.18, 1.0); // Subtle dark bg
            let border = hsla(240.0 / 360.0, 0.2, 0.35, 1.0); // Muted border
            let hover_bg = hsla(220.0 / 360.0, 0.15, 0.22, 1.0); // Slightly lighter on hover
            let hover_border = hsla(240.0 / 360.0, 0.4, 0.55, 1.0); // More vibrant on hover
            let icon_color = hsla(240.0 / 360.0, 0.6, 0.7, 1.0); // Blue-ish icon
            let text_color = hsla(0.0, 0.0, 0.85, 1.0); // Light text

            render_collapsed_markdown(
                title,
                content,
                zoom,
                popover_bg,
                border,
                hover_bg,
                hover_border,
                icon_color,
                text_color,
            )
        }
    }
}

/// Render all canvas items with positioning and selection
pub fn render_items(
    items: &[CanvasItem],
    canvas_offset: Point<Pixels>,
    zoom: f32,
    selected_items: &std::collections::HashSet<u64>,
    youtube_webviews: &HashMap<u64, YouTubeWebView>,
    audio_webviews: &HashMap<u64, AudioWebView>,
    video_webviews: &HashMap<u64, VideoWebView>,
    cx: &Context<Humanboard>,
) -> Vec<Div> {
    let offset_x = f32::from(canvas_offset.x);
    let offset_y = f32::from(canvas_offset.y);

    let fg = cx.theme().foreground;
    let muted_fg = cx.theme().muted_foreground;
    let muted_bg = cx.theme().muted;
    let danger = cx.theme().danger;
    let primary = cx.theme().primary;

    items
        .iter()
        .map(|item| {
            let x = item.position.0 * zoom + offset_x;
            let y = item.position.1 * zoom + offset_y;
            let w = item.size.0 * zoom;
            let h = item.size.1 * zoom;
            let is_selected = selected_items.contains(&item.id);

            div()
                .absolute()
                .left(px(x))
                .top(px(y))
                .w(px(w))
                .h(px(h))
                .child(render_item_content(
                    item,
                    zoom,
                    youtube_webviews,
                    audio_webviews,
                    video_webviews,
                    fg,
                    muted_fg,
                    muted_bg,
                    danger,
                ))
                .when(is_selected, |d| {
                    d
                        // Selection border
                        .border_2()
                        .border_color(primary)
                        .rounded(px(8.0 * zoom))
                        .child(
                            // Resize handle - small corner indicator
                            div()
                                .absolute()
                                .right(px(-2.0))
                                .bottom(px(-2.0))
                                .w(px(10.0 * zoom))
                                .h(px(10.0 * zoom))
                                .bg(primary)
                                .rounded(px(2.0 * zoom))
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
    selected_items: &std::collections::HashSet<u64>,
    youtube_webviews: &HashMap<u64, YouTubeWebView>,
    audio_webviews: &HashMap<u64, AudioWebView>,
    video_webviews: &HashMap<u64, VideoWebView>,
    marquee: Option<(Point<Pixels>, Point<Pixels>)>,
    cx: &Context<Humanboard>,
) -> Div {
    let bg = cx.theme().background;
    let primary = cx.theme().primary;

    div()
        .size_full()
        .bg(bg)
        .overflow_hidden()
        .relative()
        .child(render_canvas(canvas_offset, zoom, items.to_vec()))
        .children(render_items(
            items,
            canvas_offset,
            zoom,
            selected_items,
            youtube_webviews,
            audio_webviews,
            video_webviews,
            cx,
        ))
        // Render marquee selection rectangle
        .when_some(marquee, |d, (start, current)| {
            let min_x = f32::from(start.x).min(f32::from(current.x));
            let max_x = f32::from(start.x).max(f32::from(current.x));
            let min_y = f32::from(start.y).min(f32::from(current.y));
            let max_y = f32::from(start.y).max(f32::from(current.y));
            let width = max_x - min_x;
            let height = max_y - min_y;

            // Only show if has some size
            if width > 2.0 && height > 2.0 {
                d.child(
                    div()
                        .absolute()
                        .left(px(min_x))
                        .top(px(min_y - 40.0)) // Account for header offset
                        .w(px(width))
                        .h(px(height))
                        .border_1()
                        .border_color(primary)
                        .bg(primary.opacity(0.1))
                        .rounded(px(2.0)),
                )
            } else {
                d
            }
        })
}
