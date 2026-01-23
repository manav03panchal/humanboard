//! Preview panel rendering - tabs, content, splitter
//!
//! This module handles the preview panel for PDFs and Markdown files including:
//! - Tab bar with multiple file support
//! - PDF viewer with navigation
//! - Markdown preview and editing
//! - Resizable splitter

use crate::app::{Humanboard, PreviewTab, SplitDirection};
use crate::focus::FocusContext;
use crate::focus_ring::focus_ring_shadow;
use crate::loading::render_loading_spinner;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::Disableable as _;
use gpui_component::InteractiveElementExt as _;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::input::Input;
use gpui_component::{ActiveTheme as _, Icon, IconName, Sizable, h_flex, v_flex};
use std::path::PathBuf;

/// Render the tab bar for the preview panel
/// `is_left_pane` indicates which pane this tab bar belongs to for proper event routing
pub fn render_tab_bar(
    tabs: &Vec<PreviewTab>,
    active_tab: usize,
    scroll_handle: &ScrollHandle,
    dragging_tab: Option<usize>,
    drag_target: Option<usize>,
    is_left_pane: bool,
    cx: &mut Context<Humanboard>,
) -> Stateful<Div> {
    let bg = cx.theme().title_bar;
    let border = cx.theme().border;
    let fg = cx.theme().foreground;
    let muted_fg = cx.theme().muted_foreground;
    let list_active = cx.theme().list_active;
    let list_hover = cx.theme().list_hover;
    let primary = cx.theme().primary;
    let danger = cx.theme().danger;

    div()
        .id("preview-tab-bar")
        .h(px(36.0))
        .w_full()
        .bg(bg)
        .border_b_1()
        .border_color(border)
        .flex()
        .items_center()
        .overflow_x_scroll()
        .track_scroll(scroll_handle)
        // Cancel drag if mouse leaves tab bar
        .on_mouse_up(
            MouseButton::Left,
            cx.listener(|this, _event, _window, cx| {
                if this.dragging_tab.is_some() {
                    this.finish_tab_drag(cx);
                } else if this.tab_drag_pending.is_some() {
                    this.cancel_pending_drag(cx);
                }
            }),
        )
        .child(
            h_flex()
                .flex_shrink_0()
                .children(tabs.iter().enumerate().map(|(index, tab)| {
                    let is_active = index == active_tab;
                    let filename = tab.title();
                    let is_markdown = matches!(tab, PreviewTab::Markdown { .. });
                    let is_code = matches!(tab, PreviewTab::Code { .. });
                    let is_table = matches!(tab, PreviewTab::Table { .. });
                    // Check dirty state - for tables, check the delegate's data source
                    let is_dirty = match tab {
                        PreviewTab::Code { dirty: true, .. } => true,
                        PreviewTab::Table { table_state: Some(state), .. } => {
                            state.read(cx).delegate().is_dirty()
                        }
                        _ => false,
                    };
                    let is_preview = tab.is_preview();
                    let is_pinned = tab.is_pinned();
                    let is_being_dragged = dragging_tab == Some(index);
                    let is_drag_target = drag_target == Some(index)
                        && dragging_tab.is_some()
                        && dragging_tab != Some(index);

                    let display_name = if filename.len() > 20 {
                        format!("{}...", &filename[..17])
                    } else {
                        filename
                    };

                    let tab_index = index;
                    let tab_index_close = index;
                    let tab_index_pin = index;
                    let tab_index_drag = index;

                    h_flex()
                        .id(ElementId::Name(format!("tab-{}", index).into()))
                        .flex_shrink_0()
                        .gap_2()
                        .px_3()
                        .py_1()
                        .bg(if is_active { list_active } else { bg })
                        .border_r_1()
                        .border_color(border)
                        // Show drop indicator
                        .when(is_drag_target, |d| d.border_l_2().border_color(primary))
                        // Dim the tab being dragged
                        .when(is_being_dragged, |d| d.opacity(0.5))
                        .hover(|style| style.bg(list_hover))
                        // Focus ring for keyboard navigation (WCAG compliance)
                        .focus(|s| s.shadow(focus_ring_shadow(primary)))
                        .cursor(if dragging_tab.is_some() {
                            CursorStyle::ClosedHand
                        } else {
                            CursorStyle::PointingHand
                        })
                        .on_click(cx.listener(move |this, _event, _window, cx| {
                            if this.dragging_tab.is_none() {
                                this.switch_tab_in_pane(tab_index, is_left_pane, cx);
                            }
                        }))
                        // Double-click converts preview to permanent
                        .on_double_click(cx.listener(move |this, _event, _window, cx| {
                            this.make_tab_permanent_in_pane(tab_index, is_left_pane, cx);
                        }))
                        // Start drag on mouse down
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, event: &MouseDownEvent, _window, cx| {
                                this.start_tab_drag_in_pane(tab_index_drag, event.position, is_left_pane, cx);
                            }),
                        )
                        // Update drag target and position on mouse move
                        .on_mouse_move(cx.listener(
                            move |this, event: &MouseMoveEvent, _window, cx| {
                                // Check pending drag threshold or update active drag
                                if this.tab_drag_pending.is_some() || this.dragging_tab.is_some() {
                                    this.update_tab_drag_position(event.position, cx);
                                    if this.dragging_tab.is_some() {
                                        this.update_tab_drag_target(tab_index_drag, cx);
                                    }
                                }
                            },
                        ))
                        // Finish drag on mouse up
                        .on_mouse_up(
                            MouseButton::Left,
                            cx.listener(move |this, _event, _window, cx| {
                                if this.dragging_tab.is_some() {
                                    this.finish_tab_drag(cx);
                                } else if this.tab_drag_pending.is_some() {
                                    // Threshold wasn't reached - cancel pending and let click handle tab switch
                                    this.cancel_pending_drag(cx);
                                }
                            }),
                        )
                        // Pin icon for pinned tabs
                        .when(is_pinned, |d| {
                            d.child(div().text_xs().text_color(muted_fg).mr_1().child("📌"))
                        })
                        .child(if is_table {
                            Icon::new(IconName::LayoutDashboard)
                                .xsmall()
                                .text_color(primary)
                        } else if is_code {
                            Icon::new(IconName::SquareTerminal)
                                .xsmall()
                                .text_color(primary) // Use theme primary for code
                        } else if is_markdown {
                            Icon::new(IconName::File).xsmall().text_color(primary)
                        } else {
                            Icon::new(IconName::File).xsmall().text_color(danger) // PDF
                        })
                        .child(
                            div()
                                .text_xs()
                                .whitespace_nowrap()
                                .text_color(if is_active { fg } else { muted_fg })
                                // Italicize preview tabs
                                .when(is_preview, |d| d.italic())
                                .child(display_name),
                        )
                        .child(
                            div()
                                .w(px(14.0))
                                .h(px(14.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .rounded(px(2.0))
                                .when(is_dirty, |d| {
                                    // Show dot indicator when dirty
                                    d.child(div().w(px(8.0)).h(px(8.0)).rounded_full().bg(primary))
                                })
                                .when(!is_dirty && !is_pinned, |d| {
                                    // Show close button when not dirty and not pinned
                                    d.text_xs()
                                        .text_color(muted_fg)
                                        .hover(|style| style.bg(list_hover).text_color(fg))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _event, _window, cx| {
                                                this.close_tab_in_pane(tab_index_close, is_left_pane, cx);
                                            }),
                                        )
                                        .child("×")
                                })
                                .when(is_pinned && !is_dirty, |d| {
                                    // Show unpin option on hover for pinned tabs
                                    d.text_xs()
                                        .text_color(muted_fg)
                                        .hover(|style| style.text_color(fg))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _event, _window, cx| {
                                                this.toggle_tab_pinned_in_pane(tab_index_pin, is_left_pane, cx);
                                            }),
                                        )
                                        .child("×")
                                }),
                        )
                })),
        )
}

