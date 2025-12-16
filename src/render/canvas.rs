//! Canvas rendering - item backgrounds, content, and the infinite canvas
//!
//! This module handles all canvas-related rendering including:
//! - The infinite canvas background with grid
//! - Item background shapes (painted via GPU)
//! - Individual item content rendering
//! - Item selection and resize handles

use crate::app::Humanboard;
use crate::audio_webview::AudioWebView;
use crate::markdown_card::{render_collapsed_code, render_collapsed_markdown};
use crate::types::{CanvasItem, ItemContent};
use crate::video_webview::VideoWebView;
use crate::youtube_webview::YouTubeWebView;
use gpui::prelude::FluentBuilder;
use gpui::{PathBuilder, *};
use gpui_component::input::{Input, InputState};
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
        // Skip items that render themselves (images, markdown cards, code files, shapes, arrows, textboxes)
        if matches!(
            &item.content,
            ItemContent::Image(_)
                | ItemContent::Markdown { .. }
                | ItemContent::Code { .. }
                | ItemContent::TextBox { .. }
                | ItemContent::Arrow { .. }
                | ItemContent::Shape { .. }
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
    editing_textbox_id: Option<u64>,
    textbox_input: Option<&Entity<InputState>>,
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
        } => div().size_full().p(px(8.0 * zoom)).child(
            img(thumb_path.clone())
                .size_full()
                .object_fit(ObjectFit::Contain)
                .rounded(px(4.0 * zoom)),
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

        ItemContent::Code { path, language } => {
            // Use theme colors for code file cards
            let popover_bg = hsla(220.0 / 360.0, 0.15, 0.14, 1.0); // Darker bg for code
            let border = hsla(200.0 / 360.0, 0.3, 0.35, 1.0); // Cyan-ish border
            let hover_bg = hsla(220.0 / 360.0, 0.15, 0.18, 1.0);
            let hover_border = hsla(200.0 / 360.0, 0.5, 0.5, 1.0); // Brighter cyan on hover
            let icon_color = hsla(40.0 / 360.0, 0.8, 0.6, 1.0); // Orange-ish icon for code
            let text_color = hsla(0.0, 0.0, 0.85, 1.0);
            let badge_bg = hsla(200.0 / 360.0, 0.4, 0.25, 1.0); // Cyan badge bg
            let badge_text = hsla(200.0 / 360.0, 0.6, 0.8, 1.0); // Cyan badge text

            let filename = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown");

            render_collapsed_code(
                filename,
                language,
                zoom,
                popover_bg,
                border,
                hover_bg,
                hover_border,
                icon_color,
                text_color,
                badge_bg,
                badge_text,
            )
        }

        ItemContent::TextBox {
            text,
            font_size,
            color,
        } => {
            // Parse color from hex string, fallback to theme foreground
            let text_color = parse_hex_color(color).unwrap_or(fg);
            let scaled_font = font_size * zoom;

            // Check if this textbox is being edited
            let is_editing = editing_textbox_id == Some(item.id);

            if is_editing {
                if let Some(input) = textbox_input {
                    // Render the input field for inline editing (multiline)
                    // No background - transparent textbox with just a border
                    div()
                        .size_full()
                        .rounded(px(4.0 * zoom))
                        .border_1()
                        .border_color(fg.opacity(0.3))
                        .overflow_hidden()
                        .child(Input::new(input).appearance(false).size_full())
                } else {
                    // Fallback to static text if input not available
                    div()
                        .size_full()
                        .rounded(px(4.0 * zoom))
                        .p(px(8.0 * zoom))
                        .overflow_hidden()
                        .flex()
                        .flex_col()
                        .children(text.lines().map(|line| {
                            div()
                                .text_size(px(scaled_font))
                                .text_color(text_color)
                                .child(if line.is_empty() {
                                    " ".to_string() // Preserve empty lines
                                } else {
                                    line.to_string()
                                })
                        }))
                }
            } else {
                // Normal display mode - just text, no background
                div()
                    .size_full()
                    .rounded(px(4.0 * zoom))
                    .p(px(8.0 * zoom))
                    .overflow_hidden()
                    .flex()
                    .flex_col()
                    .children(text.lines().map(|line| {
                        div()
                            .text_size(px(scaled_font))
                            .text_color(text_color)
                            .child(if line.is_empty() {
                                " ".to_string() // Preserve empty lines
                            } else {
                                line.to_string()
                            })
                    }))
            }
        }

        ItemContent::Arrow {
            color,
            thickness,
            end_offset,
            head_style,
        } => {
            // Parse color from hex string
            let arrow_color = parse_hex_color(color).unwrap_or(fg);
            let scaled_thickness = *thickness * zoom;
            let dx = end_offset.0 * zoom;
            let dy = end_offset.1 * zoom;
            let head = *head_style;
            let item_w = item.size.0 * zoom;
            let item_h = item.size.1 * zoom;

            // Use a canvas element to draw the arrow with PathBuilder
            div().size_full().child(
                canvas(
                    move |_, _, _| {},
                    move |bounds, _, window, _| {
                        // Arrow start point depends on direction of end_offset
                        // If dx >= 0, start is on left; if dx < 0, start is on right
                        // If dy >= 0, start is on top; if dy < 0, start is on bottom
                        let start_x = if dx >= 0.0 {
                            bounds.origin.x
                        } else {
                            bounds.origin.x + px(item_w)
                        };
                        let start_y = if dy >= 0.0 {
                            bounds.origin.y
                        } else {
                            bounds.origin.y + px(item_h)
                        };
                        let start = point(start_x, start_y);
                        let end = point(start.x + px(dx), start.y + px(dy));

                        // Draw the line
                        let mut path = PathBuilder::stroke(px(scaled_thickness));
                        path.move_to(start);
                        path.line_to(end);
                        if let Ok(built_path) = path.build() {
                            window.paint_path(built_path, arrow_color);
                        }

                        // Draw arrow head if needed
                        if head != crate::types::ArrowHead::None {
                            let angle = dy.atan2(dx);
                            let head_size = (scaled_thickness * 4.0).max(8.0);
                            let head_angle = 0.5; // ~30 degrees

                            // Calculate arrow head points
                            let angle1 = angle + std::f32::consts::PI - head_angle;
                            let angle2 = angle + std::f32::consts::PI + head_angle;

                            let p1 = point(
                                end.x + px(head_size * angle1.cos()),
                                end.y + px(head_size * angle1.sin()),
                            );
                            let p2 = point(
                                end.x + px(head_size * angle2.cos()),
                                end.y + px(head_size * angle2.sin()),
                            );

                            // Draw arrow head as filled triangle
                            let mut head_path = PathBuilder::fill();
                            head_path.move_to(end);
                            head_path.line_to(p1);
                            head_path.line_to(p2);
                            head_path.close();
                            if let Ok(built_head) = head_path.build() {
                                window.paint_path(built_head, arrow_color);
                            }
                        }
                    },
                )
                .size_full(),
            )
        }

        ItemContent::Shape {
            shape_type,
            fill_color,
            border_color,
            border_width,
        } => {
            let fill = fill_color.as_ref().and_then(|c| parse_hex_color(c));
            let stroke = parse_hex_color(border_color).unwrap_or(fg);
            let scaled_border = (border_width * zoom).max(1.0);

            let radius = match shape_type {
                crate::types::ShapeType::Rectangle => px(0.0),
                crate::types::ShapeType::RoundedRect => px(8.0 * zoom),
                crate::types::ShapeType::Ellipse => px(9999.0),
            };

            div()
                .size_full()
                .rounded(radius)
                .border(px(scaled_border))
                .border_color(stroke)
                .when_some(fill, |d, c| d.bg(c))
        }
    }
}

