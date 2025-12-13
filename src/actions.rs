use crate::app::Humanboard;
use gpui::*;
use std::sync::mpsc;

// Action definitions
actions!(
    humanboard,
    [
        Quit,
        ZoomIn,
        ZoomOut,
        ZoomReset,
        DeleteSelected,
        DuplicateSelected,
        SelectAll,
        Undo,
        Redo,
        ClosePreview,
        ToggleSplit,
        NextPage,
        PrevPage,
        PdfZoomIn,
        PdfZoomOut,
        PdfZoomReset,
        NextTab,
        PrevTab,
        CloseTab,
        OpenFile,
        GoHome,
        NewBoard,
        ShowShortcuts,
        Paste,
        CommandPalette,
        ToggleCommandPalette,
        CloseCommandPalette,
        OpenSettings,
        NudgeUp,
        NudgeDown,
        NudgeLeft,
        NudgeRight,
        // Command palette navigation
        CmdPaletteUp,
        CmdPaletteDown,
        CmdPaletteSelect
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
                        _ => None,
                    })
                    .collect();

                // Close preview tabs for deleted items
                if let Some(ref mut preview) = self.preview {
                    let mut tabs_to_remove: Vec<usize> = Vec::new();
                    for (i, tab) in preview.tabs.iter().enumerate() {
                        if deleted_paths.contains(tab.path()) {
                            tabs_to_remove.push(i);
                        }
                    }
                    // Remove in reverse order to preserve indices
                    for i in tabs_to_remove.into_iter().rev() {
                        preview.tabs.remove(i);
                    }
                    // Adjust active tab if needed
                    if preview.tabs.is_empty() {
                        self.preview = None;
                    } else if preview.active_tab >= preview.tabs.len() {
                        preview.active_tab = preview.tabs.len() - 1;
                    }
                }

                board.items.retain(|item| !selected.contains(&item.id));
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
