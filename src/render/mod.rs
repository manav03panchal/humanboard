//! Render module - modular rendering for Humanboard
//!
//! This module is split into submodules for maintainability:
//! - `canvas`: Canvas and item rendering
//! - `dock`: Tool dock (left sidebar)
//! - `preview`: Preview panel, tabs, splitter
//! - `overlays`: Header, footer, shortcuts, command palette

pub mod canvas;
pub mod dock;
pub mod overlays;
pub mod preview;

// Re-export commonly used items
pub use canvas::{render_canvas, render_canvas_area, render_items};
pub use dock::render_tool_dock;
pub use overlays::{
    render_chart_config_modal, render_command_palette, render_create_board_modal,
    render_footer_bar, render_header_bar, render_settings_modal, render_shortcuts_overlay,
};
pub use preview::{
    render_drag_ghost, render_preview_panel, render_search_bar, render_selected_item_label,
    render_split_drop_zones, render_split_panes, render_splitter, render_tab_bar,
    render_tab_content,
};

use crate::actions::{
    CancelTextboxEdit, CloseCommandPalette, ClosePreview, CloseTab, CmdPaletteDown, CmdPaletteUp,
    CommandPalette, DeleteSelected, DeselectAll, DuplicateSelected, GoBack, GoForward, GoHome,
    MoveTabToOtherPane, NewBoard, NextPage, NextSearchMatch, NextTab, NudgeDown, NudgeLeft,
    NudgeRight, NudgeUp, OpenFile, OpenSettings, Paste, PdfZoomIn, PdfZoomOut, PdfZoomReset,
    PrevPage, PrevSearchMatch, PrevTab, Redo, ReopenClosedTab, SaveCode, SelectAll, ShowShortcuts,
    ToggleCommandPalette, TogglePaneSplit, TogglePreviewSearch, ToggleSplit, ToolArrow, ToolSelect,
    ToolShape, ToolText, Undo, ZoomIn, ZoomOut, ZoomReset,
};
use crate::app::{AppView, Humanboard, SplitDirection};
use crate::focus::FocusContext;
use crate::home::render_home_screen;
use crate::landing::render_landing_page;
use crate::notifications::render_toast_container;
use crate::onboarding::render_onboarding_page;
use gpui::DefiniteLength::Fraction;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::ActiveTheme as _;

/// UI Font used throughout the application
pub const UI_FONT: &str = "Iosevka Nerd Font";

// ============================================================================
// Render implementation for Humanboard
// ============================================================================

impl Render for Humanboard {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Start frame timing
        self.perf_monitor.begin_frame();

        self.update_fps();

        // Process any pending command from Enter key press
        self.process_pending_command(window, cx);

        // Update pan animation and request next frame if still animating
        if self.update_pan_animation() {
            window.request_animation_frame();
        }

        // Update modal animations and request next frame if still animating
        if self.modal_animations.update() {
            window.request_animation_frame();
        }

        // Check if settings close animation is complete
        if self.modal_animations.settings_close_complete() {
            self.show_settings = false;
            self.modal_animations.settings = None;
        }

        // Check if create board close animation is complete
        if self.modal_animations.create_board_close_complete() {
            self.finish_close_create_board();
            self.modal_animations.create_board = None;
        }

        // Check if command palette close animation is complete
        if self.modal_animations.command_palette_close_complete() {
            self.finish_close_command_palette();
            self.modal_animations.command_palette = None;
        }

        // Restore focus to canvas if needed (e.g., after closing command palette via blur)
        self.focus.restore_focus_if_needed(window);

        // Route based on current view
        let content = match &self.view {
            AppView::Onboarding => self.render_onboarding_view(cx),
            AppView::Home => self.render_home_view(window, cx),
            AppView::Landing => self.render_landing_view(cx),
            AppView::Board(_) => self.render_board_view(window, cx),
        };

        // Remove expired toasts
        self.toast_manager.remove_expired();

