//! Canvas rendering - item backgrounds, content, and the infinite canvas
//!
//! This module handles all canvas-related rendering including:
//! - The infinite canvas background with grid
//! - Item background shapes (painted via GPU)
//! - Individual item content rendering
//! - Item selection and resize handles
//!
//! ## Performance Notes
//!
//! This is a hot path - rendering happens every frame. Key optimizations:
//! - Early culling of off-screen items (viewport culling)
//! - Batched GPU paint operations for backgrounds
//! - Minimal allocations in render loop
//!
//! Enable profiling with `cargo build --features profiling` to see timing.

use crate::app::Humanboard;
use crate::audio_webview::AudioWebView;
use crate::constants::HEADER_HEIGHT;
use crate::data::{DataSourceDelegate, VirtualScrollState};
use crate::markdown_card::{render_collapsed_code, render_collapsed_markdown};
use crate::profile_scope;
use crate::types::{CanvasItem, DataSource, ItemContent};
use crate::video_webview::VideoWebView;
use crate::youtube_webview::YouTubeWebView;
use gpui::prelude::FluentBuilder;
use gpui::{PathBuilder, *};
use gpui_component::chart::{BarChart, LineChart, PieChart};
use gpui_component::input::{Input, InputState};
use gpui_component::table::TableState;
use gpui_component::{ActiveTheme as _, Icon, IconName, h_flex, v_flex};
use std::collections::HashMap;

/// Theme-aware colors for different content types
#[derive(Clone, Copy)]
pub struct ContentTypeColors {
    pub video: Hsla,
    pub audio: Hsla,
    pub text: Hsla,
    pub pdf: Hsla,
    pub link: Hsla,
    pub youtube: Hsla,
    pub unknown: Hsla,
    pub border: Hsla,
}

impl ContentTypeColors {
    /// Create content type colors from the current theme
    pub fn from_theme(theme: &gpui_component::theme::Theme) -> Self {
        let is_dark = theme.mode.is_dark();

        // Use theme-aware colors with good contrast in both light and dark modes
        // Colors are chosen to be visually distinct while respecting theme luminance
        let base_saturation = if is_dark { 0.5 } else { 0.6 };
        let base_lightness = if is_dark { 0.4 } else { 0.55 };
        let alpha = 0.9;

        Self {
            video: hsla(280.0 / 360.0, base_saturation, base_lightness, alpha),    // Purple for video
            audio: hsla(320.0 / 360.0, base_saturation, base_lightness, alpha),    // Pink for audio
            text: hsla(210.0 / 360.0, base_saturation, base_lightness, alpha),     // Blue for text
            pdf: hsla(25.0 / 360.0, base_saturation + 0.1, base_lightness, alpha), // Orange for PDF
            link: hsla(180.0 / 360.0, base_saturation, base_lightness, alpha),     // Cyan for links
            youtube: hsla(0.0, 0.7, if is_dark { 0.45 } else { 0.5 }, alpha),      // Red for YouTube
            unknown: theme.muted,
            border: theme.border.opacity(0.5),
        }
    }

    /// Get color for a specific content type
    pub fn for_content(&self, content: &ItemContent) -> Hsla {
        match content {
            ItemContent::Video(_) => self.video,
            ItemContent::Audio(_) => self.audio,
            ItemContent::Text(_) => self.text,
            ItemContent::Pdf { .. } => self.pdf,
            ItemContent::Link(_) => self.link,
            ItemContent::YouTube(_) => self.youtube,
            _ => self.unknown,
        }
    }
}

/// Render the main canvas with item backgrounds and connection lines
pub fn render_canvas(
    canvas_offset: Point<Pixels>,
    zoom: f32,
    items: Vec<CanvasItem>,
    colors: ContentTypeColors,
) -> impl IntoElement {
    canvas(
        move |_bounds, _window, _cx| (),
        move |bounds, _data, window, _cx| {
            render_item_backgrounds(bounds, window, &items, canvas_offset, zoom, colors);
            render_connection_lines(bounds, window, &items, canvas_offset, zoom);
        },
    )
    .absolute()
    .size_full()
}

