//! Command palette methods - show/hide, search, execute commands

use super::{AppView, CmdPaletteMode, Humanboard, PanAnimation};
use crate::focus::FocusContext;
use crate::settings::Settings;
use gpui::*;
use gpui_component::input::InputState;
use std::time::{Duration, Instant};

impl Humanboard {
    pub fn show_command_palette(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Set focus context to CommandPalette
        self.focus.focus(FocusContext::CommandPalette, window);

        let input = cx
            .new(|cx| InputState::new(window, cx).placeholder("Type to search or use commands..."));

        // Focus the input
        input.update(cx, |state, cx| {
            state.focus(window, cx);
        });

        // Start fade-in animation
        self.modal_animations.open_command_palette();

        // Subscribe to input events
        cx.subscribe(
            &input,
            |this, input, event: &gpui_component::input::InputEvent, cx| {
                match event {
                    gpui_component::input::InputEvent::PressEnter { .. } => {
                        // Execute the command when Enter is pressed
                        if this.command_palette.is_some() {
                            this.execute_command_from_subscription(cx);
                        }
                    }
                    gpui_component::input::InputEvent::Change { .. } => {
                        // Update search results as user types
                        let text = input.read(cx).text().to_string();
                        this.update_search_results(&text, cx);
                    }
                    gpui_component::input::InputEvent::Blur => {
                        // Don't close on blur - this causes race conditions with Enter key
                        // The palette is closed by:
                        // - Clicking the backdrop (has its own handler)
                        // - Pressing Escape (CloseCommandPalette action)
                        // - Executing a command that should close it
                    }
                    _ => {}
                }
            },
        )
        .detach();

        // Note: Arrow key navigation is handled by on_key_down in render/overlays.rs
        // Do NOT add observe_keystrokes here - it would duplicate the handling

        self.command_palette = Some(input);

        // Show all items initially
        self.update_search_results("", cx);
    }

    /// Hide command palette and release focus (when window is available)
    pub fn hide_command_palette(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Start fade-out animation - cleanup happens when animation completes
        self.modal_animations.close_command_palette();
        // Release focus back to canvas
        self.focus.release(FocusContext::CommandPalette, window);
        cx.notify();
    }

    /// Clear command palette state without focus management
    /// Used when window is not available (e.g., from Blur callback)
    pub fn clear_command_palette_state(&mut self, cx: &mut Context<Self>) {
        // Start fade-out animation
        self.modal_animations.close_command_palette();
        // Mark that focus should return to canvas (actual focus happens in render)
        self.focus.mark_needs_canvas_focus();
        cx.notify();
    }

    /// Clean up command palette after close animation completes
    pub fn finish_close_command_palette(&mut self) {
        self.command_palette = None;
        self.search_results.clear();
        self.selected_result = 0;
        self.cmd_palette_mode = CmdPaletteMode::Items;
    }

