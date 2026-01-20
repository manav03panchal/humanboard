//! Landing page UI components.
//!
//! This module renders the home screen shown when no board is open,
//! displaying a grid of board cards and management controls.
//!
//! ## Components
//!
//! - **Header**: App title and "New Board" button
//! - **Board Grid**: Cards for each board with edit/delete actions
//! - **Trash Section**: Collapsible list of deleted boards (30-day retention)
//! - **Empty State**: Shown when no boards exist
//! - **Delete Dialog**: Confirmation modal for board deletion

use crate::board_index::{BoardIndex, BoardMetadata};
use crate::focus_ring::focus_ring_shadow;
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
    let primary = cx.theme().primary;

    h_flex()
        .w_full()
        .h(px(52.0))
        .bg(bg)
        .border_b_1()
        .border_color(border)
        .items_center()
        .justify_between()
        .pl(px(80.0))
        .pr_4()
        .child(
            h_flex()
                .gap_3()
                .items_center()
                .child(
                    Icon::new(IconName::LayoutDashboard)
                        .size(px(20.0))
                        .text_color(primary),
                )
                .child(
                    div()
                        .text_lg()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(fg)
                        .child("Humanboard"),
                )
                .child(
                    div()
                        .h(px(16.0))
                        .w(px(1.0))
                        .bg(border)
                        .mx_1(),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(muted_fg)
                        .child("Your Boards"),
                ),
        )
        .child(
            Button::new("new-board")
                .primary()
                .small()
                .icon(Icon::new(IconName::Plus))
                .label("New Board")
                .tooltip("Create a new board")
                .on_click(cx.listener(|this, _, window, cx| {
                    this.create_new_board(window, cx);
                })),
        )
}

/// Render a single board card
pub fn render_board_card(
    metadata: &BoardMetadata,
    is_editing: bool,
    edit_input: Option<&Entity<InputState>>,
    cx: &mut Context<crate::app::Humanboard>,
) -> Stateful<Div> {
    let board_id_for_click = metadata.id.clone();
    let board_id_for_edit = metadata.id.clone();
    let board_id_for_delete = metadata.id.clone();

    let bg = cx.theme().popover;
    let border = cx.theme().border;
    let list_hover = cx.theme().list_hover;
    let muted = cx.theme().muted;
    let fg = cx.theme().foreground;
    let muted_fg = cx.theme().muted_foreground;
    let primary = cx.theme().primary;

    v_flex()
        .id(ElementId::Name(format!("board-card-{}", metadata.id).into()))
        .w(px(240.0))
        .bg(bg)
        .border_1()
        .border_color(border)
        .rounded(px(12.0))
        .overflow_hidden()
        .hover(|s| s.border_color(list_hover).bg(list_hover))
        // Focus ring for keyboard navigation (WCAG compliance)
        .focus(|s| s.shadow(focus_ring_shadow(primary)))
        .when(!is_editing, |d| d.cursor_pointer())
        .child(
            // Thumbnail area (clickable to open board)
            div()
                .h(px(140.0))
                .w_full()
                .bg(muted)
                .rounded_t(px(12.0))
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
                .child(
                    Icon::new(IconName::LayoutDashboard)
                        .size(px(32.0))
                        .text_color(muted_fg),
                ),
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
                                                .tooltip("Save board name")
                                                .on_click(cx.listener(|this, _, _, cx| {
                                                    this.finish_editing_board(cx);
                                                })),
                                        )
                                        .child(
                                            Button::new("cancel-edit")
                                                .ghost()
                                                .small()
                                                .label("Cancel")
                                                .tooltip("Cancel editing")
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
                                        .tooltip("Edit board name")
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
                                        .tooltip("Move board to trash")
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

/// Render a single trashed board card (with restore/delete options)
pub fn render_trashed_board_card(
    metadata: &BoardMetadata,
    cx: &mut Context<crate::app::Humanboard>,
) -> Stateful<Div> {
    let board_id_for_restore = metadata.id.clone();
    let board_id_for_delete = metadata.id.clone();

    let bg = cx.theme().popover;
    let border = cx.theme().border;
    let muted = cx.theme().muted;
    let fg = cx.theme().foreground;
    let muted_fg = cx.theme().muted_foreground;
    let primary = cx.theme().primary;

    v_flex()
        .id(ElementId::Name(format!("trashed-card-{}", metadata.id).into()))
        .w(px(200.0))
        .bg(bg)
        .border_1()
        .border_color(border)
        .rounded(px(12.0))
        .overflow_hidden()
        .opacity(0.7)
        // Focus ring for keyboard navigation (WCAG compliance)
        .focus(|s| s.shadow(focus_ring_shadow(primary)))
        .cursor(CursorStyle::Arrow)
        .child(
            // Thumbnail area (greyed out)
            div()
                .h(px(100.0))
                .w_full()
                .bg(muted)
                .rounded_t(px(12.0))
                .flex()
                .items_center()
                .justify_center()
                .child(
                    Icon::new(IconName::Delete)
                        .size(px(32.0))
                        .text_color(muted_fg),
                ),
        )
        .child(
            // Info area
            v_flex()
                .p_3()
                .gap_2()
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(fg)
                        .overflow_hidden()
                        .text_ellipsis()
                        .child(metadata.name.clone()),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(muted_fg)
                        .child(format!(
                            "Deleted {}",
                            metadata.deleted_ago().unwrap_or_else(|| "recently".to_string())
                        )),
                )
                .child(
                    h_flex()
                        .gap_2()
                        .mt_1()
                        .child(
                            Button::new(SharedString::from(format!(
                                "restore-{}",
                                board_id_for_restore
                            )))
                            .ghost()
                            .xsmall()
                            .icon(Icon::new(IconName::ArrowLeft).size(px(12.0)))
                            .label("Restore")
                            .tooltip("Restore board from trash")
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.restore_board(&board_id_for_restore, cx);
                            })),
                        )
                        .child(
                            Button::new(SharedString::from(format!(
                                "perma-delete-{}",
                                board_id_for_delete
                            )))
                            .danger()
                            .xsmall()
                            .icon(Icon::new(IconName::Close).size(px(12.0)))
                            .label("Delete")
                            .tooltip("Permanently delete board")
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.permanently_delete_board(&board_id_for_delete, cx);
                            })),
                        ),
                ),
        )
}