/// Render a ghost tab that follows the cursor during drag
pub fn render_drag_ghost(
    tab: &PreviewTab,
    position: Point<Pixels>,
    cx: &mut Context<Humanboard>,
) -> Div {
    let primary = cx.theme().primary;
    let bg = cx.theme().background;
    let fg = cx.theme().foreground;

    let filename = tab.title();
    let display_name = if filename.len() > 20 {
        format!("{}...", &filename[..17])
    } else {
        filename
    };

    div()
        .absolute()
        .left(position.x - px(50.0))
        .top(position.y - px(15.0))
        .px_3()
        .py_1()
        .bg(bg)
        .border_1()
        .border_color(primary)
        .rounded(px(4.0))
        .shadow_lg()
        .opacity(0.9)
        .flex()
        .items_center()
        .gap_2()
        .child(Icon::new(IconName::File).xsmall().text_color(primary))
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(fg)
                .child(display_name),
        )
}

/// Render a single drop zone indicator (visual only - parent handles events)
fn render_drop_zone_indicator(
    _id: &'static str,
    zone: crate::app::SplitDropZone,
    active_zone: Option<crate::app::SplitDropZone>,
    label: &'static str,
    cx: &Context<Humanboard>,
) -> Div {
    let primary = cx.theme().primary;
    let bg = cx.theme().background;
    let is_active = active_zone == Some(zone);

    div()
        .rounded(px(8.0))
        .border_2()
        .border_color(if is_active {
            primary
        } else {
            primary.opacity(0.4)
        })
        .bg(if is_active {
            primary.opacity(0.25)
        } else {
            bg.opacity(0.9)
        })
        .flex()
        .items_center()
        .justify_center()
        .shadow_lg()
        .child(
            div()
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(if is_active {
                    primary
                } else {
                    primary.opacity(0.7)
                })
                .child(label),
        )
}