        // Process completed background tasks
        self.background.process_results();

        // Check for settings file changes
        self.check_settings_reload(cx);

        // Check for debounced save
        if let Some(ref mut board) = self.board {
            if board.should_save() {
                if let Err(e) = board.flush_save() {
                    // Show error toast for save failures with retry option
                    self.toast_manager
                        .push(crate::notifications::Toast::error(format!(
                            "Save failed: {}",
                            e
                        )).with_action(crate::notifications::ToastAction::retry()));
                }
            }
        }

        // Wrap everything in a container with overlays on top
        let bg = cx.theme().background;
        let toasts = self.toast_manager.toasts().to_vec();
        let reduce_motion = crate::settings::app_settings().should_reduce_motion();

        // End frame timing (measures our render logic, not GPUI's paint)
        self.perf_monitor.end_frame();

        div()
            .size_full()
            .bg(bg)
            .font_family(self.settings.font.clone())
            .relative()
            .child(content)
            .when(self.show_shortcuts, |d| {
                d.child(render_shortcuts_overlay(cx))
            })
            .when(self.show_settings, |d| {
                d.child(render_settings_modal(
                    &self.settings.theme,
                    &self.settings.font,
                    self.settings_theme_index,
                    &self.settings_theme_scroll,
                    self.settings_tab,
                    &self.focus.modal,
                    self.modal_animations.settings_opacity(),
                    cx,
                ))
            })
            .when_some(
                self.create_board_input
                    .as_ref()
                    .filter(|_| self.show_create_board_modal),
                |d, input| {
                    d.child(render_create_board_modal(
                        input,
                        &self.create_board_location,
                        &self.focus.modal,
                        self.modal_animations.create_board_opacity(),
                        cx,
                    ))
                },
            )
            // Chart configuration modal
            .when_some(self.chart_config_modal.as_ref(), |d, modal| {
                d.child(render_chart_config_modal(modal, cx))
            })
            // Toast notifications
            .when(!toasts.is_empty(), |d| {
                d.child(render_toast_container(&toasts, reduce_motion, cx))
            })
    }
}

// ============================================================================
// Humanboard view rendering methods
// ============================================================================

