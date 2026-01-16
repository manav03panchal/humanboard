//! Tab management - close, reopen, navigation history, and drag/drop reordering.

use super::{FocusedPane, Humanboard, PreviewTab, SplitDropZone};
use gpui::*;

impl Humanboard {
    pub fn close_preview(&mut self, cx: &mut Context<Self>) {
        // Clean up all webview resources before dropping the preview panel
        if let Some(ref mut preview) = self.preview {
            preview.cleanup(cx);
        }
        self.preview = None;
        cx.notify();
    }

    pub fn close_tab(&mut self, tab_index: usize, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            // Close tab in the focused pane
            if preview.is_pane_split && preview.focused_pane == FocusedPane::Right {
                // Closing from right pane
                if tab_index < preview.right_tabs.len() {
                    if preview.right_tabs[tab_index].is_pinned() {
                        return;
                    }

                    let mut closed_tab = preview.right_tabs.remove(tab_index);
                    // Immediately cleanup (hide PDF webviews, etc.) before storing
                    closed_tab.cleanup(cx);
                    preview.closed_tabs.push(closed_tab);
                    if preview.closed_tabs.len() > 20 {
                        preview.closed_tabs.remove(0);
                    }

                    if preview.right_tabs.is_empty() {
                        // Right pane empty, close the split
                        preview.is_pane_split = false;
                        preview.focused_pane = FocusedPane::Left;
                    } else {
                        if preview.right_active_tab >= preview.right_tabs.len() {
                            preview.right_active_tab = preview.right_tabs.len() - 1;
                        } else if tab_index < preview.right_active_tab {
                            preview.right_active_tab -= 1;
                        }
                    }
                    cx.notify();
                }
            } else {
                // Closing from left pane
                if tab_index < preview.tabs.len() {
                    if preview.tabs[tab_index].is_pinned() {
                        return;
                    }

                    let mut closed_tab = preview.tabs.remove(tab_index);
                    // Immediately cleanup (hide PDF webviews, etc.) before storing
                    closed_tab.cleanup(cx);
                    preview.closed_tabs.push(closed_tab);
                    if preview.closed_tabs.len() > 20 {
                        preview.closed_tabs.remove(0);
                    }

                    if preview.tabs.is_empty() {
                        if preview.is_pane_split && !preview.right_tabs.is_empty() {
                            // Left pane empty but right has tabs - move right to left
                            preview.tabs = preview.right_tabs.drain(..).collect();
                            preview.active_tab = preview.right_active_tab;
                            preview.is_pane_split = false;
                            preview.focused_pane = FocusedPane::Left;
                        } else {
                            // No tabs anywhere, clean up and close preview panel
                            preview.cleanup(cx);
                            self.preview = None;
                        }
                    } else {
                        if preview.active_tab >= preview.tabs.len() {
                            preview.active_tab = preview.tabs.len() - 1;
                        } else if tab_index < preview.active_tab {
                            preview.active_tab -= 1;
                        }
                    }
                    cx.notify();
                }
            }
        }
    }

    /// Convert a preview tab to a permanent tab
    pub fn make_tab_permanent(&mut self, tab_index: usize, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if let Some(tab) = preview.tabs.get_mut(tab_index) {
                tab.make_permanent();
                cx.notify();
            }
        }
    }

    /// Toggle the pinned state of a tab
    pub fn toggle_tab_pinned(&mut self, tab_index: usize, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if let Some(tab) = preview.tabs.get_mut(tab_index) {
                let was_pinned = tab.is_pinned();
                tab.toggle_pinned();

                // Reorder tabs: pinned tabs stay at the left
                if !was_pinned && tab.is_pinned() {
                    // Tab was just pinned - move it to after the last pinned tab
                    let pinned_count = preview.tabs.iter().filter(|t| t.is_pinned()).count();
                    if tab_index >= pinned_count {
                        let tab = preview.tabs.remove(tab_index);
                        preview.tabs.insert(pinned_count - 1, tab);
                        // Adjust active tab index
                        if preview.active_tab == tab_index {
                            preview.active_tab = pinned_count - 1;
                        } else if preview.active_tab < tab_index
                            && preview.active_tab >= pinned_count - 1
                        {
                            preview.active_tab += 1;
                        }
                    }
                }
                cx.notify();
            }
        }
    }

    /// Reopen the most recently closed tab
    pub fn reopen_closed_tab(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if let Some(tab) = preview.closed_tabs.pop() {
                preview.tabs.push(tab);
                preview.active_tab = preview.tabs.len() - 1;
                cx.notify();
            }
        }
    }

    /// Navigate back in tab history
    pub fn go_back(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if let Some(prev_index) = preview.back_stack.pop() {
                if prev_index < preview.tabs.len() {
                    // Push current to forward stack
                    preview.forward_stack.push(preview.active_tab);
                    preview.active_tab = prev_index;
                    cx.notify();
                }
            }
        }
    }

    /// Navigate forward in tab history
    pub fn go_forward(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if let Some(next_index) = preview.forward_stack.pop() {
                if next_index < preview.tabs.len() {
                    // Push current to back stack
                    preview.back_stack.push(preview.active_tab);
                    preview.active_tab = next_index;
                    cx.notify();
                }
            }
        }
    }

    /// Start dragging a tab for reordering
    pub fn start_tab_drag(
        &mut self,
        tab_index: usize,
        position: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        tracing::debug!("start_tab_drag: index={}", tab_index);
        self.dragging_tab = Some(tab_index);
        self.tab_drag_target = Some(tab_index);
        self.tab_drag_split_zone = None;
        self.tab_drag_position = Some(position);
        cx.notify();
    }

    /// Update drag position as mouse moves
    /// If there's a pending drag, check if threshold is reached and promote to actual drag
    pub fn update_tab_drag_position(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        // Check if we should promote pending drag to actual drag
        if let Some((tab_index, start_pos, _is_left_pane)) = self.tab_drag_pending {
            let dx = (f32::from(position.x) - f32::from(start_pos.x)).abs();
            let dy = (f32::from(position.y) - f32::from(start_pos.y)).abs();
            let distance = (dx * dx + dy * dy).sqrt();

            // Threshold of 5px before starting actual drag
            if distance > 5.0 {
                // Promote to actual drag
                self.dragging_tab = Some(tab_index);
                self.tab_drag_target = Some(tab_index);
                self.tab_drag_split_zone = None;
                self.tab_drag_position = Some(position);
                self.tab_drag_pending = None;
                tracing::debug!("Promoted pending drag to actual drag: index={}", tab_index);
                cx.notify();
                return;
            }
        }

        // Update position for active drag
        if self.dragging_tab.is_some() {
            self.tab_drag_position = Some(position);
            cx.notify();
        }
    }

    /// Update the drag target position as mouse moves over tabs
    pub fn update_tab_drag_target(&mut self, target_index: usize, cx: &mut Context<Self>) {
        if self.dragging_tab.is_some() {
            // Clear split zone when hovering over tabs
            if self.tab_drag_split_zone.is_some() {
                self.tab_drag_split_zone = None;
            }
            if self.tab_drag_target != Some(target_index) {
                self.tab_drag_target = Some(target_index);
            }
            cx.notify();
        }
    }

    /// Set the split drop zone when dragging to edges
    pub fn set_tab_drag_split_zone(&mut self, zone: Option<SplitDropZone>, cx: &mut Context<Self>) {
        if self.dragging_tab.is_some() && self.tab_drag_split_zone != zone {
            tracing::debug!("set_tab_drag_split_zone: {:?}", zone);
            self.tab_drag_split_zone = zone;
            // Clear tab target when in split zone
            if zone.is_some() {
                self.tab_drag_target = None;
            }
            cx.notify();
        }
    }

    /// Finish tab drag - either reorder or create split
    pub fn finish_tab_drag(&mut self, cx: &mut Context<Self>) {
        tracing::debug!(
            "finish_tab_drag: dragging={:?}, split_zone={:?}, target={:?}",
            self.dragging_tab,
            self.tab_drag_split_zone,
            self.tab_drag_target
        );
        // Check if dropping on a split zone
        if let (Some(from), Some(zone)) = (self.dragging_tab, self.tab_drag_split_zone) {
            if let Some(ref mut preview) = self.preview {
                // Determine which pane we're dragging from
                let from_right_pane =
                    preview.is_pane_split && preview.focused_pane == FocusedPane::Right;

                // Get the source tabs list
                let (source_tabs, source_active) = if from_right_pane {
                    (&mut preview.right_tabs, &mut preview.right_active_tab)
                } else {
                    (&mut preview.tabs, &mut preview.active_tab)
                };

                if from < source_tabs.len() {
                    let tab = source_tabs.remove(from);

                    // Update source active tab
                    if *source_active >= source_tabs.len() {
                        *source_active = source_tabs.len().saturating_sub(1);
                    } else if *source_active > from {
                        *source_active -= 1;
                    }

                    // Hide webview before move
                    if let PreviewTab::Pdf {
                        webview: Some(wv), ..
                    } = &tab
                    {
                        wv.hide(cx);
                    }

                    // Determine target based on zone
                    let target_is_right =
                        matches!(zone, SplitDropZone::Right | SplitDropZone::Bottom);

                    if !preview.is_pane_split {
                        // Create new split
                        preview.is_pane_split = true;
                        preview.pane_ratio = 0.5;
                        preview.pane_split_horizontal =
                            matches!(zone, SplitDropZone::Top | SplitDropZone::Bottom);

                        if target_is_right {
                            preview.right_tabs.push(tab);
                            preview.right_active_tab = preview.right_tabs.len() - 1;
                            preview.focused_pane = FocusedPane::Right;
                        } else {
                            // Move existing tabs to right, put dragged tab in left
                            let left_tabs: Vec<PreviewTab> = preview.tabs.drain(..).collect();
                            preview.tabs.push(tab);
                            preview.active_tab = 0;
                            for t in left_tabs {
                                preview.right_tabs.push(t);
                            }
                            preview.right_active_tab = 0;
                            preview.focused_pane = FocusedPane::Left;
                        }
                    } else {
                        // Already split - move tab to target pane
                        if target_is_right {
                            preview.right_tabs.push(tab);
                            preview.right_active_tab = preview.right_tabs.len() - 1;
                            preview.focused_pane = FocusedPane::Right;
                        } else {
                            preview.tabs.push(tab);
                            preview.active_tab = preview.tabs.len() - 1;
                            preview.focused_pane = FocusedPane::Left;
                        }

                        // Check if source pane is now empty - close split
                        if from_right_pane && preview.right_tabs.is_empty() {
                            preview.is_pane_split = false;
                            preview.focused_pane = FocusedPane::Left;
                        } else if !from_right_pane && preview.tabs.is_empty() {
                            // Move right tabs to left
                            preview.tabs = preview.right_tabs.drain(..).collect();
                            preview.active_tab = preview.right_active_tab;
                            preview.is_pane_split = false;
                            preview.focused_pane = FocusedPane::Left;
                        }
                    }
                }
            }
            self.dragging_tab = None;
            self.tab_drag_target = None;
            self.tab_drag_split_zone = None;
            self.tab_drag_position = None;
            cx.notify();
            return;
        }

        // Normal tab reorder
        if let (Some(from), Some(to)) = (self.dragging_tab, self.tab_drag_target) {
            if from != to {
                if let Some(ref mut preview) = self.preview {
                    // Don't allow moving tabs before pinned tabs (unless the dragged tab is pinned)
                    let pinned_count = preview.tabs.iter().filter(|t| t.is_pinned()).count();
                    let is_dragged_pinned = preview
                        .tabs
                        .get(from)
                        .map(|t| t.is_pinned())
                        .unwrap_or(false);

                    let effective_to = if !is_dragged_pinned && to < pinned_count {
                        pinned_count // Can't move before pinned tabs
                    } else {
                        to
                    };

                    if from != effective_to && from < preview.tabs.len() {
                        let tab = preview.tabs.remove(from);
                        let insert_pos = if effective_to > from {
                            effective_to.min(preview.tabs.len())
                        } else {
                            effective_to
                        };
                        preview.tabs.insert(insert_pos, tab);

                        // Update active tab index
                        if preview.active_tab == from {
                            preview.active_tab = insert_pos;
                        } else if from < preview.active_tab && insert_pos >= preview.active_tab {
                            preview.active_tab -= 1;
                        } else if from > preview.active_tab && insert_pos <= preview.active_tab {
                            preview.active_tab += 1;
                        }
                    }
                }
            }
        }
        self.dragging_tab = None;
        self.tab_drag_target = None;
        self.tab_drag_split_zone = None;
        self.tab_drag_position = None;
        cx.notify();
    }

    /// Cancel tab drag without reordering
    pub fn cancel_tab_drag(&mut self, cx: &mut Context<Self>) {
        self.dragging_tab = None;
        self.tab_drag_target = None;
        self.tab_drag_split_zone = None;
        self.tab_drag_position = None;
        cx.notify();
    }
}
