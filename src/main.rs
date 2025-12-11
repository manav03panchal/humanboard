use gpui::*;
use humanboard::actions::{
    ClosePreview, CloseTab, CommandPalette, DeleteSelected, GoHome, NewBoard, NextTab, OpenFile,
    OpenSettings, Paste, PrevTab, Quit, Redo, ShowShortcuts, Undo, ZoomIn, ZoomOut, ZoomReset,
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

        // Asset not found - this is normal for optional assets
        Ok(None)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let assets_path = std::env::current_dir()
            .ok()
            .map(|p| p.join("assets").join(path));

        if let Some(path) = assets_path {
            if path.is_dir() {
                let entries: Vec<SharedString> = std::fs::read_dir(&path)?
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
            KeyBinding::new("cmd-z", Undo, None),
            KeyBinding::new("cmd-shift-z", Redo, None),
            KeyBinding::new("cmd-shift-]", NextTab, None),
            KeyBinding::new("cmd-shift-[", PrevTab, None),
            KeyBinding::new("cmd-w", CloseTab, None),
            KeyBinding::new("cmd-n", NewBoard, None),
            KeyBinding::new("cmd-h", GoHome, None),
            KeyBinding::new("cmd-/", ShowShortcuts, None),
            KeyBinding::new(":", CommandPalette, Some("Canvas")),
            KeyBinding::new("cmd-,", OpenSettings, None),
        ]);

        // Canvas-only shortcuts (not active when text input is focused)
        cx.bind_keys([
            KeyBinding::new("backspace", DeleteSelected, Some("Canvas")),
            KeyBinding::new("delete", DeleteSelected, Some("Canvas")),
            KeyBinding::new("escape", ClosePreview, Some("Canvas")),
            KeyBinding::new("cmd-v", Paste, Some("Canvas")), // Canvas paste (for images)
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
