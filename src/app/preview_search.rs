//! Preview search functionality - find in file.

use super::{Humanboard, PreviewTab};
use gpui::*;
use gpui_component::input::InputState;

impl Humanboard {
    /// Toggle the preview search bar
    pub fn toggle_preview_search(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.preview_search.is_some() {
            self.close_preview_search(cx);
        } else {
            self.open_preview_search(window, cx);
        }
    }

    /// Open the preview search bar
    pub fn open_preview_search(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.preview.is_none() {
            return;
        }

        let input = cx.new(|cx| InputState::new(window, cx).placeholder("Find in file..."));

        // Focus the input
        input.update(cx, |state, cx| {
            state.focus(window, cx);
        });

        // Subscribe to input changes
        cx.subscribe(
            &input,
            |this, input, event: &gpui_component::input::InputEvent, cx| {
                match event {
                    gpui_component::input::InputEvent::Change { .. } => {
                        let query = input.read(cx).text().to_string();
                        this.update_preview_search(&query, cx);
                    }
                    gpui_component::input::InputEvent::PressEnter { .. } => {
                        // Go to next match
                        this.next_search_match(cx);
                    }
                    gpui_component::input::InputEvent::Blur => {
                        // Don't close on blur - let user click away
                    }
                    _ => {}
                }
            },
        )
        .detach();

        self.preview_search = Some(input);
        self.preview_search_query.clear();
        self.preview_search_matches.clear();
        self.preview_search_current = 0;
        cx.notify();
    }

    /// Close the preview search bar
    pub fn close_preview_search(&mut self, cx: &mut Context<Self>) {
        self.preview_search = None;
        self.preview_search_query.clear();
        self.preview_search_matches.clear();
        self.preview_search_current = 0;
        cx.notify();
    }

    /// Update search matches based on query
    fn update_preview_search(&mut self, query: &str, cx: &mut Context<Self>) {
        self.preview_search_query = query.to_string();
        self.preview_search_matches.clear();
        self.preview_search_current = 0;

        if query.is_empty() {
            cx.notify();
            return;
        }

        // Get content from active tab
        let content = if let Some(ref preview) = self.preview {
            if let Some(tab) = preview.tabs.get(preview.active_tab) {
                match tab {
                    PreviewTab::Markdown { content, .. } => Some(content.clone()),
                    PreviewTab::Code { content, .. } => Some(content.clone()),
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        };

        if let Some(content) = content {
            let query_lower = query.to_lowercase();
            for (line_idx, line) in content.lines().enumerate() {
                let line_lower = line.to_lowercase();
                let mut start = 0;
                while let Some(col) = line_lower[start..].find(&query_lower) {
                    self.preview_search_matches.push((line_idx, start + col));
                    start += col + 1;
                }
            }
        }

        cx.notify();
    }

    /// Go to next search match
    pub fn next_search_match(&mut self, cx: &mut Context<Self>) {
        if !self.preview_search_matches.is_empty() {
            self.preview_search_current =
                (self.preview_search_current + 1) % self.preview_search_matches.len();
            cx.notify();
        }
    }

    /// Go to previous search match
    pub fn prev_search_match(&mut self, cx: &mut Context<Self>) {
        if !self.preview_search_matches.is_empty() {
            self.preview_search_current = if self.preview_search_current == 0 {
                self.preview_search_matches.len() - 1
            } else {
                self.preview_search_current - 1
            };
            cx.notify();
        }
    }
}
