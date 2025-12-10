use gpui::*;
use humanboard::actions::{
    ClosePreview, DeleteSelected, NextPage, PdfZoomIn, PdfZoomOut, PdfZoomReset, PrevPage, Quit,
    Redo, ToggleSplit, Undo, ZoomIn, ZoomOut, ZoomReset,
};
use humanboard::app::Humanboard;

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("cmd-=", ZoomIn, None),
            KeyBinding::new("cmd-+", ZoomIn, None),
            KeyBinding::new("cmd--", ZoomOut, None),
            KeyBinding::new("cmd-0", ZoomReset, None),
            KeyBinding::new("backspace", DeleteSelected, None),
            KeyBinding::new("delete", DeleteSelected, None),
            KeyBinding::new("cmd-z", Undo, None),
            KeyBinding::new("cmd-shift-z", Redo, None),
            KeyBinding::new("escape", ClosePreview, None),
            KeyBinding::new("t", ToggleSplit, None),
            KeyBinding::new("right", NextPage, None),
            KeyBinding::new("left", PrevPage, None),
            KeyBinding::new(".", NextPage, None),
            KeyBinding::new(",", PrevPage, None),
            KeyBinding::new("=", PdfZoomIn, None),
            KeyBinding::new("+", PdfZoomIn, None),
            KeyBinding::new("-", PdfZoomOut, None),
            KeyBinding::new("0", PdfZoomReset, None),
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
                    appears_transparent: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| cx.new(Humanboard::new),
        )
        .unwrap();
    });
}
