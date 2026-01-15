//! Command Registry - Extensible command system for the application.
//!
//! This module provides a registry-based command system that makes it easy
//! to add new commands without modifying core palette logic.
//!
//! ## Usage
//!
//! ```ignore
//! // Register a command
//! registry.register(MyCommand::new());
//!
//! // Execute by ID
//! if let Some(cmd) = registry.get("my_command") {
//!     if cmd.is_enabled(app) {
//!         cmd.execute(app, cx);
//!     }
//! }
//! ```

use std::collections::HashMap;
use std::sync::Arc;

/// A command that can be executed from the command palette or keyboard shortcut.
pub trait Command: Send + Sync {
    /// Unique identifier for this command (e.g., "file:open", "edit:undo")
    fn id(&self) -> &'static str;

    /// Human-readable name shown in the command palette
    fn name(&self) -> &str;

    /// Optional description for the command
    fn description(&self) -> Option<&str> {
        None
    }

    /// Category for grouping in the command palette (e.g., "File", "Edit", "View")
    fn category(&self) -> &str {
        "General"
    }

    /// Keyboard shortcut hint (for display only, actual binding is separate)
    fn shortcut_hint(&self) -> Option<&str> {
        None
    }

    /// Check if this command can be executed in the current state
    fn is_enabled(&self, app: &crate::app::Humanboard) -> bool;

    /// Execute the command
    fn execute(
        &self,
        app: &mut crate::app::Humanboard,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<crate::app::Humanboard>,
    );

    /// Keywords for fuzzy search (in addition to name)
    fn keywords(&self) -> &[&str] {
        &[]
    }
}

/// Registry for application commands.
#[derive(Default)]
pub struct CommandRegistry {
    commands: HashMap<&'static str, Arc<dyn Command>>,
    /// Commands sorted by category for display
    by_category: HashMap<String, Vec<&'static str>>,
}

impl CommandRegistry {
    /// Create a new empty command registry.
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            by_category: HashMap::new(),
        }
    }

    /// Register a command.
    pub fn register<C: Command + 'static>(&mut self, cmd: C) {
        let id = cmd.id();
        let category = cmd.category().to_string();

        self.commands.insert(id, Arc::new(cmd));

        self.by_category
            .entry(category)
            .or_default()
            .push(id);
    }

    /// Get a command by ID.
    pub fn get(&self, id: &str) -> Option<Arc<dyn Command>> {
        self.commands.get(id).cloned()
    }

    /// Get all registered commands.
    pub fn all(&self) -> impl Iterator<Item = &Arc<dyn Command>> {
        self.commands.values()
    }

    /// Get all command IDs.
    pub fn ids(&self) -> impl Iterator<Item = &&'static str> {
        self.commands.keys()
    }

    /// Get commands by category.
    pub fn by_category(&self, category: &str) -> Vec<Arc<dyn Command>> {
        self.by_category
            .get(category)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.commands.get(id).cloned())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get all categories.
    pub fn categories(&self) -> impl Iterator<Item = &String> {
        self.by_category.keys()
    }

    /// Search commands by query (fuzzy match on name and keywords).
    pub fn search(&self, query: &str, app: &crate::app::Humanboard) -> Vec<Arc<dyn Command>> {
        let query_lower = query.to_lowercase();

        let mut matches: Vec<_> = self
            .commands
            .values()
            .filter(|cmd| cmd.is_enabled(app))
            .filter(|cmd| {
                let name_match = cmd.name().to_lowercase().contains(&query_lower);
                let keyword_match = cmd
                    .keywords()
                    .iter()
                    .any(|k| k.to_lowercase().contains(&query_lower));
                let category_match = cmd.category().to_lowercase().contains(&query_lower);
                let id_match = cmd.id().to_lowercase().contains(&query_lower);

                name_match || keyword_match || category_match || id_match
            })
            .cloned()
            .collect();

        // Sort by relevance (exact name match first, then by name)
        matches.sort_by(|a, b| {
            let a_exact = a.name().to_lowercase() == query_lower;
            let b_exact = b.name().to_lowercase() == query_lower;
            match (a_exact, b_exact) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name().cmp(b.name()),
            }
        });

        matches
    }

    /// Get the number of registered commands.
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
}