/// Render split drop zones when dragging a tab - shows 4 directional zones
pub fn render_split_drop_zones(
    active_zone: Option<crate::app::SplitDropZone>,
    cx: &mut Context<Humanboard>,
) -> Div {
    use crate::app::SplitDropZone;

    // Zone strip dimensions
    let side_width = px(120.0);
    let top_bottom_height = px(100.0);

    div()
        .absolute()
        .inset_0()
        .bg(gpui::black().opacity(0.3))
        // Only handle mouse up on the background (no mouse move here - children handle zones)
        .on_mouse_up(
            MouseButton::Left,
            cx.listener(|this, _event, _window, cx| {
                this.finish_tab_drag(cx);
            }),
        )
        // Center area - this clears the zone when hovering in the middle
        .child(
            div()
                .id("zone-center")
                .absolute()
                .top(top_bottom_height)
                .bottom(top_bottom_height)
                .left(side_width)
                .right(side_width)
                .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _window, cx| {
                    this.update_tab_drag_position(event.position, cx);
                    this.set_tab_drag_split_zone(None, cx);
                }))
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(|this, _event, _window, cx| {
                        this.finish_tab_drag(cx);
                    }),
                ),
        )
        // Left zone
        .child(
            div()
                .id("zone-left-strip")
                .absolute()
                .left_0()
                .top_0()
                .bottom_0()
                .w(side_width)
                .flex()
                .items_center()
                .justify_center()
                .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _window, cx| {
                    this.update_tab_drag_position(event.position, cx);
                    this.set_tab_drag_split_zone(Some(SplitDropZone::Left), cx);
                }))
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(|this, _event, _window, cx| {
                        this.finish_tab_drag(cx);
                    }),
                )
                .child(
                    render_drop_zone_indicator(
                        "zone-left",
                        SplitDropZone::Left,
                        active_zone,
                        "← Left",
                        cx,
                    )
                    .w(px(90.0))
                    .h(px(90.0)),
                ),
        )
        // Right zone
        .child(
            div()
                .id("zone-right-strip")
                .absolute()
                .right_0()
                .top_0()
                .bottom_0()
                .w(side_width)
                .flex()
                .items_center()
                .justify_center()
                .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _window, cx| {
                    this.update_tab_drag_position(event.position, cx);
                    this.set_tab_drag_split_zone(Some(SplitDropZone::Right), cx);
                }))
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(|this, _event, _window, cx| {
                        this.finish_tab_drag(cx);
                    }),
                )
                .child(
                    render_drop_zone_indicator(
                        "zone-right",
                        SplitDropZone::Right,
                        active_zone,
                        "Right →",
                        cx,
                    )
                    .w(px(90.0))
                    .h(px(90.0)),
                ),
        )
        // Top zone (between left and right strips)
        .child(
            div()
                .id("zone-top-strip")
                .absolute()
                .top_0()
                .left(side_width)
                .right(side_width)
                .h(top_bottom_height)
                .flex()
                .items_center()
                .justify_center()
                .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _window, cx| {
                    this.update_tab_drag_position(event.position, cx);
                    this.set_tab_drag_split_zone(Some(SplitDropZone::Top), cx);
                }))
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(|this, _event, _window, cx| {
                        this.finish_tab_drag(cx);
                    }),
                )
                .child(
                    render_drop_zone_indicator(
                        "zone-top",
                        SplitDropZone::Top,
                        active_zone,
                        "↑ Top",
                        cx,
                    )
                    .w(px(90.0))
                    .h(px(70.0)),
                ),
        )
        // Bottom zone (between left and right strips)
        .child(
            div()
                .id("zone-bottom-strip")
                .absolute()
                .bottom_0()
                .left(side_width)
                .right(side_width)
                .h(top_bottom_height)
                .flex()
                .items_center()
                .justify_center()
                .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _window, cx| {
                    this.update_tab_drag_position(event.position, cx);
                    this.set_tab_drag_split_zone(Some(SplitDropZone::Bottom), cx);
                }))
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(|this, _event, _window, cx| {
                        this.finish_tab_drag(cx);
                    }),
                )
                .child(
                    render_drop_zone_indicator(
                        "zone-bottom",
                        SplitDropZone::Bottom,
                        active_zone,
                        "Bottom ↓",
                        cx,
                    )
                    .w(px(90.0))
                    .h(px(70.0)),
                ),
        )
}