    /// Update search results based on input text
    fn update_search_results(&mut self, text: &str, cx: &mut Context<Self>) {
        let text = text.trim();

        // Check if user typed "theme " to enter theme mode
        if text.starts_with("theme ") {
            self.cmd_palette_mode = CmdPaletteMode::Themes;
            let filter = text.strip_prefix("theme ").unwrap_or("").trim();
            let themes = Settings::available_themes(cx);
            if filter.is_empty() {
                self.search_results = themes
                    .into_iter()
                    .enumerate()
                    .map(|(idx, name)| (idx as u64, name))
                    .collect();
            } else {
                self.search_results = themes
                    .into_iter()
                    .enumerate()
                    .filter(|(_, name)| name.to_lowercase().contains(&filter.to_lowercase()))
                    .map(|(idx, name)| (idx as u64, name))
                    .collect();
            }
            self.selected_result = 0;
            cx.notify();
            return;
        }

        // Handle theme mode (when entered via click or command selection)
        if self.cmd_palette_mode == CmdPaletteMode::Themes {
            let themes = Settings::available_themes(cx);
            // If text is just "theme" (entered via command), treat as empty filter
            let filter = if text.eq_ignore_ascii_case("theme") {
                ""
            } else {
                text
            };

            if filter.is_empty() {
                // Show all themes
                self.search_results = themes
                    .into_iter()
                    .enumerate()
                    .map(|(idx, name)| (idx as u64, name))
                    .collect();
            } else {
                // Filter themes by search text
                self.search_results = themes
                    .into_iter()
                    .enumerate()
                    .filter(|(_, name)| name.to_lowercase().contains(&filter.to_lowercase()))
                    .map(|(idx, name)| (idx as u64, name))
                    .collect();
            }
            self.selected_result = 0;
            cx.notify();
            return;
        }

        // Check if typing a command prefix - show matching commands
        if !text.is_empty() && text.len() <= 7 {
            let text_lower = text.to_lowercase();
            // Available commands with special IDs (using high numbers to avoid collision with item IDs)
            let commands = [
                (u64::MAX - 1, "theme", "Change theme"),
                (u64::MAX - 2, "md", "Create markdown note"),
            ];

            let matching_commands: Vec<(u64, String)> = commands
                .iter()
                .filter(|(_, cmd, _)| cmd.starts_with(&text_lower))
                .map(|(id, cmd, desc)| (*id, format!("{} - {}", cmd, desc)))
                .collect();

            if !matching_commands.is_empty() {
                self.search_results = matching_commands;
                self.selected_result = 0;
                cx.notify();
                return;
            }
        }

        // Check if it's a complete command
        if text.starts_with("md ") || text == "md" {
            self.search_results.clear();
            self.selected_result = 0;
            cx.notify();
            return;
        }

        // Search canvas items (empty string shows all items)
        if let Some(ref board) = self.board {
            if text.is_empty() {
                // Show all searchable items when no search text
                self.search_results = board
                    .items
                    .iter()
                    .filter(|item| item.content.is_searchable())
                    .map(|item| {
                        // For tables, use the data source name instead of generic "Table"
                        let name = match &item.content {
                            crate::types::ItemContent::Table { data_source_id, .. } => {
                                board.data_sources
                                    .get(data_source_id)
                                    .map(|ds| ds.name.clone())
                                    .unwrap_or_else(|| "Table".to_string())
                            }
                            _ => item.content.display_name(),
                        };
                        (item.id, name)
                    })
                    .collect();
            } else {
                self.search_results = board.find_items(text);
            }
            self.selected_result = 0;
        } else {
            self.search_results.clear();
        }
        cx.notify();
    }

    /// Enter theme selection mode in command palette
    pub fn enter_theme_mode(&mut self, cx: &mut Context<Self>) {
        self.cmd_palette_mode = CmdPaletteMode::Themes;
        // Show all themes
        self.update_search_results("", cx);
    }

    /// Navigate search results
    pub fn select_next_result(&mut self, cx: &mut Context<Self>) {
        if !self.search_results.is_empty() {
            self.selected_result = (self.selected_result + 1) % self.search_results.len();
            self.cmd_palette_scroll.scroll_to_item(self.selected_result);
            cx.notify();
        }
    }

    pub fn select_prev_result(&mut self, cx: &mut Context<Self>) {
        if !self.search_results.is_empty() {
            self.selected_result = if self.selected_result == 0 {
                self.search_results.len() - 1
            } else {
                self.selected_result - 1
            };
            self.cmd_palette_scroll.scroll_to_item(self.selected_result);
            cx.notify();
        }
    }

    /// Called from action when Enter is pressed - stores command for deferred execution
    pub fn execute_command_from_action(&mut self, cx: &mut Context<Self>) {
        self.execute_command_from_subscription(cx);
    }

