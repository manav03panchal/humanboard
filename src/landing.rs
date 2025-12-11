use crate::board_index::{BoardIndex, BoardMetadata};
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::Sizable;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::input::{Input, InputState};
use gpui_component::{ActiveTheme as _, Icon, IconName, h_flex, v_flex};

/// Render the landing page header bar
pub fn render_landing_header(cx: &mut Context<crate::app::Humanboard>) -> Div {
    let bg = cx.theme().title_bar;
    let border = cx.theme().border;
    let fg = cx.theme().foreground;
    let muted_fg = cx.theme().muted_foreground;

    h_flex()
        .w_full()
        .h(px(64.0))
        .bg(bg)
        .border_b_1()
        .border_color(border)
        .items_center()
        .justify_between()
        .px_6()
        .child(
            h_flex()
                .gap_3()
                .child(
                    div()
                        .text_xl()
                        .font_weight(FontWeight::BOLD)
                        .text_color(fg)
                        .child("Humanboard"),
                )
                .child(div().text_sm().text_color(muted_fg).child("Your boards")),
        )
        .child(
            Button::new("new-board")
                .primary()
                .icon(Icon::new(IconName::Plus))
                .label("New Board")
                .on_click(cx.listener(|this, _, _, cx| {
                    this.create_new_board(cx);
                })),
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

    let bg = cx.theme().popover;
    let border = cx.theme().border;
    let list_hover = cx.theme().list_hover;
    let muted = cx.theme().muted;
    let fg = cx.theme().foreground;
    let muted_fg = cx.theme().muted_foreground;

    v_flex()
        .w(px(240.0))
        .bg(bg)
        .border_1()
        .border_color(border)
        .rounded(px(12.0))
        .overflow_hidden()
        .hover(|s| s.border_color(list_hover).bg(list_hover))
        .when(!is_editing, |d| d.cursor_pointer())
        .child(
            // Thumbnail area (clickable to open board)
            div()
                .h(px(140.0))
                .w_full()
                .bg(muted)
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
                .child(div().text_2xl().text_color(muted_fg).child("▦")),
        )
        .child(
            // Info area
            v_flex()
                .p_3()
                .gap_2()
                .child(
                    // Board name (editable)
                    if is_editing {
                        if let Some(input_state) = edit_input {
                            v_flex()
                                .gap_2()
                                .child(Input::new(input_state).small().w_full())
                                .child(
                                    h_flex()
                                        .gap_2()
                                        .child(
                                            Button::new("save-board")
                                                .primary()
                                                .small()
                                                .label("Save")
                                                .on_click(cx.listener(|this, _, _, cx| {
                                                    this.finish_editing_board(cx);
                                                })),
                                        )
                                        .child(
                                            Button::new("cancel-edit")
                                                .ghost()
                                                .small()
                                                .label("Cancel")
                                                .on_click(cx.listener(|this, _, _, cx| {
                                                    this.cancel_editing(cx);
                                                })),
                                        ),
                                )
                        } else {
                            div()
                        }
                    } else {
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(fg)
                            .overflow_hidden()
                            .child(metadata.name.clone())
                    },
                )
                .when(!is_editing, |d| {
                    d.child(
                        // Date and actions row
                        h_flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(muted_fg)
                                    .child(metadata.formatted_date()),
                            )
                            .child(
                                h_flex()
                                    .gap_1()
                                    .child(
                                        Button::new(SharedString::from(format!(
                                            "edit-{}",
                                            board_id_for_edit
                                        )))
                                        .ghost()
                                        .xsmall()
                                        .icon(Icon::new(IconName::Settings).size(px(12.0)))
                                        .label("Edit")
                                        .on_click(
                                            cx.listener(move |this, _, window, cx| {
                                                this.start_editing_board(
                                                    board_id_for_edit.clone(),
                                                    window,
                                                    cx,
                                                );
                                            }),
                                        ),
                                    )
                                    .child(
                                        Button::new(SharedString::from(format!(
                                            "delete-{}",
                                            board_id_for_delete
                                        )))
                                        .danger()
                                        .xsmall()
                                        .icon(Icon::new(IconName::Delete).size(px(12.0)))
                                        .label("Delete")
                                        .on_click(
                                            cx.listener(move |this, _, _, cx| {
                                                this.confirm_delete_board(
                                                    board_id_for_delete.clone(),
                                                    cx,
                                                );
                                            }),
                                        ),
                                    ),
                            ),
                    )
                }),
        )
}

/// Render the empty state when no boards exist
pub fn render_empty_state(cx: &mut Context<crate::app::Humanboard>) -> Div {
    let fg = cx.theme().foreground;
    let muted_fg = cx.theme().muted_foreground;

    v_flex()
        .flex_1()
        .items_center()
        .justify_center()
        .gap_6()
        .child(div().text_2xl().text_color(muted_fg).child("▦"))
        .child(
            v_flex()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .text_xl()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(fg)
                        .child("No boards yet"),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(muted_fg)
                        .child("Create your first board to get started"),
                ),
        )
        .child(
            Button::new("create-first-board")
                .primary()
                .icon(Icon::new(IconName::Plus))
                .label("Create Board")
                .on_click(cx.listener(|this, _, _, cx| {
                    this.create_new_board(cx);
                })),
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

    let popover = cx.theme().popover;
    let border = cx.theme().border;
    let fg = cx.theme().foreground;
    let muted_fg = cx.theme().muted_foreground;
    let danger = cx.theme().danger;

    // Modal backdrop
    div()
        .absolute()
        .inset_0()
        .bg(hsla(0.0, 0.0, 0.0, 0.7))
        .flex()
        .items_center()
        .justify_center()
        .child(
            v_flex()
                .id("delete-dialog")
                .w(px(400.0))
                .bg(popover)
                .border_1()
                .border_color(border)
                .rounded(px(12.0))
                .p_6()
                .gap_4()
                .child(
                    h_flex()
                        .gap_3()
                        .child(
                            Icon::new(IconName::TriangleAlert)
                                .size(px(24.0))
                                .text_color(danger),
                        )
                        .child(
                            div()
                                .text_lg()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(fg)
                                .child("Delete Board?"),
                        ),
                )
                .child(div().text_sm().text_color(muted_fg).child(format!(
                    "Are you sure you want to delete \"{}\"? This action cannot be undone.",
                    board_name
                )))
                .child(
                    h_flex()
                        .justify_end()
                        .gap_3()
                        .mt_2()
                        .child(
                            Button::new("cancel-delete")
                                .ghost()
                                .label("Cancel")
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.cancel_delete(cx);
                                })),
                        )
                        .child(
                            Button::new("confirm-delete")
                                .danger()
                                .label("Delete")
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.delete_board(board_id_confirm.clone(), cx);
                                })),
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
    let bg = cx.theme().background;

    let base = div()
        .size_full()
        .bg(bg)
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
