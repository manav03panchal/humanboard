use crate::actions::{
    ClosePreview, DeleteSelected, NextPage, PrevPage, Redo, ToggleSplit, Undo, ZoomIn, ZoomOut,
    ZoomReset,
};
use crate::app::{Humanboard, SplitDirection};
use crate::types::{CanvasItem, ItemContent};
use gpui::DefiniteLength::Fraction;
use gpui::prelude::FluentBuilder;
use gpui::*;

// Render helper functions
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

fn render_item_backgrounds(
    bounds: Bounds<Pixels>,
    window: &mut Window,
    items: &[CanvasItem],
    canvas_offset: Point<Pixels>,
    zoom: f32,
) {
    for item in items {
        if matches!(&item.content, ItemContent::Image(_)) {
            continue; // Skip images - rendered as DOM elements
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
            ItemContent::Pdf(_) => hsla(0.0, 0.7, 0.5, 0.9),
            ItemContent::Link(_) => hsla(0.35, 0.7, 0.5, 0.9),
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

pub fn render_items(
    items: Vec<CanvasItem>,
    canvas_offset: Point<Pixels>,
    zoom: f32,
    selected_item_id: Option<u64>,
) -> Vec<Div> {
    items
        .iter()
        .map(|item| {
            let x = item.position.0 * zoom + f32::from(canvas_offset.x);
            let y = item.position.1 * zoom + f32::from(canvas_offset.y);
            let w = item.size.0 * zoom;
            let h = item.size.1 * zoom;
            let is_selected = selected_item_id == Some(item.id);

            div()
                .relative()
                .child(
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
                        }),
                )
                .when(is_selected, |parent| {
                    parent.child(
                        div()
                            .absolute()
                            .left(px(x + w - 20.0 * zoom))
                            .top(px(y + h - 20.0 * zoom))
                            .w(px(20.0 * zoom))
                            .h(px(20.0 * zoom))
                            .bg(hsla(0.0, 0.0, 1.0, 0.7))
                            .rounded_tl(px(4.0 * zoom))
                            .border_2()
                            .border_color(hsla(0.0, 0.0, 1.0, 1.0)),
                    )
                })
        })
        .collect()
}

pub fn render_stats_overlay(
    fps: f32,
    frame_count: u64,
    item_count: usize,
    zoom: f32,
    canvas_offset: Point<Pixels>,
) -> Div {
    div()
        .absolute()
        .top_4()
        .left_4()
        .p_3()
        .bg(hsla(0.0, 0.0, 0.0, 0.8))
        .rounded_md()
        .text_xs()
        .text_color(rgb(0xffffff))
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .font_weight(FontWeight::BOLD)
                .text_sm()
                .child("Humanboard"),
        )
        .child(div().child(format!("FPS: {:.1}", fps)))
        .child(div().child(format!("Frame: {}", frame_count)))
        .child(div().child(format!("Items: {}", item_count)))
        .child(div().child(format!("Zoom: {:.2}x", zoom)))
        .child(div().child(format!(
            "X: {:.0} Y: {:.0}",
            f32::from(canvas_offset.x),
            f32::from(canvas_offset.y)
        )))
        .child(
            div()
                .text_color(hsla(0.0, 0.0, 0.6, 1.0))
                .child("Drag to pan • Scroll to zoom • Drop files"),
        )
}

pub fn render_selected_item_label(name: String) -> Div {
    div()
        .absolute()
        .bottom_4()
        .left_4()
        .p_3()
        .bg(hsla(0.0, 0.0, 0.0, 0.8))
        .rounded_md()
        .text_sm()
        .text_color(rgb(0xffffff))
        .child(name)
}

