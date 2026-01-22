//! Humanboard - A Miro-style infinite canvas moodboard application
//!
//! This is the main entry point for the Humanboard application.
//! Following Zed's initialization patterns for proper startup sequence.

use anyhow::{Context, Result};
use gpui::*;
use humanboard::actions::{
    CancelTextboxEdit, CloseCommandPalette, CloseTab, CmdPaletteDown, CmdPaletteUp, DeleteSelected,
    DeselectAll, DuplicateSelected, GoBack, GoForward, GoHome, ModalFocusNext, ModalFocusPrev,
    MoveTabToOtherPane, NewBoard, NextSearchMatch, NextTab, NudgeDown, NudgeLeft, NudgeRight,
    NudgeUp, OpenFile, OpenSettings, Paste, PrevSearchMatch, PrevTab, Quit, Redo, ReopenClosedTab,
    SaveCode, SelectAll, ShowShortcuts, ToggleCommandPalette, TogglePaneSplit, TogglePreviewSearch,
    Undo, ZoomIn, ZoomOut, ZoomReset,
};
use humanboard::app::Humanboard;
use humanboard::focus::FocusContext;
use once_cell::sync::Lazy;
use std::borrow::Cow;
use std::time::Instant;
use tracing::{debug, error, info};

/// Application startup time for performance monitoring
static STARTUP_TIME: Lazy<Instant> = Lazy::new(Instant::now);

/// Asset source that loads from the assets directory.
/// Follows Zed's pattern of checking multiple locations for resources.
struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>, anyhow::Error> {
        // Try current directory first (for development)
        if let Some(cwd_path) = std::env::current_dir()
            .ok()
            .map(|p| p.join("assets").join(path))
        {
            if cwd_path.exists() {
                let data = std::fs::read(&cwd_path)
                    .with_context(|| format!("Failed to read asset from {:?}", cwd_path))?;
                return Ok(Some(Cow::Owned(data)));
            }
        }

        // Try relative to executable
        if let Some(exe_path) = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .map(|p| p.join("assets").join(path))
        {
            if exe_path.exists() {
                let data = std::fs::read(&exe_path)
                    .with_context(|| format!("Failed to read asset from {:?}", exe_path))?;
                return Ok(Some(Cow::Owned(data)));
            }
        }

        // Try macOS bundle Resources folder
        if let Some(bundle_path) = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf())) // MacOS/
            .and_then(|p| p.parent().map(|p| p.to_path_buf())) // Contents/
            .map(|p| p.join("Resources").join("assets").join(path))
        {
            if bundle_path.exists() {
                let data = std::fs::read(&bundle_path)
                    .with_context(|| format!("Failed to read asset from {:?}", bundle_path))?;
                return Ok(Some(Cow::Owned(data)));
            }
        }

        // Asset not found - this is normal for optional assets
        debug!("Asset not found: {}", path);
        Ok(None)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>, anyhow::Error> {
        // Try current directory first
        if let Some(cwd_path) = std::env::current_dir()
            .ok()
            .map(|p| p.join("assets").join(path))
        {
            if cwd_path.is_dir() {
                let entries: Vec<SharedString> = std::fs::read_dir(&cwd_path)
                    .with_context(|| format!("Failed to list directory {:?}", cwd_path))?
                    .filter_map(|e| e.ok())
                    .filter_map(|e| e.file_name().into_string().ok())
                    .map(SharedString::from)
                    .collect();
                return Ok(entries);
            }
        }

        // Try macOS bundle Resources folder
        if let Some(bundle_path) = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .map(|p| p.join("Resources").join("assets").join(path))
        {
            if bundle_path.is_dir() {
                let entries: Vec<SharedString> = std::fs::read_dir(&bundle_path)
                    .with_context(|| format!("Failed to list directory {:?}", bundle_path))?
                    .filter_map(|e| e.ok())
                    .filter_map(|e| e.file_name().into_string().ok())
                    .map(SharedString::from)
                    .collect();
                return Ok(entries);
            }
        }

        Ok(vec![])
    }
}

