//! Loading states and spinners
//!
//! This module provides reusable loading state components for the application.

use gpui::*;
use gpui_component::{h_flex, v_flex};

/// A simple loading indicator with dots
pub fn render_loading_dots(primary: Hsla, muted_fg: Hsla) -> Div {
    h_flex()
        .gap(px(4.0))
        .items_center()
        .child(div().size(px(6.0)).rounded_full().bg(primary))
        .child(div().size(px(6.0)).rounded_full().bg(muted_fg.opacity(0.6)))
        .child(div().size(px(6.0)).rounded_full().bg(muted_fg.opacity(0.3)))
}

/// A loading spinner with text
pub fn render_loading_spinner(message: &str, primary: Hsla, muted_fg: Hsla) -> Div {
    v_flex()
        .items_center()
        .gap(px(12.0))
        .child(
            // Circular spinner representation using dots in a circle pattern
            div()
                .size(px(32.0))
                .flex()
                .items_center()
                .justify_center()
                .child(
                    h_flex()
                        .gap(px(3.0))
                        .child(div().size(px(8.0)).rounded_full().bg(primary))
                        .child(div().size(px(8.0)).rounded_full().bg(primary.opacity(0.6)))
                        .child(div().size(px(8.0)).rounded_full().bg(primary.opacity(0.3))),
                ),
        )
        .child(
            div()
                .text_sm()
                .text_color(muted_fg)
                .child(message.to_string()),
        )
}

/// A compact inline loading indicator
pub fn render_inline_loading(primary: Hsla) -> Div {
    h_flex()
        .gap(px(2.0))
        .child(div().size(px(4.0)).rounded_full().bg(primary))
        .child(div().size(px(4.0)).rounded_full().bg(primary.opacity(0.5)))
        .child(div().size(px(4.0)).rounded_full().bg(primary.opacity(0.25)))
}

/// A loading overlay for containers
pub fn render_loading_overlay(message: &str, bg: Hsla, primary: Hsla, muted_fg: Hsla) -> Div {
    div()
        .absolute()
        .inset_0()
        .bg(bg.opacity(0.9))
        .flex()
        .items_center()
        .justify_center()
        .child(
            v_flex()
                .items_center()
                .gap(px(16.0))
                .child(
                    h_flex()
                        .gap(px(6.0))
                        .child(div().size(px(10.0)).rounded_full().bg(primary))
                        .child(div().size(px(10.0)).rounded_full().bg(primary.opacity(0.6)))
                        .child(div().size(px(10.0)).rounded_full().bg(primary.opacity(0.3))),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(muted_fg)
                        .child(message.to_string()),
                ),
        )
}

/// A skeleton loading placeholder for content
pub fn render_skeleton(width: Pixels, height: Pixels, muted: Hsla) -> Div {
    div().w(width).h(height).rounded(px(4.0)).bg(muted)
}

/// A skeleton loading placeholder for text lines
pub fn render_text_skeleton(lines: usize, muted: Hsla) -> Div {
    let mut container = v_flex().gap(px(8.0));

    for i in 0..lines {
        // Vary the width slightly for natural appearance
        let width_percent = match i % 3 {
            0 => 100.0,
            1 => 85.0,
            _ => 70.0,
        };

        container = container.child(
            div()
                .h(px(12.0))
                .rounded(px(2.0))
                .bg(muted)
                .w(relative(width_percent / 100.0)),
        );
    }

    container
}
