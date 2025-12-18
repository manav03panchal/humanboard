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
pub const DOCK_WIDTH: f32 = 48.0;

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