/// Render a single pane with tab bar and content
fn render_pane(
    id: &'static str,
    content_id: &'static str,
    tabs: &Vec<PreviewTab>,
    active_tab: usize,
    scroll: &ScrollHandle,
    is_focused: bool,
    dragging_tab: Option<usize>,
    drag_target: Option<usize>,
    search_input: Option<&Entity<gpui_component::input::InputState>>,
    search_match_count: usize,
    search_current: usize,
    is_left_pane: bool,
    cx: &mut Context<Humanboard>,
) -> Stateful<Div> {
    let bg = cx.theme().background;
    let border = cx.theme().border;
    let primary = cx.theme().primary;

    v_flex()
        .id(id)
        .flex_1()
        .min_h_0()
        .min_w_0()
        .bg(bg)
        .overflow_hidden()
        .border_1()
        .border_color(if is_focused { primary } else { border })
        .rounded(px(4.0))
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _event, _window, cx| {
                if is_left_pane {
                    this.focus_left_pane(cx);
                } else {
                    this.focus_right_pane(cx);
                }
            }),
        )
        .child(render_tab_bar(
            tabs,
            active_tab,
            scroll,
            if is_focused { dragging_tab } else { None },
            if is_focused { drag_target } else { None },
            is_left_pane,
            cx,
        ))
        .when(is_focused, |d| {
            d.when_some(search_input, |d, input| {
                d.child(render_search_bar(
                    input,
                    search_match_count,
                    search_current,
                    cx,
                ))
            })
        })
        .child(
            div()
                .id(content_id)
                .flex_1()
                .min_h_0()
                .min_w_0()
                .w_full()
                .overflow_hidden()
                .flex()
                .flex_col()
                .when_some(tabs.get(active_tab), |d, tab| {
                    d.child(render_tab_content(tab, true, active_tab, cx))
                }),
        )
}

