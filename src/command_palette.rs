//! Command Palette Module
//!
//! This module provides a command palette for searching items and executing commands,
//! following Zed's pattern of separating UI concerns into dedicated components.
//!
//! ## Features
//!
//! - **Item Search**: Search canvas items by name/content
//! - **Theme Selection**: Quick theme switching
//! - **Commands**: Execute various commands (md, theme, etc.)
//! - **Fuzzy Matching**: Filter results as user types

use crate::settings::Settings;
use gpui::*;
use gpui_component::input::InputState;

/// Mode for the command palette - what type of items we're showing.
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum PaletteMode {
    /// Searching canvas items
    #[default]
    Items,
    /// Selecting theme
    Themes,
}

/// A search result entry in the command palette.
#[derive(Clone, Debug)]
pub struct SearchResult {
    /// Unique ID for this result
    pub id: u64,
    /// Display name/title
    pub name: String,
    /// Optional subtitle/description
    pub description: Option<String>,
}

impl SearchResult {
    pub fn new(id: u64, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            description: None,
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}

/// Special command IDs that don't correspond to canvas items.
/// Using high values to avoid collision with real item IDs.
pub mod command_ids {
    pub const THEME: u64 = u64::MAX - 1;
    pub const CREATE_MARKDOWN: u64 = u64::MAX - 2;
}

/// Available commands in the palette.
pub struct Command {
    pub id: u64,
    pub name: &'static str,
    pub description: &'static str,
}

/// List of all available commands.
pub const COMMANDS: &[Command] = &[
    Command {
        id: command_ids::THEME,
        name: "theme",
        description: "Change theme",
    },
    Command {
        id: command_ids::CREATE_MARKDOWN,
        name: "md",
        description: "Create markdown note",
    },
];

/// Command palette state and logic.
pub struct CommandPalette {
    /// The input entity for the palette
    pub input: Entity<InputState>,
    /// Current search/filter text
    pub query: String,
    /// Current mode
    pub mode: PaletteMode,
    /// Search results
    pub results: Vec<SearchResult>,
    /// Currently selected result index
    pub selected_index: usize,
    /// Scroll handle for results
    pub scroll_handle: ScrollHandle,
}

impl CommandPalette {
    /// Create a new command palette.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| {
            InputState::new(window, cx).placeholder("Type to search or use commands...")
        });

        Self {
            input,
            query: String::new(),
            mode: PaletteMode::Items,
            results: Vec::new(),
            selected_index: 0,
            scroll_handle: ScrollHandle::new(),
        }
    }

    /// Focus the input field.
    pub fn focus(&self, window: &mut Window, cx: &mut Context<Self>) {
        self.input.update(cx, |state, cx| {
            state.focus(window, cx);
        });
    }

    /// Get the current query text.
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Update the query and recalculate results.
    pub fn set_query(&mut self, query: String, cx: &mut Context<Self>) {
        self.query = query;
        self.update_results(cx);
    }

    /// Update search results based on current query and mode.
    fn update_results(&mut self, cx: &mut Context<Self>) {
        let query = self.query.trim();

        // Check for theme command prefix
        if query.starts_with("theme ") {
            self.mode = PaletteMode::Themes;
            let filter = query.strip_prefix("theme ").unwrap_or("").trim();
            self.results = Self::filter_themes(filter, cx);
            self.selected_index = 0;
            return;
        }

        // In theme mode, filter themes
        if self.mode == PaletteMode::Themes {
            let filter = if query.eq_ignore_ascii_case("theme") {
                ""
            } else {
                query
            };
            self.results = Self::filter_themes(filter, cx);
            self.selected_index = 0;
            return;
        }

        // Check for command matches
        if !query.is_empty() && query.len() <= 7 {
            let query_lower = query.to_lowercase();
            let matching: Vec<SearchResult> = COMMANDS
                .iter()
                .filter(|cmd| cmd.name.starts_with(&query_lower))
                .map(|cmd| {
                    SearchResult::new(cmd.id, format!("{} - {}", cmd.name, cmd.description))
                })
                .collect();

            if !matching.is_empty() {
                self.results = matching;
                self.selected_index = 0;
                return;
            }
        }

        // For item search, we'll emit an event to let the parent handle it
        // since it has access to the board data
        self.results.clear();
        self.selected_index = 0;
    }

    /// Filter available themes by query.
    fn filter_themes(filter: &str, cx: &App) -> Vec<SearchResult> {
        let themes = Settings::available_themes(cx);

        if filter.is_empty() {
            themes
                .into_iter()
                .enumerate()
                .map(|(idx, name)| SearchResult::new(idx as u64, name))
                .collect()
        } else {
            themes
                .into_iter()
                .enumerate()
                .filter(|(_, name)| name.to_lowercase().contains(&filter.to_lowercase()))
                .map(|(idx, name)| SearchResult::new(idx as u64, name))
                .collect()
        }
    }

    /// Set results from external source (e.g., canvas item search).
    pub fn set_results(&mut self, results: Vec<SearchResult>) {
        self.results = results;
        self.selected_index = 0;
    }

    /// Enter theme selection mode.
    pub fn enter_theme_mode(&mut self, cx: &mut Context<Self>) {
        self.mode = PaletteMode::Themes;
        self.results = Self::filter_themes("", cx);
        self.selected_index = 0;
    }

    /// Get the currently selected result, if any.
    pub fn selected_result(&self) -> Option<&SearchResult> {
        self.results.get(self.selected_index)
    }

    /// Select the next result.
    pub fn select_next(&mut self) {
        if !self.results.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.results.len();
            self.scroll_handle.scroll_to_item(self.selected_index);
        }
    }

    /// Select the previous result.
    pub fn select_previous(&mut self) {
        if !self.results.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.results.len() - 1
            } else {
                self.selected_index - 1
            };
            self.scroll_handle.scroll_to_item(self.selected_index);
        }
    }

    /// Check if results are empty.
    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    /// Get the number of results.
    pub fn result_count(&self) -> usize {
        self.results.len()
    }

    /// Reset the palette to its initial state.
    pub fn reset(&mut self) {
        self.query.clear();
        self.mode = PaletteMode::Items;
        self.results.clear();
        self.selected_index = 0;
    }
}

