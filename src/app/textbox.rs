//! Textbox editing and utility methods.

use super::Humanboard;
use crate::focus::FocusContext;
use crate::types::ItemContent;
use gpui::*;
use gpui_component::input::InputState;
use std::time::{Duration, Instant};

impl Humanboard {
    // ==================== TextBox Editing Methods ====================

    /// Start editing a textbox inline on the canvas
    pub fn start_textbox_editing(
        &mut self,
        item_id: u64,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Clear any pending drag/resize state from the first click of the double-click
        self.dragging_item = None;
        self.item_drag_offset = None;
        self.resizing_item = None;
        self.resize_start_size = None;
        self.resize_start_pos = None;
        self.resize_start_font_size = None;

        // Get the current text from the item
        let current_text = if let Some(ref board) = self.board {
            board.get_item(item_id).and_then(|item| {
                if let ItemContent::TextBox { text, .. } = &item.content {
                    Some(text.clone())
                } else {
                    None
                }
            })
        } else {
            None
        };

        if let Some(text) = current_text {
            // Count lines to position cursor at end
            let lines: Vec<&str> = text.lines().collect();
            let last_line = lines.len().saturating_sub(1) as u32;
            let last_char = lines.last().map(|l| l.len() as u32).unwrap_or(0);

            // Create the input with multiline support (code_editor enables multiline)
            let input = cx.new(|cx| {
                InputState::new(window, cx)
                    .code_editor("plaintext") // Enable multiline editing
                    .line_number(false) // No line numbers for textbox
                    .soft_wrap(true) // Wrap long lines
                    .default_value(text)
            });

            // Focus the input and position cursor at end after it's been added to the render tree
            // We need to defer this because the Input isn't mounted yet
            let input_clone = input.clone();
            window.defer(cx, move |window, cx| {
                input_clone.update(cx, |state, cx| {
                    // Position cursor at the end of the text
                    let end_pos = gpui_component::input::Position {
                        line: last_line,
                        character: last_char,
                    };
                    state.set_cursor_position(end_pos, window, cx);
                });
            });

            // Subscribe to input events
            // Note: PressEnter is NOT handled here - Enter creates new lines in multiline mode
            // Use Escape or click outside to finish editing
            cx.subscribe(
                &input,
                |this, input, event: &gpui_component::input::InputEvent, cx| {
                    match event {
                        gpui_component::input::InputEvent::Change { .. } => {
                            // Update text as user types
                            if let Some(item_id) = this.editing_textbox_id {
                                let new_text = input.read(cx).text().to_string();
                                if let Some(ref mut board) = this.board {
                                    if let Some(item) = board.get_item_mut(item_id) {
                                        if let ItemContent::TextBox { text, .. } = &mut item.content
                                        {
                                            *text = new_text;
                                        }
                                    }
                                    board.mark_dirty();
                                }
                            }
                        }
                        gpui_component::input::InputEvent::Blur => {
                            // Save on blur (click outside)
                            this.finish_textbox_editing(cx);
                        }
                        _ => {}
                    }
                },
            )
            .detach();

            // Set focus context to TextboxEditing so keyboard shortcuts don't intercept input
            self.focus
                .set_context_without_focus(FocusContext::TextboxEditing);

            self.editing_textbox_id = Some(item_id);
            self.textbox_input = Some(input);
            cx.notify();
        }
    }

    /// Finish editing and save the textbox content
    pub fn finish_textbox_editing(&mut self, cx: &mut Context<Self>) {
        if let Some(item_id) = self.editing_textbox_id.take() {
            if let Some(input) = self.textbox_input.take() {
                let new_text = input.read(cx).text().to_string();

                if let Some(ref mut board) = self.board {
                    if let Some(item) = board.get_item_mut(item_id) {
                        if let ItemContent::TextBox { text, .. } = &mut item.content {
                            *text = new_text;
                        }
                    }
                    board.push_history();
                    if let Err(e) = board.flush_save() {
                        self.toast_manager
                            .push(crate::notifications::Toast::error(format!(
                                "Save failed: {}",
                                e
                            )).with_action(crate::notifications::ToastAction::retry()));
                    }
                }
            }
        }

        // Release focus back to canvas (mark for restore since we don't have window)
        self.focus.mark_needs_canvas_focus();
        cx.notify();
    }

    /// Finish editing with explicit window access (preferred when window available)
    pub fn finish_textbox_editing_with_window(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(item_id) = self.editing_textbox_id.take() {
            if let Some(input) = self.textbox_input.take() {
                let new_text = input.read(cx).text().to_string();

                if let Some(ref mut board) = self.board {
                    if let Some(item) = board.get_item_mut(item_id) {
                        if let ItemContent::TextBox { text, .. } = &mut item.content {
                            *text = new_text;
                        }
                    }
                    board.push_history();
                    if let Err(e) = board.flush_save() {
                        self.toast_manager
                            .push(crate::notifications::Toast::error(format!(
                                "Save failed: {}",
                                e
                            )).with_action(crate::notifications::ToastAction::retry()));
                    }
                }
            }
        }

        // Release focus back to canvas
        self.focus.release(FocusContext::TextboxEditing, window);
        cx.notify();
    }

    /// Cancel textbox editing without saving
    pub fn cancel_textbox_editing(&mut self, cx: &mut Context<Self>) {
        self.editing_textbox_id = None;
        self.textbox_input = None;

        // Release focus back to canvas (mark for restore since we don't have window)
        self.focus.mark_needs_canvas_focus();
        cx.notify();
    }

    /// Cancel textbox editing with explicit window access (preferred when window available)
    pub fn cancel_textbox_editing_with_window(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editing_textbox_id = None;
        self.textbox_input = None;

        // Release focus back to canvas
        self.focus.release(FocusContext::TextboxEditing, window);
        cx.notify();
    }

    // ==================== Utility Methods ====================

    pub fn update_fps(&mut self) {
        let now = Instant::now();
        let frame_time = now.duration_since(self.last_frame);
        self.last_frame = now;
        self.frame_count += 1;

        self.frame_times.push(frame_time);
        if self.frame_times.len() > 60 {
            self.frame_times.remove(0);
        }
    }

    pub fn calculate_fps(&self) -> f32 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        let avg_frame_time: Duration =
            self.frame_times.iter().sum::<Duration>() / self.frame_times.len() as u32;
        if avg_frame_time.as_secs_f32() > 0.0 {
            1.0 / avg_frame_time.as_secs_f32()
        } else {
            0.0
        }
    }

    pub fn pdf_zoom_in(&mut self, _cx: &mut Context<Self>) {
        // WebView handles PDF zoom internally
    }

    pub fn pdf_zoom_out(&mut self, _cx: &mut Context<Self>) {
        // WebView handles PDF zoom internally
    }

    pub fn pdf_zoom_reset(&mut self, _cx: &mut Context<Self>) {
        // WebView handles PDF zoom internally
    }
}
