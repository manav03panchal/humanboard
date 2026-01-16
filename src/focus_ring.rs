//! Focus ring utilities for WCAG-compliant visible focus indicators.
//!
//! This module provides helpers to create consistent focus rings across all
//! focusable elements in the application. Focus rings:
//! - Use theme-consistent colors (primary color for visibility)
//! - Meet 3:1 minimum contrast ratio for WCAG compliance
//! - Are implemented via box-shadow to avoid layout shifts

use gpui::{BoxShadow, Hsla, Styled, point, px};
use gpui_component::ActiveTheme;

/// Default focus ring width in pixels
pub const FOCUS_RING_WIDTH: f32 = 2.0;

/// Default focus ring offset (gap between element and ring)
pub const FOCUS_RING_OFFSET: f32 = 1.0;

/// Create a focus ring box shadow using the primary theme color.
///
/// The ring is created using a box-shadow with:
/// - No blur for a crisp outline
/// - Spread radius to create the ring effect
/// - Primary color for theme consistency and visibility
pub fn focus_ring_shadow(color: Hsla) -> Vec<BoxShadow> {
    vec![BoxShadow {
        color,
        offset: point(px(0.), px(0.)),
        blur_radius: px(0.),
        spread_radius: px(FOCUS_RING_WIDTH),
    }]
}

/// Create a focus ring with offset (for elements that need a gap).
///
/// Uses two shadows: an inner transparent one for offset, outer colored one for ring.
pub fn focus_ring_shadow_with_offset(color: Hsla, background: Hsla) -> Vec<BoxShadow> {
    vec![
        // Inner shadow (offset/gap - matches background)
        BoxShadow {
            color: background,
            offset: point(px(0.), px(0.)),
            blur_radius: px(0.),
            spread_radius: px(FOCUS_RING_OFFSET),
        },
        // Outer shadow (the actual ring)
        BoxShadow {
            color,
            offset: point(px(0.), px(0.)),
            blur_radius: px(0.),
            spread_radius: px(FOCUS_RING_OFFSET + FOCUS_RING_WIDTH),
        },
    ]
}

/// Extension trait for adding focus ring styling to elements.
pub trait FocusRingExt: Styled + Sized {
    /// Apply a focus ring shadow to this element.
    ///
    /// # Arguments
    /// * `focused` - Whether the element is currently focused
    /// * `color` - The focus ring color (typically theme primary)
    fn focus_ring(self, focused: bool, color: Hsla) -> Self {
        if focused {
            self.shadow(focus_ring_shadow(color))
        } else {
            self
        }
    }

    /// Apply a focus ring with offset (gap between element and ring).
    ///
    /// # Arguments
    /// * `focused` - Whether the element is currently focused
    /// * `color` - The focus ring color
    /// * `background` - Background color for the offset gap
    fn focus_ring_offset(self, focused: bool, color: Hsla, background: Hsla) -> Self {
        if focused {
            self.shadow(focus_ring_shadow_with_offset(color, background))
        } else {
            self
        }
    }
}

// Implement for all Styled types
impl<T: Styled> FocusRingExt for T {}

/// Get the focus ring color from theme.
///
/// Uses the primary color which should have good contrast against
/// most backgrounds. For accessibility, ensure the theme's primary
/// color meets 3:1 contrast ratio against adjacent colors.
pub fn get_focus_ring_color(cx: &gpui::App) -> Hsla {
    // Use primary color for high visibility
    // The primary color should be chosen to have good contrast
    cx.theme().primary
}

/// Get an alternative focus ring color for dark backgrounds.
///
/// When the primary color might not have enough contrast,
/// use this lighter variant.
pub fn get_focus_ring_color_light(cx: &gpui::App) -> Hsla {
    // Use a lighter variant for dark backgrounds
    let primary = cx.theme().primary;
    // Increase lightness for better visibility on dark backgrounds
    gpui::hsla(primary.h, primary.s, (primary.l + 0.2).min(1.0), primary.a)
}
