//! Canvas rendering module.
//!
//! Handles rendering of the canvas background and canvas items.

use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::types::{CanvasItem, ItemContent};

/// Render the complete canvas area with items
pub fn render_canvas_area(
    canvas_offset: Point<Pixels>,
    zoom: f32,
    items: Vec<CanvasItem>,
    items_for_render: Vec<CanvasItem>,
    selected_item_id: Option<u64>,
) -> Div {
    div()
        .size_full()
        .bg(rgb(0x000000))
        .overflow_hidden()
        .relative()
        .child(render_canvas_background(canvas_offset, zoom, items))
        .children(render_items(
            items_for_render,
            canvas_offset,
            zoom,
            selected_item_id,
        ))
}

/// Render the canvas background with GPU-accelerated quads for non-image items
pub fn render_canvas_background(
    canvas_offset: Point<Pixels>,
    zoom: f32,
    items: Vec<CanvasItem>,
) -> impl IntoElement {
    canvas(
        move |_bounds, _window, _cx| {},
        move |bounds, _data, window, _cx| {
            render_item_backgrounds(bounds, window, &items, canvas_offset, zoom);
        },
    )
    .absolute()
    .size_full()
}

/// Render colored background quads for non-image items
fn render_item_backgrounds(
    bounds: Bounds<Pixels>,
    window: &mut Window,
    items: &[CanvasItem],
    canvas_offset: Point<Pixels>,
    zoom: f32,
) {
    for item in items {
        // Skip images - they're rendered as DOM elements
        if matches!(&item.content, ItemContent::Image(_)) {
            continue;
        }

        let item_bounds = calculate_item_bounds(bounds, item, canvas_offset, zoom);
        let (bg_color, border_color) = get_item_colors(&item.content);

        window.paint_quad(quad(
            item_bounds,
            px(8.0 * zoom),
            bg_color,
            px(2.0 * zoom),
            border_color,
            Default::default(),
        ));
    }
}

/// Calculate screen bounds for an item
#[inline]
fn calculate_item_bounds(
    container_bounds: Bounds<Pixels>,
    item: &CanvasItem,
    canvas_offset: Point<Pixels>,
    zoom: f32,
) -> Bounds<Pixels> {
    Bounds {
        origin: point(
            container_bounds.origin.x + px(item.position.0 * zoom) + canvas_offset.x,
            container_bounds.origin.y + px(item.position.1 * zoom) + canvas_offset.y,
        ),
        size: size(px(item.size.0 * zoom), px(item.size.1 * zoom)),
    }
}

/// Get colors for an item type
#[inline]
fn get_item_colors(content: &ItemContent) -> (Hsla, Hsla) {
    let border = hsla(0.0, 0.0, 1.0, 0.3);

    let bg = match content {
        ItemContent::Video(_) => hsla(0.15, 0.7, 0.5, 0.9),
        ItemContent::Text(_) => hsla(0.6, 0.7, 0.5, 0.9),
        ItemContent::Pdf { .. } => hsla(0.0, 0.7, 0.5, 0.9),
        ItemContent::Link(_) => hsla(0.35, 0.7, 0.5, 0.9),
        ItemContent::Image(_) => hsla(0.0, 0.0, 0.5, 0.9),
    };

    (bg, border)
}

/// Render all items as DOM elements
pub fn render_items(
    items: Vec<CanvasItem>,
    canvas_offset: Point<Pixels>,
    zoom: f32,
    selected_item_id: Option<u64>,
) -> Vec<Div> {
    items
        .iter()
        .map(|item| render_item(item, canvas_offset, zoom, selected_item_id))
        .collect()
}

/// Render a single item
fn render_item(
    item: &CanvasItem,
    canvas_offset: Point<Pixels>,
    zoom: f32,
    selected_item_id: Option<u64>,
) -> Div {
    let x = item.position.0 * zoom + f32::from(canvas_offset.x);
    let y = item.position.1 * zoom + f32::from(canvas_offset.y);
    let w = item.size.0 * zoom;
    let h = item.size.1 * zoom;
    let is_selected = selected_item_id == Some(item.id);

    div()
        .relative()
        .child(render_item_content(item, x, y, w, h, zoom))
        .when(is_selected, |parent| {
            parent.child(render_resize_handle(x, y, w, h, zoom))
        })
}

/// Render the content of an item (image, PDF thumbnail, etc.)
fn render_item_content(item: &CanvasItem, x: f32, y: f32, w: f32, h: f32, zoom: f32) -> Div {
    div()
        .absolute()
        .left(px(x))
        .top(px(y))
        .w(px(w))
        .h(px(h))
        .overflow_hidden()
        .rounded(px(8.0 * zoom))
        .when(matches!(&item.content, ItemContent::Image(_)), |d| {
            if let ItemContent::Image(path) = &item.content {
                d.child(
                    img(path.clone())
                        .absolute()
                        .size_full()
                        .object_fit(ObjectFit::Contain),
                )
            } else {
                d
            }
        })
        .when(matches!(&item.content, ItemContent::Pdf { .. }), |d| {
            if let ItemContent::Pdf {
                thumbnail: Some(thumb_path),
                ..
            } = &item.content
            {
                d.child(
                    img(thumb_path.clone())
                        .absolute()
                        .size_full()
                        .object_fit(ObjectFit::Contain),
                )
            } else {
                d
            }
        })
}

/// Render the resize handle for a selected item
fn render_resize_handle(x: f32, y: f32, w: f32, h: f32, zoom: f32) -> Div {
    let handle_size = 20.0 * zoom;

    div()
        .absolute()
        .left(px(x + w - handle_size))
        .top(px(y + h - handle_size))
        .w(px(handle_size))
        .h(px(handle_size))
        .bg(hsla(0.0, 0.0, 1.0, 0.7))
        .rounded_tl(px(4.0 * zoom))
        .border_2()
        .border_color(hsla(0.0, 0.0, 1.0, 1.0))
}
