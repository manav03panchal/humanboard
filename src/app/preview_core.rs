//! Core preview panel methods - opening, PDF/code webviews, markdown/code editing.

use super::{FocusedPane, Humanboard, PreviewPanel, PreviewTab, SplitDirection, TabMeta};
use crate::constants::{DOCK_WIDTH, FOOTER_HEIGHT, HEADER_HEIGHT};
use crate::data::DataSourceDelegate;
use crate::focus::FocusContext;
use crate::pdf_webview::PdfWebView;
use gpui::*;
use gpui_component::input::InputState;
use gpui_component::table::TableState;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::error;

impl Humanboard {
    /// Open a file in the preview panel as a permanent tab
    pub fn open_preview(&mut self, path: PathBuf, window: &mut Window, cx: &mut Context<Self>) {
        self.open_preview_internal(path, false, window, cx);
    }

    /// Open a file as a preview (temporary) tab - will replace existing preview tab
    pub fn open_as_preview_tab(
        &mut self,
        path: PathBuf,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_preview_internal(path, true, window, cx);
    }

    /// Internal method to open preview with preview/permanent mode option
    fn open_preview_internal(
        &mut self,
        path: PathBuf,
        as_preview: bool,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Determine tab type based on extension
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        let meta = TabMeta {
            is_preview: as_preview,
            is_pinned: false,
        };

        let tab = if ext == "md" {
            // Load markdown content
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            // Create editor immediately for edit mode
            let content_clone = content.clone();
            let editor = Some(cx.new(|cx| {
                InputState::new(_window, cx)
                    .code_editor("markdown")
                    .soft_wrap(true)
                    .line_number(true)
                    .default_value(content_clone)
            }));
            PreviewTab::Markdown {
                path: path.clone(),
                content,
                editing: true, // Open in edit mode
                editor,
                meta,
            }
        } else if ext == "pdf" {
            PreviewTab::Pdf {
                path: path.clone(),
                webview: None,
                meta,
            }
        } else if let Some(language) = crate::types::language_from_extension(ext) {
            // Code file - load content
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            PreviewTab::Code {
                path: path.clone(),
                language: language.to_string(),
                content,
                editing: true, // Always editable
                dirty: false,
                editor: None,
                meta,
            }
        } else {
            // Default to PDF for unknown types (or could be Text)
            PreviewTab::Pdf {
                path: path.clone(),
                webview: None,
                meta,
            }
        };

        if let Some(ref mut preview) = self.preview {
            // Check if file is already open in left pane
            if let Some(index) = preview.tabs.iter().position(|t| t.path() == Some(&path)) {
                // File already open - just switch to it and make permanent if not preview mode
                if !as_preview {
                    preview.tabs[index].make_permanent();
                }
                preview.active_tab = index;
                preview.focused_pane = FocusedPane::Left;
                cx.notify();
                return;
            }
            // Check if file is already open in right pane (when split)
            if preview.is_pane_split {
                if let Some(index) = preview.right_tabs.iter().position(|t| t.path() == Some(&path)) {
                    if !as_preview {
                        preview.right_tabs[index].make_permanent();
                    }
                    preview.right_active_tab = index;
                    preview.focused_pane = FocusedPane::Right;
                    cx.notify();
                    return;
                }
            }
            // File not open yet - add it
            if as_preview {
                // Preview mode: replace existing preview tab if one exists
                if let Some(preview_idx) = preview.tabs.iter().position(|t| t.is_preview()) {
                    preview.tabs[preview_idx] = tab;
                    preview.active_tab = preview_idx;
                } else {
                    // No existing preview tab, add new one
                    preview.tabs.push(tab);
                    preview.active_tab = preview.tabs.len() - 1;
                }
            } else {
                // Permanent mode: add new tab
                preview.tabs.push(tab);
                preview.active_tab = preview.tabs.len() - 1;
            }
        } else {
            // Create new preview panel with first tab
            let mut panel = PreviewPanel::new(SplitDirection::Vertical, 0.4);
            panel.tabs.push(tab);
            self.preview = Some(panel);
        }
        cx.notify();
    }

