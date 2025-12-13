use gpui::*;
use humanboard::actions::{
    CancelTextboxEdit, CloseCommandPalette, ClosePreview, CloseTab, CmdPaletteDown,
    CmdPaletteSelect, CmdPaletteUp, DeleteSelected, DuplicateSelected, GoHome, NewBoard, NextTab,
    NudgeDown, NudgeLeft, NudgeRight, NudgeUp, OpenFile, OpenSettings, Paste, PrevTab, Quit, Redo,
    SaveCode, SelectAll, ShowShortcuts, ToggleCommandPalette, Undo, ZoomIn, ZoomOut, ZoomReset,
};
use humanboard::app::Humanboard;
use std::borrow::Cow;

/// Asset source that loads from the assets directory
struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        // Try current directory first (for development)
        let cwd_path = std::env::current_dir()
            .ok()
            .map(|p| p.join("assets").join(path));

        if let Some(ref p) = cwd_path {
            if p.exists() {
                return Ok(Some(Cow::Owned(std::fs::read(p)?)));
            }
        }

        // Try relative to executable
        let exe_path = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .map(|p| p.join("assets").join(path));

        if let Some(ref p) = exe_path {
            if p.exists() {
                return Ok(Some(Cow::Owned(std::fs::read(p)?)));
            }
        }

        // Try macOS bundle Resources folder
        let bundle_path = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf())) // MacOS/
            .and_then(|p| p.parent().map(|p| p.to_path_buf())) // Contents/
            .map(|p| p.join("Resources").join("assets").join(path));

        if let Some(ref p) = bundle_path {
            if p.exists() {
                return Ok(Some(Cow::Owned(std::fs::read(p)?)));
            }
        }

        // Asset not found - this is normal for optional assets
        Ok(None)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        // Try current directory first
        let cwd_path = std::env::current_dir()
            .ok()
            .map(|p| p.join("assets").join(path));

        if let Some(ref p) = cwd_path {
            if p.is_dir() {
                let entries: Vec<SharedString> = std::fs::read_dir(p)?
                    .filter_map(|e| e.ok())
                    .filter_map(|e| e.file_name().into_string().ok())
                    .map(SharedString::from)
                    .collect();
                return Ok(entries);
            }
        }

        // Try macOS bundle Resources folder
        let bundle_path = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .map(|p| p.join("Resources").join("assets").join(path));

        if let Some(ref p) = bundle_path {
            if p.is_dir() {
                let entries: Vec<SharedString> = std::fs::read_dir(p)?
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

fn main() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
        cx.activate(true);
        gpui_component::init(cx);

        // Initialize themes from themes directory
        humanboard::settings::init_themes(cx);

        cx.on_action(|_: &Quit, cx| cx.quit());
        // Global shortcuts (always active)
        cx.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("cmd-o", OpenFile, None),
            KeyBinding::new("cmd-=", ZoomIn, None),
            KeyBinding::new("cmd-+", ZoomIn, None),
            KeyBinding::new("cmd--", ZoomOut, None),
            KeyBinding::new("cmd-0", ZoomReset, None),
            KeyBinding::new("cmd-shift-]", NextTab, None),
            KeyBinding::new("cmd-shift-[", PrevTab, None),
            KeyBinding::new("cmd-w", CloseTab, None),
            KeyBinding::new("cmd-n", NewBoard, None),
            KeyBinding::new("cmd-h", GoHome, None),
            KeyBinding::new("cmd-/", ShowShortcuts, None),
            KeyBinding::new("cmd-,", OpenSettings, None),
            // Undo/Redo only on Canvas - code editor handles its own
            KeyBinding::new("cmd-z", Undo, Some("Canvas")),
            KeyBinding::new("cmd-shift-z", Redo, Some("Canvas")),
        ]);

        // Code editor shortcuts (only active when editing code)
        cx.bind_keys([KeyBinding::new("cmd-s", SaveCode, Some("CodeEditor"))]);

        // Canvas-only shortcuts (not active when text input is focused)
        // These use "Canvas" context which is only set when no input is active
        cx.bind_keys([
            KeyBinding::new("backspace", DeleteSelected, Some("Canvas")),
            KeyBinding::new("delete", DeleteSelected, Some("Canvas")),
            KeyBinding::new("cmd-d", DuplicateSelected, Some("Canvas")),
            KeyBinding::new("cmd-a", SelectAll, Some("Canvas")),
            KeyBinding::new("escape", ClosePreview, Some("Canvas")),
            KeyBinding::new("escape", CloseCommandPalette, Some("Canvas")),
            KeyBinding::new("cmd-k", ToggleCommandPalette, Some("Canvas")),
            KeyBinding::new("cmd-v", Paste, Some("Canvas")), // Canvas paste (for images)
            // Arrow keys to nudge selected items
            KeyBinding::new("up", NudgeUp, Some("Canvas")),
            KeyBinding::new("down", NudgeDown, Some("Canvas")),
            KeyBinding::new("left", NudgeLeft, Some("Canvas")),
            KeyBinding::new("right", NudgeRight, Some("Canvas")),
        ]);

        // Shortcuts that work even when input is active (CanvasInputActive context)
        // These are safe shortcuts that don't conflict with text input
        cx.bind_keys([
            KeyBinding::new("escape", CloseCommandPalette, Some("CanvasInputActive")),
            KeyBinding::new("escape", CancelTextboxEdit, Some("CanvasInputActive")),
            KeyBinding::new("cmd-k", ToggleCommandPalette, Some("CanvasInputActive")),
        ]);

        // Landing page shortcuts
        cx.bind_keys([KeyBinding::new(
            "cmd-k",
            ToggleCommandPalette,
            Some("Landing"),
        )]);

        // Command palette navigation (higher priority than Input context)
        cx.bind_keys([
            KeyBinding::new("up", CmdPaletteUp, Some("CommandPalette")),
            KeyBinding::new("down", CmdPaletteDown, Some("CommandPalette")),
            KeyBinding::new("enter", CmdPaletteSelect, Some("CommandPalette")),
        ]);

        // Modal context shortcuts (settings, dialogs)
        cx.bind_keys([
            KeyBinding::new("escape", OpenSettings, Some("Modal")), // Escape closes settings
        ]);

        cx.open_window(
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
            },
            |window, cx| {
                let app_view = cx.new(Humanboard::new);
                cx.new(|cx| gpui_component::Root::new(app_view, window, cx))
            },
        )
        .unwrap();
    });
}
