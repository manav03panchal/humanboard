//! Chart configuration modal component.
//!
//! Provides a modal for configuring chart parameters before creation:
//! - Chart type selection (Bar, Line, Area, Pie, Scatter)
//! - X axis column selection
//! - Y axis column selection (multi-select)

use crate::app::ChartConfigModal;
use crate::app::Humanboard;
use crate::types::ChartType;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::{h_flex, v_flex, ActiveTheme as _, Icon, IconName};

/// Render the chart configuration modal
pub fn render_chart_config_modal(
    modal: &ChartConfigModal,
    cx: &mut Context<Humanboard>,
) -> impl IntoElement {
    let bg = cx.theme().background;
    let border = cx.theme().border;
    let fg = cx.theme().foreground;
    let muted_fg = cx.theme().muted_foreground;
    let primary = cx.theme().primary;
    let list_hover = cx.theme().list_hover;
    let list_active = cx.theme().list_active;

    let selected_type = modal.chart_type;
    let x_col = modal.x_column;
    let y_cols = modal.y_columns.clone();
    let column_names = modal.column_names.clone();

    deferred(
        div()
            .id("chart-config-backdrop")
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .bg(hsla(0.0, 0.0, 0.0, 0.6))
            .flex()
            .items_center()
            .justify_center()
            .on_mouse_down(MouseButton::Left, |_, _, _| {})
            .on_click(cx.listener(|this, _, _, cx| {
                this.close_chart_config_modal(cx);
            }))
            .child(
                v_flex()
                    .id("chart-config-modal")
                    .on_mouse_down(MouseButton::Left, |_, _, _| {})
                    .on_click(|_, _, _| {}) // Prevent click-through
                    .w(px(480.0))
                    .bg(bg)
                    .border_1()
                    .border_color(border)
                    .rounded(px(12.0))
                    .overflow_hidden()
                    .shadow_lg()
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
                                    .child("Create Chart"),
                            )
                            .child(
                                div()
                                    .id("close-chart-modal")
                                    .cursor_pointer()
                                    .p(px(4.0))
                                    .rounded(px(4.0))
                                    .hover(|s| s.bg(list_hover))
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.close_chart_config_modal(cx);
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
                            .gap(px(20.0))
                            // Chart type selector
                            .child(
                                v_flex()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(fg)
                                            .child("Chart Type"),
                                    )
                                    .child(
                                        h_flex()
                                            .gap(px(8.0))
                                            .children(ChartType::all().iter().map(|&chart_type| {
                                                let is_selected = chart_type == selected_type;
                                                div()
                                                    .id(ElementId::Name(format!("chart-type-{:?}", chart_type).into()))
                                                    .px(px(12.0))
                                                    .py(px(8.0))
                                                    .rounded(px(6.0))
                                                    .bg(if is_selected { primary } else { list_hover })
                                                    .text_color(if is_selected { cx.theme().primary_foreground } else { fg })
                                                    .text_size(px(12.0))
                                                    .font_weight(if is_selected { FontWeight::MEDIUM } else { FontWeight::NORMAL })
                                                    .cursor_pointer()
                                                    .hover(|s| if !is_selected { s.bg(list_active) } else { s })
                                                    .on_click(cx.listener(move |this, _, _, cx| {
                                                        this.set_chart_config_type(chart_type, cx);
                                                    }))
                                                    .child(chart_type.label())
                                            })),
                                    ),
                            )
                            // X Axis column selector
                            .child(
                                v_flex()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(fg)
                                            .child("X Axis (Labels)"),
                                    )
                                    .child(
                                        v_flex()
                                            .gap(px(4.0))
                                            .children(column_names.iter().enumerate().map(|(i, name)| {
                                                let is_selected = i == x_col;
                                                h_flex()
                                                    .id(ElementId::Name(format!("x-col-{}", i).into()))
                                                    .w_full()
                                                    .px(px(12.0))
                                                    .py(px(8.0))
                                                    .rounded(px(6.0))
                                                    .bg(if is_selected { list_active } else { gpui::transparent_black() })
                                                    .cursor_pointer()
                                                    .hover(|s| s.bg(list_hover))
                                                    .on_click(cx.listener(move |this, _, _, cx| {
                                                        this.set_chart_config_x_column(i, cx);
                                                    }))
                                                    .gap(px(8.0))
                                                    .child(
                                                        div()
                                                            .w(px(16.0))
                                                            .h(px(16.0))
                                                            .rounded(px(8.0))
                                                            .border_1()
                                                            .border_color(if is_selected { primary } else { border })
                                                            .bg(if is_selected { primary } else { gpui::transparent_black() })
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .when(is_selected, |d| d.child(
                                                                div()
                                                                    .w(px(6.0))
                                                                    .h(px(6.0))
                                                                    .rounded(px(3.0))
                                                                    .bg(cx.theme().primary_foreground)
                                                            ))
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(13.0))
                                                            .text_color(fg)
                                                            .child(name.clone())
                                                    )
                                            })),
                                    ),
                            )
                            // Y Axis column selector (multi-select)
                            .child(
                                v_flex()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(fg)
                                            .child("Y Axis (Values)"),
                                    )
                                    .child(
                                        v_flex()
                                            .gap(px(4.0))
                                            .children(column_names.iter().enumerate().map(|(i, name)| {
                                                let is_selected = y_cols.contains(&i);
                                                h_flex()
                                                    .id(ElementId::Name(format!("y-col-{}", i).into()))
                                                    .w_full()
                                                    .px(px(12.0))
                                                    .py(px(8.0))
                                                    .rounded(px(6.0))
                                                    .bg(if is_selected { list_active } else { gpui::transparent_black() })
                                                    .cursor_pointer()
                                                    .hover(|s| s.bg(list_hover))
                                                    .on_click(cx.listener(move |this, _, _, cx| {
                                                        this.toggle_chart_config_y_column(i, cx);
                                                    }))
                                                    .gap(px(8.0))
                                                    .child(
                                                        div()
                                                            .w(px(16.0))
                                                            .h(px(16.0))
                                                            .rounded(px(4.0))
                                                            .border_1()
                                                            .border_color(if is_selected { primary } else { border })
                                                            .bg(if is_selected { primary } else { gpui::transparent_black() })
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .when(is_selected, |d| d.child(
                                                                Icon::new(IconName::Check)
                                                                    .size(px(12.0))
                                                                    .text_color(cx.theme().primary_foreground)
                                                            ))
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(13.0))
                                                            .text_color(fg)
                                                            .child(name.clone())
                                                    )
                                            })),
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
                                Button::new("cancel-chart")
                                    .label("Cancel")
                                    .ghost()
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.close_chart_config_modal(cx);
                                    })),
                            )
                            .child(
                                Button::new("create-chart")
                                    .label("Create Chart")
                                    .primary()
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.confirm_chart_config(cx);
                                    })),
                            ),
                    ),
            ),
    )
    .with_priority(1600)
}