    /// Ensure PDF webviews are created for PDF tabs.
    /// Returns a list of error messages for any webviews that failed to create.
    pub fn ensure_pdf_webview(&mut self, window: &mut Window, cx: &mut App) -> Vec<String> {
        let mut errors = Vec::new();
        if let Some(ref mut preview) = self.preview {
            let active_tab = preview.active_tab;
            let right_active_tab = preview.right_active_tab;
            let is_pane_split = preview.is_pane_split;
            let is_horizontal_split = preview.pane_split_horizontal;

            // Calculate preview panel bounds
            let bounds = window.bounds();
            let window_width = f32::from(bounds.size.width);
            let window_height = f32::from(bounds.size.height);

            // Preview panel position and size
            let header_height = HEADER_HEIGHT;
            let footer_height = FOOTER_HEIGHT;
            let dock_width = DOCK_WIDTH;
            let tab_bar_height = 36.0;

            let main_splitter_size = 8.0; // Splitter between canvas and preview panel
            let (panel_x, panel_y, panel_width, panel_height) = match preview.split {
                SplitDirection::Vertical => {
                    // Account for splitter between canvas and preview
                    let panel_x = dock_width + (window_width - dock_width) * (1.0 - preview.size) + main_splitter_size;
                    let panel_y = header_height;
                    let panel_width = (window_width - dock_width) * preview.size - main_splitter_size;
                    let panel_height = window_height - header_height - footer_height;
                    (panel_x, panel_y, panel_width, panel_height)
                }
                SplitDirection::Horizontal => {
                    let panel_x = dock_width;
                    // Account for splitter between canvas and preview
                    let panel_y = header_height
                        + (window_height - header_height - footer_height) * (1.0 - preview.size) + main_splitter_size;
                    let panel_width = window_width - dock_width;
                    let panel_height =
                        (window_height - header_height - footer_height) * preview.size - main_splitter_size;
                    (panel_x, panel_y, panel_width, panel_height)
                }
            };

            // Calculate pane bounds (left/top pane and right/bottom pane)
            let pane_ratio = preview.pane_ratio;
            let splitter_size = 6.0; // Splitter handle size
            let (
                left_pane_x,
                left_pane_y,
                left_pane_w,
                left_pane_h,
                right_pane_x,
                right_pane_y,
                right_pane_w,
                right_pane_h,
            ) = if is_pane_split {
                if is_horizontal_split {
                    // Top/Bottom split - use pane_ratio for heights
                    let available_height = panel_height - tab_bar_height * 2.0 - splitter_size;
                    let first_pane_height = available_height * pane_ratio;
                    let second_pane_height = available_height * (1.0 - pane_ratio);
                    (
                        panel_x,
                        panel_y + tab_bar_height,
                        panel_width,
                        first_pane_height,
                        panel_x,
                        panel_y
                            + tab_bar_height
                            + first_pane_height
                            + splitter_size
                            + tab_bar_height,
                        panel_width,
                        second_pane_height,
                    )
                } else {
                    // Left/Right split - use pane_ratio for widths
                    let available_width = panel_width - splitter_size;
                    let first_pane_width = available_width * pane_ratio;
                    let second_pane_width = available_width * (1.0 - pane_ratio);
                    (
                        panel_x,
                        panel_y + tab_bar_height,
                        first_pane_width,
                        panel_height - tab_bar_height,
                        panel_x + first_pane_width + splitter_size,
                        panel_y + tab_bar_height,
                        second_pane_width,
                        panel_height - tab_bar_height,
                    )
                }
            } else {
                // Single pane
                (
                    panel_x,
                    panel_y + tab_bar_height,
                    panel_width,
                    panel_height - tab_bar_height,
                    0.0,
                    0.0,
                    0.0,
                    0.0, // Right pane not used
                )
            };

            // Ensure all PDF tabs in left pane have their WebViews created and positioned
            for (idx, tab) in preview.tabs.iter_mut().enumerate() {
                if let PreviewTab::Pdf { path, webview, .. } = tab {
                    if webview.is_none() {
                        match PdfWebView::new(path.clone(), window, cx) {
                            Ok(wv) => {
                                if idx != active_tab {
                                    wv.hide(cx);
                                }
                                *webview = Some(wv);
                            }
                            Err(e) => {
                                let filename = path.file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("PDF");
                                errors.push(format!("Failed to open '{}': {}", filename, e));
                                error!("Failed to create PDF WebView: {}", e);
                            }
                        }
                    }

                    if let Some(wv) = webview {
                        if idx == active_tab {
                            wv.set_bounds(left_pane_x, left_pane_y, left_pane_w, left_pane_h, cx);
                            wv.show(cx);
                        } else {
                            wv.hide(cx);
                        }
                    }
                }
            }

            // Ensure all PDF tabs in right pane have their WebViews created and positioned (when split)
            if is_pane_split {
                for (idx, tab) in preview.right_tabs.iter_mut().enumerate() {
                    if let PreviewTab::Pdf { path, webview, .. } = tab {
                        if webview.is_none() {
                            match PdfWebView::new(path.clone(), window, cx) {
                                Ok(wv) => {
                                    if idx != right_active_tab {
                                        wv.hide(cx);
                                    }
                                    *webview = Some(wv);
                                }
                                Err(e) => {
                                    let filename = path.file_name()
                                        .and_then(|n| n.to_str())
                                        .unwrap_or("PDF");
                                    errors.push(format!("Failed to open '{}': {}", filename, e));
                                    error!("Failed to create PDF WebView for right pane: {}", e);
                                }
                            }
                        }

                        if let Some(wv) = webview {
                            if idx == right_active_tab {
                                wv.set_bounds(
                                    right_pane_x,
                                    right_pane_y,
                                    right_pane_w,
                                    right_pane_h,
                                    cx,
                                );
                                wv.show(cx);
                            } else {
                                wv.hide(cx);
                            }
                        }
                    }
                }
            }
        }
        errors
    }