/// Draw connection lines between charts and their source tables
fn render_connection_lines(
    bounds: Bounds<Pixels>,
    window: &mut Window,
    items: &[CanvasItem],
    canvas_offset: Point<Pixels>,
    zoom: f32,
) {
    // Build a map of item id -> position/size for quick lookup
    let item_map: std::collections::HashMap<u64, ((f32, f32), (f32, f32))> = items
        .iter()
        .map(|item| (item.id, (item.position, item.size)))
        .collect();

    // Line style: soft blue color
    let line_color = hsla(210.0 / 360.0, 0.6, 0.5, 0.6);

    // Find charts with source_item_id and draw lines to their sources
    for item in items {
        if let ItemContent::Chart { source_item_id: Some(source_id), .. } = &item.content {
            if let Some(&(source_pos, source_size)) = item_map.get(source_id) {
                // Calculate screen positions
                let offset_x = f32::from(canvas_offset.x);
                let offset_y = f32::from(canvas_offset.y);

                // Source table: connect from right edge, center height
                let source_x = f32::from(bounds.origin.x) + source_pos.0 * zoom + source_size.0 * zoom + offset_x;
                let source_y = f32::from(bounds.origin.y) + source_pos.1 * zoom + source_size.1 * zoom / 2.0 + offset_y;

                // Chart: connect to left edge, center height
                let chart_x = f32::from(bounds.origin.x) + item.position.0 * zoom + offset_x;
                let chart_y = f32::from(bounds.origin.y) + item.position.1 * zoom + item.size.1 * zoom / 2.0 + offset_y;

                // Draw the connection line using quadratic bezier
                // curve_to takes (destination, control_point)
                let mut path = PathBuilder::stroke(px(2.0 * zoom));
                path.move_to(point(px(source_x), px(source_y)));

                // Use midpoint as control point for a nice curve
                let mid_x = (source_x + chart_x) / 2.0;
                let mid_y = (source_y + chart_y) / 2.0;
                path.curve_to(
                    point(px(chart_x), px(chart_y)),  // destination
                    point(px(mid_x), px(mid_y)),      // control point
                );

                if let Ok(built_path) = path.build() {
                    window.paint_path(built_path, line_color);
                }

                // Draw a small filled diamond at the source end
                let size = 5.0 * zoom;
                let mut diamond = PathBuilder::fill();
                diamond.move_to(point(px(source_x + size), px(source_y)));
                diamond.line_to(point(px(source_x), px(source_y + size)));
                diamond.line_to(point(px(source_x - size), px(source_y)));
                diamond.line_to(point(px(source_x), px(source_y - size)));
                diamond.close();

                if let Ok(built_diamond) = diamond.build() {
                    window.paint_path(built_diamond, line_color);
                }
            }
        }
    }
}

/// Paint item background shapes directly to GPU
fn render_item_backgrounds(
    bounds: Bounds<Pixels>,
    window: &mut Window,
    items: &[CanvasItem],
    canvas_offset: Point<Pixels>,
    zoom: f32,
    colors: ContentTypeColors,
) {
    profile_scope!("render_item_backgrounds");

    // Early exit if no items
    if items.is_empty() {
        return;
    }

    // Viewport bounds with margin for culling (prevents pop-in at edges)
    use crate::constants::CULLING_MARGIN;
    let vp_left = f32::from(bounds.origin.x) - CULLING_MARGIN;
    let vp_top = f32::from(bounds.origin.y) - CULLING_MARGIN;
    let vp_right = f32::from(bounds.origin.x) + f32::from(bounds.size.width) + CULLING_MARGIN;
    let vp_bottom = f32::from(bounds.origin.y) + f32::from(bounds.size.height) + CULLING_MARGIN;

    // Count items for profiling
    #[cfg(feature = "profiling")]
    let mut painted_count = 0usize;
    #[cfg(feature = "profiling")]
    let mut culled_count = 0usize;

    for item in items {
        // Skip items that render themselves (images, markdown cards, code files, shapes, arrows, textboxes, tables, charts)
        if matches!(
            &item.content,
            ItemContent::Image(_)
                | ItemContent::Markdown { .. }
                | ItemContent::Code { .. }
                | ItemContent::TextBox { .. }
                | ItemContent::Arrow { .. }
                | ItemContent::Shape { .. }
                | ItemContent::Table { .. }
                | ItemContent::Chart { .. }
        ) {
            continue;
        }

        // Calculate screen-space position for culling check
        let item_x = f32::from(bounds.origin.x) + item.position.0 * zoom + f32::from(canvas_offset.x);
        let item_y = f32::from(bounds.origin.y) + item.position.1 * zoom + f32::from(canvas_offset.y);
        let item_w = item.size.0 * zoom;
        let item_h = item.size.1 * zoom;

        // VIEWPORT CULLING: Skip items completely outside visible area
        if item_x + item_w < vp_left || item_x > vp_right ||
           item_y + item_h < vp_top || item_y > vp_bottom {
            #[cfg(feature = "profiling")]
            {
                culled_count += 1;
            }
            continue;
        }

        let item_bounds = Bounds {
            origin: point(px(item_x), px(item_y)),
            size: size(px(item_w), px(item_h)),
        };

        // Use theme-aware colors for content types
        let bg_color = colors.for_content(&item.content);

        window.paint_quad(quad(
            item_bounds,
            px(8.0 * zoom),
            bg_color,
            px(2.0 * zoom),
            colors.border,
            Default::default(),
        ));

        #[cfg(feature = "profiling")]
        {
            painted_count += 1;
        }
    }

    #[cfg(feature = "profiling")]
    if painted_count > 0 || culled_count > 0 {
        tracing::trace!(painted = painted_count, culled = culled_count, "Item backgrounds");
    }
}

