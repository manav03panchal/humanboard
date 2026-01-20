//! Tool dock rendering - left-side toolbar for Miro-style tools

use crate::app::Humanboard;
use crate::constants::DOCK_WIDTH;
use crate::focus_ring::focus_ring_shadow;
use crate::types::ToolType;
use gpui::*;
use gpui_component::ActiveTheme as _;

/// Render a single tool button with focus ring support
fn render_tool_button(
    tool: ToolType,
    selected: bool,
    label: &'static str,
    cx: &App,
) -> Stateful<Div> {
    let bg = if selected {
        cx.theme().primary
    } else {
        cx.theme().transparent
    };
    let fg = if selected {
        cx.theme().primary_foreground
    } else {
        cx.theme().muted_foreground
    };
    let hover_bg = cx.theme().muted;
    let focus_color = cx.theme().primary;

    div()
        .id(ElementId::Name(format!("tool-{:?}", tool).into()))
        .w(px(32.0))
        .h(px(32.0))
        .my(px(2.0))
        .rounded(px(6.0))
        .bg(bg)
        .hover(|s| s.bg(if selected { bg } else { hover_bg }))
        // Focus ring for keyboard navigation (WCAG compliance)
        .focus(|s| s.shadow(focus_ring_shadow(focus_color)))
        .cursor_pointer()
        .flex()
        .items_center()
        .justify_center()
        .child(
            div()
                .text_size(px(13.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(fg)
                .child(label),
        )
}

/// Render the tool dock
pub fn render_tool_dock<F>(
    selected_tool: ToolType,
    on_select: F,
    cx: &Context<Humanboard>,
) -> Stateful<Div>
where
    F: Fn(&mut Humanboard, ToolType, &mut Window, &mut Context<Humanboard>) + 'static + Clone,
{
    let on_select1 = on_select.clone();
    let on_select2 = on_select.clone();
    let on_select3 = on_select.clone();
    let on_select4 = on_select.clone();
    let on_select5 = on_select.clone();

    let border_color = cx.theme().border;
    let divider_color = cx.theme().muted;

    div()
        .id("tool-dock")
        .w(px(DOCK_WIDTH))
        .h_full()
        .flex()
        .flex_col()
        .items_center()
        .py(px(8.0))
        .gap(px(4.0))
        .border_r_1()
        .border_color(border_color)
        // Selection tool
        .child(
            render_tool_button(ToolType::Select, selected_tool == ToolType::Select, "V", cx)
                .on_click(cx.listener(move |this, _, window, cx| {
                    on_select1(this, ToolType::Select, window, cx);
                })),
        )
        // Text tool
        .child(
            render_tool_button(ToolType::Text, selected_tool == ToolType::Text, "T", cx).on_click(
                cx.listener(move |this, _, window, cx| {
                    on_select2(this, ToolType::Text, window, cx);
                }),
            ),
        )
        // Arrow tool
        .child(
            render_tool_button(ToolType::Arrow, selected_tool == ToolType::Arrow, "→", cx)
                .on_click(cx.listener(move |this, _, window, cx| {
                    on_select3(this, ToolType::Arrow, window, cx);
                })),
        )
        // Shape tool
        .child(
            render_tool_button(ToolType::Shape, selected_tool == ToolType::Shape, "□", cx)
                .on_click(cx.listener(move |this, _, window, cx| {
                    on_select4(this, ToolType::Shape, window, cx);
                })),
        )
        // Divider for data visualization tools
        .child(
            div()
                .w(px(24.0))
                .h(px(1.0))
                .my(px(4.0))
                .bg(divider_color)
        )
        // Table tool
        .child(
            render_tool_button(ToolType::Table, selected_tool == ToolType::Table, "▦", cx)
                .on_click(cx.listener(move |this, _, window, cx| {
                    on_select5(this, ToolType::Table, window, cx);
                })),
        )
}