    /// Ensure code editors are created for code tabs (for syntax-highlighted viewing)
    pub fn ensure_code_editors(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            let is_split = preview.is_pane_split;

            // Ensure code editors for left pane
            for tab in preview.tabs.iter_mut() {
                if let PreviewTab::Code {
                    content,
                    language,
                    editor,
                    dirty,
                    ..
                } = tab
                {
                    if editor.is_none() {
                        // Create editor with syntax highlighting
                        let content_clone = content.clone();
                        let lang = language.clone();
                        *editor = Some(cx.new(|cx| {
                            InputState::new(_window, cx)
                                .code_editor(lang)
                                .line_number(true)
                                .default_value(content_clone)
                        }));
                    } else if let Some(ed) = editor {
                        // Check if content changed (for dirty indicator)
                        let editor_content = ed.read(cx).text().to_string();
                        *dirty = editor_content != *content;
                    }
                }
            }

            // Ensure code editors for right pane (when split)
            if is_split {
                for tab in preview.right_tabs.iter_mut() {
                    if let PreviewTab::Code {
                        content,
                        language,
                        editor,
                        dirty,
                        ..
                    } = tab
                    {
                        if editor.is_none() {
                            let content_clone = content.clone();
                            let lang = language.clone();
                            *editor = Some(cx.new(|cx| {
                                InputState::new(_window, cx)
                                    .code_editor(lang)
                                    .line_number(true)
                                    .default_value(content_clone)
                            }));
                        } else if let Some(ed) = editor {
                            let editor_content = ed.read(cx).text().to_string();
                            *dirty = editor_content != *content;
                        }
                    }
                }
            }
        }
    }

    /// Ensure table states are created for preview panel table tabs
    /// Also syncs dirty changes from preview back to board's data sources
    pub fn ensure_preview_table_states(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // First, sync any dirty data from existing table states back to board
        if let Some(ref mut preview) = self.preview {
            let mut dirty_updates: Vec<(u64, crate::types::DataSource)> = Vec::new();

            // Check left pane tabs for dirty data
            for tab in preview.tabs.iter() {
                if let PreviewTab::Table { data_source_id, table_state: Some(state), .. } = tab {
                    let delegate_ds = state.read(cx).delegate().data_source();
                    if delegate_ds.is_dirty() {
                        dirty_updates.push((*data_source_id, (**delegate_ds).clone()));
                    }
                }
            }

            // Check right pane tabs for dirty data
            for tab in preview.right_tabs.iter() {
                if let PreviewTab::Table { data_source_id, table_state: Some(state), .. } = tab {
                    let delegate_ds = state.read(cx).delegate().data_source();
                    if delegate_ds.is_dirty() {
                        dirty_updates.push((*data_source_id, (**delegate_ds).clone()));
                    }
                }
            }

            // Apply dirty updates to board
            if !dirty_updates.is_empty() {
                if let Some(ref mut board) = self.board {
                    for (ds_id, updated_ds) in dirty_updates {
                        board.data_sources.insert(ds_id, updated_ds);
                    }
                    board.push_history();
                    let _ = board.flush_save();
                }
            }
        }

        // Collect data sources we need for table tabs
        let data_sources_map = if let Some(ref board) = self.board {
            board.data_sources.clone()
        } else {
            return;
        };

        if let Some(ref mut preview) = self.preview {
            let is_split = preview.is_pane_split;

            // Ensure table states for left pane
            for tab in preview.tabs.iter_mut() {
                if let PreviewTab::Table {
                    data_source_id,
                    table_state,
                    ..
                } = tab
                {
                    if table_state.is_none() {
                        if let Some(ds) = data_sources_map.get(data_source_id) {
                            // Create the delegate with a large width for preview panel
                            let delegate = DataSourceDelegate::with_width(Arc::new(ds.clone()), 800.0);
                            let state = cx.new(|cx| TableState::new(delegate, window, cx));
                            *table_state = Some(state);
                        }
                    }
                }
            }

            // Ensure table states for right pane (when split)
            if is_split {
                for tab in preview.right_tabs.iter_mut() {
                    if let PreviewTab::Table {
                        data_source_id,
                        table_state,
                        ..
                    } = tab
                    {
                        if table_state.is_none() {
                            if let Some(ds) = data_sources_map.get(data_source_id) {
                                let delegate = DataSourceDelegate::with_width(Arc::new(ds.clone()), 800.0);
                                let state = cx.new(|cx| TableState::new(delegate, window, cx));
                                *table_state = Some(state);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn toggle_markdown_edit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if let Some(tab) = preview.tabs.get_mut(preview.active_tab) {
                if let PreviewTab::Markdown {
                    editing,
                    content,
                    editor,
                    ..
                } = tab
                {
                    *editing = !*editing;
                    if *editing {
                        // Set focus context to Preview for editor input
                        self.focus.focus(FocusContext::Preview, window);

                        if editor.is_none() {
                            // Create editor with current content - use code_editor for multiline support
                            let content_clone = content.clone();
                            *editor = Some(cx.new(|cx| {
                                InputState::new(window, cx)
                                    .code_editor("markdown")
                                    .soft_wrap(true)
                                    .line_number(true)
                                    .default_value(content_clone)
                            }));
                        }
                        // Focus the editor so user can type immediately
                        if let Some(ed) = editor {
                            ed.update(cx, |state, cx| {
                                state.focus(window, cx);
                            });
                        }
                    } else {
                        // Release focus back to canvas when exiting edit mode
                        self.focus.release(FocusContext::Preview, window);
                    }
                }
            }
        }
        cx.notify();
    }

    pub fn save_markdown(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if let Some(tab) = preview.tabs.get_mut(preview.active_tab) {
                if let PreviewTab::Markdown {
                    path,
                    content,
                    editor,
                    editing,
                    ..
                } = tab
                {
                    if let Some(ed) = editor {
                        let new_content = ed.read(cx).text().to_string();
                        let _ = std::fs::write(path.as_path(), &new_content);
                        if let Some(ref mut board) = self.board {
                            for item in board.items.iter_mut() {
                                if let crate::types::ItemContent::Markdown {
                                    path: item_path,
                                    content: item_content,
                                    ..
                                } = &mut item.content
                                {
                                    if item_path == path {
                                        *item_content = new_content.clone();
                                    }
                                }
                            }
                        }
                        *content = new_content;
                        *editing = false;
                    }
                }
            }
        }
        cx.notify();
    }

    pub fn save_code(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut preview) = self.preview {
            if let Some(tab) = preview.tabs.get_mut(preview.active_tab) {
                match tab {
                    PreviewTab::Code {
                        path,
                        content,
                        editor,
                        dirty,
                        ..
                    } => {
                        if let Some(ed) = editor {
                            let new_content = ed.read(cx).text().to_string();
                            if let Err(e) = std::fs::write(path.as_path(), &new_content) {
                                error!("Failed to save code file: {}", e);
                            }
                            *content = new_content;
                            *dirty = false;
                        }
                    }
                    PreviewTab::Markdown { path, content, editor, editing, .. } => {
                        if *editing {
                            if let Some(ed) = editor {
                                let new_content = ed.read(cx).text().to_string();
                                let _ = std::fs::write(path.as_path(), &new_content);
                                if let Some(ref mut board) = self.board {
                                    for item in board.items.iter_mut() {
                                        if let crate::types::ItemContent::Markdown {
                                            path: item_path,
                                            content: item_content,
                                            ..
                                        } = &mut item.content
                                        {
                                            if item_path == path {
                                                *item_content = new_content.clone();
                                            }
                                        }
                                    }
                                }
                                *content = new_content;
                                *editing = false; // Exit edit mode to show preview
                            }
                        }
                    }
                    PreviewTab::Table { data_source_id, name, table_state, .. } => {
                        // Save the data source to file
                        let ds_id = *data_source_id;
                        let table_name = name.clone();
                        if let Some(ref mut board) = self.board {
                            match board.save_data_source_to_file(ds_id) {
                                Ok(_path) => {
                                    // Clear dirty flag on the delegate's data source
                                    if let Some(state) = table_state {
                                        state.update(cx, |state, _cx| {
                                            let delegate = state.delegate_mut();
                                            // Get the clean data source from board and update delegate
                                            if let Some(clean_ds) = board.data_sources.get(&ds_id) {
                                                delegate.set_data_source(Arc::new(clean_ds.clone()));
                                            }
                                        });
                                    }
                                    self.show_toast(crate::notifications::Toast::success(
                                        format!("Saved {}", table_name)
                                    ));
                                }
                                Err(e) => {
                                    self.show_toast(crate::notifications::Toast::error(e));
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        cx.notify();
    }

    /// Open a table in the preview panel
    pub fn open_table_preview(
        &mut self,
        data_source_id: u64,
        name: String,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let meta = TabMeta {
            is_preview: false,
            is_pinned: false,
        };

        let tab = PreviewTab::Table {
            data_source_id,
            name: name.clone(),
            table_state: None, // Will be created when rendering
            meta,
        };

        if let Some(ref mut preview) = self.preview {
            // Check if table is already open (by data_source_id)
            if let Some(index) = preview.tabs.iter().position(|t| {
                matches!(t, PreviewTab::Table { data_source_id: id, .. } if *id == data_source_id)
            }) {
                preview.active_tab = index;
                preview.focused_pane = FocusedPane::Left;
                cx.notify();
                return;
            }
            // Check right pane too
            if preview.is_pane_split {
                if let Some(index) = preview.right_tabs.iter().position(|t| {
                    matches!(t, PreviewTab::Table { data_source_id: id, .. } if *id == data_source_id)
                }) {
                    preview.right_active_tab = index;
                    preview.focused_pane = FocusedPane::Right;
                    cx.notify();
                    return;
                }
            }
            // Add new tab
            preview.tabs.push(tab);
            preview.active_tab = preview.tabs.len() - 1;
        } else {
            // Create new preview panel with first tab
            let mut panel = PreviewPanel::new(SplitDirection::Vertical, 0.4);
            panel.tabs.push(tab);
            self.preview = Some(panel);
        }
        cx.notify();
    }
}