/// Render a single canvas item based on its content type
fn render_item_content(
    item: &CanvasItem,
    zoom: f32,
    youtube_webviews: &HashMap<u64, YouTubeWebView>,
    audio_webviews: &HashMap<u64, AudioWebView>,
    video_webviews: &HashMap<u64, VideoWebView>,
    data_sources: &HashMap<u64, DataSource>,
    _table_scroll_states: &HashMap<u64, VirtualScrollState>,
    _table_states: &HashMap<u64, Entity<TableState<DataSourceDelegate>>>,
    editing_textbox_id: Option<u64>,
    textbox_input: Option<&Entity<InputState>>,
    _editing_table_cell: Option<(u64, usize, usize)>,
    _table_cell_input: Option<&Entity<InputState>>,
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
                    div()
                        .size_full()
                        .rounded(px(4.0 * zoom))
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

        ItemContent::Table { data_source_id, show_headers: _, stripe: _ } => {
            let border_color = muted_fg.opacity(0.3);
            let header_height = 32.0 * zoom;
            let row_height = 24.0 * zoom;
            let font_size = 11.0 * zoom;
            let small_font = 10.0 * zoom;
            let item_width = item.size.0 * zoom;
            let accent_color = hsla(210.0 / 360.0, 0.7, 0.5, 1.0); // Blue accent

            if let Some(data_source) = data_sources.get(data_source_id) {
                let row_count = data_source.rows.len();
                let col_count = data_source.column_count();
                let table_name = data_source.file_path()
                    .and_then(|p| p.file_stem())
                    .and_then(|n| n.to_str())
                    .unwrap_or("Table");

                // Show thumbnail view with header, stats, and preview rows
                let preview_rows = 5.min(row_count);
                let col_width = if col_count > 0 {
                    ((item_width - 2.0 * zoom) / col_count.min(10) as f32).max(60.0 * zoom)
                } else {
                    100.0 * zoom
                };

                div()
                    .size_full()
                    .bg(muted_bg.opacity(0.05))
                    .rounded(corner_radius)
                    .border_1()
                    .border_color(border_color)
                    .overflow_hidden()
                    .flex()
                    .flex_col()
                    // Header with table name and stats
                    .child(
                        div()
                            .h(px(header_height))
                            .w_full()
                            .bg(accent_color.opacity(0.15))
                            .border_b_1()
                            .border_color(border_color)
                            .px(px(8.0 * zoom))
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0 * zoom))
                                    .child(
                                        Icon::new(IconName::LayoutDashboard)
                                            .size(px(14.0 * zoom))
                                            .text_color(accent_color)
                                    )
                                    .child(
                                        div()
                                            .text_size(px(font_size))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(fg)
                                            .child(table_name.to_string())
                                    )
                            )
                            .child(
                                div()
                                    .text_size(px(small_font))
                                    .text_color(muted_fg)
                                    .child(format!("{} rows × {} cols", row_count, col_count))
                            )
                    )
                    // Column headers
                    .child(
                        div()
                            .h(px(row_height))
                            .w_full()
                            .bg(muted_bg.opacity(0.3))
                            .border_b_1()
                            .border_color(border_color)
                            .flex()
                            .overflow_hidden()
                            .children(
                                data_source.columns.iter().take(10).map(|col| {
                                    div()
                                        .w(px(col_width))
                                        .h_full()
                                        .px(px(4.0 * zoom))
                                        .flex()
                                        .items_center()
                                        .border_r_1()
                                        .border_color(border_color.opacity(0.5))
                                        .child(
                                            div()
                                                .text_size(px(small_font))
                                                .font_weight(FontWeight::MEDIUM)
                                                .text_color(fg)
                                                .overflow_hidden()
                                                .text_ellipsis()
                                                .child(col.name.clone())
                                        )
                                })
                            )
                    )
                    // Preview rows
                    .child(
                        div()
                            .flex_1()
                            .w_full()
                            .overflow_hidden()
                            .children(
                                (0..preview_rows).map(|row_idx| {
                                    let is_alt = row_idx % 2 == 1;
                                    div()
                                        .h(px(row_height))
                                        .w_full()
                                        .when(is_alt, |d| d.bg(muted_bg.opacity(0.15)))
                                        .border_b_1()
                                        .border_color(border_color.opacity(0.3))
                                        .flex()
                                        .overflow_hidden()
                                        .children(
                                            (0..col_count.min(10)).map(|col_idx| {
                                                let cell_value = data_source.rows
                                                    .get(row_idx)
                                                    .and_then(|r| r.cells.get(col_idx))
                                                    .map(|c| c.to_string())
                                                    .unwrap_or_default();
                                                div()
                                                    .w(px(col_width))
                                                    .h_full()
                                                    .px(px(4.0 * zoom))
                                                    .flex()
                                                    .items_center()
                                                    .border_r_1()
                                                    .border_color(border_color.opacity(0.3))
                                                    .child(
                                                        div()
                                                            .text_size(px(small_font))
                                                            .text_color(muted_fg)
                                                            .overflow_hidden()
                                                            .text_ellipsis()
                                                            .child(cell_value)
                                                    )
                                            })
                                        )
                                })
                            )
                    )
                    // Footer hint
                    .child(
                        div()
                            .h(px(row_height))
                            .w_full()
                            .bg(muted_bg.opacity(0.2))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_size(px(small_font))
                                    .text_color(muted_fg.opacity(0.7))
                                    .child(if row_count > preview_rows {
                                        format!("... {} more rows • Double-click to view", row_count - preview_rows)
                                    } else {
                                        "Double-click to view full table".to_string()
                                    })
                            )
                    )
            } else {
                // Data source not found - show placeholder
                div()
                    .size_full()
                    .bg(muted_bg)
                    .rounded(corner_radius)
                    .border_1()
                    .border_color(border_color)
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_size(px(14.0 * zoom))
                            .text_color(muted_fg)
                            .child("Data source not found")
                    )
            }
        }

        ItemContent::Chart { data_source_id, config, .. } => {
            let border_color = muted_fg.opacity(0.3);
            let chart_type_label = config.chart_type.label();
            let header_height = 36.0 * zoom;
            let padding = 12.0 * zoom;
            let font_size = 11.0 * zoom;

            if let Some(data_source) = data_sources.get(data_source_id) {
                let col_count = data_source.column_count();

                // Get column names for context
                let x_col = config.x_column.unwrap_or(0);
                let y_col = if config.y_columns.is_empty() {
                    if col_count > 1 { 1 } else { 0 }
                } else {
                    config.y_columns[0]
                };

                let x_col_name = data_source.columns.get(x_col).map(|c| c.name.as_str()).unwrap_or("X");
                let y_col_name = data_source.columns.get(y_col).map(|c| c.name.as_str()).unwrap_or("Value");

                // Build chart data points
                #[derive(Clone)]
                struct ChartPoint {
                    label: String,
                    value: f64,
                    color: Hsla,
                }

                // Highly distinct colors - maximally separated on color wheel
                let chart_colors = [
                    hsla(220.0 / 360.0, 0.85, 0.55, 1.0),  // Bright Blue
                    hsla(140.0 / 360.0, 0.75, 0.45, 1.0),  // Green
                    hsla(30.0 / 360.0, 0.95, 0.55, 1.0),   // Orange
                    hsla(270.0 / 360.0, 0.75, 0.55, 1.0),  // Violet/Purple
                    hsla(0.0 / 360.0, 0.80, 0.55, 1.0),    // Red
                    hsla(175.0 / 360.0, 0.75, 0.45, 1.0),  // Cyan/Teal
                    hsla(55.0 / 360.0, 0.90, 0.50, 1.0),   // Yellow
                    hsla(320.0 / 360.0, 0.75, 0.55, 1.0),  // Pink/Magenta
                ];

                // Build chart data with grouping, aggregation, and sorting
                let data: Vec<ChartPoint> = {
                    use std::collections::HashMap;
                    use crate::types::{AggregationType, SortOrder};

                    // Group raw values by label (X column), preserving insertion order
                    let mut group_order: Vec<String> = Vec::new();
                    let mut groups: HashMap<String, Vec<f64>> = HashMap::new();
                    for row in &data_source.rows {
                        let label = row.cells.get(x_col).map(|c| c.to_string()).unwrap_or_default();
                        let value = row.cells.get(y_col).map(|c| c.to_f64()).unwrap_or(0.0);
                        if !groups.contains_key(&label) {
                            group_order.push(label.clone());
                        }
                        groups.entry(label).or_default().push(value);
                    }

                    // Apply aggregation in insertion order
                    let mut points: Vec<(String, f64)> = group_order.into_iter().map(|label| {
                        let values = groups.get(&label).unwrap();
                        let aggregated = match config.aggregation {
                            AggregationType::None => values.first().copied().unwrap_or(0.0),
                            AggregationType::Sum => values.iter().sum(),
                            AggregationType::Average => {
                                if values.is_empty() { 0.0 }
                                else { values.iter().sum::<f64>() / values.len() as f64 }
                            }
                            AggregationType::Count => values.len() as f64,
                            AggregationType::Min => values.iter().cloned().fold(f64::INFINITY, f64::min),
                            AggregationType::Max => values.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
                        };
                        (label, aggregated)
                    }).collect();

                    // Apply sorting
                    match config.sort_order {
                        SortOrder::None => {} // Keep original insertion order
                        SortOrder::LabelAsc => points.sort_by(|a, b| a.0.cmp(&b.0)),
                        SortOrder::LabelDesc => points.sort_by(|a, b| b.0.cmp(&a.0)),
                        SortOrder::ValueAsc => points.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)),
                        SortOrder::ValueDesc => points.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)),
                    }

                    // Convert to ChartPoints with colors, limit for readability
                    points.into_iter()
                        .take(12)
                        .enumerate()
                        .map(|(i, (label, value))| {
                            ChartPoint {
                                label,
                                value,
                                color: chart_colors[i % chart_colors.len()],
                            }
                        })
                        .collect()
                };

                let mut chart_container = div()
                    .size_full()
                    .bg(muted_bg.opacity(0.05))
                    .rounded(corner_radius)
                    .border_1()
                    .border_color(border_color)
                    .flex()
                    .flex_col()
                    .overflow_hidden();

                // Header with title and Y-axis column name
                let title = config.title.as_deref().unwrap_or(&data_source.name);
                chart_container = chart_container.child(
                    div()
                        .w_full()
                        .h(px(header_height))
                        .px(px(padding))
                        .flex()
                        .items_center()
                        .justify_between()
                        .border_b_1()
                        .border_color(border_color)
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .child(
                                    div()
                                        .text_size(px(font_size * 1.1))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(fg)
                                        .child(title.to_string())
                                )
                                .child(
                                    div()
                                        .text_size(px(font_size * 0.85))
                                        .text_color(muted_fg)
                                        .child({
                                            use crate::types::AggregationType;
                                            let agg_label = match config.aggregation {
                                                AggregationType::None => "",
                                                AggregationType::Sum => "Sum of ",
                                                AggregationType::Average => "Avg of ",
                                                AggregationType::Count => "Count of ",
                                                AggregationType::Min => "Min of ",
                                                AggregationType::Max => "Max of ",
                                            };
                                            format!("{}{} by {}", agg_label, y_col_name, x_col_name)
                                        })
                                )
                        )
                        .child(
                            div()
                                .px(px(6.0 * zoom))
                                .py(px(2.0 * zoom))
                                .bg(muted_bg.opacity(0.3))
                                .rounded(px(4.0 * zoom))
                                .text_size(px(font_size * 0.85))
                                .text_color(muted_fg)
                                .child(chart_type_label.to_string())
                        )
                );

                // Helper to format axis values
                fn format_axis_value(val: f64) -> String {
                    if val == 0.0 {
                        "0".to_string()
                    } else if val.abs() >= 1_000_000.0 {
                        format!("{:.1}M", val / 1_000_000.0)
                    } else if val.abs() >= 1_000.0 {
                        format!("{:.1}K", val / 1_000.0)
                    } else if val.abs() >= 100.0 {
                        format!("{:.0}", val)
                    } else if val.fract() == 0.0 {
                        format!("{:.0}", val)
                    } else {
                        format!("{:.1}", val)
                    }
                }

                // Calculate Y-axis range for bar/line/scatter charts
                let max_val = data.iter().map(|d| d.value).fold(0.0_f64, |a, b| a.max(b));
                let y_axis_font_size = font_size * 0.75;
                let y_axis_width = 32.0 * zoom;

                // Chart content
                match config.chart_type {
                    crate::types::ChartType::Bar => {
                        let bar_chart = BarChart::new(data.clone())
                            .x(|d| d.label.clone())
                            .y(|d| d.value)
                            .fill(|d| d.color);

                        chart_container = chart_container.child(
                            div()
                                .flex_1()
                                .w_full()
                                .p(px(padding))
                                .flex()
                                .flex_row()
                                // Y-axis labels
                                .child(
                                    div()
                                        .w(px(y_axis_width))
                                        .h_full()
                                        .flex_shrink_0()
                                        .flex()
                                        .flex_col()
                                        .justify_between()
                                        .pr(px(4.0 * zoom))
                                        .text_size(px(y_axis_font_size))
                                        .text_color(muted_fg)
                                        .child(div().text_right().child(format_axis_value(max_val)))
                                        .child(div().text_right().child(format_axis_value(max_val * 0.5)))
                                        .child(div().text_right().child("0"))
                                )
                                // Chart area
                                .child(
                                    div()
                                        .flex_1()
                                        .h_full()
                                        .child(bar_chart)
                                )
                        );
                    }
                    crate::types::ChartType::Line | crate::types::ChartType::Area => {
                        let line_color = chart_colors[0];
                        let line_chart = LineChart::new(data.clone())
                            .x(|d| d.label.clone())
                            .y(|d| d.value)
                            .stroke(line_color)
                            .dot();

                        chart_container = chart_container.child(
                            div()
                                .flex_1()
                                .w_full()
                                .p(px(padding))
                                .flex()
                                .flex_row()
                                // Y-axis labels
                                .child(
                                    div()
                                        .w(px(y_axis_width))
                                        .h_full()
                                        .flex_shrink_0()
                                        .flex()
                                        .flex_col()
                                        .justify_between()
                                        .pr(px(4.0 * zoom))
                                        .text_size(px(y_axis_font_size))
                                        .text_color(muted_fg)
                                        .child(div().text_right().child(format_axis_value(max_val)))
                                        .child(div().text_right().child(format_axis_value(max_val * 0.5)))
                                        .child(div().text_right().child("0"))
                                )
                                // Chart area
                                .child(
                                    div()
                                        .flex_1()
                                        .h_full()
                                        .child(line_chart)
                                )
                        );
                    }
                    crate::types::ChartType::Pie => {
                        // Calculate pie size based on available height
                        let available_height = item.size.1 * zoom - header_height - padding * 2.0;
                        let pie_size = (available_height * 0.4).min(80.0 * zoom).max(30.0 * zoom);

                        let pie_chart = PieChart::new(data.clone())
                            .value(|d| d.value as f32)
                            .color(|d| d.color)
                            .outer_radius(pie_size)
                            .inner_radius(pie_size * 0.55);  // Donut style

                        // Pie with legend side by side
                        let total: f64 = data.iter().map(|d| d.value).sum();
                        chart_container = chart_container.child(
                            div()
                                .flex_1()
                                .w_full()
                                .p(px(padding))
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap(px(padding * 1.5))
                                // Pie container - fixed width
                                .child(
                                    div()
                                        .w(px(pie_size * 2.2))
                                        .h(px(pie_size * 2.2))
                                        .flex_shrink_0()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .child(pie_chart)
                                )
                                // Legend container - takes remaining space
                                .child({
                                    let mut legend = div()
                                        .flex_1()
                                        .flex()
                                        .flex_col()
                                        .gap(px(3.0 * zoom))
                                        .overflow_hidden();

                                    for point in data.iter().take(6) {
                                        let pct = if total > 0.0 { point.value / total * 100.0 } else { 0.0 };
                                        legend = legend.child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap(px(6.0 * zoom))
                                                .child(
                                                    div()
                                                        .w(px(12.0 * zoom))
                                                        .h(px(12.0 * zoom))
                                                        .flex_shrink_0()
                                                        .bg(point.color)
                                                        .rounded(px(3.0 * zoom))
                                                )
                                                .child(
                                                    div()
                                                        .flex_1()
                                                        .min_w_0()
                                                        .text_size(px(font_size * 0.85))
                                                        .text_color(fg)
                                                        .overflow_hidden()
                                                        .text_ellipsis()
                                                        .whitespace_nowrap()
                                                        .child(point.label.clone())
                                                )
                                                .child(
                                                    div()
                                                        .flex_shrink_0()
                                                        .text_size(px(font_size * 0.8))
                                                        .text_color(muted_fg)
                                                        .child(format!("{:.0}%", pct))
                                                )
                                        );
                                    }
                                    legend
                                })
                        );
                    }
                    crate::types::ChartType::Scatter => {
                        // Use line chart without connecting lines (just dots)
                        let line_color = chart_colors[0];
                        let scatter_chart = LineChart::new(data.clone())
                            .x(|d| d.label.clone())
                            .y(|d| d.value)
                            .stroke(line_color.opacity(0.0))  // No line
                            .dot();

                        chart_container = chart_container.child(
                            div()
                                .flex_1()
                                .w_full()
                                .p(px(padding))
                                .flex()
                                .flex_row()
                                // Y-axis labels
                                .child(
                                    div()
                                        .w(px(y_axis_width))
                                        .h_full()
                                        .flex_shrink_0()
                                        .flex()
                                        .flex_col()
                                        .justify_between()
                                        .pr(px(4.0 * zoom))
                                        .text_size(px(y_axis_font_size))
                                        .text_color(muted_fg)
                                        .child(div().text_right().child(format_axis_value(max_val)))
                                        .child(div().text_right().child(format_axis_value(max_val * 0.5)))
                                        .child(div().text_right().child("0"))
                                )
                                // Chart area
                                .child(
                                    div()
                                        .flex_1()
                                        .h_full()
                                        .child(scatter_chart)
                                )
                        );
                    }
                }

                chart_container
            } else {
                // Data source not found
                div()
                    .size_full()
                    .bg(muted_bg)
                    .rounded(corner_radius)
                    .border_1()
                    .border_color(border_color)
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_size(px(14.0 * zoom))
                            .text_color(muted_fg)
                            .child("No data")
                    )
            }
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

    if (max - min).abs() < f32::EPSILON {
        return Some(hsla(0.0, 0.0, l, 1.0));
    }

    let d = max - min;
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };

    let h = if (max - r).abs() < f32::EPSILON {
        ((g - b) / d + if g < b { 6.0 } else { 0.0 }) / 6.0
    } else if (max - g).abs() < f32::EPSILON {
        ((b - r) / d + 2.0) / 6.0
    } else {
        ((r - g) / d + 4.0) / 6.0
    };

    Some(hsla(h, s, l, 1.0))
}

