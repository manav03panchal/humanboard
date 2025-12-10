use crate::actions::{
    ClosePreview, CloseTab, DeleteSelected, NextPage, NextTab, PdfZoomIn, PdfZoomOut, PdfZoomReset,
    PrevPage, PrevTab, Redo, ToggleSplit, Undo, ZoomIn, ZoomOut, ZoomReset,
};
use crate::app::{Humanboard, PdfTab, SplitDirection};
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
            ItemContent::Pdf { .. } => hsla(0.0, 0.7, 0.5, 0.9),
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

pub fn render_footer_bar(
    _fps: f32,
    _frame_count: u64,
    item_count: usize,
    zoom: f32,
    canvas_offset: Point<Pixels>,
    selected_item_name: Option<String>,
) -> Div {
    div()
        .absolute()
        .bottom_0()
        .left_0()
        .right_0()
        .h(px(28.0))
        .bg(hsla(0.0, 0.0, 0.0, 0.95))
        .border_t_1()
        .border_color(hsla(0.0, 0.0, 0.3, 1.0))
        .flex()
        .items_center()
        .justify_between()
        .px_4()
        .gap_6()
        .text_xs()
        .text_color(rgb(0xaaaaaa))
        .child(
            div()
                .flex()
                .gap_6()
                .child(
                    div()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xffffff))
                        .child("Humanboard"),
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
    render_footer_bar(fps, frame_count, item_count, zoom, canvas_offset, None)
}

pub fn render_selected_item_label(_name: String) -> Div {
    div().size_0()
}

pub fn render_tab_bar(tabs: &Vec<PdfTab>, active_tab: usize, cx: &mut Context<Humanboard>) -> Div {
    div()
        .h(px(36.0))
        .w_full()
        .bg(rgb(0x000000))
        .border_b_1()
        .border_color(rgb(0x333333))
        .flex()
        .items_center()
        .overflow_x_hidden()
        .children(tabs.iter().enumerate().map(|(index, tab)| {
            let is_active = index == active_tab;
            let filename = tab
                .path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string();

            let display_name = if filename.len() > 20 {
                format!("{}...", &filename[..17])
            } else {
                filename
            };

            let tab_index = index;

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
                        this.switch_tab(tab_index, cx);
                    }),
                )
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
                                this.close_tab(tab_index, cx);
                            }),
                        )
                        .child("×"),
                )
        }))
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

pub fn render_splitter(direction: SplitDirection) -> Div {
    match direction {
        SplitDirection::Vertical => div()
            .w(px(16.0))
            .h_full()
            .bg(rgb(0x000000))
            .hover(|s| s.bg(rgb(0x1a1a1a)))
            .cursor(CursorStyle::ResizeLeftRight)
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .w(px(1.0))
                    .h(px(60.0))
                    .bg(rgb(0x333333))
                    .rounded(px(1.0)),
            ),
        SplitDirection::Horizontal => div()
            .h(px(16.0))
            .w_full()
            .bg(rgb(0x000000))
            .hover(|s| s.bg(rgb(0x1a1a1a)))
            .cursor(CursorStyle::ResizeUpDown)
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .h(px(1.0))
                    .w(px(60.0))
                    .bg(rgb(0x333333))
                    .rounded(px(1.0)),
            ),
    }
}

// Render the canvas area (moodboard content)
fn render_canvas_area(
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
        .child(render_canvas(canvas_offset, zoom, items))
        .children(render_items(
            items_for_render,
            canvas_offset,
            zoom,
            selected_item_id,
        ))
}

// Render implementation for Humanboard
impl Render for Humanboard {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.update_fps();

        // Ensure PDF WebView is created if preview is active
        if self.preview.is_some() {
            self.ensure_pdf_webview(window, cx);
        }

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

        // Extract preview info including WebView entities for rendering
        let preview_info = self
            .preview
            .as_ref()
            .map(|p| (p.split, p.size, &p.tabs, p.active_tab));

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
            .on_drop(cx.listener(|this, paths: &ExternalPaths, window, cx| {
                if let Some(first_path) = paths.paths().first() {
                    // Use tracked mouse position, or fall back to canvas center
                    let drop_pos = if let Some(pos) = this.last_drop_pos {
                        pos
                    } else {
                        // Fallback: drop at canvas center
                        let bounds = window.bounds();
                        let window_size = bounds.size;

                        let (canvas_center_x, canvas_center_y) = if let Some(ref preview) = this.preview
                        {
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

                    this.board
                        .handle_file_drop(drop_pos, vec![first_path.clone()]);
                    cx.notify();
                }
            }));

        match preview_info {
            Some((split, size, tabs, active_tab)) => {
                let canvas_size = 1.0 - size;
                let preview_size = size;

                match split {
                    SplitDirection::Vertical => {
                        // Horizontal layout: canvas | splitter | PDF WebView
                        base.flex()
                            .flex_row()
                            .pb(px(28.0))
                            .child(
                                div()
                                    .flex_shrink_0()
                                    .w(Fraction(canvas_size))
                                    .h_full()
                                    .child(render_canvas_area(
                                        canvas_offset,
                                        zoom,
                                        items.clone(),
                                        items_for_render.clone(),
                                        selected_item_id,
                                    )),
                            )
                            .child(render_splitter(SplitDirection::Vertical))
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
                                    .child(div().flex_1().relative().overflow_hidden().children(
                                        tabs.iter().enumerate().map(|(index, tab)| {
                                            let is_active = index == active_tab;
                                            div()
                                                .absolute()
                                                .when(is_active, |d| d.size_full())
                                                .when(!is_active, |d| d.size_0())
                                                .when_some(
                                                    tab.webview.as_ref().map(|wv| wv.webview()),
                                                    |d, wv| d.child(wv),
                                                )
                                        }),
                                    )),
                            )
                    }
                    SplitDirection::Horizontal => {
                        // Vertical layout: canvas / splitter / PDF WebView
                        base.flex()
                            .flex_col()
                            .pb(px(28.0))
                            .child(
                                div()
                                    .flex_shrink_0()
                                    .h(Fraction(canvas_size))
                                    .w_full()
                                    .child(render_canvas_area(
                                        canvas_offset,
                                        zoom,
                                        items.clone(),
                                        items_for_render.clone(),
                                        selected_item_id,
                                    )),
                            )
                            .child(render_splitter(SplitDirection::Horizontal))
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
                                    .child(div().flex_1().relative().overflow_hidden().children(
                                        tabs.iter().enumerate().map(|(index, tab)| {
                                            let is_active = index == active_tab;
                                            div()
                                                .absolute()
                                                .when(is_active, |d| d.size_full())
                                                .when(!is_active, |d| d.size_0())
                                                .when_some(
                                                    tab.webview.as_ref().map(|wv| wv.webview()),
                                                    |d, wv| d.child(wv),
                                                )
                                        }),
                                    )),
                            )
                    }
                }
            }
            None => {
                // No preview - just show canvas
                base.pb(px(28.0)).child(render_canvas_area(
                    canvas_offset,
                    zoom,
                    items.clone(),
                    items_for_render.clone(),
                    selected_item_id,
                ))
            }
        }
        .child(render_footer_bar(
            fps,
            frame_count,
            item_count,
            zoom,
            canvas_offset,
            selected_item_name,
        ))
    }
}
