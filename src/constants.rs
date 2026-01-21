//! Application-wide constants.
//!
//! Centralizes magic numbers and layout values to make the codebase
//! more maintainable and self-documenting.

// ============================================================================
// Layout Constants
// ============================================================================

/// Height of the header bar in pixels
pub const HEADER_HEIGHT: f32 = 40.0;

/// Width of the tool dock (left sidebar) in pixels
pub const DOCK_WIDTH: f32 = 44.0;

/// Height of the footer bar in pixels
pub const FOOTER_HEIGHT: f32 = 28.0;

/// Width of the preview splitter drag handle in pixels
pub const SPLITTER_WIDTH: f32 = 16.0;

/// Minimum hit area for interactive elements
pub const MIN_HIT_AREA: f32 = 8.0;

// ============================================================================
// Item Defaults
// ============================================================================

/// Default font size for text boxes
pub const DEFAULT_FONT_SIZE: f32 = 16.0;

/// Minimum font size for text boxes
pub const MIN_FONT_SIZE: f32 = 8.0;

/// Maximum font size for text boxes
pub const MAX_FONT_SIZE: f32 = 200.0;

/// Maximum dimension for images (scaled down if larger)
pub const MAX_IMAGE_DIMENSION: f32 = 800.0;

/// Default image fallback size
pub const DEFAULT_IMAGE_SIZE: (f32, f32) = (800.0, 600.0);

/// Default text box minimum size
pub const MIN_TEXTBOX_SIZE: (f32, f32) = (100.0, 40.0);

/// Default shape minimum size
pub const MIN_SHAPE_SIZE: (f32, f32) = (30.0, 30.0);

// ============================================================================
// Animation & Timing
// ============================================================================

/// Save debounce delay in milliseconds
pub const SAVE_DEBOUNCE_MS: u64 = 500;

/// Maximum undo history states to keep
pub const MAX_HISTORY_STATES: usize = 50;

/// Pan animation duration in seconds
pub const PAN_ANIMATION_DURATION: f64 = 0.3;

// ============================================================================
// Zoom & Pan
// ============================================================================

/// Minimum zoom level
pub const MIN_ZOOM: f32 = 0.1;

/// Maximum zoom level
pub const MAX_ZOOM: f32 = 5.0;

/// Default zoom level
pub const DEFAULT_ZOOM: f32 = 1.0;

/// Zoom step for scroll wheel
pub const ZOOM_STEP: f32 = 0.1;

// ============================================================================
// Preview Panel
// ============================================================================

/// Default preview panel size (as fraction of window)
pub const DEFAULT_PREVIEW_SIZE: f32 = 0.4;

/// Minimum preview panel size
pub const MIN_PREVIEW_SIZE: f32 = 0.2;

/// Maximum preview panel size
pub const MAX_PREVIEW_SIZE: f32 = 0.8;

// ============================================================================
// Colors (default hex values)
// ============================================================================

/// Default text color (white)
pub const DEFAULT_TEXT_COLOR: &str = "#ffffff";

/// Default arrow color (white)
pub const DEFAULT_ARROW_COLOR: &str = "#ffffff";

/// Default shape border color (white)
pub const DEFAULT_BORDER_COLOR: &str = "#ffffff";

// ============================================================================
// Arrow & Shape Defaults
// ============================================================================

/// Default arrow thickness
pub const DEFAULT_ARROW_THICKNESS: f32 = 2.0;

/// Minimum arrow thickness
pub const MIN_ARROW_THICKNESS: f32 = 1.0;

/// Maximum arrow thickness
pub const MAX_ARROW_THICKNESS: f32 = 20.0;

/// Default shape border width
pub const DEFAULT_BORDER_WIDTH: f32 = 2.0;

/// Minimum shape border width
pub const MIN_BORDER_WIDTH: f32 = 0.0;

/// Maximum shape border width
pub const MAX_BORDER_WIDTH: f32 = 50.0;

// ============================================================================
// Viewport Culling
// ============================================================================

/// Margin in pixels around viewport for culling (prevents pop-in at edges)
pub const CULLING_MARGIN: f32 = 50.0;

// ============================================================================
// Memory Optimization - Webview Lifecycle
// ============================================================================

/// Distance from viewport edge (in canvas units) at which webviews are destroyed
pub const WEBVIEW_UNLOAD_DISTANCE: f32 = 2000.0;

/// Distance from viewport edge (in canvas units) at which webviews are preloaded
pub const WEBVIEW_PRELOAD_DISTANCE: f32 = 500.0;

/// Minimum time (in milliseconds) a webview must be out of range before unloading
/// 5 minutes - preserves playback state for reasonable pan-away durations
pub const WEBVIEW_UNLOAD_DELAY_MS: u64 = 300_000;

// ============================================================================
// UI Spacing & Visual Consistency
// ============================================================================

/// Standard border radius for small elements (buttons, badges)
pub const BORDER_RADIUS_SM: f32 = 4.0;

/// Standard border radius for medium elements (cards, inputs)
pub const BORDER_RADIUS_MD: f32 = 6.0;

/// Standard border radius for large elements (modals, panels)
pub const BORDER_RADIUS_LG: f32 = 10.0;

/// Standard border radius for extra-large elements (full cards)
pub const BORDER_RADIUS_XL: f32 = 12.0;

/// Standard padding for compact elements
pub const PADDING_XS: f32 = 4.0;

/// Standard padding for small elements
pub const PADDING_SM: f32 = 8.0;

/// Standard padding for medium elements
pub const PADDING_MD: f32 = 12.0;

/// Standard padding for large elements
pub const PADDING_LG: f32 = 16.0;

/// Standard gap for tight spacing
pub const GAP_XS: f32 = 2.0;

/// Standard gap for small spacing
pub const GAP_SM: f32 = 4.0;

/// Standard gap for medium spacing
pub const GAP_MD: f32 = 8.0;

/// Standard gap for large spacing
pub const GAP_LG: f32 = 12.0;

/// Icon size for small icons
pub const ICON_SIZE_SM: f32 = 12.0;

/// Icon size for medium icons
pub const ICON_SIZE_MD: f32 = 14.0;

/// Icon size for large icons
pub const ICON_SIZE_LG: f32 = 16.0;

/// Standard button height (small)
pub const BUTTON_HEIGHT_SM: f32 = 28.0;

/// Standard button height (medium)
pub const BUTTON_HEIGHT_MD: f32 = 32.0;

/// Standard button height (large)
pub const BUTTON_HEIGHT_LG: f32 = 40.0;