/// Result of executing a command palette action.
#[derive(Debug, Clone)]
pub enum PaletteAction {
    /// No action (stay open)
    None,
    /// Close the palette
    Close,
    /// Jump to an item on the canvas
    JumpToItem(u64),
    /// Set the theme
    SetTheme(String),
    /// Create a markdown note with the given name
    CreateMarkdown(String),
    /// Enter theme selection mode
    EnterThemeMode,
}

/// Process the current selection and return the action to take.
pub fn process_selection(palette: &CommandPalette) -> PaletteAction {
    // If in theme mode, apply the selected theme
    if palette.mode == PaletteMode::Themes {
        if let Some(result) = palette.selected_result() {
            return PaletteAction::SetTheme(result.name.clone());
        }
        return PaletteAction::Close;
    }

    // Check for command selection
    if let Some(result) = palette.selected_result() {
        match result.id {
            command_ids::THEME => PaletteAction::EnterThemeMode,
            command_ids::CREATE_MARKDOWN => {
                let name = palette
                    .query
                    .strip_prefix("md ")
                    .unwrap_or("Untitled")
                    .trim();
                PaletteAction::CreateMarkdown(name.to_string())
            }
            _ => {
                // Regular item - jump to it
                PaletteAction::JumpToItem(result.id)
            }
        }
    } else {
        // No result selected, check if query is a command
        let query = palette.query.trim();
        if query == "md" || query.starts_with("md ") {
            let name = query.strip_prefix("md ").unwrap_or("Untitled").trim();
            PaletteAction::CreateMarkdown(name.to_string())
        } else {
            PaletteAction::Close
        }
    }
}
