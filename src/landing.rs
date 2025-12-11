use crate::board_index::{BoardIndex, BoardMetadata};
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::Sizable;
use gpui_component::input::{Input, InputState};

/// Render the landing page header bar
pub fn render_landing_header(cx: &mut Context<crate::app::Humanboard>) -> Div {
    div()
        .w_full()
        .h(px(64.0))
        .bg(rgb(0x0a0a0a))
        .border_b_1()
        .border_color(rgb(0x222222))
        .flex()
        .items_center()
        .justify_between()
        .px_6()
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .child(
                    div()
                        .text_xl()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xffffff))
                        .child("Humanboard"),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(0x666666))
                        .child("Your boards"),
                ),
        )
        .child(
            div()
                .px_4()
                .py_2()
                .bg(rgb(0x2563eb))
                .hover(|s| s.bg(rgb(0x3b82f6)))
                .rounded(px(6.0))
                .cursor_pointer()
                .text_sm()
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(0xffffff))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, _, cx| {
                        this.create_new_board(cx);
                    }),
                )
                .child("+ New Board"),
        )
}

/// Render a single board card
pub fn render_board_card(
    metadata: &BoardMetadata,
    is_editing: bool,
    edit_input: Option<&Entity<InputState>>,
    cx: &mut Context<crate::app::Humanboard>,
) -> Div {
    let board_id_for_click = metadata.id.clone();
    let board_id_for_edit = metadata.id.clone();
    let board_id_for_delete = metadata.id.clone();

    div()
        .w(px(240.0))
        .bg(rgb(0x141414))
        .border_1()
        .border_color(rgb(0x2a2a2a))
        .rounded(px(12.0))
        .overflow_hidden()
        .hover(|s| s.border_color(rgb(0x3a3a3a)).bg(rgb(0x1a1a1a)))
        .when(!is_editing, |d| d.cursor_pointer())
        .child(
            // Thumbnail area (clickable to open board)
            div()
                .h(px(140.0))
                .w_full()
                .bg(rgb(0x1a1a1a))
                .flex()
                .items_center()
                .justify_center()
                .when(!is_editing, |d| {
                    d.on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            this.open_board(board_id_for_click.clone(), cx);
                        }),
                    )
                })
                .child(div().text_2xl().text_color(rgb(0x333333)).child("▦")),
        )
        .child(
            // Info area
            div()
                .p_3()
                .flex()
                .flex_col()
                .gap_2()
                .child(
                    // Board name (editable)
                    if is_editing {
                        if let Some(input_state) = edit_input {
                            div()
                                .flex()
                                .flex_col()
                                .gap_2()
                                .child(Input::new(input_state).small().w_full())
                                .child(
                                    div()
                                        .flex()
                                        .gap_2()
                                        .child(
                                            div()
                                                .px_2()
                                                .py_1()
                                                .bg(rgb(0x2563eb))
                                                .hover(|s| s.bg(rgb(0x3b82f6)))
                                                .rounded(px(4.0))
                                                .text_xs()
                                                .text_color(rgb(0xffffff))
                                                .cursor_pointer()
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(|this, _, _, cx| {
                                                        this.finish_editing_board(cx);
                                                    }),
                                                )
                                                .child("Save"),
                                        )
                                        .child(
                                            div()
                                                .px_2()
                                                .py_1()
                                                .bg(rgb(0x333333))
                                                .hover(|s| s.bg(rgb(0x444444)))
                                                .rounded(px(4.0))
                                                .text_xs()
                                                .text_color(rgb(0xffffff))
                                                .cursor_pointer()
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(|this, _, _, cx| {
                                                        this.cancel_editing(cx);
                                                    }),
                                                )
                                                .child("Cancel"),
                                        ),
                                )
                        } else {
                            div()
                        }
                    } else {
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(0xffffff))
                            .overflow_hidden()
                            .child(metadata.name.clone())
                    },
                )
                .when(!is_editing, |d| {
                    d.child(
                        // Date and actions row
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x666666))
                                    .child(metadata.formatted_date()),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap_1()
                                    .child(
                                        // Edit button
                                        div()
                                            .px_2()
                                            .py_1()
                                            .rounded(px(4.0))
                                            .text_xs()
                                            .text_color(rgb(0x888888))
                                            .hover(|s| {
                                                s.bg(rgb(0x2a2a2a)).text_color(rgb(0xffffff))
                                            })
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(move |this, _, window, cx| {
                                                    this.start_editing_board(
                                                        board_id_for_edit.clone(),
                                                        window,
                                                        cx,
                                                    );
                                                }),
                                            )
                                            .child("Edit"),
                                    )
                                    .child(
                                        // Delete button
                                        div()
                                            .px_2()
                                            .py_1()
                                            .rounded(px(4.0))
                                            .text_xs()
                                            .text_color(rgb(0x888888))
                                            .hover(|s| {
                                                s.bg(rgb(0x3a2a2a)).text_color(rgb(0xff6b6b))
                                            })
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(move |this, _, _, cx| {
                                                    this.confirm_delete_board(
                                                        board_id_for_delete.clone(),
                                                        cx,
                                                    );
                                                }),
                                            )
                                            .child("Delete"),
                                    ),
                            ),
                    )
                }),
        )
}

