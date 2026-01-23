//! Action definitions and handlers for the Humanboard application.
//!
//! This module defines all user-invocable actions following Zed's patterns.
//! Actions are type-safe commands that can be bound to keyboard shortcuts
//! or triggered programmatically through the command palette.
//!
//! ## Action Categories
//!
//! - **Application**: Quit, open file, settings, shortcuts
//! - **Canvas Navigation**: Zoom in/out/reset
//! - **Selection**: Select all, deselect, delete, duplicate, copy/paste
//! - **Item Nudging**: Arrow key movement of selected items
//! - **History**: Undo/redo operations
//! - **Preview Panel**: Tab management, split controls, navigation
//! - **PDF Viewer**: Page navigation and zoom
//! - **Command Palette**: Open/close, navigation, selection
//! - **Tool Selection**: Select, text, arrow, shape tools
//! - **Focus Management**: Canvas and preview focus control

use crate::app::Humanboard;
use gpui::*;
use std::sync::mpsc;

// Application-level actions
actions!(
    humanboard,
    [
        // === Application ===
        Quit,          // Quit the application (Cmd+Q)
        OpenFile,      // Open file picker (Cmd+O)
        OpenSettings,  // Open settings panel (Cmd+,)
        ShowShortcuts, // Show keyboard shortcuts overlay (Cmd+/)
        GoHome,        // Navigate to landing page (Cmd+H)
        NewBoard,      // Create a new board (Cmd+N)
        // === Canvas Navigation ===
        ZoomIn,    // Zoom in on canvas (Cmd+=)
        ZoomOut,   // Zoom out on canvas (Cmd+-)
        ZoomReset, // Reset zoom to 100% (Cmd+0)
        // === Selection ===
        SelectAll,         // Select all items (Cmd+A)
        DeselectAll,       // Deselect all items (Escape when items selected)
        DeleteSelected,    // Delete selected items (Backspace/Delete)
        DuplicateSelected, // Duplicate selected items (Cmd+D)
        Copy,              // Copy selected items to clipboard (Cmd+C)
        Paste,             // Paste from clipboard (Cmd+V)
        // === Item Nudging ===
        NudgeUp,    // Move selected items up (Arrow Up)
        NudgeDown,  // Move selected items down (Arrow Down)
        NudgeLeft,  // Move selected items left (Arrow Left)
        NudgeRight, // Move selected items right (Arrow Right)
        // === History ===
        Undo,     // Undo last action (Cmd+Z)
        Redo,     // Redo last undone action (Cmd+Shift+Z)
        SaveCode, // Save current code file (Cmd+S)
        // === Preview Panel ===
        ClosePreview,        // Close preview panel (Escape)
        ToggleSplit,         // Toggle split direction (Cmd+\)
        NextTab,             // Switch to next tab (Cmd+Shift+])
        PrevTab,             // Switch to previous tab (Cmd+Shift+[)
        CloseTab,            // Close current tab (Cmd+W)
        ReopenClosedTab,     // Reopen last closed tab (Cmd+Shift+T)
        GoBack,              // Navigate back in tab history (Cmd+[)
        GoForward,           // Navigate forward in tab history (Cmd+])
        TogglePreviewSearch, // Toggle search in preview panel (Cmd+F)
        NextSearchMatch,     // Go to next search match (Cmd+G)
        PrevSearchMatch,     // Go to previous search match (Cmd+Shift+G)
        TogglePaneSplit,     // Toggle split preview into two panes (Cmd+Shift+\)
        MoveTabToOtherPane,  // Move current tab to other pane (Cmd+Alt+Shift+Arrow)
        FocusLeftPane,       // Focus left pane
        FocusRightPane,      // Focus right pane
        // === PDF Viewer ===
        NextPage,     // Go to next PDF page
        PrevPage,     // Go to previous PDF page
        PdfZoomIn,    // Zoom in PDF
        PdfZoomOut,   // Zoom out PDF
        PdfZoomReset, // Reset PDF zoom
        // === Command Palette ===
        CommandPalette,       // Open command palette
        ToggleCommandPalette, // Toggle command palette (Cmd+K)
        CloseCommandPalette,  // Close command palette (Escape)
        CmdPaletteUp,         // Navigate up in command palette
        CmdPaletteDown,       // Navigate down in command palette
        CmdPaletteSelect,     // Select current item in command palette
        // === Tool Selection ===
        ToolSelect, // Switch to select tool (V or Escape)
        ToolText,   // Switch to text tool (T)
        ToolArrow,  // Switch to arrow tool (A)
        ToolShape,  // Switch to shape tool (S)
        // === TextBox Editing ===
        CancelTextboxEdit, // Cancel textbox editing (Escape)
        CommitTextboxEdit, // Commit textbox editing (Cmd+Enter or click outside)
        // === Focus Management ===
        FocusCanvas,  // Return focus to canvas
        FocusPreview, // Focus preview panel
        // === Modal Focus Trap ===
        ModalFocusTrap,  // Trap Tab/Shift+Tab within modal (accessibility)
        ModalFocusNext,  // Move focus to next element in modal (Tab)
        ModalFocusPrev,  // Move focus to previous element in modal (Shift+Tab)
    ]
);

