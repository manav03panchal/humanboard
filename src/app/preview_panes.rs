//! Pane management - tab switching, split panes, focus management.

use super::{FocusedPane, Humanboard, PreviewTab, SplitDirection};
use gpui::*;

impl Humanboard {
    pub fn switch_tab(&mut self, tab_index: usize, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if tab_index < preview.tabs.len() && tab_index != preview.active_tab {
                // Record in back stack for navigation history
                preview.back_stack.push(preview.active_tab);
                // Clear forward stack when user manually switches tabs
                preview.forward_stack.clear();

                // Hide/show PDF webviews based on active tab
                for (idx, tab) in preview.tabs.iter().enumerate() {
                    if let PreviewTab::Pdf {
                        webview: Some(wv), ..
                    } = tab
                    {
                        if idx == tab_index {
                            wv.webview().update(cx, |view, _| view.show());
                        } else {
                            wv.webview().update(cx, |view, _| view.hide());
                        }
                    }
                }

                preview.active_tab = tab_index;
                cx.notify();
            }
        }
    }

    /// Switch to a specific tab in the specified pane (is_left_pane determines which pane)
    pub fn switch_tab_in_pane(&mut self, tab_index: usize, is_left_pane: bool, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if is_left_pane {
                // Switch in left pane
                if tab_index < preview.tabs.len() && tab_index != preview.active_tab {
                    preview.back_stack.push(preview.active_tab);
                    preview.forward_stack.clear();

                    // Hide/show PDF webviews based on active tab
                    for (idx, tab) in preview.tabs.iter().enumerate() {
                        if let PreviewTab::Pdf {
                            webview: Some(wv), ..
                        } = tab
                        {
                            if idx == tab_index {
                                wv.webview().update(cx, |view, _| view.show());
                            } else {
                                wv.webview().update(cx, |view, _| view.hide());
                            }
                        }
                    }

                    preview.active_tab = tab_index;
                    preview.focused_pane = FocusedPane::Left;
                    cx.notify();
                }
            } else {
                // Switch in right pane
                if tab_index < preview.right_tabs.len() && tab_index != preview.right_active_tab {
                    preview.right_back_stack.push(preview.right_active_tab);
                    preview.right_forward_stack.clear();

                    // Hide/show PDF webviews based on active tab
                    for (idx, tab) in preview.right_tabs.iter().enumerate() {
                        if let PreviewTab::Pdf {
                            webview: Some(wv), ..
                        } = tab
                        {
                            if idx == tab_index {
                                wv.webview().update(cx, |view, _| view.show());
                            } else {
                                wv.webview().update(cx, |view, _| view.hide());
                            }
                        }
                    }

                    preview.right_active_tab = tab_index;
                    preview.focused_pane = FocusedPane::Right;
                    cx.notify();
                }
            }
        }
    }

    /// Make a tab permanent (convert from preview tab) in the specified pane
    pub fn make_tab_permanent_in_pane(&mut self, tab_index: usize, is_left_pane: bool, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if is_left_pane {
                if let Some(tab) = preview.tabs.get_mut(tab_index) {
                    tab.make_permanent();
                    cx.notify();
                }
            } else {
                if let Some(tab) = preview.right_tabs.get_mut(tab_index) {
                    tab.make_permanent();
                    cx.notify();
                }
            }
        }
    }

    /// Close a tab in the specified pane
    pub fn close_tab_in_pane(&mut self, tab_index: usize, is_left_pane: bool, cx: &mut Context<Self>) {
        let mut should_close_preview = false;

        if let Some(ref mut preview) = self.preview {
            if is_left_pane {
                // Close from left pane
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
                        // Left pane empty - merge right into left if split exists
                        if preview.is_pane_split && !preview.right_tabs.is_empty() {
                            preview.tabs = std::mem::take(&mut preview.right_tabs);
                            preview.active_tab = preview.right_active_tab;
                            preview.back_stack = std::mem::take(&mut preview.right_back_stack);
                            preview.forward_stack = std::mem::take(&mut preview.right_forward_stack);
                            preview.is_pane_split = false;
                            preview.focused_pane = FocusedPane::Left;
                        } else {
                            // No tabs left - close the preview panel entirely
                            should_close_preview = true;
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
            } else {
                // Close from right pane
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
            }
        }

        // Close preview if all tabs are gone (after the borrow is released)
        if should_close_preview {
            self.preview = None;
            cx.notify();
        }
    }

    /// Toggle tab pinned state in the specified pane
    pub fn toggle_tab_pinned_in_pane(&mut self, tab_index: usize, is_left_pane: bool, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            let tabs = if is_left_pane {
                &mut preview.tabs
            } else {
                &mut preview.right_tabs
            };
            let active_tab = if is_left_pane {
                &mut preview.active_tab
            } else {
                &mut preview.right_active_tab
            };

            if let Some(tab) = tabs.get_mut(tab_index) {
                let was_pinned = tab.is_pinned();
                tab.toggle_pinned();

                // If we just pinned, move to end of pinned section
                if !was_pinned && tab.is_pinned() {
                    let pinned_count = tabs.iter().filter(|t| t.is_pinned()).count();
                    if tab_index + 1 < pinned_count {
                        // Already in correct position
                    } else if pinned_count > 0 && tab_index >= pinned_count {
                        // Move to end of pinned section
                        let tab = tabs.remove(tab_index);
                        let insert_pos = pinned_count - 1;
                        tabs.insert(insert_pos, tab);

                        // Adjust active_tab
                        if tab_index == *active_tab {
                            *active_tab = insert_pos;
                        } else if tab_index > *active_tab && insert_pos <= *active_tab {
                            *active_tab += 1;
                        } else if tab_index < *active_tab && insert_pos >= *active_tab {
                            *active_tab -= 1;
                        }
                    }
                }
                cx.notify();
            }
        }
    }

    /// Start tab drag in the specified pane (sets pending state, actual drag starts after threshold)
    pub fn start_tab_drag_in_pane(&mut self, tab_index: usize, position: Point<Pixels>, is_left_pane: bool, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            // Set the focused pane to the one being dragged from
            preview.focused_pane = if is_left_pane { FocusedPane::Left } else { FocusedPane::Right };
        }
        // Set pending drag - actual drag starts after mouse moves beyond threshold
        self.tab_drag_pending = Some((tab_index, position, is_left_pane));
        // Don't set dragging_tab yet - wait for threshold
        cx.notify();
    }

    /// Cancel pending drag (called on mouse up if threshold wasn't reached)
    pub fn cancel_pending_drag(&mut self, cx: &mut Context<Self>) {
        self.tab_drag_pending = None;
        cx.notify();
    }

    pub fn next_tab(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if !preview.tabs.is_empty() {
                preview.active_tab = (preview.active_tab + 1) % preview.tabs.len();
                cx.notify();
            }
        }
    }

    pub fn prev_tab(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if !preview.tabs.is_empty() {
                preview.active_tab = if preview.active_tab == 0 {
                    preview.tabs.len() - 1
                } else {
                    preview.active_tab - 1
                };
                cx.notify();
            }
        }
    }

    pub fn close_current_tab(&mut self, cx: &mut Context<Self>) {
        if let Some(ref preview) = self.preview {
            let active = if preview.is_pane_split && preview.focused_pane == FocusedPane::Right {
                preview.right_active_tab
            } else {
                preview.active_tab
            };
            self.close_tab(active, cx);
        }
    }

    pub fn toggle_split_direction(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            preview.split = match preview.split {
                SplitDirection::Vertical => SplitDirection::Horizontal,
                SplitDirection::Horizontal => SplitDirection::Vertical,
            };
            cx.notify();
        }
    }

    /// Split the preview panel into two panes
    pub fn split_pane(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if !preview.is_pane_split {
                preview.is_pane_split = true;
                preview.pane_ratio = 0.5;
                // Focus stays on left pane
                cx.notify();
            }
        }
    }

    /// Close the split and merge into single pane
    pub fn close_split_pane(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if preview.is_pane_split {
                // Move right pane tabs to left pane
                let mut right_tabs: Vec<PreviewTab> = preview.right_tabs.drain(..).collect();
                for tab in &mut right_tabs {
                    // Hide PDF webviews before move - they will be recreated in new position
                    if let PreviewTab::Pdf { webview, .. } = tab {
                        if let Some(wv) = webview.take() {
                            wv.hide(cx);
                        }
                    }
                }
                for tab in right_tabs {
                    preview.tabs.push(tab);
                }
                preview.right_active_tab = 0;
                preview.right_back_stack.clear();
                preview.right_forward_stack.clear();
                preview.is_pane_split = false;
                preview.focused_pane = FocusedPane::Left;
                cx.notify();
            }
        }
    }

    /// Toggle split pane on/off
    pub fn toggle_pane_split(&mut self, cx: &mut Context<Self>) {
        if let Some(ref preview) = self.preview {
            if preview.is_pane_split {
                self.close_split_pane(cx);
            } else {
                self.split_pane(cx);
            }
        }
    }

    /// Focus the left pane
    pub fn focus_left_pane(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            preview.focused_pane = FocusedPane::Left;
            cx.notify();
        }
    }

    /// Focus the right pane
    pub fn focus_right_pane(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if preview.is_pane_split {
                preview.focused_pane = FocusedPane::Right;
                cx.notify();
            }
        }
    }

    /// Move current tab to the other pane
    pub fn move_tab_to_other_pane(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if !preview.is_pane_split {
                // Auto-split when moving tab
                preview.is_pane_split = true;
                preview.pane_ratio = 0.5;
            }

            match preview.focused_pane {
                FocusedPane::Left => {
                    if !preview.tabs.is_empty() && preview.active_tab < preview.tabs.len() {
                        let mut tab = preview.tabs.remove(preview.active_tab);
                        // Hide PDF webview before move - it will be recreated in new pane
                        if let PreviewTab::Pdf { webview, .. } = &mut tab {
                            if let Some(wv) = webview.take() {
                                wv.hide(cx);
                            }
                        }
                        preview.right_tabs.push(tab);
                        preview.right_active_tab = preview.right_tabs.len() - 1;
                        // Adjust left active tab
                        if preview.active_tab >= preview.tabs.len() && !preview.tabs.is_empty() {
                            preview.active_tab = preview.tabs.len() - 1;
                        }
                        // Focus the right pane where we moved the tab
                        preview.focused_pane = FocusedPane::Right;
                    }
                }
                FocusedPane::Right => {
                    if !preview.right_tabs.is_empty()
                        && preview.right_active_tab < preview.right_tabs.len()
                    {
                        let mut tab = preview.right_tabs.remove(preview.right_active_tab);
                        // Hide PDF webview before move - it will be recreated in new pane
                        if let PreviewTab::Pdf { webview, .. } = &mut tab {
                            if let Some(wv) = webview.take() {
                                wv.hide(cx);
                            }
                        }
                        preview.tabs.push(tab);
                        preview.active_tab = preview.tabs.len() - 1;
                        // Adjust right active tab
                        if preview.right_active_tab >= preview.right_tabs.len()
                            && !preview.right_tabs.is_empty()
                        {
                            preview.right_active_tab = preview.right_tabs.len() - 1;
                        }
                        // Focus the left pane where we moved the tab
                        preview.focused_pane = FocusedPane::Left;
                    }
                }
            }

            // If right pane is empty after move, close split
            if preview.right_tabs.is_empty() {
                preview.is_pane_split = false;
                preview.focused_pane = FocusedPane::Left;
            }

            cx.notify();
        }
    }

    pub fn next_page(&mut self, _cx: &mut Context<Self>) {
        // WebView handles PDF navigation internally
    }

    pub fn prev_page(&mut self, _cx: &mut Context<Self>) {
        // WebView handles PDF navigation internally
    }
}
