//! Tool dock rendering - left-side toolbar for Miro-style tools

use crate::app::Humanboard;
use crate::constants::DOCK_WIDTH;
use crate::focus_ring::focus_ring_shadow;
use crate::types::ToolType;
use gpui::*;
use gpui_component::ActiveTheme as _;

/// Tool button configuration
struct ToolConfig {
    tool: ToolType,
    label: &'static str,
    tooltip: &'static str,
    shortcut: &'static str,
}

const TOOL_CONFIGS: [ToolConfig; 4] = [
    ToolConfig {
        tool: ToolType::Select,
        label: "V",
        tooltip: "Select tool",
        shortcut: "V",
    },
    ToolConfig {
        tool: ToolType::Text,
        label: "T",
        tooltip: "Text tool",
        shortcut: "T",
    },
    ToolConfig {
        tool: ToolType::Arrow,
        label: "→",
        tooltip: "Arrow tool",
        shortcut: "A",
    },
    ToolConfig {
        tool: ToolType::Shape,
        label: "□",
        tooltip: "Shape tool",
        shortcut: "S",
    },
];

/// Render a single tool button with focus ring, tooltip, and active state
fn render_tool_button(
    tool: ToolType,
    selected: bool,
    label: &'static str,
    tooltip: &'static str,
    shortcut: &'static str,
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
    let active_bg = cx.theme().muted.opacity(0.8);
    let focus_color = cx.theme().primary;
    let tooltip_bg = cx.theme().popover;
    let tooltip_border = cx.theme().border;
    let tooltip_fg = cx.theme().foreground;
    let tooltip_muted = cx.theme().muted_foreground;

    div()
        .id(ElementId::Name(format!("tool-{:?}", tool).into()))
        .relative()
        .group("tool-btn")
        .w(px(32.0))
        .h(px(32.0))
        .my(px(2.0))
        .rounded(px(6.0))
        .bg(bg)
        .hover(|s| s.bg(if selected { bg } else { hover_bg }))
        .active(|s| s.bg(if selected { bg } else { active_bg }))
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
        // Tooltip on hover (positioned to the right of the button)
        .child(
            div()
                .invisible()
                .group_hover("tool-btn", |s| s.visible())
                .absolute()
                .left(px(40.0))
                .top(px(4.0))
                .px(px(8.0))
                .py(px(4.0))
                .bg(tooltip_bg)
                .border_1()
                .border_color(tooltip_border)
                .rounded(px(4.0))
                .shadow_md()
                .whitespace_nowrap()
                .flex()
                .items_center()
                .gap(px(8.0))
                .child(
                    div()
                        .text_xs()
                        .text_color(tooltip_fg)
                        .child(tooltip),
                )
                .child(
                    div()
                        .px(px(4.0))
                        .py(px(1.0))
                        .bg(cx.theme().muted)
                        .rounded(px(2.0))
                        .text_xs()
                        .text_color(tooltip_muted)
                        .child(shortcut),
                ),
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
    let border_color = cx.theme().border;

    let mut dock = div()
        .id("tool-dock")
        .w(px(DOCK_WIDTH))
        .h_full()
        .flex()
        .flex_col()
        .items_center()
        .py(px(8.0))
        .gap(px(4.0))
        .border_r_1()
        .border_color(border_color);

    // Add tool buttons from config
    for config in &TOOL_CONFIGS {
        let tool = config.tool;
        let on_select_clone = on_select.clone();
        dock = dock.child(
            render_tool_button(
                tool,
                selected_tool == tool,
                config.label,
                config.tooltip,
                config.shortcut,
                cx,
            )
            .on_click(cx.listener(move |this, _, window, cx| {
                on_select_clone(this, tool, window, cx);
            })),
        );
    }

    dock
}