// ============================================================================
// Built-in Commands
// ============================================================================

/// Go to home/landing page
pub struct GoHomeCommand;

impl Command for GoHomeCommand {
    fn id(&self) -> &'static str {
        "navigation:home"
    }

    fn name(&self) -> &str {
        "Go Home"
    }

    fn category(&self) -> &str {
        "Navigation"
    }

    fn shortcut_hint(&self) -> Option<&str> {
        Some("Cmd+Shift+H")
    }

    fn is_enabled(&self, app: &crate::app::Humanboard) -> bool {
        matches!(app.view, crate::app::AppView::Board(_))
    }

    fn execute(
        &self,
        app: &mut crate::app::Humanboard,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<crate::app::Humanboard>,
    ) {
        app.go_home(cx);
    }

    fn keywords(&self) -> &[&str] {
        &["landing", "boards", "back"]
    }
}

/// Create a new board
pub struct NewBoardCommand;

impl Command for NewBoardCommand {
    fn id(&self) -> &'static str {
        "board:new"
    }

    fn name(&self) -> &str {
        "New Board"
    }

    fn category(&self) -> &str {
        "Board"
    }

    fn shortcut_hint(&self) -> Option<&str> {
        Some("Cmd+N")
    }

    fn is_enabled(&self, _app: &crate::app::Humanboard) -> bool {
        true
    }

    fn execute(
        &self,
        app: &mut crate::app::Humanboard,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<crate::app::Humanboard>,
    ) {
        app.create_new_board(window, cx);
    }

    fn keywords(&self) -> &[&str] {
        &["create", "add"]
    }
}

/// Toggle settings modal
pub struct OpenSettingsCommand;

impl Command for OpenSettingsCommand {
    fn id(&self) -> &'static str {
        "app:settings"
    }

    fn name(&self) -> &str {
        "Open Settings"
    }

    fn category(&self) -> &str {
        "Application"
    }

    fn shortcut_hint(&self) -> Option<&str> {
        Some("Cmd+,")
    }

    fn is_enabled(&self, _app: &crate::app::Humanboard) -> bool {
        true
    }

    fn execute(
        &self,
        app: &mut crate::app::Humanboard,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<crate::app::Humanboard>,
    ) {
        app.toggle_settings(window, cx);
    }

    fn keywords(&self) -> &[&str] {
        &["preferences", "config", "theme", "font"]
    }
}

/// Show keyboard shortcuts
pub struct ShowShortcutsCommand;

impl Command for ShowShortcutsCommand {
    fn id(&self) -> &'static str {
        "help:shortcuts"
    }

    fn name(&self) -> &str {
        "Show Keyboard Shortcuts"
    }

    fn category(&self) -> &str {
        "Help"
    }

    fn shortcut_hint(&self) -> Option<&str> {
        Some("?")
    }

    fn is_enabled(&self, _app: &crate::app::Humanboard) -> bool {
        true
    }

    fn execute(
        &self,
        app: &mut crate::app::Humanboard,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<crate::app::Humanboard>,
    ) {
        app.toggle_shortcuts(cx);
    }

    fn keywords(&self) -> &[&str] {
        &["keys", "bindings", "hotkeys", "help"]
    }
}

/// Undo last action
pub struct UndoCommand;

impl Command for UndoCommand {
    fn id(&self) -> &'static str {
        "edit:undo"
    }

    fn name(&self) -> &str {
        "Undo"
    }

    fn category(&self) -> &str {
        "Edit"
    }

    fn shortcut_hint(&self) -> Option<&str> {
        Some("Cmd+Z")
    }

    fn is_enabled(&self, app: &crate::app::Humanboard) -> bool {
        // Check if there's history to undo (history_index > 0)
        app.board.is_some()
    }

    fn execute(
        &self,
        app: &mut crate::app::Humanboard,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<crate::app::Humanboard>,
    ) {
        app.undo(cx);
    }
}