/// Render the empty state when no boards exist
pub fn render_empty_state(cx: &mut Context<crate::app::Humanboard>) -> Div {
    div()
        .flex_1()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap_6()
        .child(div().text_2xl().text_color(rgb(0x333333)).child("▦"))
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .text_xl()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(0xffffff))
                        .child("No boards yet"),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(0x666666))
                        .child("Create your first board to get started"),
                ),
        )
        .child(
            div()
                .mt_4()
                .px_6()
                .py_3()
                .bg(rgb(0x2563eb))
                .hover(|s| s.bg(rgb(0x3b82f6)))
                .rounded(px(8.0))
                .cursor_pointer()
                .text_base()
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(0xffffff))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, _, cx| {
                        this.create_new_board(cx);
                    }),
                )
                .child("+ Create Board"),
        )
}

/// Render the board grid
pub fn render_board_grid(
    boards: &[BoardMetadata],
    editing_board_id: Option<&str>,
    edit_input: Option<&Entity<InputState>>,
    cx: &mut Context<crate::app::Humanboard>,
) -> Div {
    div()
        .flex_1()
        .p_6()
        .overflow_hidden()
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap_4()
                .children(boards.iter().map(|meta| {
                    let is_editing = editing_board_id == Some(&meta.id);
                    let input = if is_editing { edit_input } else { None };
                    render_board_card(meta, is_editing, input, cx)
                })),
        )
}

/// Render delete confirmation dialog
pub fn render_delete_dialog(
    board_name: &str,
    board_id: &str,
    cx: &mut Context<crate::app::Humanboard>,
) -> Div {
    let board_id_confirm = board_id.to_string();

    div()
        .absolute()
        .inset_0()
        .bg(hsla(0.0, 0.0, 0.0, 0.7))
        .flex()
        .items_center()
        .justify_center()
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(|this, _, _, cx| {
                this.cancel_delete(cx);
            }),
        )
        .child(
            div()
                .w(px(400.0))
                .bg(rgb(0x1a1a1a))
                .border_1()
                .border_color(rgb(0x333333))
                .rounded(px(12.0))
                .p_6()
                .flex()
                .flex_col()
                .gap_4()
                .on_mouse_down(MouseButton::Left, |_, _, _| {
                    // Dialog area - clicking here doesn't close
                })
                .child(
                    div()
                        .text_lg()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0xffffff))
                        .child("Delete Board?"),
                )
                .child(div().text_sm().text_color(rgb(0xaaaaaa)).child(format!(
                    "Are you sure you want to delete \"{}\"? This action cannot be undone.",
                    board_name
                )))
                .child(
                    div()
                        .flex()
                        .justify_end()
                        .gap_3()
                        .mt_2()
                        .child(
                            div()
                                .px_4()
                                .py_2()
                                .bg(rgb(0x2a2a2a))
                                .hover(|s| s.bg(rgb(0x3a3a3a)))
                                .rounded(px(6.0))
                                .cursor_pointer()
                                .text_sm()
                                .text_color(rgb(0xffffff))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _, _, cx| {
                                        this.cancel_delete(cx);
                                    }),
                                )
                                .child("Cancel"),
                        )
                        .child(
                            div()
                                .px_4()
                                .py_2()
                                .bg(rgb(0xdc2626))
                                .hover(|s| s.bg(rgb(0xef4444)))
                                .rounded(px(6.0))
                                .cursor_pointer()
                                .text_sm()
                                .text_color(rgb(0xffffff))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _, _, cx| {
                                        this.delete_board(board_id_confirm.clone(), cx);
                                    }),
                                )
                                .child("Delete"),
                        ),
                ),
        )
}

/// Render the complete landing page
pub fn render_landing_page(
    board_index: &BoardIndex,
    editing_board_id: Option<&str>,
    edit_input: Option<&Entity<InputState>>,
    deleting_board: Option<(&str, &str)>, // (id, name)
    cx: &mut Context<crate::app::Humanboard>,
) -> Div {
    let base = div()
        .size_full()
        .bg(rgb(0x0a0a0a))
        .flex()
        .flex_col()
        .child(render_landing_header(cx));

    let base = if board_index.boards.is_empty() {
        base.child(render_empty_state(cx))
    } else {
        base.child(render_board_grid(
            &board_index.boards,
            editing_board_id,
            edit_input,
            cx,
        ))
    };

    // Add delete confirmation dialog if needed
    if let Some((id, name)) = deleting_board {
        base.child(render_delete_dialog(name, id, cx))
    } else {
        base
    }
}