    /// Called from subscription when Enter is pressed - stores command for deferred execution
    fn execute_command_from_subscription(&mut self, cx: &mut Context<Self>) {
        // Handle theme mode
        if self.cmd_palette_mode == CmdPaletteMode::Themes {
            if !self.search_results.is_empty() {
                let (_, theme_name) = &self.search_results[self.selected_result];
                self.pending_command = Some(format!("__theme:{}", theme_name));
            }
            self.command_palette = None;
            self.search_results.clear();
            self.selected_result = 0;
            self.cmd_palette_mode = CmdPaletteMode::Items;
            cx.notify();
            return;
        }

        // If we have search results selected, check if it's a command or an item
        if !self.search_results.is_empty() {
            let (item_id, _) = &self.search_results[self.selected_result];

            // Check for special command IDs (u64::MAX - N for commands)
            const CMD_THEME: u64 = u64::MAX - 1;
            const CMD_MD: u64 = u64::MAX - 2;

            match *item_id {
                CMD_THEME => {
                    // Enter theme mode directly
                    self.cmd_palette_mode = CmdPaletteMode::Themes;
                    let themes = Settings::available_themes(cx);
                    self.search_results = themes
                        .into_iter()
                        .enumerate()
                        .map(|(idx, name)| (idx as u64, name))
                        .collect();
                    self.selected_result = 0;
                    cx.notify();
                    return; // Don't close palette, stay in theme mode
                }
                CMD_MD => {
                    self.pending_command = Some("md".to_string());
                }
                _ => {
                    // Regular item - jump to it
                    self.pending_command = Some(format!("__jump:{}", item_id));
                }
            }
        } else {
            let command = self
                .command_palette
                .as_ref()
                .map(|input| input.read(cx).text().to_string())
                .unwrap_or_default();
            self.pending_command = Some(command);
        }

        self.command_palette = None;
        self.search_results.clear();
        self.selected_result = 0;
        cx.notify();
    }

    /// Process any pending command (called from render where we have window access)
    pub fn process_pending_command(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(command) = self.pending_command.take() {
            let command = command.trim();

            // Handle jump command (from search result selection)
            if command.starts_with("__jump:") {
                if let Ok(item_id) = command
                    .strip_prefix("__jump:")
                    .unwrap_or("0")
                    .parse::<u64>()
                {
                    self.jump_to_item(item_id, window, cx);
                }
            } else if command.starts_with("__theme:") {
                let theme_name = command.strip_prefix("__theme:").unwrap_or("");
                if !theme_name.is_empty() {
                    self.set_theme(theme_name.to_string(), cx);
                }
            } else if command.starts_with("md ") {
                let name = command.strip_prefix("md ").unwrap_or("Untitled");
                self.create_markdown_note(name.to_string(), window, cx);
            } else if command == "md" {
                self.create_markdown_note("Untitled".to_string(), window, cx);
            }
        }
    }

