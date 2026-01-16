//! Create board modal component.

use crate::actions::{ModalFocusNext, ModalFocusPrev};
use crate::app::Humanboard;
use crate::focus::FocusContext;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::input::{Escape, Input};
use gpui_component::{h_flex, v_flex, ActiveTheme as _, Icon, IconName};

/// Render the create board modal with name input and storage location picker
pub fn render_create_board_modal(
    input: &Entity<gpui_component::input::InputState>,
    current_location: &crate::app::StorageLocation,
    modal_focus: &FocusHandle,
    opacity: f32,
    cx: &mut Context<Humanboard>,
) -> impl IntoElement {
    let bg = cx.theme().background;
    let border = cx.theme().border;
    let fg = cx.theme().foreground;
    let muted_fg = cx.theme().muted_foreground;
    let primary = cx.theme().primary;
    let list_hover = cx.theme().list_hover;
    let list_active = cx.theme().list_active;

    let is_default_selected = *current_location == crate::app::StorageLocation::Default;
    let is_icloud_selected = *current_location == crate::app::StorageLocation::ICloud;
    let is_icloud_available = crate::app::StorageLocation::ICloud.is_available();

    deferred(
        div()
            .id("create-board-backdrop")
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .bg(hsla(0.0, 0.0, 0.0, 0.6 * opacity))
            .flex()
            .items_center()
            .justify_center()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.create_board_backdrop_clicked = true;
                    cx.notify();
                }),
            )
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| {
                    if this.create_board_backdrop_clicked {
                        this.close_create_board_modal(window, cx);
                    }
                    this.create_board_backdrop_clicked = false;
                }),
            )
            .on_action(cx.listener(|this, _: &Escape, window, cx| {
                this.close_create_board_modal(window, cx);
            }))
            .child(
                v_flex()
                    .id("create-board-modal")
                    .track_focus(modal_focus)
                    .key_context(FocusContext::KEY_MODAL)
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, _| {
                            this.create_board_backdrop_clicked = false;
                        }),
                    )
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(|this, _, _, _| {
                            this.create_board_backdrop_clicked = false;
                        }),
                    )
                    .w(px(420.0))
                    .bg(bg.opacity(opacity))
                    .border_1()
                    .border_color(border.opacity(opacity))
                    .rounded(px(12.0))
                    .overflow_hidden()
                    .shadow_lg()
                    .on_action(cx.listener(|this, _: &ModalFocusNext, window, cx| {
                        this.modal_focus_next(window, cx);
                    }))
                    .on_action(cx.listener(|this, _: &ModalFocusPrev, window, cx| {
                        this.modal_focus_prev(window, cx);
                    }))
                    // Header
                    .child(
                        h_flex()
                            .w_full()
                            .px(px(20.0))
                            .py(px(16.0))
                            .border_b_1()
                            .border_color(border)
                            .justify_between()
                            .child(
                                div()
                                    .text_size(px(16.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(fg)
                                    .child("Create New Board"),
                            )
                            .child(
                                div()
                                    .id("close-modal")
                                    .cursor_pointer()
                                    .p(px(4.0))
                                    .rounded(px(4.0))
                                    .hover(|s| s.bg(list_hover))
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.close_create_board_modal(window, cx);
                                    }))
                                    .child(
                                        Icon::new(IconName::Close)
                                            .size(px(16.0))
                                            .text_color(muted_fg),
                                    ),
                            ),
                    )
                    // Content
                    .child(
                        v_flex()
                            .w_full()
                            .p(px(20.0))
                            .gap(px(16.0))
                            // Board name input
                            .child(
                                v_flex()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(fg)
                                            .child("Board Name"),
                                    )
                                    .child(Input::new(input).w_full().cleanable(true)),
                            )
                            // Storage location picker
                            .child(
                                v_flex()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(fg)
                                            .child("Storage Location"),
                                    )
                                    .child(
                                        v_flex()
                                            .gap(px(4.0))
                                            // Default location option
                                            .child(
                                                h_flex()
                                                    .id("loc-local")
                                                    .w_full()
                                                    .px(px(12.0))
                                                    .py(px(10.0))
                                                    .rounded(px(6.0))
                                                    .bg(if is_default_selected { list_active } else { gpui::transparent_black() })
                                                    .cursor_pointer()
                                                    .hover(|s| s.bg(list_hover))
                                                    .on_click(cx.listener(|this, _, _, cx| {
                                                        this.set_create_board_location(
                                                            crate::app::StorageLocation::Default,
                                                            cx,
                                                        );
                                                    }))
                                                    .gap(px(12.0))
                                                    .child(
                                                        Icon::new(IconName::Folder)
                                                            .size(px(20.0))
                                                            .text_color(if is_default_selected { primary } else { muted_fg }),
                                                    )
                                                    .child(
                                                        v_flex()
                                                            .flex_1()
                                                            .gap(px(2.0))
                                                            .child(
                                                                div()
                                                                    .text_size(px(13.0))
                                                                    .font_weight(if is_default_selected { FontWeight::MEDIUM } else { FontWeight::NORMAL })
                                                                    .text_color(fg)
                                                                    .child("Local Storage"),
                                                            )
                                                            .child(
                                                                div()
                                                                    .text_size(px(11.0))
                                                                    .text_color(muted_fg)
                                                                    .child("~/Library/Application Support/humanboard"),
                                                            ),
                                                    )
                                                    .when(is_default_selected, |d| {
                                                        d.child(
                                                            Icon::new(IconName::Check)
                                                                .size(px(16.0))
                                                                .text_color(primary),
                                                        )
                                                    }),
                                            )
                                            // iCloud option
                                            .child(
                                                h_flex()
                                                    .id("loc-icloud")
                                                    .w_full()
                                                    .px(px(12.0))
                                                    .py(px(10.0))
                                                    .rounded(px(6.0))
                                                    .bg(if is_icloud_selected { list_active } else { gpui::transparent_black() })
                                                    .opacity(if is_icloud_available { 1.0 } else { 0.5 })
                                                    .when(is_icloud_available, |d| {
                                                        d.cursor_pointer()
                                                            .hover(|s| s.bg(list_hover))
                                                            .on_click(cx.listener(|this, _, _, cx| {
                                                                this.set_create_board_location(
                                                                    crate::app::StorageLocation::ICloud,
                                                                    cx,
                                                                );
                                                            }))
                                                    })
                                                    .when(!is_icloud_available, |d| {
                                                        d.cursor(CursorStyle::OperationNotAllowed)
                                                    })
                                                    .gap(px(12.0))
                                                    .child(
                                                        Icon::new(IconName::Globe)
                                                            .size(px(20.0))
                                                            .text_color(if is_icloud_selected { primary } else { muted_fg }),
                                                    )
                                                    .child(
                                                        v_flex()
                                                            .flex_1()
                                                            .gap(px(2.0))
                                                            .child(
                                                                div()
                                                                    .text_size(px(13.0))
                                                                    .font_weight(if is_icloud_selected { FontWeight::MEDIUM } else { FontWeight::NORMAL })
                                                                    .text_color(fg)
                                                                    .child("iCloud Drive"),
                                                            )
                                                            .child(
                                                                div()
                                                                    .text_size(px(11.0))
                                                                    .text_color(muted_fg)
                                                                    .child(if is_icloud_available { "Sync across devices" } else { "iCloud Drive not available" }),
                                                            ),
                                                    )
                                                    .when(is_icloud_selected, |d| {
                                                        d.child(
                                                            Icon::new(IconName::Check)
                                                                .size(px(16.0))
                                                                .text_color(primary),
                                                        )
                                                    }),
                                            ),
                                    ),
                            ),
                    )
                    // Footer with buttons
                    .child(
                        h_flex()
                            .w_full()
                            .px(px(20.0))
                            .py(px(16.0))
                            .border_t_1()
                            .border_color(border)
                            .justify_end()
                            .gap(px(12.0))
                            .child(
                                Button::new("cancel")
                                    .label("Cancel")
                                    .ghost()
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.close_create_board_modal(window, cx);
                                    })),
                            )
                            .child(
                                Button::new("create")
                                    .label("Create Board")
                                    .primary()
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.confirm_create_board(window, cx);
                                    })),
                            ),
                    ),
            ),
    )
    .with_priority(1600)
}