/// Render all canvas items with positioning and selection
///
/// This is a key hot path - called every frame for all visible items.
/// Performance optimizations:
/// - Viewport culling: Only renders items within the visible viewport
/// - Pre-computed bounds: Calculates screen positions once per item
/// - Early exit for items outside viewport
pub fn render_items(
    items: &[CanvasItem],
    canvas_offset: Point<Pixels>,
    zoom: f32,
    selected_items: &std::collections::HashSet<u64>,
    youtube_webviews: &HashMap<u64, YouTubeWebView>,
    audio_webviews: &HashMap<u64, AudioWebView>,
    video_webviews: &HashMap<u64, VideoWebView>,
    data_sources: &HashMap<u64, DataSource>,
    table_scroll_states: &HashMap<u64, VirtualScrollState>,
    table_states: &HashMap<u64, Entity<TableState<DataSourceDelegate>>>,
    editing_textbox_id: Option<u64>,
    textbox_input: Option<&Entity<InputState>>,
    editing_table_cell: Option<(u64, usize, usize)>,
    table_cell_input: Option<&Entity<InputState>>,
    viewport_size: Size<Pixels>,
    cx: &Context<Humanboard>,
) -> Vec<Div> {
    profile_scope!("render_items");

    let offset_x = f32::from(canvas_offset.x);
    let offset_y = f32::from(canvas_offset.y);

    // Viewport bounds for culling (with margin to prevent pop-in)
    use crate::constants::CULLING_MARGIN;
    let vp_left = -CULLING_MARGIN;
    let vp_top = -CULLING_MARGIN;
    let vp_right = f32::from(viewport_size.width) + CULLING_MARGIN;
    let vp_bottom = f32::from(viewport_size.height) + CULLING_MARGIN;

    let fg = cx.theme().foreground;
    let muted_fg = cx.theme().muted_foreground;
    let muted_bg = cx.theme().muted;
    let danger = cx.theme().danger;
    let primary = cx.theme().primary;

    // Pre-allocate with estimated visible items to reduce allocations
    let mut result = Vec::with_capacity(items.len().min(100));

    for item in items {
        let x = item.position.0 * zoom + offset_x;
        let y = item.position.1 * zoom + offset_y;
        let w = item.size.0 * zoom;
        let h = item.size.1 * zoom;

        // VIEWPORT CULLING: Skip items completely outside visible area
        if x + w < vp_left || x > vp_right || y + h < vp_top || y > vp_bottom {
            continue;
        }

        let is_selected = selected_items.contains(&item.id);

        // Check if this textbox is currently being edited
        let is_editing_this = editing_textbox_id == Some(item.id);
        // Don't show selection border while editing textbox (it has its own editing border)
        let show_selection = is_selected && !is_editing_this;

        // Check if this is a table item (for chart creation button)
        let is_table = matches!(&item.content, ItemContent::Table { .. });
        let item_id = item.id;

        result.push(
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
                    data_sources,
                    table_scroll_states,
                    table_states,
                    editing_textbox_id,
                    textbox_input,
                    editing_table_cell,
                    table_cell_input,
                    fg,
                    muted_fg,
                    muted_bg,
                    danger,
                ))
                // Add double-click handler for table cell editing (only when NOT currently editing this table)
                .when(is_table && editing_table_cell.map(|(id, _, _)| id != item_id).unwrap_or(true), |d| {
                    // Get table dimensions for cell position calculation
                    let cell_height = 28.0 * zoom;
                    let header_height = cell_height; // Header is same height as cells
                    let table_width = w;
                    let table_x = x;
                    let table_y = y;

                    d.child(
                        div()
                            .id(ElementId::Name(format!("table-click-{}", item_id).into()))
                            .absolute()
                            .inset_0()
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, event: &MouseDownEvent, window, cx| {
                                // Check for double-click
                                if event.click_count == 2 {
                                    // Calculate which cell was clicked
                                    // event.position is in window coordinates, need to convert to canvas coordinates
                                    let click_x: f32 = event.position.x.into();
                                    let click_y: f32 = event.position.y.into();

                                    // Subtract dock width and header height to get canvas-relative coordinates
                                    let canvas_click_x = click_x - crate::constants::DOCK_WIDTH;
                                    let canvas_click_y = click_y - crate::constants::HEADER_HEIGHT;

                                    if let Some(ref board) = this.board {
                                        if let Some(table_item) = board.items.iter().find(|i| i.id == item_id) {
                                            if let ItemContent::Table { data_source_id, show_headers, .. } = &table_item.content {
                                                if let Some(ds) = board.data_sources.get(data_source_id) {
                                                    let col_count = ds.column_count();
                                                    if col_count == 0 { return; }

                                                    let col_width = table_width / col_count as f32;

                                                    // Calculate row and column from click position (relative to table)
                                                    let local_y = canvas_click_y - table_y;
                                                    let local_x = canvas_click_x - table_x;

                                                    // Skip header row if shown
                                                    let header_offset = if *show_headers { header_height } else { 0.0 };

                                                    if local_y < header_offset {
                                                        return; // Clicked on header, don't edit
                                                    }

                                                    let row = ((local_y - header_offset) / cell_height).floor() as usize;
                                                    let col = (local_x / col_width).floor() as usize;

                                                    // Bounds check
                                                    if row < ds.row_count() && col < col_count {
                                                        this.start_table_cell_editing(item_id, row, col, window, cx);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }))
                    )
                })
                .when(show_selection, |d| {
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
                }),
        );

        // Add chart toolbar as SEPARATE element (not child) for selected tables
        // This avoids clipping issues with the parent item bounds
        if is_table && show_selection {
            let primary_fg = cx.theme().primary_foreground;
            let btn_height = 28.0 * zoom;
            let btn_padding = 10.0 * zoom;
            let btn_gap = 8.0 * zoom;
            let toolbar_y = y - btn_height - 8.0 * zoom;

            // Check if table has file origin for reload button
            let has_file_origin = if let ItemContent::Table { data_source_id, .. } = &item.content {
                data_sources.get(data_source_id).map(|d| d.has_file_origin()).unwrap_or(false)
            } else {
                false
            };

            let mut toolbar = div()
                .absolute()
                .left(px(x))
                .top(px(toolbar_y))
                .w(px(w))
                .h(px(btn_height))
                .flex()
                .flex_row()
                .justify_end()
                .gap(px(btn_gap));

            // Reload button (only shown if table has file origin)
            if has_file_origin {
                toolbar = toolbar.child(
                    div()
                        .id(ElementId::Name(format!("reload-table-btn-{}", item_id).into()))
                        .h(px(btn_height))
                        .px(px(btn_padding))
                        .bg(muted_bg)
                        .rounded(px(6.0 * zoom))
                        .cursor_pointer()
                        .flex()
                        .flex_row()
                        .items_center()
                        .shadow_md()
                        .hover(|s| s.opacity(0.85))
                        .on_mouse_down(MouseButton::Left, |_, _, cx| {
                            cx.stop_propagation();
                        })
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.reload_table_from_file(item_id, cx);
                        }))
                        .child(
                            div()
                                .text_size(px(12.0 * zoom))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(fg)
                                .child("Reload")
                        )
                );
            }

            // Create Chart button
            toolbar = toolbar.child(
                div()
                    .id(ElementId::Name(format!("create-chart-btn-{}", item_id).into()))
                    .h(px(btn_height))
                    .px(px(btn_padding))
                    .bg(primary)
                    .rounded(px(6.0 * zoom))
                    .cursor_pointer()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(6.0 * zoom))
                    .shadow_md()
                    .hover(|s| s.opacity(0.85))
                    .on_mouse_down(MouseButton::Left, |_, _, cx| {
                        cx.stop_propagation();
                    })
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.show_chart_config_modal(item_id, cx);
                    }))
                    .child(
                        div()
                            .text_size(px(14.0 * zoom))
                            .font_weight(FontWeight::BOLD)
                            .text_color(primary_fg)
                            .child("+")
                    )
                    .child(
                        div()
                            .text_size(px(12.0 * zoom))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(primary_fg)
                            .child("Chart")
                    )
            );

            result.push(toolbar);
        }
    }

    result
}

