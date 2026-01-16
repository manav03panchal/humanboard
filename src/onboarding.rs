//! Onboarding flow - Welcome screen for new users

use gpui::*;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::ActiveTheme as _;

use crate::app::Humanboard;

/// Render the onboarding welcome page
pub fn render_onboarding_page(cx: &mut Context<Humanboard>) -> Div {
    let bg = cx.theme().background;
    let fg = cx.theme().foreground;
    let muted = cx.theme().muted_foreground;
    let accent = cx.theme().accent;

    div()
        .size_full()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .bg(bg)
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap_6()
                .max_w(px(480.0))
                .p_8()
                .child(
                    // Logo/icon placeholder
                    div()
                        .size(px(80.0))
                        .rounded(px(16.0))
                        .bg(accent.opacity(0.1))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .text_size(px(40.0))
                                .text_color(accent)
                                .child("H"),
                        ),
                )
                .child(
                    // Welcome title
                    div()
                        .text_size(px(32.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(fg)
                        .child("Welcome to Humanboard"),
                )
                .child(
                    // Description
                    div()
                        .text_size(px(16.0))
                        .text_color(muted)
                        .text_center()
                        .line_height(px(24.0))
                        .child("Your creative canvas for organizing ideas, notes, and media. Drop files, add text, and build visual boards for any project."),
                )
                .child(
                    // Get started button
                    div()
                        .mt_4()
                        .child(
                            Button::new("get-started")
                                .primary()
                                .label("Get Started")
                                .tooltip("Begin using Humanboard")
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.complete_onboarding(cx);
                                })),
                        ),
                ),
        )
}