/// Parse a hex color string like "#ffffff" into an Hsla color
fn parse_hex_color(hex: &str) -> Option<Hsla> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()? as f32 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()? as f32 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()? as f32 / 255.0;

    // Convert RGB to HSL
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;

    if max == min {
        return Some(hsla(0.0, 0.0, l, 1.0));
    }

    let d = max - min;
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };

    let h = if max == r {
        ((g - b) / d + if g < b { 6.0 } else { 0.0 }) / 6.0
    } else if max == g {
        ((b - r) / d + 2.0) / 6.0
    } else {
        ((r - g) / d + 4.0) / 6.0
    };

    Some(hsla(h, s, l, 1.0))
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
    editing_textbox_id: Option<u64>,
    textbox_input: Option<&Entity<InputState>>,
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
                    editing_textbox_id,
                    textbox_input,
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
    editing_textbox_id: Option<u64>,
    textbox_input: Option<&Entity<InputState>>,
    marquee: Option<(Point<Pixels>, Point<Pixels>)>,
    drawing_preview: Option<(Point<Pixels>, Point<Pixels>, crate::types::ToolType)>,
    cx: &Context<Humanboard>,
) -> Div {
    let bg = cx.theme().background;
    let primary = cx.theme().primary;
    let fg = cx.theme().foreground;

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
            editing_textbox_id,
            textbox_input,
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
        // Render drawing preview (for TextBox, Shape, Arrow while dragging)
        .when_some(drawing_preview, |d, (start, current, tool)| {
            // Account for dock width (44px) since mouse coords are in window space
            // but we're rendering in canvas space (after dock)
            let dock_offset = crate::render::dock::DOCK_WIDTH;
            let start_x = f32::from(start.x) - dock_offset;
            let start_y = f32::from(start.y);
            let current_x = f32::from(current.x) - dock_offset;
            let current_y = f32::from(current.y);

            let min_x = start_x.min(current_x);
            let max_x = start_x.max(current_x);
            let min_y = start_y.min(current_y);
            let max_y = start_y.max(current_y);
            let width = max_x - min_x;
            let height = max_y - min_y;

            // Only show if has some size
            if width > 5.0 || height > 5.0 {
                match tool {
                    crate::types::ToolType::Text | crate::types::ToolType::Shape => {
                        // Rectangle preview for TextBox and Shape
                        d.child(
                            div()
                                .absolute()
                                .left(px(min_x))
                                .top(px(min_y - 40.0)) // Account for header offset
                                .w(px(width.max(20.0)))
                                .h(px(height.max(20.0)))
                                .border_2()
                                .border_color(fg.opacity(0.8))
                                .bg(fg.opacity(0.05))
                                .rounded(px(4.0)),
                        )
                    }
                    crate::types::ToolType::Arrow => {
                        // Arrow preview - line from start to current
                        let arrow_start_x = start_x;
                        let arrow_start_y = start_y - 40.0; // Account for header
                        let arrow_end_x = current_x;
                        let arrow_end_y = current_y - 40.0;

                        d.child(
                            div()
                                .absolute()
                                .left(px(0.0))
                                .top(px(0.0))
                                .size_full()
                                .child(
                                    canvas(
                                        move |_, _, _| {},
                                        move |bounds, _, window, _| {
                                            let start_pt = point(
                                                bounds.origin.x + px(arrow_start_x),
                                                bounds.origin.y + px(arrow_start_y),
                                            );
                                            let end_pt = point(
                                                bounds.origin.x + px(arrow_end_x),
                                                bounds.origin.y + px(arrow_end_y),
                                            );

                                            // Draw line
                                            let mut path = PathBuilder::stroke(px(2.0));
                                            path.move_to(start_pt);
                                            path.line_to(end_pt);
                                            if let Ok(built) = path.build() {
                                                window.paint_path(built, fg.opacity(0.8));
                                            }

                                            // Draw arrow head
                                            let dx = f32::from(end_pt.x - start_pt.x);
                                            let dy = f32::from(end_pt.y - start_pt.y);
                                            let len = (dx * dx + dy * dy).sqrt();
                                            if len > 10.0 {
                                                let nx = dx / len;
                                                let ny = dy / len;
                                                let head_size = 12.0;
                                                let p1 = point(
                                                    end_pt.x
                                                        - px(nx * head_size - ny * head_size * 0.5),
                                                    end_pt.y
                                                        - px(ny * head_size + nx * head_size * 0.5),
                                                );
                                                let p2 = point(
                                                    end_pt.x
                                                        - px(nx * head_size + ny * head_size * 0.5),
                                                    end_pt.y
                                                        - px(ny * head_size - nx * head_size * 0.5),
                                                );

                                                let mut head = PathBuilder::fill();
                                                head.move_to(end_pt);
                                                head.line_to(p1);
                                                head.line_to(p2);
                                                head.close();
                                                if let Ok(built) = head.build() {
                                                    window.paint_path(built, fg.opacity(0.8));
                                                }
                                            }
                                        },
                                    )
                                    .size_full(),
                                ),
                        )
                    }
                    _ => d,
                }
            } else {
                d
            }
        })
}