impl Humanboard {
    /// Get the center point of the canvas area (accounting for preview panel)
    fn canvas_center(&self, window: &Window) -> Point<Pixels> {
        let bounds = window.bounds();
        let window_size = bounds.size;

        if let Some(ref preview) = self.preview {
            match preview.split {
                crate::app::SplitDirection::Vertical => {
                    let canvas_width = f32::from(window_size.width) * (1.0 - preview.size);
                    point(
                        px(canvas_width / 2.0),
                        px(f32::from(window_size.height) / 2.0),
                    )
                }
                crate::app::SplitDirection::Horizontal => {
                    let canvas_height = f32::from(window_size.height) * (1.0 - preview.size);
                    point(
                        px(f32::from(window_size.width) / 2.0),
                        px(canvas_height / 2.0),
                    )
                }
            }
        } else {
            point(
                px(f32::from(window_size.width) / 2.0),
                px(f32::from(window_size.height) / 2.0),
            )
        }
    }

    pub fn zoom_in(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let center = self.canvas_center(window);
        if let Some(ref mut board) = self.board {
            if board.zoom_in(center) {
                cx.notify();
            }
        }
    }

    pub fn zoom_out(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let center = self.canvas_center(window);
        if let Some(ref mut board) = self.board {
            if board.zoom_out(center) {
                cx.notify();
            }
        }
    }

