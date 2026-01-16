//! Board management methods - create, open, edit, delete, trash operations

use super::{AppView, Humanboard, StorageLocation};
use crate::board::Board;
use crate::board_index::BoardIndex;
use crate::focus::FocusContext;
use gpui::*;
use gpui_component::input::InputState;

impl Humanboard {
    // ==================== Landing Page Methods ====================

    /// Show the create board modal with input field
    pub fn show_create_board_modal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.focus.focus(FocusContext::Modal, window);
        self.reset_modal_focus(); // Reset focus index for Tab cycling

        let input = cx.new(|cx| InputState::new(window, cx).placeholder("Enter board name..."));

        // Focus the input
        input.update(cx, |state, cx| {
            state.focus(window, cx);
        });

        self.create_board_input = Some(input);
        self.create_board_location = StorageLocation::default();
        self.show_create_board_modal = true;
        self.modal_animations.open_create_board();
        cx.notify();
    }

    /// Close the create board modal without creating
    pub fn close_create_board_modal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Start close animation - modal will be hidden when animation completes
        self.modal_animations.close_create_board();
        self.focus.release(FocusContext::Modal, window);
        cx.notify();
    }

    /// Clean up create board modal after close animation completes
    pub fn finish_close_create_board(&mut self) {
        self.show_create_board_modal = false;
        self.create_board_input = None;
        self.create_board_location = StorageLocation::default();
    }

    /// Set the storage location for the new board
    pub fn set_create_board_location(&mut self, location: StorageLocation, cx: &mut Context<Self>) {
        self.create_board_location = location;
        cx.notify();
    }

    /// Create a new board with custom name and storage location
    pub fn confirm_create_board(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let name = self
            .create_board_input
            .as_ref()
            .map(|input| input.read(cx).text().to_string())
            .unwrap_or_default();

        let name = if name.trim().is_empty() {
            "Untitled Board".to_string()
        } else {
            name.trim().to_string()
        };

        let location = std::mem::take(&mut self.create_board_location);
        let location_name = location.display_name().to_owned();

        // Create the board with custom location
        let metadata = self.board_index.create_board_at(name, location);

        // Close modal immediately (no animation when confirming - we're navigating away)
        self.show_create_board_modal = false;
        self.create_board_input = None;
        self.modal_animations.create_board = None;
        self.focus.release(FocusContext::Modal, window);
        self.toast_manager
            .push(crate::notifications::Toast::success(format!(
                "Board created in {}",
                location_name
            )));

        // Open the new board
        self.open_board(metadata.id, cx);
    }

    /// Quick create (backwards compatible) - creates with default name and location
    pub fn create_new_board(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Show modal instead of directly creating
        self.show_create_board_modal(window, cx);
    }

    pub fn open_board(&mut self, id: String, cx: &mut Context<Self>) {
        self.board_index.touch_board(&id);
        let board = Board::load(id.clone());
        self.board = Some(board);
        self.view = AppView::Board(id);
        cx.notify();
    }

    pub fn go_home(&mut self, cx: &mut Context<Self>) {
        // Force save current board before leaving
        if let Some(ref mut board) = self.board {
            if let Err(e) = board.flush_save() {
                self.toast_manager
                    .push(crate::notifications::Toast::error(format!(
                        "Save failed: {}",
                        e
                    )).with_action(crate::notifications::ToastAction::retry()));
            }
        }
        self.board = None;
        // Clean up preview panel resources before dropping
        if let Some(ref mut preview) = self.preview {
            preview.cleanup(cx);
        }
        self.preview = None;
        self.youtube_webviews.clear(); // Clear YouTube WebViews when leaving board
        self.audio_webviews.clear(); // Clear Audio WebViews when leaving board
        self.video_webviews.clear(); // Clear Video WebViews when leaving board
        self.view = AppView::Landing;
        self.selected_items.clear();
        // Reload index to get any changes
        self.board_index = BoardIndex::load();
        cx.notify();
    }

    pub fn start_editing_board(&mut self, id: String, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(meta) = self.board_index.get_board(&id) {
            // Set focus context to Landing for input
            self.focus.focus(FocusContext::Landing, window);

            let name = meta.name.clone();
            let input = cx.new(|cx| InputState::new(window, cx).default_value(name));
            // Focus the input so user can type immediately
            input.update(cx, |state, cx| {
                state.focus(window, cx);
            });
            self.edit_input = Some(input);
            self.editing_board_id = Some(id);
            cx.notify();
        }
    }

    pub fn finish_editing_board(&mut self, cx: &mut Context<Self>) {
        if let Some(ref id) = self.editing_board_id.clone() {
            if let Some(ref input) = self.edit_input {
                let new_name = input.read(cx).value().to_string();
                if !new_name.trim().is_empty() {
                    self.board_index.rename_board(id, new_name);
                }
            }
        }
        self.editing_board_id = None;
        self.edit_input = None;
        cx.notify();
    }

    pub fn cancel_editing(&mut self, cx: &mut Context<Self>) {
        self.editing_board_id = None;
        self.edit_input = None;
        cx.notify();
    }

    pub fn confirm_delete_board(&mut self, id: impl Into<String>, cx: &mut Context<Self>) {
        self.deleting_board_id = Some(id.into());
        cx.notify();
    }

    /// Soft delete - moves to trash
    pub fn delete_board(&mut self, id: &str, cx: &mut Context<Self>) {
        self.board_index.delete_board(id);
        self.deleting_board_id = None;
        self.toast_manager
            .push(crate::notifications::Toast::info("Board moved to trash"));
        cx.notify();
    }

    pub fn cancel_delete(&mut self, cx: &mut Context<Self>) {
        self.deleting_board_id = None;
        cx.notify();
    }

    /// Restore a board from trash
    pub fn restore_board(&mut self, id: &str, cx: &mut Context<Self>) {
        if self.board_index.restore_board(id) {
            self.toast_manager
                .push(crate::notifications::Toast::success("Board restored"));
        }
        cx.notify();
    }

    /// Permanently delete a board (no recovery)
    pub fn permanently_delete_board(&mut self, id: &str, cx: &mut Context<Self>) {
        if self.board_index.permanently_delete_board(id) {
            self.toast_manager.push(crate::notifications::Toast::info(
                "Board permanently deleted",
            ));
        }
        cx.notify();
    }

    /// Empty all boards from trash
    pub fn empty_trash(&mut self, cx: &mut Context<Self>) {
        let count = self.board_index.empty_trash();
        if count > 0 {
            self.toast_manager
                .push(crate::notifications::Toast::info(format!(
                    "Permanently deleted {} board(s)",
                    count
                )));
        }
        // Hide trash section if empty
        if self.board_index.trashed_boards().is_empty() {
            self.show_trash = false;
        }
        cx.notify();
    }

    /// Toggle trash section visibility
    pub fn toggle_trash(&mut self, cx: &mut Context<Self>) {
        // Don't toggle if a modal is open
        if self.show_settings || self.show_create_board_modal {
            return;
        }
        self.show_trash = !self.show_trash;
        cx.notify();
    }

    // ==================== Onboarding Methods ====================

    /// Complete onboarding and transition to landing page
    pub fn complete_onboarding(&mut self, cx: &mut Context<Self>) {
        // Mark onboarding as completed in settings
        if let Err(e) = crate::settings::mark_onboarding_completed() {
            tracing::error!("Failed to mark onboarding completed: {}", e);
        }

        // Transition to landing page
        self.view = AppView::Landing;
        cx.notify();
    }
}