/// Initialize required directories for the application.
/// Following Zed's pattern of early directory setup.
fn init_paths() -> Result<()> {
    let home = std::env::var("HOME").context("HOME environment variable not set")?;
    let config_dir = std::path::PathBuf::from(&home)
        .join(".config")
        .join("humanboard");
    let data_dir = std::path::PathBuf::from(&home)
        .join(".local")
        .join("share")
        .join("humanboard");

    std::fs::create_dir_all(&config_dir)
        .with_context(|| format!("Failed to create config directory: {:?}", config_dir))?;
    std::fs::create_dir_all(&data_dir)
        .with_context(|| format!("Failed to create data directory: {:?}", data_dir))?;

    debug!(
        "Initialized paths - config: {:?}, data: {:?}",
        config_dir, data_dir
    );
    Ok(())
}

/// Initialize the logging system.
/// Following Zed's tracing initialization pattern.
fn init_logging() {
    use tracing_subscriber::{EnvFilter, fmt, prelude::*};

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("humanboard=info,warn"));

    tracing_subscriber::registry()
        .with(fmt::layer().with_target(true).with_line_number(true))
        .with(filter)
        .init();

    info!("Humanboard v{} starting up", env!("CARGO_PKG_VERSION"));
}

/// Build window options following Zed's patterns.
fn build_window_options() -> WindowOptions {
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(Bounds {
            origin: Point::new(px(100.0), px(100.0)),
            size: Size {
                width: px(1400.0),
                height: px(900.0),
            },
        })),
        titlebar: Some(TitlebarOptions {
            title: Some("Humanboard".into()),
            appears_transparent: true,
            ..Default::default()
        }),
        ..Default::default()
    }
}

/// Open the main application window.
fn open_main_window(cx: &mut App) -> Result<()> {
    cx.open_window(build_window_options(), |window, cx| {
        let app_view = cx.new(Humanboard::new);
        cx.new(|cx| gpui_component::Root::new(app_view, window, cx))
    })
    .context("Failed to open main window")?;

    info!("Main window opened in {:?}", STARTUP_TIME.elapsed());
    Ok(())
}