/// Render the canvas area container
///
/// This is the main entry point for canvas rendering. It composes:
/// 1. Background canvas with item backgrounds (GPU painted)
/// 2. Individual item content elements
/// 3. Selection overlays (marquee, drawing preview)
pub fn render_canvas_area(
    canvas_offset: Point<Pixels>,
    zoom: f32,
    items: &[CanvasItem],
    selected_items: &std::collections::HashSet<u64>,
    youtube_webviews: &HashMap<u64, YouTubeWebView>,
    audio_webviews: &HashMap<u64, AudioWebView>,
    video_webviews: &HashMap<u64, VideoWebView>,
    data_sources: &HashMap<u64, DataSource>,
    table_scroll_states: &HashMap<u64, VirtualScrollState>,
    table_states: &HashMap<u64, Entity<TableState<DataSourceDelegate>>>,
    editing_textbox_id: Option<u64>,
    textbox_input: Option<&Entity<InputState>>,
    editing_table_cell: Option<(u64, usize, usize)>,
    table_cell_input: Option<&Entity<InputState>>,
    marquee: Option<(Point<Pixels>, Point<Pixels>)>,
    drawing_preview: Option<(Point<Pixels>, Point<Pixels>, crate::types::ToolType)>,
    viewport_size: Size<Pixels>,
    cx: &Context<Humanboard>,
) -> Div {
    profile_scope!("render_canvas_area");

    let bg = cx.theme().background;
    let primary = cx.theme().primary;
    let fg = cx.theme().foreground;
    let content_colors = ContentTypeColors::from_theme(cx.theme());

    div()
        .size_full()
        .bg(bg)
        .overflow_hidden()
        .relative()
        .child(render_canvas(canvas_offset, zoom, items.to_vec(), content_colors))
        .children(render_items(
            items,
            canvas_offset,
            zoom,
            selected_items,
            youtube_webviews,
            audio_webviews,
            video_webviews,
            data_sources,
            table_scroll_states,
            table_states,
            editing_textbox_id,
            textbox_input,
            editing_table_cell,
            table_cell_input,
            viewport_size,
            cx,
        ))
        // Render marquee selection rectangle
        .when_some(marquee, |d, (start, current)| {
            // Account for dock width since mouse coords are in window space
            // but we're rendering in canvas space (after dock)
            let dock_offset = crate::constants::DOCK_WIDTH;
            let start_x = f32::from(start.x) - dock_offset;
            let current_x = f32::from(current.x) - dock_offset;
            let min_x = start_x.min(current_x);
            let max_x = start_x.max(current_x);
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
                        .top(px(min_y - HEADER_HEIGHT)) // Account for header offset
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
            // Account for dock width since mouse coords are in window space
            // but we're rendering in canvas space (after dock)
            let dock_offset = crate::constants::DOCK_WIDTH;
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
                                .top(px(min_y - HEADER_HEIGHT)) // Account for header offset
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
                        let arrow_start_y = start_y - HEADER_HEIGHT; // Account for header
                        let arrow_end_x = current_x;
                        let arrow_end_y = current_y - HEADER_HEIGHT;

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