/// Render the split pane container when panel is split
pub fn render_split_panes(
    preview: &crate::app::PreviewPanel,
    left_scroll: &ScrollHandle,
    right_scroll: &ScrollHandle,
    dragging_tab: Option<usize>,
    drag_target: Option<usize>,
    search_input: Option<&Entity<gpui_component::input::InputState>>,
    search_match_count: usize,
    search_current: usize,
    split_zone: Option<crate::app::SplitDropZone>,
    cx: &mut Context<Humanboard>,
) -> Div {
    use crate::app::{FocusedPane, SplitDropZone};
    use gpui::DefiniteLength::Fraction;

    let left_focused = preview.focused_pane == FocusedPane::Left;
    let right_focused = preview.focused_pane == FocusedPane::Right;
    let is_horizontal = preview.pane_split_horizontal;
    let is_dragging = dragging_tab.is_some();
    let pane_ratio = preview.pane_ratio;

    let first_pane = render_pane(
        "first-pane",
        "first-pane-content",
        &preview.tabs,
        preview.active_tab,
        left_scroll,
        left_focused,
        dragging_tab,
        drag_target,
        search_input,
        search_match_count,
        search_current,
        true,
        cx,
    );

    let second_pane = render_pane(
        "second-pane",
        "second-pane-content",
        &preview.right_tabs,
        preview.right_active_tab,
        right_scroll,
        right_focused,
        dragging_tab,
        drag_target,
        search_input,
        search_match_count,
        search_current,
        false,
        cx,
    );

    // Wrap panes with drop zone overlays when dragging
    // Use Fraction for sizing based on pane_ratio
    let first_pane_wrapped = if is_dragging {
        let target_zone = if is_horizontal {
            SplitDropZone::Top
        } else {
            SplitDropZone::Left
        };
        let is_active = split_zone == Some(target_zone);
        let primary = cx.theme().primary;

        div()
            .id("first-pane-drop")
            .when(is_horizontal, |d| d.h(Fraction(pane_ratio)).w_full())
            .when(!is_horizontal, |d| d.w(Fraction(pane_ratio)).h_full())
            .min_h_0()
            .min_w_0()
            .flex()
            .flex_col()
            .relative()
            .child(first_pane)
            .child(
                // Drop zone overlay
                div()
                    .id("first-pane-overlay")
                    .absolute()
                    .inset_0()
                    .bg(if is_active {
                        primary.opacity(0.2)
                    } else {
                        gpui::transparent_black()
                    })
                    .border_2()
                    .border_color(if is_active {
                        primary
                    } else {
                        gpui::transparent_black()
                    })
                    .rounded(px(4.0))
                    .on_mouse_move(
                        cx.listener(move |this, event: &MouseMoveEvent, _window, cx| {
                            this.update_tab_drag_position(event.position, cx);
                            this.set_tab_drag_split_zone(Some(target_zone), cx);
                        }),
                    )
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(|this, _event, _window, cx| {
                            this.finish_tab_drag(cx);
                        }),
                    ),
            )
    } else {
        div()
            .id("first-pane-wrap")
            .when(is_horizontal, |d| d.h(Fraction(pane_ratio)).w_full())
            .when(!is_horizontal, |d| d.w(Fraction(pane_ratio)).h_full())
            .min_h_0()
            .min_w_0()
            .flex()
            .flex_col()
            .child(first_pane)
    };

    let second_pane_wrapped = if is_dragging {
        let target_zone = if is_horizontal {
            SplitDropZone::Bottom
        } else {
            SplitDropZone::Right
        };
        let is_active = split_zone == Some(target_zone);
        let primary = cx.theme().primary;

        div()
            .id("second-pane-drop")
            .when(is_horizontal, |d| d.h(Fraction(1.0 - pane_ratio)).w_full())
            .when(!is_horizontal, |d| d.w(Fraction(1.0 - pane_ratio)).h_full())
            .min_h_0()
            .min_w_0()
            .flex()
            .flex_col()
            .relative()
            .child(second_pane)
            .child(
                // Drop zone overlay
                div()
                    .id("second-pane-overlay")
                    .absolute()
                    .inset_0()
                    .bg(if is_active {
                        primary.opacity(0.2)
                    } else {
                        gpui::transparent_black()
                    })
                    .border_2()
                    .border_color(if is_active {
                        primary
                    } else {
                        gpui::transparent_black()
                    })
                    .rounded(px(4.0))
                    .on_mouse_move(
                        cx.listener(move |this, event: &MouseMoveEvent, _window, cx| {
                            this.update_tab_drag_position(event.position, cx);
                            this.set_tab_drag_split_zone(Some(target_zone), cx);
                        }),
                    )
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(|this, _event, _window, cx| {
                            this.finish_tab_drag(cx);
                        }),
                    ),
            )
    } else {
        div()
            .id("second-pane-wrap")
            .when(is_horizontal, |d| d.h(Fraction(1.0 - pane_ratio)).w_full())
            .when(!is_horizontal, |d| d.w(Fraction(1.0 - pane_ratio)).h_full())
            .min_h_0()
            .min_w_0()
            .flex()
            .flex_col()
            .child(second_pane)
    };

    // Pane splitter (draggable divider)
    let pane_splitter = render_pane_splitter(is_horizontal, cx);

    if is_horizontal {
        // Top/Bottom split
        v_flex()
            .flex_1()
            .min_h_0()
            .w_full()
            .child(first_pane_wrapped)
            .child(pane_splitter)
            .child(second_pane_wrapped)
    } else {
        // Left/Right split
        h_flex()
            .flex_1()
            .min_w_0()
            .h_full()
            .child(first_pane_wrapped)
            .child(pane_splitter)
            .child(second_pane_wrapped)
    }
}

/// Render the draggable splitter between split panes
fn render_pane_splitter(is_horizontal: bool, cx: &mut Context<Humanboard>) -> Stateful<Div> {
    let border = cx.theme().border;
    let list_hover = cx.theme().list_hover;

    if is_horizontal {
        // Horizontal splitter (for top/bottom split)
        div()
            .id("pane-splitter")
            .h(px(6.0))
            .w_full()
            .cursor(CursorStyle::ResizeUpDown)
            .bg(gpui::transparent_black())
            .hover(|s| s.bg(list_hover))
            .flex()
            .items_center()
            .justify_center()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                    this.dragging_pane_splitter = true;
                    cx.notify();
                }),
            )
            .child(div().h(px(2.0)).w(px(40.0)).bg(border).rounded(px(1.0)))
    } else {
        // Vertical splitter (for left/right split)
        div()
            .id("pane-splitter")
            .w(px(6.0))
            .h_full()
            .cursor(CursorStyle::ResizeLeftRight)
            .bg(gpui::transparent_black())
            .hover(|s| s.bg(list_hover))
            .flex()
            .items_center()
            .justify_center()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                    this.dragging_pane_splitter = true;
                    cx.notify();
                }),
            )
            .child(div().w(px(2.0)).h(px(40.0)).bg(border).rounded(px(1.0)))
    }
}