impl Humanboard {
    /// Render the home screen with countdown view
    fn render_home_view(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Div {
        // Request animation frame to update countdown every second
        window.request_animation_frame();

        render_home_screen(self.countdown.as_ref(), cx)
    }

    /// Render the landing page view
    fn render_landing_view(&mut self, cx: &mut Context<Self>) -> Div {
        let deleting_board = self.deleting_board_id.as_ref().and_then(|id| {
            self.board_index
                .get_board(id)
                .map(|meta| (id.as_str(), meta.name.as_str()))
        });

        let is_editing = self.editing_board_id.is_some();

        div()
            .size_full()
            .track_focus(&self.focus.landing)
            .key_context(FocusContext::KEY_LANDING)
            // Only steal focus when not editing (so Input can receive focus)
            .when(!is_editing, |d| {
                d.on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, window, _| {
                        // Use try_focus to respect focus hierarchy
                        this.focus
                            .try_focus(crate::focus::FocusContext::Landing, window);
                    }),
                )
            })
            .on_action(
                cx.listener(|this, _: &NewBoard, window, cx| this.create_new_board(window, cx)),
            )
            .on_action(cx.listener(|this, _: &ShowShortcuts, _, cx| this.toggle_shortcuts(cx)))
            .on_action(
                cx.listener(|this, _: &OpenSettings, window, cx| this.toggle_settings(window, cx)),
            )
            .on_action(cx.listener(|this, _: &ToolSelect, _, cx| {
                this.selected_tool = crate::types::ToolType::Select;
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ToolText, _, cx| {
                this.selected_tool = crate::types::ToolType::Text;
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ToolArrow, _, cx| {
                this.selected_tool = crate::types::ToolType::Arrow;
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ToolShape, _, cx| {
                this.selected_tool = crate::types::ToolType::Shape;
                cx.notify();
            }))
            .child(render_landing_page(
                &self.board_index,
                self.editing_board_id.as_deref(),
                self.edit_input.as_ref(),
                deleting_board,
                self.show_trash,
                cx,
            ))
    }

    /// Render the onboarding welcome view
    fn render_onboarding_view(&mut self, cx: &mut Context<Self>) -> Div {
        render_onboarding_page(cx)
    }

    /// Render the board/canvas view
    fn render_board_view(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Div {
        // Poll for file picker results (from Cmd+O)
        if let Some(rx) = &self.file_drop_rx {
            if let Ok((pos, paths)) = rx.try_recv() {
                if let Some(ref mut board) = self.board {
                    let errors = board.handle_file_drop(pos, paths);
                    // Show toast notifications for any file copy errors
                    for error in errors {
                        self.toast_manager.push(crate::notifications::Toast::error(error)
                            .with_action(crate::notifications::ToastAction::retry()));
                    }
                }
                self.file_drop_rx = None;
                cx.notify();
            }
        }

        // Ensure WebViews and editors are created if preview is active
        if self.preview.is_some() {
            let pdf_errors = self.ensure_pdf_webview(window, cx);
            for error in pdf_errors {
                self.toast_manager.push(crate::notifications::Toast::error(error)
                    .with_action(crate::notifications::ToastAction::reload_webview()));
            }
            self.ensure_code_editors(window, cx);
            self.ensure_preview_table_states(window, cx);
        }

        // Ensure YouTube WebViews are created for any YouTube items
        let youtube_errors = self.ensure_youtube_webviews(window, cx);
        for error in youtube_errors {
            self.toast_manager.push(crate::notifications::Toast::error(error)
                .with_action(crate::notifications::ToastAction::reload_webview()));
        }

        // Ensure Audio WebViews are created for any Audio items
        let audio_errors = self.ensure_audio_webviews(window, cx);
        for error in audio_errors {
            self.toast_manager.push(crate::notifications::Toast::error(error)
                .with_action(crate::notifications::ToastAction::reload_webview()));
        }

        // Ensure Video WebViews are created for any Video items
        let video_errors = self.ensure_video_webviews(window, cx);
        for error in video_errors {
            self.toast_manager.push(crate::notifications::Toast::error(error)
                .with_action(crate::notifications::ToastAction::reload_webview()));
        }

        // Update webview visibility based on canvas viewport
        // This hides webviews that are scrolled out of view to prevent z-index issues
        self.update_webview_visibility(window, cx);

        // Get board data (with fallback defaults if somehow no board)
        let (canvas_offset, zoom, items, item_count, data_sources) = if let Some(ref board) = self.board {
            (
                board.canvas_offset,
                board.zoom,
                board.items.clone(),
                board.items.len(),
                board.data_sources.clone(),
            )
        } else {
            (point(px(0.0), px(0.0)), 1.0, Vec::new(), 0, std::collections::HashMap::new())
        };

        // Ensure TableState entities exist for all table items (for gpui-component Table)
        // Must be called after we have zoom value to calculate correct column widths
        self.ensure_table_states(zoom, window, cx);

        let fps = self.calculate_fps();
        let frame_count = self.frame_count;
        let selected_items = self.selected_items.clone();
        let selected_item_name = if self.selected_items.len() == 1 {
            self.selected_items.iter().next().and_then(|&id| {
                self.board
                    .as_ref()
                    .and_then(|b| b.items.iter().find(|i| i.id == id))
                    .map(|i| i.content.display_name())
            })
        } else if self.selected_items.len() > 1 {
            Some(format!("{} items selected", self.selected_items.len()))
        } else {
            None
        };

        // Marquee selection state
        let marquee = match (self.marquee_start, self.marquee_current) {
            (Some(start), Some(current)) => Some((start, current)),
            _ => None,
        };

        // Drawing preview state (for TextBox, Shape, Arrow while dragging)
        let drawing_preview = match (self.drawing_start, self.drawing_current) {
            (Some(start), Some(current)) => Some((start, current, self.selected_tool)),
            _ => None,
        };

        // Get board name from index
        let board_name = if let AppView::Board(ref id) = self.view {
            self.board_index.get_board(id).map(|m| m.name.clone())
        } else {
            None
        };

        // Extract preview info
        let preview_info = self
            .preview
            .as_ref()
            .map(|p| (p, p.split, p.size, &p.tabs, p.active_tab, p.is_pane_split));

        // Compute canvas viewport size for culling (accounts for dock, header, footer, preview)
        let window_bounds = window.bounds();
        let canvas_viewport_size = {
            use crate::constants::{DOCK_WIDTH, HEADER_HEIGHT, FOOTER_HEIGHT};
            let available_width = f32::from(window_bounds.size.width) - DOCK_WIDTH;
            let available_height = f32::from(window_bounds.size.height) - HEADER_HEIGHT - FOOTER_HEIGHT;

            if let Some(ref preview) = self.preview {
                match preview.split {
                    SplitDirection::Vertical => {
                        let canvas_width = available_width * (1.0 - preview.size);
                        gpui::size(gpui::px(canvas_width), gpui::px(available_height))
                    }
                    SplitDirection::Horizontal => {
                        let canvas_height = available_height * (1.0 - preview.size);
                        gpui::size(gpui::px(available_width), gpui::px(canvas_height))
                    }
                }
            } else {
                gpui::size(gpui::px(available_width), gpui::px(available_height))
            }
        };

        // Check if we should block canvas keyboard shortcuts
        // When input is active, we use a different key context to avoid shortcut conflicts
        let input_active = self.focus.is_input_active();
        let key_context = if input_active {
            FocusContext::KEY_CANVAS_INPUT_ACTIVE
        } else {
            FocusContext::KEY_CANVAS
        };

        let base = div()
            .size_full()
            .track_focus(&self.focus.canvas)
            .key_context(key_context)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event: &MouseDownEvent, window, cx| {
                    // Check if we're clicking in the preview panel area
                    let in_preview_area = if let Some(ref preview) = this.preview {
                        let bounds = window.bounds();
                        match preview.split {
                            SplitDirection::Vertical => {
                                let preview_start_x =
                                    f32::from(bounds.size.width) * (1.0 - preview.size);
                                f32::from(event.position.x) > preview_start_x
                            }
                            SplitDirection::Horizontal => {
                                let preview_start_y =
                                    f32::from(bounds.size.height) * (1.0 - preview.size);
                                f32::from(event.position.y) > preview_start_y
                            }
                        }
                    } else {
                        false
                    };

                    if !in_preview_area {
                        // Only handle canvas clicks if no modal/palette is open
                        // (they have their own click handlers for backdrops)
                        if this.command_palette.is_none() && !this.show_settings {
                            this.focus.force_canvas_focus(window);
                            this.handle_mouse_down(event, window, cx);
                        }
                    }
                }),
            )
            .on_mouse_up(MouseButton::Left, cx.listener(Humanboard::handle_mouse_up))
            .on_mouse_move(cx.listener(Humanboard::handle_mouse_move))
            .on_scroll_wheel(cx.listener(Humanboard::handle_scroll))
            .on_action(cx.listener(|this, _: &GoHome, _, cx| this.go_home(cx)))
            .on_action(cx.listener(|this, _: &OpenFile, window, cx| this.open_file(window, cx)))
            .on_action(cx.listener(|this, _: &ZoomIn, window, cx| this.zoom_in(window, cx)))
            .on_action(cx.listener(|this, _: &ZoomOut, window, cx| this.zoom_out(window, cx)))
            .on_action(cx.listener(|this, _: &ZoomReset, _, cx| this.zoom_reset(cx)))
            .on_action(cx.listener(|this, _: &DeleteSelected, _, cx| this.delete_selected(cx)))
            .on_action(
                cx.listener(|this, _: &DuplicateSelected, _, cx| this.duplicate_selected(cx)),
            )
            .on_action(cx.listener(|this, _: &SelectAll, _, cx| this.select_all(cx)))
            .on_action(cx.listener(|this, _: &DeselectAll, _, cx| this.deselect_all(cx)))
            .on_action(cx.listener(|this, _: &Paste, window, cx| this.paste(window, cx)))
            .on_action(cx.listener(|this, _: &NudgeUp, _, cx| this.nudge_up(cx)))
            .on_action(cx.listener(|this, _: &NudgeDown, _, cx| this.nudge_down(cx)))
            .on_action(cx.listener(|this, _: &NudgeLeft, _, cx| this.nudge_left(cx)))
            .on_action(cx.listener(|this, _: &NudgeRight, _, cx| this.nudge_right(cx)))
            .on_action(cx.listener(|this, _: &Undo, _, cx| this.undo(cx)))
            .on_action(cx.listener(|this, _: &Redo, _, cx| this.redo(cx)))
            .on_action(cx.listener(|this, _: &SaveCode, _, cx| this.save_code(cx)))
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
            .on_action(cx.listener(|this, _: &ReopenClosedTab, _, cx| this.reopen_closed_tab(cx)))
            .on_action(cx.listener(|this, _: &GoBack, _, cx| this.go_back(cx)))
            .on_action(cx.listener(|this, _: &GoForward, _, cx| this.go_forward(cx)))
            .on_action(cx.listener(|this, _: &TogglePreviewSearch, window, cx| {
                this.toggle_preview_search(window, cx)
            }))
            .on_action(cx.listener(|this, _: &NextSearchMatch, _, cx| this.next_search_match(cx)))
            .on_action(cx.listener(|this, _: &PrevSearchMatch, _, cx| this.prev_search_match(cx)))
            .on_action(cx.listener(|this, _: &TogglePaneSplit, _, cx| this.toggle_pane_split(cx)))
            .on_action(
                cx.listener(|this, _: &MoveTabToOtherPane, _, cx| this.move_tab_to_other_pane(cx)),
            )
            .on_action(cx.listener(|this, _: &ShowShortcuts, _, cx| this.toggle_shortcuts(cx)))
            .on_action(cx.listener(|this, _: &CommandPalette, window, cx| {
                this.show_command_palette(window, cx)
            }))
            .on_action(cx.listener(|this, _: &ToggleCommandPalette, window, cx| {
                this.toggle_command_palette(window, cx)
            }))
            .on_action(cx.listener(|this, _: &CloseCommandPalette, window, cx| {
                if this.command_palette.is_some() {
                    this.close_command_palette(window, cx)
                }
            }))
            // Command palette arrow navigation - handlers at root level to catch global keybindings
            .on_action(cx.listener(|this, _: &CmdPaletteUp, _, cx| {
                if this.command_palette.is_some() {
                    this.select_prev_result(cx);
                }
            }))
            .on_action(cx.listener(|this, _: &CmdPaletteDown, _, cx| {
                if this.command_palette.is_some() {
                    this.select_next_result(cx);
                }
            }))
            .on_action(
                cx.listener(|this, _: &OpenSettings, window, cx| this.toggle_settings(window, cx)),
            )
            .on_action(cx.listener(|this, _: &ToolSelect, _, cx| {
                this.selected_tool = crate::types::ToolType::Select;
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ToolText, _, cx| {
                this.selected_tool = crate::types::ToolType::Text;
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ToolArrow, _, cx| {
                this.selected_tool = crate::types::ToolType::Arrow;
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ToolShape, _, cx| {
                this.selected_tool = crate::types::ToolType::Shape;
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &CancelTextboxEdit, window, cx| {
                if this.editing_textbox_id.is_some() {
                    this.cancel_textbox_editing_with_window(window, cx);
                }
            }))
            .on_drop(cx.listener(|this, paths: &ExternalPaths, window, cx| {
                let all_paths: Vec<_> = paths.paths().to_vec();
                if all_paths.is_empty() {
                    return;
                }

                let drop_pos = if let Some(pos) = this.last_drop_pos {
                    pos
                } else {
                    let bounds = window.bounds();
                    let window_size = bounds.size;

                    let (canvas_center_x, canvas_center_y) = if let Some(ref preview) = this.preview
                    {
                        match preview.split {
                            SplitDirection::Vertical => {
                                let canvas_width =
                                    f32::from(window_size.width) * (1.0 - preview.size);
                                (canvas_width / 2.0, f32::from(window_size.height) / 2.0)
                            }
                            SplitDirection::Horizontal => {
                                let canvas_height =
                                    f32::from(window_size.height) * (1.0 - preview.size);
                                (f32::from(window_size.width) / 2.0, canvas_height / 2.0)
                            }
                        }
                    } else {
                        (
                            f32::from(window_size.width) / 2.0,
                            f32::from(window_size.height) / 2.0,
                        )
                    };

                    point(px(canvas_center_x), px(canvas_center_y))
                };

                let count = all_paths.len();
                if let Some(ref mut board) = this.board {
                    board.handle_file_drop(drop_pos, all_paths);
                }
                // Show toast notification
                let msg = if count == 1 {
                    "Added 1 item".to_string()
                } else {
                    format!("Added {} items", count)
                };
                this.show_toast(crate::notifications::Toast::success(msg));
                cx.notify();
            }));

        let selected_tool = self.selected_tool;
        let content = match preview_info {
            Some((preview_ref, split, size, tabs, active_tab, is_pane_split)) => {
                let canvas_size = 1.0 - size;
                let preview_size = size;

                match split {
                    SplitDirection::Vertical => base
                        .flex()
                        .flex_row()
                        .pt(px(40.0))
                        .pb(px(28.0))
                        .child(render_tool_dock(
                            selected_tool,
                            |this, tool, _, cx| {
                                this.selected_tool = tool;
                                cx.notify();
                            },
                            cx,
                        ))
                        // Wrap canvas + splitter + preview in a flex_1 container
                        // so Fraction calculations are based on remaining space after dock
                        .child(
                            div()
                                .flex_1()
                                .h_full()
                                .flex()
                                .flex_row()
                                .overflow_hidden()
                                .child(
                                    div()
                                        .flex_shrink_0()
                                        .w(Fraction(canvas_size))
                                        .h_full()
                                        .child(render_canvas_area(
                                            canvas_offset,
                                            zoom,
                                            &items,
                                            &selected_items,
                                            &self.youtube_webviews,
                                            &self.audio_webviews,
                                            &self.video_webviews,
                                            &data_sources,
                                            &self.table_scroll_states,
                                            &self.table_states,
                                            self.editing_textbox_id,
                                            self.textbox_input.as_ref(),
                                            self.editing_table_cell,
                                            self.table_cell_input.as_ref(),
                                            marquee,
                                            drawing_preview,
                                            canvas_viewport_size,
                                            cx,
                                        )),
                                )
                                .child(render_splitter(SplitDirection::Vertical, cx))
                                .child({
                                    let bg = cx.theme().background;
                                    let container = div()
                                        .flex_shrink_0()
                                        .w(Fraction(preview_size))
                                        .h_full()
                                        .bg(bg)
                                        .flex()
                                        .flex_col()
                                        .overflow_hidden();

                                    if is_pane_split {
                                        // Render split panes
                                        container.child(render_split_panes(
                                            preview_ref,
                                            &self.preview_tab_scroll,
                                            &self.preview_right_tab_scroll,
                                            self.dragging_tab,
                                            self.tab_drag_target,
                                            self.preview_search.as_ref(),
                                            self.preview_search_matches.len(),
                                            self.preview_search_current,
                                            self.tab_drag_split_zone,
                                            cx,
                                        ))
                                    } else {
                                        // Render single pane with drop zones overlay
                                        let dragging = self.dragging_tab;
                                        let split_zone = self.tab_drag_split_zone;
                                        container
                                            .relative()
                                            .child(render_tab_bar(
                                                tabs,
                                                active_tab,
                                                &self.preview_tab_scroll,
                                                self.dragging_tab,
                                                self.tab_drag_target,
                                                true, // is_left_pane (single pane = left)
                                                cx,
                                            ))
                                            // Search bar (when active)
                                            .when_some(
                                                self.preview_search.as_ref(),
                                                |d, search_input| {
                                                    d.child(render_search_bar(
                                                        search_input,
                                                        self.preview_search_matches.len(),
                                                        self.preview_search_current,
                                                        cx,
                                                    ))
                                                },
                                            )
                                            .child(
                                                div()
                                                    .id(ElementId::Name(
                                                        format!("tab-container-v-{}", active_tab)
                                                            .into(),
                                                    ))
                                                    .flex_1()
                                                    .min_h_0()
                                                    .w_full()
                                                    .overflow_hidden()
                                                    .flex()
                                                    .flex_col()
                                                    .relative()
                                                    // Hide content when dragging (webviews overlap everything)
                                                    .when(dragging.is_none(), |d| {
                                                        d.when_some(
                                                            tabs.get(active_tab),
                                                            |d, tab| {
                                                                d.child(render_tab_content(
                                                                    tab, true, active_tab, cx,
                                                                ))
                                                            },
                                                        )
                                                    })
                                                    // Show drop zones when dragging a tab
                                                    .when(dragging.is_some(), |d| {
                                                        d.child(render_split_drop_zones(
                                                            split_zone, cx,
                                                        ))
                                                    }),
                                            )
                                    }
                                }),
                        ),
                    SplitDirection::Horizontal => base
                        .flex()
                        .flex_row()
                        .pt(px(40.0))
                        .pb(px(28.0))
                        .child(render_tool_dock(
                            selected_tool,
                            |this, tool, _, cx| {
                                this.selected_tool = tool;
                                cx.notify();
                            },
                            cx,
                        ))
                        .child(
                            div()
                                .flex_1()
                                .h_full()
                                .flex()
                                .flex_col()
                                .child(
                                    div()
                                        .flex_shrink_0()
                                        .h(Fraction(canvas_size))
                                        .w_full()
                                        .child(render_canvas_area(
                                            canvas_offset,
                                            zoom,
                                            &items,
                                            &selected_items,
                                            &self.youtube_webviews,
                                            &self.audio_webviews,
                                            &self.video_webviews,
                                            &data_sources,
                                            &self.table_scroll_states,
                                            &self.table_states,
                                            self.editing_textbox_id,
                                            self.textbox_input.as_ref(),
                                            self.editing_table_cell,
                                            self.table_cell_input.as_ref(),
                                            marquee,
                                            drawing_preview,
                                            canvas_viewport_size,
                                            cx,
                                        )),
                                )
                                .child(render_splitter(SplitDirection::Horizontal, cx))
                                .child({
                                    let bg = cx.theme().background;
                                    let container = div()
                                        .flex_shrink_0()
                                        .h(Fraction(preview_size))
                                        .w_full()
                                        .bg(bg)
                                        .flex()
                                        .flex_col()
                                        .overflow_hidden();

                                    if is_pane_split {
                                        // Render split panes
                                        container.child(render_split_panes(
                                            preview_ref,
                                            &self.preview_tab_scroll,
                                            &self.preview_right_tab_scroll,
                                            self.dragging_tab,
                                            self.tab_drag_target,
                                            self.preview_search.as_ref(),
                                            self.preview_search_matches.len(),
                                            self.preview_search_current,
                                            self.tab_drag_split_zone,
                                            cx,
                                        ))
                                    } else {
                                        // Render single pane with drop zones overlay
                                        let dragging = self.dragging_tab;
                                        let split_zone = self.tab_drag_split_zone;
                                        container
                                            .relative()
                                            .child(render_tab_bar(
                                                tabs,
                                                active_tab,
                                                &self.preview_tab_scroll,
                                                self.dragging_tab,
                                                self.tab_drag_target,
                                                true, // is_left_pane (single pane = left)
                                                cx,
                                            ))
                                            // Search bar (when active)
                                            .when_some(
                                                self.preview_search.as_ref(),
                                                |d, search_input| {
                                                    d.child(render_search_bar(
                                                        search_input,
                                                        self.preview_search_matches.len(),
                                                        self.preview_search_current,
                                                        cx,
                                                    ))
                                                },
                                            )
                                            .child(
                                                div()
                                                    .id(ElementId::Name(
                                                        format!("tab-container-h-{}", active_tab)
                                                            .into(),
                                                    ))
                                                    .flex_1()
                                                    .min_h_0()
                                                    .w_full()
                                                    .overflow_hidden()
                                                    .flex()
                                                    .flex_col()
                                                    .relative()
                                                    // Hide content when dragging (webviews overlap everything)
                                                    .when(dragging.is_none(), |d| {
                                                        d.when_some(
                                                            tabs.get(active_tab),
                                                            |d, tab| {
                                                                d.child(render_tab_content(
                                                                    tab, true, active_tab, cx,
                                                                ))
                                                            },
                                                        )
                                                    })
                                                    // Show drop zones when dragging a tab
                                                    .when(dragging.is_some(), |d| {
                                                        d.child(render_split_drop_zones(
                                                            split_zone, cx,
                                                        ))
                                                    }),
                                            )
                                    }
                                }),
                        ),
                }
            }
            None => base
                .flex()
                .flex_row()
                .pt(px(40.0))
                .pb(px(28.0))
                .child(render_tool_dock(
                    selected_tool,
                    |this, tool, _, cx| {
                        this.selected_tool = tool;
                        cx.notify();
                    },
                    cx,
                ))
                .child(div().flex_1().h_full().child(render_canvas_area(
                    canvas_offset,
                    zoom,
                    &items,
                    &selected_items,
                    &self.youtube_webviews,
                    &self.audio_webviews,
                    &self.video_webviews,
                    &data_sources,
                    &self.table_scroll_states,
                    &self.table_states,
                    self.editing_textbox_id,
                    self.textbox_input.as_ref(),
                    self.editing_table_cell,
                    self.table_cell_input.as_ref(),
                    marquee,
                    drawing_preview,
                    canvas_viewport_size,
                    cx,
                ))),
        }
        .child(render_footer_bar(
            fps,
            frame_count,
            item_count,
            zoom,
            canvas_offset,
            selected_item_name,
            None,
            self.board.as_ref().is_some_and(|b| b.is_dirty()),
            cx,
        ))
        .child(render_header_bar(
            board_name,
            self.command_palette.as_ref(),
            &self.search_results,
            self.selected_result,
            &self.cmd_palette_scroll,
            self.cmd_palette_mode,
            &self.focus.command_palette,
            cx,
        ));

        // Add drag ghost overlay when dragging a tab
        let content =
            if let (Some(drag_idx), Some(drag_pos)) = (self.dragging_tab, self.tab_drag_position) {
                if let Some(ref preview) = self.preview {
                    if let Some(tab) = preview.tabs.get(drag_idx) {
                        content.child(render_drag_ghost(tab, drag_pos, cx))
                    } else {
                        content
                    }
                } else {
                    content
                }
            } else {
                content
            };

        content
    }
}