/// Register all global keybindings.
/// Following Zed's pattern of organized keybinding registration.
fn register_keybindings(cx: &mut App) {
    // Register quit action handler
    cx.on_action(|_: &Quit, cx| {
        info!("Application quit requested");
        cx.quit();
    });

    // Global shortcuts (always active, no context)
    // Register both cmd- (macOS) and ctrl- (Linux/Windows) variants
    cx.bind_keys([
        // Quit - cmd-q on macOS, ctrl-q on Linux/Windows
        KeyBinding::new("cmd-q", Quit, None),
        KeyBinding::new("ctrl-q", Quit, None),
        // Open file
        KeyBinding::new("cmd-o", OpenFile, None),
        KeyBinding::new("ctrl-o", OpenFile, None),
        // Zoom
        KeyBinding::new("cmd-=", ZoomIn, None),
        KeyBinding::new("ctrl-=", ZoomIn, None),
        KeyBinding::new("cmd-+", ZoomIn, None),
        KeyBinding::new("ctrl-+", ZoomIn, None),
        KeyBinding::new("cmd--", ZoomOut, None),
        KeyBinding::new("ctrl--", ZoomOut, None),
        KeyBinding::new("cmd-0", ZoomReset, None),
        KeyBinding::new("ctrl-0", ZoomReset, None),
        // Tab navigation
        KeyBinding::new("cmd-shift-]", NextTab, None),
        KeyBinding::new("ctrl-shift-]", NextTab, None),
        KeyBinding::new("ctrl-tab", NextTab, None),
        KeyBinding::new("cmd-shift-[", PrevTab, None),
        KeyBinding::new("ctrl-shift-[", PrevTab, None),
        KeyBinding::new("ctrl-shift-tab", PrevTab, None),
        KeyBinding::new("cmd-w", CloseTab, None),
        KeyBinding::new("ctrl-w", CloseTab, None),
        KeyBinding::new("cmd-shift-t", ReopenClosedTab, None),
        KeyBinding::new("ctrl-shift-t", ReopenClosedTab, None),
        // History navigation
        KeyBinding::new("cmd-[", GoBack, None),
        KeyBinding::new("ctrl-[", GoBack, None),
        KeyBinding::new("alt-left", GoBack, None),
        KeyBinding::new("cmd-]", GoForward, None),
        KeyBinding::new("ctrl-]", GoForward, None),
        KeyBinding::new("alt-right", GoForward, None),
        // Search
        KeyBinding::new("cmd-f", TogglePreviewSearch, None),
        KeyBinding::new("ctrl-f", TogglePreviewSearch, None),
        KeyBinding::new("cmd-g", NextSearchMatch, None),
        KeyBinding::new("ctrl-g", NextSearchMatch, None),
        KeyBinding::new("cmd-shift-g", PrevSearchMatch, None),
        KeyBinding::new("ctrl-shift-g", PrevSearchMatch, None),
        // Pane split
        KeyBinding::new("cmd-shift-d", TogglePaneSplit, None),
        KeyBinding::new("ctrl-shift-d", TogglePaneSplit, None),
        KeyBinding::new("cmd-alt-shift-right", MoveTabToOtherPane, None),
        KeyBinding::new("ctrl-alt-shift-right", MoveTabToOtherPane, None),
        KeyBinding::new("cmd-alt-shift-left", MoveTabToOtherPane, None),
        KeyBinding::new("ctrl-alt-shift-left", MoveTabToOtherPane, None),
        // Board management
        KeyBinding::new("cmd-n", NewBoard, None),
        KeyBinding::new("ctrl-n", NewBoard, None),
        KeyBinding::new("cmd-h", GoHome, None),
        KeyBinding::new("ctrl-h", GoHome, None),
        // Help & settings
        KeyBinding::new("cmd-/", ShowShortcuts, None),
        KeyBinding::new("ctrl-/", ShowShortcuts, None),
        KeyBinding::new("cmd-,", OpenSettings, None),
        KeyBinding::new("ctrl-,", OpenSettings, None),
    ]);

    // Canvas-specific shortcuts (undo/redo - code editor handles its own)
    cx.bind_keys([
        KeyBinding::new("cmd-z", Undo, Some(FocusContext::KEY_CANVAS)),
        KeyBinding::new("ctrl-z", Undo, Some(FocusContext::KEY_CANVAS)),
        KeyBinding::new("cmd-shift-z", Redo, Some(FocusContext::KEY_CANVAS)),
        KeyBinding::new("ctrl-shift-z", Redo, Some(FocusContext::KEY_CANVAS)),
        KeyBinding::new("ctrl-y", Redo, Some(FocusContext::KEY_CANVAS)), // Windows-style redo
    ]);

    // Save shortcuts for different contexts
    cx.bind_keys([
        KeyBinding::new("cmd-s", SaveCode, Some(FocusContext::KEY_CODE_EDITOR)),
        KeyBinding::new("ctrl-s", SaveCode, Some(FocusContext::KEY_CODE_EDITOR)),
        KeyBinding::new("cmd-s", SaveCode, Some(FocusContext::KEY_CANVAS)),
        KeyBinding::new("ctrl-s", SaveCode, Some(FocusContext::KEY_CANVAS)),
        KeyBinding::new("cmd-s", SaveCode, Some(FocusContext::KEY_CANVAS_INPUT_ACTIVE)),
        KeyBinding::new("ctrl-s", SaveCode, Some(FocusContext::KEY_CANVAS_INPUT_ACTIVE)),
        KeyBinding::new("cmd-s", SaveCode, Some(FocusContext::KEY_PREVIEW)),
        KeyBinding::new("ctrl-s", SaveCode, Some(FocusContext::KEY_PREVIEW)),
    ]);

    // Canvas-only shortcuts (not active when text input is focused)
    cx.bind_keys([
        // Selection actions
        KeyBinding::new("backspace", DeleteSelected, Some(FocusContext::KEY_CANVAS)),
        KeyBinding::new("delete", DeleteSelected, Some(FocusContext::KEY_CANVAS)),
        KeyBinding::new("cmd-d", DuplicateSelected, Some(FocusContext::KEY_CANVAS)),
        KeyBinding::new("ctrl-d", DuplicateSelected, Some(FocusContext::KEY_CANVAS)),
        KeyBinding::new("cmd-a", SelectAll, Some(FocusContext::KEY_CANVAS)),
        KeyBinding::new("ctrl-a", SelectAll, Some(FocusContext::KEY_CANVAS)),
        KeyBinding::new("escape", DeselectAll, Some(FocusContext::KEY_CANVAS)),
        // Paste (handles URLs including YouTube)
        KeyBinding::new("cmd-v", Paste, Some(FocusContext::KEY_CANVAS)),
        KeyBinding::new("ctrl-v", Paste, Some(FocusContext::KEY_CANVAS)),
        // Command palette (cmd-k / ctrl-k toggles open/close)
        KeyBinding::new("cmd-k", ToggleCommandPalette, Some(FocusContext::KEY_CANVAS)),
        KeyBinding::new("ctrl-k", ToggleCommandPalette, Some(FocusContext::KEY_CANVAS)),
        // Arrow keys to nudge selected items
        KeyBinding::new("up", NudgeUp, Some(FocusContext::KEY_CANVAS)),
        KeyBinding::new("down", NudgeDown, Some(FocusContext::KEY_CANVAS)),
        KeyBinding::new("left", NudgeLeft, Some(FocusContext::KEY_CANVAS)),
        KeyBinding::new("right", NudgeRight, Some(FocusContext::KEY_CANVAS)),
    ]);

    // Shortcuts that work even when input is active
    cx.bind_keys([
        KeyBinding::new("cmd-k", ToggleCommandPalette, Some(FocusContext::KEY_CANVAS_INPUT_ACTIVE)),
        KeyBinding::new("ctrl-k", ToggleCommandPalette, Some(FocusContext::KEY_CANVAS_INPUT_ACTIVE)),
        KeyBinding::new("escape", CancelTextboxEdit, Some(FocusContext::KEY_CANVAS_INPUT_ACTIVE)),
    ]);

    // Landing page shortcuts
    cx.bind_keys([
        KeyBinding::new("cmd-k", ToggleCommandPalette, Some(FocusContext::KEY_LANDING)),
        KeyBinding::new("ctrl-k", ToggleCommandPalette, Some(FocusContext::KEY_LANDING)),
    ]);

    // Command palette navigation - use global bindings because when Input has focus,
    // the CommandPalette key context is NOT in the focus path (Input's own focus takes over)
    // The action handlers check if the palette is open before acting
    cx.bind_keys([
        KeyBinding::new("up", CmdPaletteUp, None),      // Global - handler checks if palette open
        KeyBinding::new("down", CmdPaletteDown, None),  // Global - handler checks if palette open
        KeyBinding::new("escape", CloseCommandPalette, None), // Global - handler checks if palette open
    ]);

    // Modal context shortcuts (settings, dialogs)
    // Focus trap: Tab and Shift+Tab cycle focus within modal
    cx.bind_keys([
        KeyBinding::new("escape", OpenSettings, Some(FocusContext::KEY_MODAL)),
        KeyBinding::new("tab", ModalFocusNext, Some(FocusContext::KEY_MODAL)),
        KeyBinding::new("shift-tab", ModalFocusPrev, Some(FocusContext::KEY_MODAL)),
    ]);

    debug!("Keybindings registered");
}