/// Render the content area for a preview tab
pub fn render_tab_content(
    tab: &PreviewTab,
    _is_active: bool,
    _tab_index: usize,
    cx: &mut Context<Humanboard>,
) -> Div {
    let bg = cx.theme().background;
    let title_bar = cx.theme().title_bar;
    let border = cx.theme().border;
    let muted_fg = cx.theme().muted_foreground;
    let primary = cx.theme().primary;

    match tab {
        PreviewTab::Pdf { .. } => {
            // PDF webviews are positioned explicitly via set_bounds in ensure_pdf_webview
            // We just need an empty container - the webview renders as a native overlay
            div().flex_1().w_full().min_h_0()
        }
        PreviewTab::Markdown {
            content,
            editing,
            editor,
            ..
        } => {
            let is_editing = *editing;

            v_flex()
                .flex_1()
                .w_full()
                .min_h_0()
                .bg(bg)
                .child(
                    // Content area
                    div()
                        .id("md-content-scroll")
                        .flex_1()
                        .overflow_y_scroll()
                        .bg(bg)
                        .when(!is_editing, |d| {
                            // Preview mode - show rendered markdown (scrollable)
                            d.child(crate::markdown_card::render_markdown_content(
                                content, 1.0, cx,
                            ))
                        })
                        .when(is_editing, |d| {
                            // Edit mode - code editor with markdown syntax highlighting
                            if let Some(ed) = editor {
                                d.child(Input::new(ed).size_full().appearance(false))
                            } else {
                                d.child(div().p_4().child(render_loading_spinner(
                                    "Loading editor...",
                                    cx.theme().primary,
                                    cx.theme().muted_foreground,
                                )))
                            }
                        }),
                )
                .child(
                    // Footer with action buttons
                    h_flex()
                        .h(px(40.0))
                        .bg(title_bar)
                        .border_t_1()
                        .border_color(border)
                        .items_center()
                        .justify_between()
                        .px_3()
                        .child(
                            div()
                                .text_xs()
                                .text_color(muted_fg)
                                .child(format!("{} chars", content.len())),
                        )
                        .child(
                            h_flex()
                                .gap_2()
                                .when(is_editing, |d| {
                                    d.child(
                                        Button::new("save-markdown")
                                            .primary()
                                            .small()
                                            .label("Save")
                                            .tooltip("Save markdown changes")
                                            .on_click(cx.listener(|this, _, _window, cx| {
                                                this.save_markdown(cx);
                                            })),
                                    )
                                })
                                .child(if is_editing {
                                    Button::new("toggle-edit")
                                        .ghost()
                                        .small()
                                        .label("Cancel")
                                        .tooltip("Cancel editing")
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.toggle_markdown_edit(window, cx);
                                        }))
                                } else {
                                    Button::new("toggle-edit")
                                        .primary()
                                        .small()
                                        .label("Edit")
                                        .tooltip("Edit markdown")
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.toggle_markdown_edit(window, cx);
                                        }))
                                }),
                        ),
                )
        }
        PreviewTab::Code {
            content,
            language,
            dirty,
            editor,
            ..
        } => {
            let is_dirty = *dirty;
            let lang = language.clone();
            let line_count = content.lines().count();

            v_flex()
                .flex_1()
                .w_full()
                .min_h_0()
                .bg(bg)
                .child({
                    // Content area - always editable
                    let editor_entity = editor.clone();
                    let code_editor_focus = cx.focus_handle();
                    div()
                        .id("code-content-scroll")
                        .flex_1()
                        .overflow_y_scroll()
                        .bg(bg)
                        .track_focus(&code_editor_focus)
                        .key_context(FocusContext::KEY_CODE_EDITOR)
                        .on_click(cx.listener(move |this, _event, window, cx| {
                            // Set focus context to CodeEditor and focus the editor
                            this.focus
                                .focus(crate::focus::FocusContext::CodeEditor, window);
                            code_editor_focus.focus(window);
                            if let Some(ref ed) = editor_entity {
                                ed.update(cx, |state, cx| {
                                    state.focus(window, cx);
                                });
                            }
                        }))
                        .child(if let Some(ed) = editor {
                            Input::new(ed).size_full().appearance(false).into_any_element()
                        } else {
                            div()
                                .p_4()
                                .child(render_loading_spinner(
                                    "Loading code...",
                                    cx.theme().primary,
                                    cx.theme().muted_foreground,
                                ))
                                .into_any_element()
                        })
                })
                .child(
                    // Footer with action buttons
                    h_flex()
                        .h(px(40.0))
                        .bg(title_bar)
                        .border_t_1()
                        .border_color(border)
                        .items_center()
                        .justify_between()
                        .px_3()
                        .child(
                            h_flex()
                                .gap_3()
                                .child(
                                    div()
                                        .px(px(6.0))
                                        .py(px(2.0))
                                        .bg(primary.opacity(0.2))
                                        .rounded(px(3.0))
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(primary)
                                        .child(lang.to_uppercase()),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(muted_fg)
                                        .child(format!("{} lines", line_count)),
                                ),
                        )
                        .when(is_dirty, |d| {
                            d.child(div().text_xs().text_color(muted_fg).child("⌘S to save"))
                        }),
                )
        }

        PreviewTab::Table {
            name,
            table_state,
            ..
        } => {
            let table_name = name.clone();
            // Check if table has unsaved changes
            let is_dirty = table_state
                .as_ref()
                .map(|state| state.read(cx).delegate().is_dirty())
                .unwrap_or(false);

            // Focus handle for key context
            let table_focus = cx.focus_handle();

            v_flex()
                .flex_1()
                .w_full()
                .min_h_0()
                .bg(bg)
                .child(
                    // Table content area
                    div()
                        .id("table-content-scroll")
                        .flex_1()
                        .overflow_hidden()
                        .bg(bg)
                        .track_focus(&table_focus)
                        .key_context(FocusContext::KEY_PREVIEW)
                        .on_click(cx.listener(move |this, _event, window, cx| {
                            // Set focus context to Preview and focus the table
                            this.focus.focus(crate::focus::FocusContext::Preview, window);
                            table_focus.focus(window);
                            cx.notify();
                        }))
                        .when_some(table_state.as_ref(), |d, state| {
                            use gpui_component::table::Table;
                            d.child(
                                Table::new(state)
                                    .bordered(true)
                                    .stripe(true)
                            )
                        })
                        .when(table_state.is_none(), |d| {
                            d.child(
                                div()
                                    .p_4()
                                    .child(render_loading_spinner(
                                        "Loading table...",
                                        cx.theme().primary,
                                        cx.theme().muted_foreground,
                                    ))
                            )
                        }),
                )
                .child(
                    // Footer with table info
                    h_flex()
                        .h(px(40.0))
                        .bg(title_bar)
                        .border_t_1()
                        .border_color(border)
                        .items_center()
                        .justify_between()
                        .px_3()
                        .child(
                            h_flex()
                                .gap_3()
                                .child(
                                    div()
                                        .px(px(6.0))
                                        .py(px(2.0))
                                        .bg(primary.opacity(0.2))
                                        .rounded(px(3.0))
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(primary)
                                        .child("CSV"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(muted_fg)
                                        .child(table_name),
                                ),
                        )
                        .when(is_dirty, |d| {
                            d.child(div().text_xs().text_color(muted_fg).child("⌘S to save"))
                        }),
                )
        }
    }
}

