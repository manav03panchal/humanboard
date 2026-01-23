//! Application state - the Humanboard struct definition.

use super::{CmdPaletteMode, CountdownState, PreviewPanel, SettingsTab, StorageLocation};
use crate::animations::ModalAnimationState;
use crate::audio_webview::AudioWebView;
use crate::background::BackgroundExecutor;
use crate::board::Board;
use crate::board_index::BoardIndex;
use crate::data::{DataSourceDelegate, VirtualScrollState};
use crate::focus::FocusManager;
use crate::hit_testing::HitTester;
use crate::notifications::ToastManager;
use crate::perf::PerfMonitor;
use crate::settings::Settings;
use crate::settings_watcher::SettingsWatcher;
use crate::types::ToolType;
use crate::video_webview::VideoWebView;
use crate::youtube_webview::YouTubeWebView;
use gpui::*;
use gpui_component::input::InputState;
use gpui_component::table::TableState;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

use super::PanAnimation;

/// State for the chart configuration modal
#[derive(Clone)]
pub struct ChartConfigModal {
    /// ID of the table item being configured
    pub table_item_id: u64,
    /// Data source ID for the table
    pub data_source_id: u64,
    /// Selected chart type
    pub chart_type: crate::types::ChartType,
    /// Selected X axis column index
    pub x_column: usize,
    /// Selected Y axis column indices
    pub y_columns: Vec<usize>,
    /// Column names for display
    pub column_names: Vec<String>,
    /// Aggregation method for duplicate X values
    pub aggregation: crate::types::AggregationType,
    /// Sort order for chart data
    pub sort_order: crate::types::SortOrder,
}

impl ChartConfigModal {
    pub fn new(table_item_id: u64, data_source_id: u64, column_names: Vec<String>) -> Self {
        Self {
            table_item_id,
            data_source_id,
            chart_type: crate::types::ChartType::Bar,
            x_column: 0,
            y_columns: if column_names.len() > 1 { vec![1] } else { vec![0] },
            column_names,
            aggregation: crate::types::AggregationType::default(),
            sort_order: crate::types::SortOrder::default(),
        }
    }
}

pub struct Humanboard {
    // View state
    pub view: super::AppView,
    pub board_index: BoardIndex,

    // Landing page state
    pub editing_board_id: Option<String>,
    pub edit_input: Option<Entity<InputState>>,
    pub deleting_board_id: Option<String>,

    // Board creation modal state
    pub show_create_board_modal: bool,
    pub create_board_input: Option<Entity<InputState>>,
    pub create_board_location: StorageLocation,
    pub create_board_backdrop_clicked: bool,

    // Trash visibility on landing page
    pub show_trash: bool,

    // Board state (only populated when view is Board)
    pub board: Option<Board>,
    pub dragging: bool,
    pub last_mouse_pos: Option<Point<Pixels>>,
    pub dragging_item: Option<u64>,
    pub item_drag_offset: Option<Point<Pixels>>,
    pub resizing_item: Option<u64>,
    pub resize_start_size: Option<(f32, f32)>,
    pub resize_start_pos: Option<Point<Pixels>>,
    pub resize_start_font_size: Option<f32>,
    pub selected_items: HashSet<u64>,

    // Marquee selection state
    pub marquee_start: Option<Point<Pixels>>,
    pub marquee_current: Option<Point<Pixels>>,

    pub frame_times: Vec<Duration>,
    pub last_frame: Instant,
    pub frame_count: u64,
    /// Focus manager for handling focus across different contexts
    pub focus: FocusManager,
    pub preview: Option<PreviewPanel>,
    pub dragging_tab: Option<usize>,    // Index of tab being dragged (only set after threshold)
    pub tab_drag_target: Option<usize>, // Target position for tab drop
    pub tab_drag_split_zone: Option<super::SplitDropZone>, // Drop zone for creating split
    pub tab_drag_position: Option<Point<Pixels>>, // Current drag position for ghost
    pub tab_drag_pending: Option<(usize, Point<Pixels>, bool)>, // (tab_index, start_pos, is_left_pane) - pending drag before threshold
    pub preview_search: Option<Entity<InputState>>, // Search input for preview panel
    pub preview_search_query: String,   // Current search query
    pub preview_search_matches: Vec<(usize, usize)>, // (line, column) positions of matches
    pub preview_search_current: usize,  // Current match index
    pub dragging_splitter: bool,
    pub splitter_drag_start: Option<Point<Pixels>>,
    pub dragging_pane_splitter: bool, // Dragging the splitter between split panes
    pub last_drop_pos: Option<Point<Pixels>>,
    pub file_drop_rx: Option<Receiver<(Point<Pixels>, Vec<PathBuf>)>>,