pub fn render_preview_panel(
    file_name: String,
    page_image_path: Option<std::path::PathBuf>,
    current_page: usize,
    page_count: usize,
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
        .bg(rgb(0x2d2d2d))
        .child(
            // Header bar - compact layout
            div()
                .flex()
                .items_center()
                .gap_2()
                .px_3()
                .py_2()
                .bg(rgb(0x3d3d3d))
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
                // Keyboard hints
                .child(
                    div()
                        .flex_shrink_0()
                        .text_xs()
                        .text_color(rgb(0x666666))
                        .child("←→ T Esc"),
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
                .bg(rgb(0x1a1a1a))
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

pub fn render_splitter(direction: SplitDirection) -> Div {
    match direction {
        SplitDirection::Vertical => div()
            .w(px(6.0))
            .h_full()
            .bg(rgb(0x404040))
            .hover(|s| s.bg(rgb(0x606060)))
            .cursor(CursorStyle::ResizeLeftRight),
        SplitDirection::Horizontal => div()
            .h(px(6.0))
            .w_full()
            .bg(rgb(0x404040))
            .hover(|s| s.bg(rgb(0x606060)))
            .cursor(CursorStyle::ResizeUpDown),
    }
}

// Render the canvas area (moodboard content)
fn render_canvas_area(
    canvas_offset: Point<Pixels>,
    zoom: f32,
    items: Vec<CanvasItem>,
    items_for_render: Vec<CanvasItem>,
    selected_item_id: Option<u64>,
    fps: f32,
    frame_count: u64,
    item_count: usize,
    selected_item_name: Option<String>,
) -> Div {
    div()
        .size_full()
        .bg(rgb(0x1e1e1e))
        .overflow_hidden()
        .relative()
        .child(render_canvas(canvas_offset, zoom, items))
        .children(render_items(
            items_for_render,
            canvas_offset,
            zoom,
            selected_item_id,
        ))
        .child(render_stats_overlay(
            fps,
            frame_count,
            item_count,
            zoom,
            canvas_offset,
        ))
        .when_some(selected_item_name, |parent, name| {
            parent.child(render_selected_item_label(name))
        })
}

// Render implementation for Humanboard
impl Render for Humanboard {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.update_fps();
        cx.notify();

        let canvas_offset = self.board.canvas_offset;
        let zoom = self.board.zoom;
        let fps = self.calculate_fps();
        let frame_count = self.frame_count;
        let items = self.board.items.clone();
        let items_for_render = self.board.items.clone();
        let item_count = items.len();
        let selected_item_id = self.selected_item;
        let selected_item_name = self.selected_item.and_then(|id| {
            self.board
                .items
                .iter()
                .find(|i| i.id == id)
                .map(|i| i.content.display_name())
        });

        // Extract preview info without cloning the whole struct
        let preview_info = self.preview.as_ref().map(|p| {
            (
                p.split,
                p.size,
                p.path.clone(),
                p.current_page_image.clone(),
                p.pdf_doc
                    .as_ref()
                    .map(|pdf| (pdf.current_page, pdf.page_count)),
            )
        });

        let base = div()
            .size_full()
            .track_focus(&self.focus_handle)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event, window, cx| {
                    this.focus_handle.focus(window);
                    this.handle_mouse_down(event, window, cx);
                }),
            )
            .on_mouse_up(MouseButton::Left, cx.listener(Humanboard::handle_mouse_up))
            .on_mouse_move(cx.listener(Humanboard::handle_mouse_move))
            .on_scroll_wheel(cx.listener(Humanboard::handle_scroll))
            .on_action(cx.listener(|this, _: &ZoomIn, _, cx| this.zoom_in(cx)))
            .on_action(cx.listener(|this, _: &ZoomOut, _, cx| this.zoom_out(cx)))
            .on_action(cx.listener(|this, _: &ZoomReset, _, cx| this.zoom_reset(cx)))
            .on_action(cx.listener(|this, _: &DeleteSelected, _, cx| this.delete_selected(cx)))
            .on_action(cx.listener(|this, _: &Undo, _, cx| this.undo(cx)))
            .on_action(cx.listener(|this, _: &Redo, _, cx| this.redo(cx)))
            .on_action(cx.listener(|this, _: &ClosePreview, _, cx| this.close_preview(cx)))
            .on_action(cx.listener(|this, _: &ToggleSplit, _, cx| this.toggle_split_direction(cx)))
            .on_action(cx.listener(|this, _: &NextPage, _, cx| this.next_page(cx)))
            .on_action(cx.listener(|this, _: &PrevPage, _, cx| this.prev_page(cx)))
            .on_drop(cx.listener(|this, paths: &ExternalPaths, _, cx| {
                if let Some(first_path) = paths.paths().first() {
                    let drop_pos = point(px(400.0), px(300.0));
                    this.board
                        .handle_file_drop(drop_pos, vec![first_path.clone()]);
                    cx.notify();
                }
            }));

        match preview_info {
            Some((split, size, path, page_image, pdf_info)) => {
                let canvas_size = 1.0 - size;
                let preview_size = size;
                let file_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("PDF")
                    .to_string();
                let (current_page, page_count) =
                    pdf_info.map(|(c, p)| (c + 1, p)).unwrap_or((0, 0));

                match split {
                    SplitDirection::Vertical => {
                        // Horizontal layout: canvas | splitter | preview
                        base.flex()
                            .flex_row()
                            .child(
                                div()
                                    .flex_shrink_0()
                                    .w(Fraction(canvas_size))
                                    .h_full()
                                    .child(render_canvas_area(
                                        canvas_offset,
                                        zoom,
                                        items,
                                        items_for_render,
                                        selected_item_id,
                                        fps,
                                        frame_count,
                                        item_count,
                                        selected_item_name,
                                    )),
                            )
                            .child(render_splitter(SplitDirection::Vertical))
                            .child(
                                div()
                                    .flex_shrink_0()
                                    .w(Fraction(preview_size))
                                    .h_full()
                                    .child(render_preview_panel(
                                        file_name,
                                        page_image,
                                        current_page,
                                        page_count,
                                    )),
                            )
                    }
                    SplitDirection::Horizontal => {
                        // Vertical layout: canvas / splitter / preview
                        base.flex()
                            .flex_col()
                            .child(
                                div()
                                    .flex_shrink_0()
                                    .h(Fraction(canvas_size))
                                    .w_full()
                                    .child(render_canvas_area(
                                        canvas_offset,
                                        zoom,
                                        items,
                                        items_for_render,
                                        selected_item_id,
                                        fps,
                                        frame_count,
                                        item_count,
                                        selected_item_name,
                                    )),
                            )
                            .child(render_splitter(SplitDirection::Horizontal))
                            .child(
                                div()
                                    .flex_shrink_0()
                                    .h(Fraction(preview_size))
                                    .w_full()
                                    .child(render_preview_panel(
                                        file_name,
                                        page_image,
                                        current_page,
                                        page_count,
                                    )),
                            )
                    }
                }
            }
            None => {
                // No preview - just show canvas
                base.child(render_canvas_area(
                    canvas_offset,
                    zoom,
                    items,
                    items_for_render,
                    selected_item_id,
                    fps,
                    frame_count,
                    item_count,
                    selected_item_name,
                ))
            }
        }
    }
}
