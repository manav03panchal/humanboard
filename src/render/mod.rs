//! Render module - modular rendering components for the UI.
//!
//! This module is split into focused submodules for better maintainability:
//! - `canvas`: Canvas and item rendering
//! - `preview`: PDF preview panel rendering
//! - `ui`: UI components (footer, tabs, splitter)

mod canvas;
mod preview;
mod ui;

pub use canvas::*;
pub use preview::*;
pub use ui::*;

use gpui::*;

use crate::actions::{
    ClosePreview, CloseTab, DeleteSelected, NextPage, NextTab, OpenFile, PdfZoomIn, PdfZoomOut,
    PdfZoomReset, PrevPage, PrevTab, Redo, ToggleSplit, Undo, ZoomIn, ZoomOut, ZoomReset,
};
use crate::app::{Humanboard, SplitDirection};

/// Main render implementation for Humanboard
impl Render for Humanboard {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Update FPS tracking
        self.update_fps();

        // Poll for file picker results
        self.poll_file_picker(cx);

        // Ensure PDF WebViews are created if preview is active
        if self.preview.is_some() {
            self.ensure_pdf_webview(window, cx);
        }

        // Request next frame
        cx.notify();

        // Build the UI
        self.build_ui(window, cx)
    }
}

impl Humanboard {
    /// Poll for file picker results from async channel
    fn poll_file_picker(&mut self, cx: &mut Context<Self>) {
        if let Some(rx) = &self.file_drop_rx {
            if let Ok((pos, paths)) = rx.try_recv() {
                self.board.handle_file_drop(pos, paths);
                self.file_drop_rx = None;
                cx.notify();
            }
        }
    }

