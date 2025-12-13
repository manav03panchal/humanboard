//! Toast notification system for Humanboard
//!
//! Provides a simple, elegant toast notification system with support for
//! multiple concurrent toasts, auto-dismiss, and different visual variants.

use gpui::*;
use std::time::Duration;

/// Visual variant for toast notifications
#[derive(Clone, Debug, PartialEq)]
pub enum ToastVariant {
    /// Success notification (green/positive)
    Success,
    /// Error notification (red/negative)
    Error,
    /// Informational notification (blue/neutral)
    Info,
    /// Warning notification (yellow/caution)
    Warning,
}

impl ToastVariant {
    /// Get the background color for this variant
    pub fn background_color(&self) -> Hsla {
        match self {
            ToastVariant::Success => hsla(145.0 / 360.0, 0.6, 0.35, 0.95),
            ToastVariant::Error => hsla(0.0, 0.7, 0.45, 0.95),
            ToastVariant::Info => hsla(210.0 / 360.0, 0.6, 0.45, 0.95),
            ToastVariant::Warning => hsla(45.0 / 360.0, 0.8, 0.45, 0.95),
        }
    }

    /// Get the default duration for this variant
    pub fn default_duration(&self) -> Duration {
        match self {
            ToastVariant::Success => Duration::from_secs(3),
            ToastVariant::Info => Duration::from_secs(3),
            ToastVariant::Warning => Duration::from_secs(4),
            ToastVariant::Error => Duration::from_secs(5),
        }
    }

    /// Get the icon for this variant
    pub fn icon(&self) -> &'static str {
        match self {
            ToastVariant::Success => "✓",
            ToastVariant::Error => "✗",
            ToastVariant::Info => "ℹ",
            ToastVariant::Warning => "⚠",
        }
    }
}

/// A single toast notification
#[derive(Clone, Debug)]
pub struct Toast {
    /// Unique identifier for this toast
    pub id: u64,
    /// Message to display
    pub message: String,
    /// Visual variant
    pub variant: ToastVariant,
    /// When this toast was created
    pub created_at: std::time::Instant,
    /// How long before auto-dismiss
    pub duration: Duration,
}

impl Toast {
    /// Create a new toast with a custom ID
    fn new_with_id(id: u64, message: impl Into<String>, variant: ToastVariant) -> Self {
        let duration = variant.default_duration();
        Self {
            id,
            message: message.into(),
            variant,
            created_at: std::time::Instant::now(),
            duration,
        }
    }

    /// Create a success toast
    pub fn success(message: impl Into<String>) -> Self {
        Self::new_with_id(0, message, ToastVariant::Success)
    }

    /// Create an error toast
    pub fn error(message: impl Into<String>) -> Self {
        Self::new_with_id(0, message, ToastVariant::Error)
    }

    /// Create an info toast
    pub fn info(message: impl Into<String>) -> Self {
        Self::new_with_id(0, message, ToastVariant::Info)
    }

    /// Create a warning toast
    pub fn warning(message: impl Into<String>) -> Self {
        Self::new_with_id(0, message, ToastVariant::Warning)
    }

    /// Create a toast with a custom duration
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Check if this toast has expired
    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() >= self.duration
    }

    /// Get the remaining lifetime as a percentage (0.0 to 1.0)
    pub fn remaining_percent(&self) -> f32 {
        let elapsed = self.created_at.elapsed().as_secs_f32();
        let total = self.duration.as_secs_f32();
        (1.0 - (elapsed / total)).max(0.0).min(1.0)
    }

    /// Get the opacity based on fade-out animation
    pub fn opacity(&self) -> f32 {
        let remaining = self.remaining_percent();
        if remaining > 0.2 {
            1.0
        } else {
            // Fade out in the last 20% of lifetime
            remaining * 5.0
        }
    }
}

/// Manager for toast notifications
#[derive(Default)]
pub struct ToastManager {
    /// Active toasts
    toasts: Vec<Toast>,
    /// Counter for generating unique IDs
    next_id: u64,
}

impl ToastManager {
    /// Create a new toast manager
    pub fn new() -> Self {
        Self {
            toasts: Vec::new(),
            next_id: 1,
        }
    }

    /// Add a toast to the manager
    pub fn push(&mut self, mut toast: Toast) {
        toast.id = self.next_id;
        self.next_id += 1;
        self.toasts.push(toast);
    }

    /// Remove expired toasts
    pub fn remove_expired(&mut self) {
        self.toasts.retain(|toast| !toast.is_expired());
    }

    /// Get all active toasts
    pub fn toasts(&self) -> &[Toast] {
        &self.toasts
    }

    /// Remove a specific toast by ID
    pub fn remove(&mut self, id: u64) {
        self.toasts.retain(|toast| toast.id != id);
    }

    /// Clear all toasts
    pub fn clear(&mut self) {
        self.toasts.clear();
    }

    /// Get the number of active toasts
    pub fn count(&self) -> usize {
        self.toasts.len()
    }

    /// Check if there are any active toasts
    pub fn is_empty(&self) -> bool {
        self.toasts.is_empty()
    }
}

/// Render a single toast notification
pub fn render_toast(toast: &Toast) -> Div {
    let bg = toast.variant.background_color();
    let opacity = toast.opacity();
    let icon = toast.variant.icon();

    div()
        .flex()
        .flex_row()
        .items_center()
        .gap_2()
        .px_4()
        .py_3()
        .mb_2()
        .bg(bg)
        .text_color(hsla(0.0, 0.0, 1.0, opacity))
        .rounded_lg()
        .shadow_lg()
        .overflow_hidden()
        .child(div().text_lg().font_weight(FontWeight::BOLD).flex_shrink_0().child(icon))
        .child(
            div()
                .flex_1()
                .text_sm()
                .line_height(rems(1.4))
                .overflow_hidden()
                .text_ellipsis()
                .whitespace_nowrap()
                .child(toast.message.clone()),
        )
}

/// Render all toasts in a container
pub fn render_toast_container(toasts: &[Toast]) -> Div {
    div()
        .absolute()
        .top(px(52.0)) // Below header bar (40px) + padding
        .right_4()
        .flex()
        .flex_col()
        .items_end()
        .gap_2()
        .children(toasts.iter().map(render_toast))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toast_creation() {
        let toast = Toast::success("Test message");
        assert_eq!(toast.message, "Test message");
        assert_eq!(toast.variant, ToastVariant::Success);
    }

    #[test]
    fn test_toast_manager() {
        let mut manager = ToastManager::new();
        assert_eq!(manager.count(), 0);

        manager.push(Toast::success("Message 1"));
        assert_eq!(manager.count(), 1);

        manager.push(Toast::error("Message 2"));
        assert_eq!(manager.count(), 2);

        manager.clear();
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_toast_expiration() {
        let toast = Toast::success("Test").with_duration(Duration::from_millis(1));
        assert!(!toast.is_expired());

        std::thread::sleep(Duration::from_millis(10));
        assert!(toast.is_expired());
    }

    #[test]
    fn test_variant_durations() {
        assert_eq!(
            ToastVariant::Success.default_duration(),
            Duration::from_secs(3)
        );
        assert_eq!(
            ToastVariant::Info.default_duration(),
            Duration::from_secs(3)
        );
        assert_eq!(
            ToastVariant::Warning.default_duration(),
            Duration::from_secs(4)
        );
        assert_eq!(
            ToastVariant::Error.default_duration(),
            Duration::from_secs(5)
        );
    }
}