/// Render the empty state when no boards exist
pub fn render_empty_state(cx: &mut Context<crate::app::Humanboard>) -> Div {
    let fg = cx.theme().foreground;
    let muted_fg = cx.theme().muted_foreground;
    let primary = cx.theme().primary;
    let border = cx.theme().border;

    v_flex()
        .flex_1()
        .items_center()
        .justify_center()
        .gap_8()
        // Hero icon with subtle background
        .child(
            div()
                .w(px(120.0))
                .h(px(120.0))
                .rounded(px(24.0))
                .border_1()
                .border_color(border)
                .flex()
                .items_center()
                .justify_center()
                .child(
                    Icon::new(IconName::LayoutDashboard)
                        .size(px(48.0))
                        .text_color(primary),
                ),
        )
        // Welcome text
        .child(
            v_flex()
                .items_center()
                .gap_3()
                .child(
                    div()
                        .text_2xl()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(fg)
                        .child("Welcome to Humanboard"),
                )
                .child(
                    div()
                        .text_base()
                        .text_color(muted_fg)
                        .text_center()
                        .max_w(px(400.0))
                        .child("Create your first board to start organizing your ideas, images, and documents in a visual workspace."),
                ),
        )
        // CTA button
        .child(
            Button::new("create-first-board")
                .primary()
                .icon(Icon::new(IconName::Plus))
                .label("Create Your First Board")
                .tooltip("Create your first board to get started")
                .on_click(cx.listener(|this, _, window, cx| {
                    this.create_new_board(window, cx);
                })),
        )
        // Keyboard shortcut hint
        .child(
            div()
                .text_xs()
                .text_color(muted_fg)
                .mt_4()
                .child("Press ⌘N to create a new board"),
        )
}

/// Render the board grid
pub fn render_board_grid(
    boards: Vec<&BoardMetadata>,
    editing_board_id: Option<&str>,
    edit_input: Option<&Entity<InputState>>,
    cx: &mut Context<crate::app::Humanboard>,
) -> Div {
    div()
        .flex()
        .flex_wrap()
        .gap_4()
        .children(boards.iter().map(|meta| {
            let is_editing = editing_board_id == Some(&meta.id);
            let input = if is_editing { edit_input } else { None };
            render_board_card(meta, is_editing, input, cx)
        }))
}