/// Render the legacy preview panel (for PDF pages)
/// Render the search bar for the preview panel
pub fn render_search_bar(
    search_input: &Entity<gpui_component::input::InputState>,
    match_count: usize,
    current_match: usize,
    cx: &mut Context<Humanboard>,
) -> Div {
    let bg = cx.theme().title_bar;
    let border = cx.theme().border;
    let muted_fg = cx.theme().muted_foreground;

    h_flex()
        .w_full()
        .h(px(32.0))
        .px_2()
        .py_1()
        .bg(bg)
        .border_b_1()
        .border_color(border)
        .gap_2()
        .items_center()
        .child(div().flex_1().child(Input::new(search_input).xsmall()))
        .child(
            div()
                .text_xs()
                .text_color(muted_fg)
                .child(if match_count > 0 {
                    format!("{}/{}", current_match + 1, match_count)
                } else {
                    "No matches".to_string()
                }),
        )
        .child(
            h_flex()
                .gap_1()
                .child(
                    Button::new("search-prev")
                        .icon(IconName::ChevronUp)
                        .xsmall()
                        .ghost()
                        .tooltip("Previous match")
                        .disabled(match_count == 0)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.prev_search_match(cx);
                        })),
                )
                .child(
                    Button::new("search-next")
                        .icon(IconName::ChevronDown)
                        .xsmall()
                        .ghost()
                        .tooltip("Next match")
                        .disabled(match_count == 0)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.next_search_match(cx);
                        })),
                )
                .child(
                    Button::new("search-close")
                        .icon(IconName::Close)
                        .xsmall()
                        .ghost()
                        .tooltip("Close search")
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.close_preview_search(cx);
                        })),
                ),
        )
}