/// Redo last undone action
pub struct RedoCommand;

impl Command for RedoCommand {
    fn id(&self) -> &'static str {
        "edit:redo"
    }

    fn name(&self) -> &str {
        "Redo"
    }

    fn category(&self) -> &str {
        "Edit"
    }

    fn shortcut_hint(&self) -> Option<&str> {
        Some("Cmd+Shift+Z")
    }

    fn is_enabled(&self, app: &crate::app::Humanboard) -> bool {
        // Check if there's history to redo
        app.board.is_some()
    }

    fn execute(
        &self,
        app: &mut crate::app::Humanboard,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<crate::app::Humanboard>,
    ) {
        app.redo(cx);
    }
}

/// Zoom in
pub struct ZoomInCommand;

impl Command for ZoomInCommand {
    fn id(&self) -> &'static str {
        "view:zoom_in"
    }

    fn name(&self) -> &str {
        "Zoom In"
    }

    fn category(&self) -> &str {
        "View"
    }

    fn shortcut_hint(&self) -> Option<&str> {
        Some("Cmd++")
    }

    fn is_enabled(&self, app: &crate::app::Humanboard) -> bool {
        app.board.is_some()
    }

    fn execute(
        &self,
        app: &mut crate::app::Humanboard,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<crate::app::Humanboard>,
    ) {
        app.zoom_in(window, cx);
    }

    fn keywords(&self) -> &[&str] {
        &["magnify", "bigger"]
    }
}

/// Zoom out
pub struct ZoomOutCommand;

impl Command for ZoomOutCommand {
    fn id(&self) -> &'static str {
        "view:zoom_out"
    }

    fn name(&self) -> &str {
        "Zoom Out"
    }

    fn category(&self) -> &str {
        "View"
    }

    fn shortcut_hint(&self) -> Option<&str> {
        Some("Cmd+-")
    }

    fn is_enabled(&self, app: &crate::app::Humanboard) -> bool {
        app.board.is_some()
    }

    fn execute(
        &self,
        app: &mut crate::app::Humanboard,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<crate::app::Humanboard>,
    ) {
        app.zoom_out(window, cx);
    }

    fn keywords(&self) -> &[&str] {
        &["shrink", "smaller"]
    }
}

/// Reset zoom to 100%
pub struct ZoomResetCommand;

impl Command for ZoomResetCommand {
    fn id(&self) -> &'static str {
        "view:zoom_reset"
    }

    fn name(&self) -> &str {
        "Reset Zoom"
    }

    fn category(&self) -> &str {
        "View"
    }

    fn shortcut_hint(&self) -> Option<&str> {
        Some("Cmd+0")
    }

    fn is_enabled(&self, app: &crate::app::Humanboard) -> bool {
        app.board.is_some()
    }

    fn execute(
        &self,
        app: &mut crate::app::Humanboard,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<crate::app::Humanboard>,
    ) {
        app.zoom_reset(cx);
    }

    fn keywords(&self) -> &[&str] {
        &["100%", "actual", "normal"]
    }
}

/// Create a command registry with all built-in commands.
pub fn create_default_registry() -> CommandRegistry {
    let mut registry = CommandRegistry::new();

    // Navigation
    registry.register(GoHomeCommand);

    // Board
    registry.register(NewBoardCommand);

    // Edit
    registry.register(UndoCommand);
    registry.register(RedoCommand);

    // View
    registry.register(ZoomInCommand);
    registry.register(ZoomOutCommand);
    registry.register(ZoomResetCommand);

    // Application
    registry.register(OpenSettingsCommand);
    registry.register(ShowShortcutsCommand);

    registry
}