    /// Build the complete UI
    fn build_ui(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> Div {
        let canvas_offset = self.board.canvas_offset;
        let zoom = self.board.zoom;
        let fps = self.calculate_fps();
        let frame_count = self.frame_count;
        let items = self.board.items.clone();
        let item_count = items.len();
        let selected_item_id = self.selected_item;
        let selected_item_name = self.get_selected_item_name();

        // Create base container with event handlers
        let base = self.create_base_container(cx);

        // Render based on preview state
        let content = if let Some(ref preview) = self.preview {
            self.render_with_preview(
                base,
                canvas_offset,
                zoom,
                items,
                selected_item_id,
                preview.split,
                preview.size,
                &preview.tabs,
                preview.active_tab,
                cx,
            )
        } else {
            self.render_canvas_only(base, canvas_offset, zoom, items, selected_item_id)
        };

        // Add footer bar
        content.child(render_footer_bar(
            fps,
            frame_count,
            item_count,
            zoom,
            canvas_offset,
            selected_item_name,
        ))
    }

    /// Get the display name of the currently selected item
    fn get_selected_item_name(&self) -> Option<String> {
        self.selected_item.and_then(|id| {
            self.board
                .items
                .iter()
                .find(|i| i.id == id)
                .map(|i| i.content.display_name())
        })
    }

    /// Create the base container with all event handlers attached
    fn create_base_container(&mut self, cx: &mut Context<Self>) -> Div {
        div()
            .size_full()
            .track_focus(&self.focus_handle)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event, window, cx| {
                    this.focus_handle.focus(window);
                    this.handle_mouse_down(event, window, cx);
                }),
            )
            .on_mouse_up(MouseButton::Left, cx.listener(Humanboard::handle_mouse_up))
            .on_mouse_move(cx.listener(Humanboard::handle_mouse_move))
            .on_scroll_wheel(cx.listener(Humanboard::handle_scroll))
            // Action handlers
            .on_action(cx.listener(|this, _: &OpenFile, window, cx| this.open_file(window, cx)))
            .on_action(cx.listener(|this, _: &ZoomIn, window, cx| this.zoom_in(window, cx)))
            .on_action(cx.listener(|this, _: &ZoomOut, window, cx| this.zoom_out(window, cx)))
            .on_action(cx.listener(|this, _: &ZoomReset, _, cx| this.zoom_reset(cx)))
            .on_action(cx.listener(|this, _: &DeleteSelected, _, cx| this.delete_selected(cx)))
            .on_action(cx.listener(|this, _: &Undo, _, cx| this.undo(cx)))
            .on_action(cx.listener(|this, _: &Redo, _, cx| this.redo(cx)))
            .on_action(cx.listener(|this, _: &ClosePreview, _, cx| this.close_preview(cx)))
            .on_action(cx.listener(|this, _: &ToggleSplit, _, cx| this.toggle_split_direction(cx)))
            .on_action(cx.listener(|this, _: &NextPage, _, cx| this.next_page(cx)))
            .on_action(cx.listener(|this, _: &PrevPage, _, cx| this.prev_page(cx)))
            .on_action(cx.listener(|this, _: &PdfZoomIn, _, cx| this.pdf_zoom_in(cx)))
            .on_action(cx.listener(|this, _: &PdfZoomOut, _, cx| this.pdf_zoom_out(cx)))
            .on_action(cx.listener(|this, _: &PdfZoomReset, _, cx| this.pdf_zoom_reset(cx)))
            .on_action(cx.listener(|this, _: &NextTab, _, cx| this.next_tab(cx)))
            .on_action(cx.listener(|this, _: &PrevTab, _, cx| this.prev_tab(cx)))
            .on_action(cx.listener(|this, _: &CloseTab, _, cx| this.close_current_tab(cx)))
            .on_drop(cx.listener(Self::handle_file_drop))
    }

    /// Handle external file drop
    fn handle_file_drop(
        &mut self,
        paths: &ExternalPaths,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(first_path) = paths.paths().first() {
            let drop_pos = self
                .last_drop_pos
                .unwrap_or_else(|| self.calculate_canvas_center(window));
            self.board
                .handle_file_drop(drop_pos, vec![first_path.clone()]);
            cx.notify();
        }
    }

    /// Render with only the canvas (no preview panel)
    fn render_canvas_only(
        &self,
        base: Div,
        canvas_offset: Point<Pixels>,
        zoom: f32,
        items: Vec<crate::types::CanvasItem>,
        selected_item_id: Option<u64>,
    ) -> Div {
        base.pb(px(28.0)).child(render_canvas_area(
            canvas_offset,
            zoom,
            items.clone(),
            items,
            selected_item_id,
        ))
    }

    /// Render with preview panel (split view)
    fn render_with_preview(
        &self,
        base: Div,
        canvas_offset: Point<Pixels>,
        zoom: f32,
        items: Vec<crate::types::CanvasItem>,
        selected_item_id: Option<u64>,
        split: SplitDirection,
        panel_size: f32,
        tabs: &[crate::app::PdfTab],
        active_tab: usize,
        cx: &mut Context<Self>,
    ) -> Div {
        use gpui::DefiniteLength::Fraction;

        let canvas_size = 1.0 - panel_size;

        match split {
            SplitDirection::Vertical => base
                .flex()
                .flex_row()
                .pb(px(28.0))
                .child(
                    div()
                        .flex_shrink_0()
                        .w(Fraction(canvas_size))
                        .h_full()
                        .child(render_canvas_area(
                            canvas_offset,
                            zoom,
                            items.clone(),
                            items,
                            selected_item_id,
                        )),
                )
                .child(render_splitter(SplitDirection::Vertical))
                .child(render_preview_container(
                    Some(Fraction(panel_size)),
                    None,
                    tabs,
                    active_tab,
                    cx,
                )),
            SplitDirection::Horizontal => base
                .flex()
                .flex_col()
                .pb(px(28.0))
                .child(
                    div()
                        .flex_shrink_0()
                        .h(Fraction(canvas_size))
                        .w_full()
                        .child(render_canvas_area(
                            canvas_offset,
                            zoom,
                            items.clone(),
                            items,
                            selected_item_id,
                        )),
                )
                .child(render_splitter(SplitDirection::Horizontal))
                .child(render_preview_container(
                    None,
                    Some(Fraction(panel_size)),
                    tabs,
                    active_tab,
                    cx,
                )),
        }
    }
}