    /// Jump to and select an item by ID with smooth animation
    fn jump_to_item(&mut self, item_id: u64, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ref board) = self.board {
            if let Some(item) = board.items.iter().find(|i| i.id == item_id) {
                // Get window size for centering
                let bounds = window.bounds();
                let screen_size = bounds.size;

                // Calculate the center of the item in canvas coordinates
                let item_center_x = item.position.0 + item.size.0 / 2.0;
                let item_center_y = item.position.1 + item.size.1 / 2.0;

                // Calculate target offset to center item on screen
                let screen_center_x = f32::from(screen_size.width) / 2.0;
                let screen_center_y = f32::from(screen_size.height) / 2.0;

                let target_offset = point(
                    px(screen_center_x - item_center_x * board.zoom),
                    px(screen_center_y - item_center_y * board.zoom),
                );

                // Start animation from current position to target
                self.pan_animation = Some(PanAnimation {
                    start_offset: board.canvas_offset,
                    target_offset,
                    start_time: Instant::now(),
                    duration: Duration::from_millis(300),
                });

                // Select the item
                self.selected_items.clear();
                self.selected_items.insert(item_id);

                // Trigger first frame
                cx.notify();
            }
        }
    }

    /// Update pan animation, returns true if animation is active
    pub fn update_pan_animation(&mut self) -> bool {
        if let Some(ref anim) = self.pan_animation {
            let elapsed = anim.start_time.elapsed();
            let progress = (elapsed.as_secs_f32() / anim.duration.as_secs_f32()).min(1.0);

            // Ease out cubic for smooth deceleration
            let eased = 1.0 - (1.0 - progress).powi(3);

            if let Some(ref mut board) = self.board {
                // Interpolate between start and target
                let start_x = f32::from(anim.start_offset.x);
                let start_y = f32::from(anim.start_offset.y);
                let target_x = f32::from(anim.target_offset.x);
                let target_y = f32::from(anim.target_offset.y);

                board.canvas_offset = point(
                    px(start_x + (target_x - start_x) * eased),
                    px(start_y + (target_y - start_y) * eased),
                );
            }

            if progress >= 1.0 {
                // Animation complete
                self.pan_animation = None;
                return false;
            }
            return true;
        }
        false
    }

    pub fn execute_command(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let command = self
            .command_palette
            .as_ref()
            .map(|input| input.read(cx).text().to_string())
            .unwrap_or_default();

        self.command_palette = None;

        // Parse command
        let command = command.trim();
        if command.starts_with("md ") {
            let name = command.strip_prefix("md ").unwrap_or("Untitled");
            self.create_markdown_note(name.to_string(), window, cx);
        } else if command == "md" {
            self.create_markdown_note("Untitled".to_string(), window, cx);
        }
        // Add more commands here as needed

        cx.notify();
    }

    fn create_markdown_note(&mut self, name: String, window: &mut Window, cx: &mut Context<Self>) {
        // Get board ID from current view
        let board_id = match &self.view {
            AppView::Board(id) => id.clone(),
            _ => return,
        };

        // Clean up name - remove .md extension if user added it
        let name = name.trim().trim_end_matches(".md").trim();
        let name = if name.is_empty() {
            "Untitled".to_string()
        } else {
            name.to_string()
        };

        if let Some(ref mut board) = self.board {
            // Create markdown file in the board's files directory
            let files_dir = crate::board_index::BoardIndex::board_files_dir(&board_id);
            let _ = std::fs::create_dir_all(&files_dir);

            // Generate safe filename - just use the name, add short suffix if exists
            let safe_name: String = name
                .chars()
                .map(|c| {
                    if c.is_alphanumeric() || c == '-' || c == '_' {
                        c
                    } else {
                        '-'
                    }
                })
                .collect();

            // Try just the name first, add short timestamp suffix if file exists
            let mut filename = format!("{}.md", safe_name);
            let mut path = files_dir.join(&filename);
            if path.exists() {
                let short_id = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
                    % 10000; // Last 4 digits
                filename = format!("{}-{}.md", safe_name, short_id);
                path = files_dir.join(&filename);
            }

            // Create markdown file with title
            let initial_content = format!("# {}\n\n", name);
            let _ = std::fs::write(&path, &initial_content);

            // Add to board at center of visible canvas (accounting for pan/zoom)
            let center_screen = point(px(600.0), px(400.0));
            let canvas_pos = board.screen_to_canvas(center_screen);
            board.add_item(
                canvas_pos,
                crate::types::ItemContent::Markdown {
                    path: path.clone(),
                    title: name.clone(),
                    content: initial_content, // Store actual content for preview
                },
            );

            // Open in preview and immediately switch to edit mode
            self.open_preview(path, window, cx);
            self.toggle_markdown_edit(window, cx);
        }
    }

    pub fn paste(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let clipboard = cx.read_from_clipboard();
        if let Some(item) = clipboard {
            if let Some(text) = item.text() {
                let text = text.trim();
                // Check if it's a URL
                if text.starts_with("http://") || text.starts_with("https://") {
                    // Get center of window for paste position
                    let bounds = window.bounds();
                    let center = point(
                        px(f32::from(bounds.size.width) / 2.0),
                        px(f32::from(bounds.size.height) / 2.0),
                    );
                    if let Some(ref mut board) = self.board {
                        board.add_url(text, center);
                        cx.notify();
                    }
                }
            }
        }
    }
}