    // UI overlays
    pub show_shortcuts: bool,
    pub command_palette: Option<Entity<InputState>>, // Command palette input
    pub pending_command: Option<String>, // Command to execute (deferred until we have window access)
    pub search_results: Vec<(u64, String)>, // Search results: (item_id, display_name)
    pub selected_result: usize,          // Currently selected search result index
    pub cmd_palette_mode: CmdPaletteMode, // Current mode: items or themes

    // YouTube WebViews (keyed by item ID)
    pub youtube_webviews: HashMap<u64, YouTubeWebView>,

    // Audio WebViews (keyed by item ID)
    pub audio_webviews: HashMap<u64, AudioWebView>,

    // Video WebViews (keyed by item ID)
    pub video_webviews: HashMap<u64, VideoWebView>,

    // Webview lifecycle tracking - when items went out of viewport (for delayed unload)
    pub webview_out_of_range_since: HashMap<u64, std::time::Instant>,

    // Settings
    pub settings: Settings,
    pub show_settings: bool,
    pub settings_backdrop_clicked: bool,
    pub settings_tab: SettingsTab,
    pub settings_theme_index: usize,
    pub settings_theme_scroll: ScrollHandle,
    pub settings_font_index: usize,
    pub settings_font_scroll: ScrollHandle,

    // Modal focus trap state
    pub modal_focus_index: usize, // Current focus index within modal (for Tab cycling)

    // Toast notifications
    pub toast_manager: ToastManager,

    // Preview tab scroll handles (left/right for split pane)
    pub preview_tab_scroll: ScrollHandle,
    pub preview_right_tab_scroll: ScrollHandle,

    // Command palette scroll handle
    pub cmd_palette_scroll: ScrollHandle,

    // Pan animation state
    pub pan_animation: Option<PanAnimation>,

    // Modal animation state
    pub modal_animations: ModalAnimationState,

    // Tool dock state
    pub selected_tool: ToolType,
    pub drawing_start: Option<Point<Pixels>>, // Start position for drawing shapes/arrows
    pub drawing_current: Option<Point<Pixels>>, // Current position while drawing (for preview)
    pub editing_textbox_id: Option<u64>,      // ID of textbox being edited
    pub textbox_input: Option<Entity<gpui_component::input::InputState>>, // Input for editing textbox
    pub pending_textbox_drag: Option<(u64, Point<Pixels>)>, // Deferred drag for textboxes (to allow double-click)

    // Table cell editing
    pub editing_table_cell: Option<(u64, usize, usize)>, // (table_item_id, row, col)
    pub table_cell_input: Option<Entity<gpui_component::input::InputState>>,

    // Table virtual scrolling (keyed by table item ID)
    pub table_scroll_states: HashMap<u64, VirtualScrollState>,

    // gpui-component Table states (keyed by table item ID)
    pub table_states: HashMap<u64, Entity<TableState<DataSourceDelegate>>>,

    // Chart configuration modal state
    pub chart_config_modal: Option<ChartConfigModal>,

    // Hit testing
    pub hit_tester: HitTester,

    // Performance monitoring
    pub perf_monitor: PerfMonitor,

    // Background task executor
    pub background: BackgroundExecutor,

    // Settings file watcher for hot-reload
    pub settings_watcher: Option<SettingsWatcher>,

    // Home screen countdown state
    pub countdown: Option<CountdownState>,
}
