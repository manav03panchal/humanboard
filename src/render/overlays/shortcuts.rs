//! Keyboard shortcuts overlay modal.

use crate::app::Humanboard;
use gpui::*;
use gpui_component::{h_flex, v_flex, ActiveTheme as _};

use super::modal_base::{render_kbd, render_shortcut_section};

/// Render the keyboard shortcuts overlay modal
pub fn render_shortcuts_overlay(cx: &mut Context<Humanboard>) -> impl IntoElement {
    let bg = cx.theme().popover;
    let border = cx.theme().border;
    let fg = cx.theme().foreground;

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
                v_flex()
                    .w(px(420.0))
                    .bg(bg)
                    .border_1()
                    .border_color(border)
                    .rounded(px(16.0))
                    .overflow_hidden()
                    .shadow_lg()
                    .on_mouse_down(MouseButton::Left, |_, _, _| {})
                    // Header
                    .child(
                        h_flex()
                            .px_5()
                            .py_4()
                            .border_b_1()
                            .border_color(border)
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(fg)
                                    .child("Keyboard Shortcuts"),
                            )
                            .child(render_kbd("Cmd+/", cx)),
                    )
                    // Content
                    .child(
                        v_flex()
                            .p_5()
                            .gap_5()
                            .child(render_shortcut_section(
                                "General",
                                vec![
                                    ("Cmd+K", "Command palette"),
                                    ("Cmd+N", "New board"),
                                    ("Cmd+H", "Go home"),
                                    ("Cmd+O", "Open file"),
                                    ("Cmd+,", "Settings"),
                                    ("Cmd+Q", "Quit"),
                                ],
                                cx,
                            ))
                            .child(render_shortcut_section(
                                "Canvas",
                                vec![
                                    ("Cmd+=", "Zoom in"),
                                    ("Cmd+-", "Zoom out"),
                                    ("Cmd+0", "Reset zoom"),
                                    ("Cmd+D", "Duplicate selected"),
                                    ("Del", "Delete selected"),
                                    ("Cmd+Z", "Undo"),
                                    ("Cmd+Shift+Z", "Redo"),
                                    ("Esc", "Close palette/preview"),
                                ],
                                cx,
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
                                cx,
                            )),
                    ),
            ),
    )
    .with_priority(1000)
}
