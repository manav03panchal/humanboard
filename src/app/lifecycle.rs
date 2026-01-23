//! Application lifecycle - initialization and cleanup methods.

use super::{AppView, CmdPaletteMode, CountdownState, Humanboard, SettingsTab, StorageLocation};
use crate::animations::ModalAnimationState;
use crate::background::BackgroundExecutor;
use crate::board_index::BoardIndex;
use crate::focus::FocusManager;
use crate::hit_testing::HitTester;
use crate::notifications::{Toast, ToastManager};
use crate::perf::PerfMonitor;
use crate::settings::Settings;
use crate::settings_watcher::{SettingsEvent, SettingsWatcher};
use crate::types::ToolType;
use gpui::*;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

impl Humanboard {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let board_index = BoardIndex::load();

        // Check if onboarding has been completed
        let initial_view = if crate::settings::is_onboarding_completed() {
            AppView::Landing
        } else {
            AppView::Onboarding
        };

        Self {
            view: initial_view,
            board_index,
            editing_board_id: None,
            edit_input: None,
            deleting_board_id: None,
            show_create_board_modal: false,
            create_board_input: None,
            create_board_location: StorageLocation::default(),
            create_board_backdrop_clicked: false,
            show_trash: false,
            board: None,
            dragging: false,
            last_mouse_pos: None,
            dragging_item: None,
            item_drag_offset: None,
            resizing_item: None,
            resize_start_size: None,
            resize_start_pos: None,
            resize_start_font_size: None,
            selected_items: HashSet::new(),
            marquee_start: None,
            marquee_current: None,
            frame_times: Vec::with_capacity(60),
            last_frame: Instant::now(),
            frame_count: 0,
            focus: FocusManager::new(cx),
            preview: None,
            dragging_tab: None,
            tab_drag_target: None,
            tab_drag_split_zone: None,
            tab_drag_position: None,
            tab_drag_pending: None,
            preview_search: None,
            preview_search_query: String::new(),
            preview_search_matches: Vec::new(),
            preview_search_current: 0,
            dragging_splitter: false,
            splitter_drag_start: None,
            dragging_pane_splitter: false,
            last_drop_pos: None,
            file_drop_rx: None,
            show_shortcuts: false,
            command_palette: None,
            pending_command: None,
            search_results: Vec::new(),
            selected_result: 0,
            cmd_palette_mode: CmdPaletteMode::default(),
            youtube_webviews: HashMap::new(),
            audio_webviews: HashMap::new(),
            video_webviews: HashMap::new(),
            webview_out_of_range_since: HashMap::new(),

            settings: Settings::load(),
            show_settings: false,
            settings_backdrop_clicked: false,
            settings_tab: SettingsTab::default(),
            settings_theme_index: 0,
            settings_theme_scroll: ScrollHandle::new(),
            settings_font_index: 0,
            settings_font_scroll: ScrollHandle::new(),
            modal_focus_index: 0,

            toast_manager: ToastManager::new(),
            preview_tab_scroll: ScrollHandle::new(),
            preview_right_tab_scroll: ScrollHandle::new(),
            cmd_palette_scroll: ScrollHandle::new(),
            pan_animation: None,
            modal_animations: ModalAnimationState::default(),
            selected_tool: ToolType::default(),
            drawing_start: None,
            drawing_current: None,
            editing_textbox_id: None,
            textbox_input: None,
            pending_textbox_drag: None,
            editing_table_cell: None,
            table_cell_input: None,
            chart_config_modal: None,
            table_scroll_states: HashMap::new(),
            table_states: HashMap::new(),
            hit_tester: HitTester::new(),
            perf_monitor: PerfMonitor::new(),
            background: BackgroundExecutor::with_default_workers(),
            settings_watcher: crate::settings_watcher::default_settings_path()
                .and_then(|p| SettingsWatcher::new(p).ok()),
            countdown: Some(CountdownState::until_midnight()),
        }
    }

    /// Check for settings file changes and reload if needed.
    pub fn check_settings_reload(&mut self, cx: &mut Context<Self>) {
        if let Some(ref mut watcher) = self.settings_watcher {
            if let Some(event) = watcher.poll() {
                match event {
                    SettingsEvent::Modified | SettingsEvent::Created => {
                        tracing::info!("Settings file changed, reloading...");
                        // Reload settings
                        self.settings = Settings::load();
                        self.toast_manager.push(Toast::info("Settings reloaded"));
                        cx.notify();
                    }
                    SettingsEvent::Deleted => {
                        tracing::warn!("Settings file deleted");
                        self.toast_manager
                            .push(Toast::warning("Settings file deleted"));
                    }
                    SettingsEvent::Error(e) => {
                        tracing::error!("Settings watch error: {}", e);
                    }
                }
            }
        }
    }

    /// Returns true if a code editor is currently in edit mode
    pub fn is_code_editing(&self) -> bool {
        self.preview
            .as_ref()
            .and_then(|p| p.tabs.get(p.active_tab))
            .map(|tab| tab.is_editing())
            .unwrap_or(false)
    }
}
