//! Error recovery methods for handling toast actions.
//!
//! This module provides handlers for error recovery actions triggered from
//! toast notifications, such as Retry, Save As, Reset Settings, and Reload.

use crate::notifications::{Toast, ToastActionType};
use gpui::*;

impl super::Humanboard {
    /// Handle a toast action button click
    pub fn handle_toast_action(
        &mut self,
        action_type: ToastActionType,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match action_type {
            ToastActionType::Retry => {
                self.handle_retry_action(cx);
            }
            ToastActionType::SaveAs => {
                self.handle_save_as_action(cx);
            }
            ToastActionType::ResetSettings => {
                self.handle_reset_settings_action(cx);
            }
            ToastActionType::ReloadWebview => {
                self.handle_reload_webview_action(cx);
            }
            ToastActionType::Dismiss => {
                // Just dismiss, no action needed
            }
        }
    }

    /// Handle retry action - attempt to redo the last failed operation
    fn handle_retry_action(&mut self, cx: &mut Context<Self>) {
        // If we have a board, try to save it again
        if let Some(ref mut board) = self.board {
            board.save();
            self.toast_manager
                .push(Toast::success("Board saved successfully"));
            cx.notify();
        }
    }

    /// Handle Save As action - open file dialog to save to alternative location
    fn handle_save_as_action(&mut self, cx: &mut Context<Self>) {
        // TODO: Implement file dialog for Save As
        // For now, show info toast that this feature is coming
        self.toast_manager
            .push(Toast::info("Save As feature coming soon"));
        cx.notify();
    }

    /// Handle reset settings action - restore settings to defaults
    fn handle_reset_settings_action(&mut self, cx: &mut Context<Self>) {
        self.settings = crate::settings::Settings::default();
        self.settings.save();
        self.toast_manager
            .push(Toast::success("Settings reset to defaults"));
        cx.notify();
    }

    /// Handle reload webview action - reload the current webview
    fn handle_reload_webview_action(&mut self, cx: &mut Context<Self>) {
        // Clear webviews to force reload on next render
        self.youtube_webviews.clear();
        self.audio_webviews.clear();
        self.video_webviews.clear();
        self.toast_manager.push(Toast::info("Webviews reloaded"));
        cx.notify();
    }
}