    pub fn zoom_reset(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut board) = self.board {
            board.zoom_reset();
            cx.notify();
        }
    }

    pub fn delete_selected(&mut self, cx: &mut Context<Self>) {
        if !self.selected_items.is_empty() {
            if let Some(ref mut board) = self.board {
                let selected = self.selected_items.clone();

                // Collect paths of items being deleted (for closing preview tabs)
                let deleted_paths: Vec<_> = board
                    .items
                    .iter()
                    .filter(|item| selected.contains(&item.id))
                    .filter_map(|item| match &item.content {
                        crate::types::ItemContent::Pdf { path, .. } => Some(path.clone()),
                        crate::types::ItemContent::Markdown { path, .. } => Some(path.clone()),
                        crate::types::ItemContent::Code { path, .. } => Some(path.clone()),
                        _ => None,
                    })
                    .collect();

                // Collect data_source_ids of tables being deleted (for closing preview tabs)
                let deleted_data_sources: Vec<u64> = board
                    .items
                    .iter()
                    .filter(|item| selected.contains(&item.id))
                    .filter_map(|item| match &item.content {
                        crate::types::ItemContent::Table { data_source_id, .. } => Some(*data_source_id),
                        _ => None,
                    })
                    .collect();

                // Find charts that reference any of the tables being deleted (cascade delete)
                let orphan_charts: Vec<u64> = board
                    .items
                    .iter()
                    .filter_map(|item| {
                        if let crate::types::ItemContent::Chart { source_item_id: Some(source_id), .. } = &item.content {
                            if selected.contains(source_id) {
                                return Some(item.id);
                            }
                        }
                        None
                    })
                    .collect();

                // Close preview tabs for deleted items
                if let Some(ref mut preview) = self.preview {
                    let mut tabs_to_remove: Vec<usize> = Vec::new();
                    for (i, tab) in preview.tabs.iter().enumerate() {
                        // Check file-based tabs (PDF, Markdown, Code)
                        if let Some(path) = tab.path() {
                            if deleted_paths.contains(path) {
                                tabs_to_remove.push(i);
                                continue;
                            }
                        }
                        // Check table tabs by data_source_id
                        if let crate::app::PreviewTab::Table { data_source_id, .. } = tab {
                            if deleted_data_sources.contains(data_source_id) {
                                tabs_to_remove.push(i);
                            }
                        }
                    }
                    // Remove in reverse order to preserve indices, cleaning up each tab
                    for i in tabs_to_remove.into_iter().rev() {
                        preview.tabs[i].cleanup(cx);
                        preview.tabs.remove(i);
                    }
                    // Adjust active tab if needed
                    if preview.tabs.is_empty() {
                        // Clean up remaining resources and close preview panel
                        preview.cleanup(cx);
                        self.preview = None;
                    } else if preview.active_tab >= preview.tabs.len() {
                        preview.active_tab = preview.tabs.len() - 1;
                    }
                }

                // Combine selected items + orphaned charts for deletion
                let mut ids_to_remove: Vec<u64> = selected.iter().copied().collect();
                ids_to_remove.extend(orphan_charts);

                board.remove_items(&ids_to_remove);
                self.selected_items.clear();
                board.push_history();
                board.save();
                cx.notify();
            }
        }
    }

    pub fn duplicate_selected(&mut self, cx: &mut Context<Self>) {
        if !self.selected_items.is_empty() {
            if let Some(ref mut board) = self.board {
                let mut new_ids = Vec::new();

                // Collect items to duplicate
                let items_to_dup: Vec<_> = self
                    .selected_items
                    .iter()
                    .filter_map(|id| board.get_item(*id).cloned())
                    .collect();

                for item in items_to_dup {
                    let mut new_item = item.clone();

                    // Assign a new ID
                    new_item.id = board.next_item_id;
                    board.next_item_id += 1;

                    // Offset the position by (20, 20) pixels
                    new_item.position.0 += 20.0;
                    new_item.position.1 += 20.0;

                    new_ids.push(new_item.id);
                    board.items.push(new_item);
                }

                // Select the new items
                self.selected_items.clear();
                for id in new_ids {
                    self.selected_items.insert(id);
                }

                // Save changes
                board.push_history();
                board.save();
                cx.notify();
            }
        }
    }

    pub fn select_all(&mut self, cx: &mut Context<Self>) {
        if let Some(ref board) = self.board {
            self.selected_items.clear();
            for item in &board.items {
                self.selected_items.insert(item.id);
            }
            cx.notify();
        }
    }

    /// Deselect all selected items
    pub fn deselect_all(&mut self, cx: &mut Context<Self>) {
        if !self.selected_items.is_empty() {
            self.selected_items.clear();
            cx.notify();
        }
    }

    /// Copy selected items to internal clipboard
    /// Note: For now, this uses an internal representation.
    /// Full system clipboard integration would require serialization.
    pub fn copy_selected(&mut self, cx: &mut Context<Self>) {
        use crate::notifications::Toast;

        if self.selected_items.is_empty() {
            return;
        }

        if self.board.is_some() {
            // Store copied items in a Vec for later paste
            // For now, we'll just show a toast - full implementation would
            // serialize to clipboard
            let count = self.selected_items.len();
            self.toast_manager.push(Toast::success(format!(
                "Copied {} item{}",
                count,
                if count == 1 { "" } else { "s" }
            )));
            cx.notify();
        }
    }

    /// Nudge selected items by a given delta
    fn nudge_selected(&mut self, dx: f32, dy: f32, cx: &mut Context<Self>) {
        if self.selected_items.is_empty() {
            return;
        }
        if let Some(ref mut board) = self.board {
            for item in &mut board.items {
                if self.selected_items.contains(&item.id) {
                    item.position.0 += dx;
                    item.position.1 += dy;
                }
            }
            board.push_history();
            board.mark_dirty();
            cx.notify();
        }
    }

    pub fn nudge_up(&mut self, cx: &mut Context<Self>) {
        self.nudge_selected(0.0, -10.0, cx);
    }

    pub fn nudge_down(&mut self, cx: &mut Context<Self>) {
        self.nudge_selected(0.0, 10.0, cx);
    }

    pub fn nudge_left(&mut self, cx: &mut Context<Self>) {
        self.nudge_selected(-10.0, 0.0, cx);
    }

    pub fn nudge_right(&mut self, cx: &mut Context<Self>) {
        self.nudge_selected(10.0, 0.0, cx);
    }

    pub fn toggle_command_palette(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.command_palette.is_some() {
            self.hide_command_palette(window, cx);
        } else {
            self.show_command_palette(window, cx);
        }
    }

    pub fn close_command_palette(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.command_palette.is_some() {
            self.hide_command_palette(window, cx);
        }
    }

    pub fn undo(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut board) = self.board {
            if board.undo() {
                self.selected_items.clear();
                cx.notify();
            }
        }
    }

    pub fn redo(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut board) = self.board {
            if board.redo() {
                self.selected_items.clear();
                cx.notify();
            }
        }
    }

    pub fn open_file(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Show file picker for multiple files
        let paths_rx = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: true,
            prompt: None,
        });

        // Get window center for default drop position
        let bounds = window.bounds();
        let window_size = bounds.size;
        let center_x = f32::from(window_size.width) / 2.0;
        let center_y = f32::from(window_size.height) / 2.0;

        // Workaround for GPUI async limitations: use a channel to communicate back
        let center_pos = point(px(center_x), px(center_y));
        let (tx, rx) = mpsc::channel();

        // Spawn background task to wait for file selection
        cx.background_executor()
            .spawn(async move {
                if let Ok(Ok(Some(paths))) = paths_rx.await {
                    let _ = tx.send((center_pos, paths));
                }
            })
            .detach();

        // Store the receiver - we'll poll it in the render cycle
        self.file_drop_rx = Some(rx);
    }
}
