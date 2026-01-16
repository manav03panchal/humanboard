//! Home screen with countdown timer view.
//!
//! This module renders the home screen that displays a countdown timer
//! and quick access to boards and actions.

use crate::app::CountdownState;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::{ActiveTheme as _, h_flex, v_flex};

/// Render the home screen with countdown timer
pub fn render_home_screen(
    countdown: Option<&CountdownState>,
    cx: &mut Context<crate::app::Humanboard>,
) -> Div {
    let bg = cx.theme().background;
    let fg = cx.theme().foreground;
    let muted = cx.theme().muted_foreground;

    div()
        .size_full()
        .bg(bg)
        .flex()
        .items_center()
        .justify_center()
        .child(
            v_flex()
                .gap_8()
                .items_center()
                .child(render_countdown_display(countdown, fg, muted))
                .child(render_quick_actions(cx)),
        )
}

/// Render the large countdown display
fn render_countdown_display(
    countdown: Option<&CountdownState>,
    fg: Hsla,
    muted: Hsla,
) -> impl IntoElement {
    let (hours, minutes, seconds, label, finished) = if let Some(state) = countdown {
        if let Some((h, m, s)) = state.remaining() {
            (h, m, s, state.label.clone(), false)
        } else {
            (0, 0, 0, state.label.clone(), true)
        }
    } else {
        (0, 0, 0, "No countdown".to_string(), true)
    };

    v_flex()
        .gap_4()
        .items_center()
        .child(
            // Label
            div()
                .text_size(px(16.0))
                .text_color(muted)
                .child(label),
        )
        .child(
            // Time display
            h_flex()
                .gap_2()
                .items_baseline()
                .when(!finished, |d| {
                    d.child(render_time_unit(hours, "h", fg))
                        .child(render_separator(muted))
                        .child(render_time_unit(minutes, "m", fg))
                        .child(render_separator(muted))
                        .child(render_time_unit(seconds, "s", fg))
                })
                .when(finished, |d| {
                    d.child(
                        div()
                            .text_size(px(48.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(fg)
                            .child("Complete!"),
                    )
                }),
        )
}

/// Render a single time unit (hours, minutes, or seconds)
fn render_time_unit(value: u64, unit: &str, fg: Hsla) -> impl IntoElement {
    h_flex()
        .items_baseline()
        .child(
            div()
                .text_size(px(72.0))
                .font_weight(FontWeight::BOLD)
                .text_color(fg)
                .min_w(px(100.0))
                .text_right()
                .child(format!("{:02}", value)),
        )
        .child(
            div()
                .text_size(px(24.0))
                .text_color(fg.opacity(0.6))
                .child(unit.to_string()),
        )
}

/// Render the colon separator between time units
fn render_separator(muted: Hsla) -> impl IntoElement {
    div()
        .text_size(px(48.0))
        .text_color(muted)
        .px_2()
        .child(":")
}

/// Render quick action buttons
fn render_quick_actions(cx: &mut Context<crate::app::Humanboard>) -> impl IntoElement {
    h_flex()
        .gap_4()
        .child(
            Button::new("view-boards")
                .label("View Boards")
                .tooltip("Go to boards list")
                .primary()
                .on_click(cx.listener(|this, _, _, cx| {
                    this.view = crate::app::AppView::Landing;
                    cx.notify();
                })),
        )
        .child(
            Button::new("new-board")
                .label("New Board")
                .tooltip("Create a new board")
                .ghost()
                .on_click(cx.listener(|this, _, window, cx| {
                    this.create_new_board(window, cx);
                })),
        )
}
