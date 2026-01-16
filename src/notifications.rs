//! Toast notification system for Humanboard
//!
//! Provides a simple, elegant toast notification system with support for
//! multiple concurrent toasts, auto-dismiss, different visual variants,
//! and action buttons for error recovery.

use gpui::*;
use gpui_component::{ActiveTheme, Sizable};
use std::time::Duration;

/// Action that can be attached to a toast for error recovery
#[derive(Clone)]
pub struct ToastAction {
    /// Label for the action button
    pub label: String,
    /// Unique identifier for this action type
    pub action_type: ToastActionType,
}

/// Types of actions that can be performed from toasts
#[derive(Clone, Debug, PartialEq)]
pub enum ToastActionType {
    /// Retry the last failed operation
    Retry,
    /// Save to an alternative location
    SaveAs,
    /// Reset settings to defaults
    ResetSettings,
    /// Reload webview
    ReloadWebview,
    /// Dismiss the toast (no action)
    Dismiss,
}

impl ToastAction {
    /// Create a Retry action
    pub fn retry() -> Self {
        Self {
            label: "Retry".to_string(),
            action_type: ToastActionType::Retry,
        }
    }

    /// Create a Save As action
    pub fn save_as() -> Self {
        Self {
            label: "Save As...".to_string(),
            action_type: ToastActionType::SaveAs,
        }
    }

    /// Create a Reset Settings action
    pub fn reset_settings() -> Self {
        Self {
            label: "Reset to Defaults".to_string(),
            action_type: ToastActionType::ResetSettings,
        }
    }

    /// Create a Reload Webview action
    pub fn reload_webview() -> Self {
        Self {
            label: "Reload".to_string(),
            action_type: ToastActionType::ReloadWebview,
        }
    }
}

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
    /// Get the background color for this variant from the theme
    pub fn background_color(&self, theme: &gpui_component::theme::Theme) -> Hsla {
        match self {
            ToastVariant::Success => theme.success,
            ToastVariant::Error => theme.danger,
            // Use a dark background for info toasts for better contrast
            ToastVariant::Info => hsla(220.0 / 360.0, 0.15, 0.18, 0.98),
            ToastVariant::Warning => theme.warning,
        }
    }

    /// Get the text color for this variant
    pub fn text_color(&self, _theme: &gpui_component::theme::Theme) -> Hsla {
        match self {
            // Dark text for warning (yellow background)
            ToastVariant::Warning => hsla(0.0, 0.0, 0.1, 1.0),
            // White text for other variants
            _ => gpui::white(),
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
#[derive(Clone)]
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
    /// Optional action button for error recovery
    pub action: Option<ToastAction>,
}

impl std::fmt::Debug for Toast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Toast")
            .field("id", &self.id)
            .field("message", &self.message)
            .field("variant", &self.variant)
            .field("duration", &self.duration)
            .field("has_action", &self.action.is_some())
            .finish()
    }
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
            action: None,
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

    /// Attach an action button to this toast
    pub fn with_action(mut self, action: ToastAction) -> Self {
        // Toasts with actions should stay longer to give users time to read and act
        if self.duration < Duration::from_secs(8) {
            self.duration = Duration::from_secs(8);
        }
        self.action = Some(action);
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
    /// If reduce_motion is true, returns 1.0 (no fade animation)
    pub fn opacity(&self, reduce_motion: bool) -> f32 {
        if reduce_motion {
            // No fade animation when reduced motion is enabled
            return 1.0;
        }
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

/// Render a single toast notification with optional action button
pub fn render_toast(
    toast: &Toast,
    theme: &gpui_component::theme::Theme,
    reduce_motion: bool,
    cx: &mut Context<crate::app::Humanboard>,
) -> Stateful<Div> {
    use gpui_component::button::{Button, ButtonVariants};

    let bg = toast.variant.background_color(theme);
    let opacity = toast.opacity(reduce_motion);
    let icon = toast.variant.icon();
    let text = toast.variant.text_color(theme).opacity(opacity);
    let toast_id = toast.id;

    let mut base = div()
        .id(ElementId::Name(format!("toast-{}", toast_id).into()))
        .flex()
        .flex_row()
        .items_center()
        .gap_2()
        .px_4()
        .py_3()
        .mb_2()
        .bg(bg)
        .text_color(text)
        .rounded_lg()
        .shadow_lg()
        .overflow_hidden()
        .child(
            div()
                .text_lg()
                .font_weight(FontWeight::BOLD)
                .flex_shrink_0()
                .child(icon),
        )
        .child(
            div()
                .flex_1()
                .text_sm()
                .line_height(rems(1.4))
                .overflow_hidden()
                .text_ellipsis()
                .whitespace_nowrap()
                .child(toast.message.clone()),
        );

    // Add action button if present
    if let Some(ref action) = toast.action {
        let action_type = action.action_type.clone();
        let label = action.label.clone();
        base = base.child(
            Button::new(SharedString::from(format!("toast-action-{}", toast_id)))
                .xsmall()
                .ghost()
                .label(label.clone())
                .tooltip(label)
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.handle_toast_action(action_type.clone(), window, cx);
                    this.toast_manager.remove(toast_id);
                    cx.notify();
                })),
        );
    }

    base
}

/// Render all toasts in a container
pub fn render_toast_container(
    toasts: &[Toast],
    reduce_motion: bool,
    cx: &mut Context<crate::app::Humanboard>,
) -> Div {
    let theme = cx.theme().clone();
    let mut rendered = Vec::with_capacity(toasts.len());
    for toast in toasts {
        rendered.push(render_toast(toast, &theme, reduce_motion, cx));
    }

    div()
        .absolute()
        .top(px(52.0)) // Below header bar (40px) + padding
        .right_4()
        .flex()
        .flex_col()
        .items_end()
        .gap_2()
        .children(rendered)
}
