use crate::actions::{
    ClosePreview, CloseTab, DeleteSelected, GoHome, NewBoard, NextPage, NextTab, OpenFile, Paste,
    PdfZoomIn, PdfZoomOut, PdfZoomReset, PrevPage, PrevTab, Redo, ShowShortcuts, ToggleSplit, Undo,
    ZoomIn, ZoomOut, ZoomReset,
};
use crate::app::{AppView, Humanboard, PreviewTab, SplitDirection};
use crate::landing::render_landing_page;
use crate::types::{CanvasItem, ItemContent};
use crate::youtube_webview::YouTubeWebView;
use gpui::DefiniteLength::Fraction;
use gpui::prelude::FluentBuilder;
use gpui::*;
use std::collections::HashMap;

const UI_FONT: &str = "Iosevka Nerd Font";

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
            ItemContent::YouTube(_) => hsla(0.0, 0.8, 0.4, 0.9), // Red for YouTube
            ItemContent::Markdown { .. } => hsla(0.55, 0.6, 0.4, 0.9), // Purple for Markdown
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
    youtube_webviews: &HashMap<u64, YouTubeWebView>,
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
                        })
                        .when(matches!(&item.content, ItemContent::YouTube(_)), |d| {
                            let padding = 8.0 * zoom;

                            // Simple border with padding around video
                            d.p(px(padding))
                                .bg(rgb(0x222222))
                                .rounded(px(4.0 * zoom))
                                .child(
                                    div().size_full().overflow_hidden().when_some(
                                        youtube_webviews.get(&item.id),
                                        |d, yt_webview| d.child(yt_webview.webview()),
                                    ),
                                )
                        })
                        .when(matches!(&item.content, ItemContent::Markdown { .. }), |d| {
                            if let ItemContent::Markdown { title, .. } = &item.content {
                                // Collapsed markdown card
                                d.bg(rgb(0x1e1e2e))
                                    .rounded(px(6.0 * zoom))
                                    .border(px(1.0 * zoom))
                                    .border_color(rgb(0x444466))
                                    .flex()
                                    .items_center()
                                    .gap(px(8.0 * zoom))
                                    .px(px(12.0 * zoom))
                                    .cursor(CursorStyle::PointingHand)
                                    .child(div().text_base().text_color(rgb(0x8888ff)).child("📝"))
                                    .child(
                                        div()
                                            .flex_1()
                                            .text_sm()
                                            .text_color(rgb(0xcccccc))
                                            .overflow_hidden()
                                            .child(title.clone()),
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
    board_name: Option<String>,
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

fn render_shortcut_row(key: &str, description: &str) -> Div {
    div()
        .h(px(28.0))
        .flex()
        .items_center()
        .justify_between()
        .child(
            div()
                .text_sm()
                .text_color(rgb(0xcccccc))
                .child(description.to_string()),
        )
        .child(
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
                .child(key.to_string()),
        )
}

fn render_shortcut_section(title: &str, shortcuts: Vec<(&str, &str)>) -> Div {
    let mut section = div().flex().flex_col().gap_1().child(
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
                div()
                    .w(px(420.0))
                    .bg(rgb(0x141414))
                    .border_1()
                    .border_color(rgb(0x2a2a2a))
                    .rounded(px(16.0))
                    .overflow_hidden()
                    .on_mouse_down(MouseButton::Left, |_, _, _| {})
                    // Header
                    .child(
                        div()
                            .px_5()
                            .py_4()
                            .border_b_1()
                            .border_color(rgb(0x2a2a2a))
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0xffffff))
                                    .child("Keyboard Shortcuts"),
                            )
                            .child(
                                div()
                                    .px(px(8.0))
                                    .py(px(4.0))
                                    .bg(rgb(0x2a2a2a))
                                    .rounded(px(6.0))
                                    .text_xs()
                                    .text_color(rgb(0x666666))
                                    .child("Cmd+/"),
                            ),
                    )
                    // Content
                    .child(
                        div()
                            .p_5()
                            .flex()
                            .flex_col()
                            .gap_5()
                            .child(render_shortcut_section(
                                "General",
                                vec![
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
            let filename = tab.title();
            let is_markdown = matches!(tab, PreviewTab::Markdown { .. });

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
                // Icon based on type
                .child(
                    div()
                        .text_xs()
                        .text_color(if is_markdown {
                            rgb(0x8888ff)
                        } else {
                            rgb(0xff6666)
                        })
                        .child(if is_markdown { "📝" } else { "📄" }),
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

pub fn render_tab_content(tab: &PreviewTab, is_active: bool, cx: &mut Context<Humanboard>) -> Div {
    let base = div()
        .absolute()
        .when(is_active, |d| d.size_full())
        .when(!is_active, |d| d.size_0());

    match tab {
        PreviewTab::Pdf { webview, .. } => {
            base.when_some(webview.as_ref().map(|wv| wv.webview()), |d, wv| d.child(wv))
        }
        PreviewTab::Markdown {
            content,
            edited_content,
            is_editing,
            path,
            ..
        } => {
            let content_clone = if *is_editing {
                edited_content.clone()
            } else {
                content.clone()
            };
            let is_editing = *is_editing;
            let path_clone = path.clone();
            let has_changes = content != edited_content;

            base.bg(rgb(0x1a1a1a))
                .flex()
                .flex_col()
                .child(
                    // Toolbar
                    div()
                        .h(px(36.0))
                        .w_full()
                        .bg(rgb(0x0d0d0d))
                        .border_b_1()
                        .border_color(rgb(0x333333))
                        .flex()
                        .items_center()
                        .justify_between()
                        .px_3()
                        .child(
                            div().flex().gap_2().child(
                                div()
                                    .px_3()
                                    .py_1()
                                    .rounded(px(4.0))
                                    .bg(if is_editing {
                                        rgb(0x333333)
                                    } else {
                                        rgb(0x4444aa)
                                    })
                                    .text_xs()
                                    .text_color(rgb(0xffffff))
                                    .cursor(CursorStyle::PointingHand)
                                    .hover(|s| s.bg(rgb(0x5555bb)))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _, _, cx| {
                                            this.toggle_markdown_edit(cx);
                                        }),
                                    )
                                    .child(if is_editing { "Preview" } else { "Edit" }),
                            ),
                        )
                        .child(div().flex().gap_2().when(has_changes, |d| {
                            let path_for_save = path_clone.clone();
                            d.child(
                                div()
                                    .px_3()
                                    .py_1()
                                    .rounded(px(4.0))
                                    .bg(rgb(0x44aa44))
                                    .text_xs()
                                    .text_color(rgb(0xffffff))
                                    .cursor(CursorStyle::PointingHand)
                                    .hover(|s| s.bg(rgb(0x55bb55)))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _, _, cx| {
                                            this.save_markdown(path_for_save.clone(), cx);
                                        }),
                                    )
                                    .child("Save"),
                            )
                        })),
                )
                .child(
                    // Content area
                    div().flex_1().overflow_y_hidden().p_4().child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xcccccc))
                            .whitespace_nowrap()
                            .child(content_clone),
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
            .w(px(16.0))
            .h_full()
            .bg(rgb(0x000000))
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
                    .w(px(1.0))
                    .h(px(60.0))
                    .bg(rgb(0x333333))
                    .rounded(px(1.0)),
            ),
        SplitDirection::Horizontal => div()
            .h(px(24.0))
            .w_full()
            .bg(rgb(0x000000))
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
    youtube_webviews: &HashMap<u64, YouTubeWebView>,
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
            youtube_webviews,
        ))
}

// Render implementation for Humanboard
impl Render for Humanboard {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.update_fps();

        // Route based on current view
        let content = match &self.view {
            AppView::Landing => self.render_landing_view(cx),
            AppView::Board(_) => self.render_board_view(window, cx),
        };

        // Wrap everything in a container with the shortcuts overlay on top
        div()
            .size_full()
            .font_family(UI_FONT)
            .relative()
            .child(content)
            .when(self.show_shortcuts, |d| {
                d.child(render_shortcuts_overlay(cx))
            })
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

        // Ensure PDF WebView is created if preview is active
        if self.preview.is_some() {
            self.ensure_pdf_webview(window, cx);
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
        let items_for_render = items.clone();
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
                                .child(div().flex_1().relative().overflow_hidden().children(
                                    tabs.iter().enumerate().map(|(index, tab)| {
                                        let is_active = index == active_tab;
                                        match tab {
                                            PreviewTab::Pdf { webview, .. } => div()
                                                .absolute()
                                                .when(is_active, |d| d.size_full())
                                                .when(!is_active, |d| d.size_0())
                                                .when_some(
                                                    webview.as_ref().map(|wv| wv.webview()),
                                                    |d, wv| d.child(wv),
                                                ),
                                            PreviewTab::Markdown { content, .. } => div()
                                                .absolute()
                                                .when(is_active, |d| d.size_full())
                                                .when(!is_active, |d| d.size_0())
                                                .bg(rgb(0x1a1a1a))
                                                .p_4()
                                                .overflow_hidden()
                                                .child(
                                                    div()
                                                        .text_sm()
                                                        .text_color(rgb(0xcccccc))
                                                        .child(content.clone()),
                                                ),
                                        }
                                    }),
                                )),
                        ),
                    SplitDirection::Horizontal => base
                        .flex()
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
                                .child(div().flex_1().relative().overflow_hidden().children(
                                    tabs.iter().enumerate().map(|(index, tab)| {
                                        let is_active = index == active_tab;
                                        match tab {
                                            PreviewTab::Pdf { webview, .. } => div()
                                                .absolute()
                                                .when(is_active, |d| d.size_full())
                                                .when(!is_active, |d| d.size_0())
                                                .when_some(
                                                    webview.as_ref().map(|wv| wv.webview()),
                                                    |d, wv| d.child(wv),
                                                ),
                                            PreviewTab::Markdown { content, .. } => div()
                                                .absolute()
                                                .when(is_active, |d| d.size_full())
                                                .when(!is_active, |d| d.size_0())
                                                .bg(rgb(0x1a1a1a))
                                                .p_4()
                                                .overflow_hidden()
                                                .child(
                                                    div()
                                                        .text_sm()
                                                        .text_color(rgb(0xcccccc))
                                                        .child(content.clone()),
                                                ),
                                        }
                                    }),
                                )),
                        ),
                }
            }
            None => base.pb(px(28.0)).child(render_canvas_area(
                canvas_offset,
                zoom,
                items.clone(),
                items_for_render.clone(),
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
            board_name,
        ));

        content
    }
}