/// Render the Recently Deleted section
/// Render a small "Show Recently Deleted" toggle button
pub fn render_trash_toggle(
    trash_count: usize,
    is_expanded: bool,
    cx: &mut Context<crate::app::Humanboard>,
) -> Div {
    let muted_fg = cx.theme().muted_foreground;
    let border = cx.theme().border;

    div()
        .w_full()
        .mt_6()
        .pt_4()
        .border_t_1()
        .border_color(border)
        .child(
            h_flex()
                .items_center()
                .gap_2()
                .cursor_pointer()
                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                    this.toggle_trash(cx);
                }))
                .child(
                    Icon::new(if is_expanded { IconName::ChevronDown } else { IconName::ChevronRight })
                        .size(px(14.0))
                        .text_color(muted_fg),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(muted_fg)
                        .child(format!("Recently Deleted ({})", trash_count)),
                ),
        )
}

/// Render the expanded trash section with boards
pub fn render_trash_section(
    trashed_boards: Vec<&BoardMetadata>,
    cx: &mut Context<crate::app::Humanboard>,
) -> Div {
    let fg = cx.theme().foreground;
    let muted_fg = cx.theme().muted_foreground;
    let border = cx.theme().border;

    v_flex()
        .w_full()
        .mt_6()
        .pt_4()
        .border_t_1()
        .border_color(border)
        .gap_4()
        .child(
            h_flex()
                .items_center()
                .justify_between()
                .child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .cursor_pointer()
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                            this.toggle_trash(cx);
                        }))
                        .child(
                            Icon::new(IconName::ChevronDown)
                                .size(px(14.0))
                                .text_color(muted_fg),
                        )
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(fg)
                                .child("Recently Deleted"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(muted_fg)
                                .child(format!("({}) - auto-deleted after 30 days", trashed_boards.len())),
                        ),
                )
                .child(
                    Button::new("empty-trash")
                        .danger()
                        .xsmall()
                        .icon(Icon::new(IconName::Delete).size(px(12.0)))
                        .label("Empty Trash")
                        .tooltip("Permanently delete all trashed boards")
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.empty_trash(cx);
                        })),
                ),
        )
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap_4()
                .children(
                    trashed_boards
                        .iter()
                        .map(|meta| render_trashed_board_card(meta, cx)),
                ),
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
        .bg(gpui::black().opacity(0.7))
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
                                .tooltip("Cancel and keep board")
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.cancel_delete(cx);
                                })),
                        )
                        .child(
                            Button::new("confirm-delete")
                                .danger()
                                .label("Delete")
                                .tooltip("Confirm deletion")
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.delete_board(&board_id_confirm, cx);
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
    show_trash: bool,
    cx: &mut Context<crate::app::Humanboard>,
) -> Div {
    let bg = cx.theme().background;
    let active_boards = board_index.active_boards();
    let trashed_boards = board_index.trashed_boards();

    let base = div()
        .size_full()
        .bg(bg)
        .flex()
        .flex_col()
        .child(render_landing_header(cx));

    let has_active = !active_boards.is_empty();
    let has_trashed = !trashed_boards.is_empty();
    let trash_count = trashed_boards.len();

    let base = if !has_active && !has_trashed {
        base.child(render_empty_state(cx))
    } else {
        base.child(
            div()
                .id("landing-content")
                .flex_1()
                .flex()
                .flex_col()
                .p_6()
                .overflow_y_scroll()
                .when(has_active, |d| {
                    d.child(render_board_grid(
                        active_boards,
                        editing_board_id,
                        edit_input,
                        cx,
                    ))
                })
                .when(!has_active && has_trashed, |d| {
                    d.child(
                        div()
                            .text_sm()
                            .text_color(cx.theme().muted_foreground)
                            .child("No active boards"),
                    )
                })
                // Show collapsed toggle or expanded trash section
                .when(has_trashed && !show_trash, |d| {
                    d.child(render_trash_toggle(trash_count, false, cx))
                })
                .when(has_trashed && show_trash, |d| {
                    d.child(render_trash_section(trashed_boards, cx))
                }),
        )
    };

    // Add delete confirmation dialog if needed
    if let Some((id, name)) = deleting_board {
        base.child(render_delete_dialog(name, id, cx))
    } else {
        base
    }
}