pub fn render_preview_panel(
    file_name: String,
    page_image_path: Option<PathBuf>,
    current_page: usize,
    page_count: usize,
    zoom: f32,
    cx: &Context<Humanboard>,
) -> Div {
    let bg = cx.theme().background;
    let border = cx.theme().border;
    let fg = cx.theme().foreground;
    let muted_fg = cx.theme().muted_foreground;
    let danger = cx.theme().danger;
    let primary = cx.theme().primary;

    // Truncate filename if too long
    let display_name = if file_name.len() > 25 {
        format!("{}...", &file_name[..22])
    } else {
        file_name
    };

    div()
        .flex()
        .flex_col()
        .size_full()
        .bg(bg)
        .child(
            // Header bar - compact layout
            div()
                .flex()
                .items_center()
                .gap_2()
                .px_3()
                .py_2()
                .bg(bg)
                .border_b_1()
                .border_color(border)
                // PDF badge
                .child(
                    div()
                        .px_2()
                        .py(px(2.0))
                        .bg(danger)
                        .rounded(px(4.0))
                        .text_xs()
                        .text_color(fg)
                        .flex_shrink_0()
                        .child("PDF"),
                )
                // Filename (truncated)
                .child(
                    div()
                        .flex_1()
                        .min_w(px(0.0))
                        .overflow_hidden()
                        .text_sm()
                        .text_color(fg)
                        .child(display_name),
                )
                // Page indicator
                .child(
                    div()
                        .flex_shrink_0()
                        .text_sm()
                        .text_color(muted_fg)
                        .child(format!("{}/{}", current_page, page_count)),
                )
                // Zoom indicator
                .child(
                    div()
                        .flex_shrink_0()
                        .text_sm()
                        .text_color(primary)
                        .child(format!("{:.0}%", zoom * 100.0)),
                )
                // Keyboard hints
                .child(
                    div()
                        .flex_shrink_0()
                        .text_xs()
                        .text_color(muted_fg)
                        .child("Scroll=Pan • ⌘+Scroll=Zoom • T=Split"),
                ),
        )
        .child(
            // PDF content area
            div()
                .flex_1()
                .flex()
                .items_center()
                .justify_center()
                .overflow_hidden()
                .bg(bg)
                .when_some(page_image_path.clone(), |d, path| {
                    d.child(
                        img(path)
                            .max_w_full()
                            .max_h_full()
                            .object_fit(ObjectFit::Contain),
                    )
                })
                .when(page_image_path.is_none(), |d| {
                    if page_count == 0 {
                        d.child(
                            div()
                                .text_color(danger)
                                .text_sm()
                                .child("Failed to load PDF"),
                        )
                    } else {
                        d.child(render_loading_spinner("Loading page...", primary, muted_fg))
                    }
                }),
        )
}

/// Render the resizable splitter between canvas and preview
pub fn render_splitter(direction: SplitDirection, cx: &mut Context<Humanboard>) -> Div {
    let title_bar = cx.theme().title_bar;
    let list_hover = cx.theme().list_hover;
    let border = cx.theme().border;

    match direction {
        SplitDirection::Vertical => div()
            .w(px(8.0))
            .h_full()
            .bg(title_bar)
            .hover(|s| s.bg(list_hover))
            .cursor(CursorStyle::ResizeLeftRight)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event: &MouseDownEvent, _, cx| {
                    this.dragging_splitter = true;
                    this.splitter_drag_start = Some(event.position);
                    cx.notify();
                }),
            )
            .flex()
            .items_center()
            .justify_center()
            .child(div().w(px(2.0)).h(px(40.0)).bg(border).rounded(px(1.0))),
        SplitDirection::Horizontal => div()
            .h(px(8.0))
            .w_full()
            .bg(title_bar)
            .hover(|s| s.bg(list_hover))
            .cursor(CursorStyle::ResizeUpDown)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event: &MouseDownEvent, _, cx| {
                    this.dragging_splitter = true;
                    this.splitter_drag_start = Some(event.position);
                    cx.notify();
                }),
            )
            .flex()
            .items_center()
            .justify_center()
            .child(div().h(px(2.0)).w(px(40.0)).bg(border).rounded(px(1.0))),
    }
}

/// Render selected item label (placeholder)
pub fn render_selected_item_label(_name: String) -> Div {
    div().size_0()
}