/// Initialize global services and subsystems.
/// Following Zed's pattern of structured initialization order.
fn initialize_subsystems(cx: &mut App) {
    // 1. Initialize UI component library
    gpui_component::init(cx);
    debug!("UI components initialized");

    // 2. Initialize themes from themes directory
    humanboard::settings::init_themes(cx);
    debug!("Themes initialized");

    // 3. Register keybindings
    register_keybindings(cx);
}

fn main() {
    // Record startup time
    let _ = *STARTUP_TIME;

    // Initialize logging first (following Zed pattern)
    init_logging();

    // Initialize required directories
    if let Err(e) = init_paths() {
        error!("Failed to initialize paths: {}", e);
        // Continue anyway - the app might still work
    }

    // Create and configure the application
    let app = Application::new().with_assets(Assets);

    // Handle dock icon click to reopen window
    app.on_reopen(|cx| {
        if cx.windows().is_empty() {
            if let Err(e) = open_main_window(cx) {
                error!("Failed to reopen window: {}", e);
            }
        }
    });

    // Run the application
    app.run(|cx: &mut App| {
        // Activate the application
        cx.activate(true);

        // Initialize all subsystems
        initialize_subsystems(cx);

        // Open the main window
        if let Err(e) = open_main_window(cx) {
            error!("Failed to open main window: {}", e);
            cx.quit();
        }

        info!(
            "Application fully initialized in {:?}",
            STARTUP_TIME.elapsed()
        );
    });
}
