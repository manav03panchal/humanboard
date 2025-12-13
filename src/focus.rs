//! Focus Management Module
//!
//! This module provides a structured approach to focus management in Humanboard,
//! inspired by Zed's focus handling patterns. The key principles are:
//!
//! 1. **Multiple FocusHandles**: Each focusable context gets its own FocusHandle
//! 2. **Focus Contexts**: Clear hierarchy of focus contexts (Modal > Palette > Preview > Canvas)
//! 3. **No Focus Stealing**: Clicking on canvas doesn't steal focus from active inputs
//! 4. **Explicit Transitions**: Focus changes are explicit and intentional
//!
//! ## Focus Hierarchy (highest to lowest priority)
//!
//! 1. **Modal** - Settings modal, delete confirmation dialogs
//! 2. **CommandPalette** - Command palette input and results
//! 3. **Preview** - Markdown editor in preview panel
//! 4. **Landing** - Board name editing input
//! 5. **Canvas** - Default canvas focus for keyboard shortcuts

use gpui::*;

/// Represents the different focus contexts in the application.
/// Listed in priority order (highest first).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FocusContext {
    /// Modal dialogs (settings, confirmations) - highest priority
    Modal,
    /// Command palette - captures keyboard input
    CommandPalette,
    /// Preview panel (markdown editor)
    Preview,
    /// Landing page (board name editing)
    Landing,
    /// Canvas - default context for keyboard shortcuts
    Canvas,
}

impl FocusContext {
    /// Returns the key_context string for this focus context
    pub fn key_context(&self) -> &'static str {
        match self {
            FocusContext::Modal => "Modal",
            FocusContext::CommandPalette => "CommandPalette",
            FocusContext::Preview => "Preview",
            FocusContext::Landing => "Landing",
            FocusContext::Canvas => "Canvas",
        }
    }

    /// Returns the priority of this context (higher = more important)
    pub fn priority(&self) -> u8 {
        match self {
            FocusContext::Modal => 5,
            FocusContext::CommandPalette => 4,
            FocusContext::Preview => 3,
            FocusContext::Landing => 2,
            FocusContext::Canvas => 1,
        }
    }
}

/// Manages focus across the application.
///
/// This struct holds FocusHandles for each context and provides methods
/// to transition focus between contexts in a controlled manner.
pub struct FocusManager {
    /// Focus handle for the canvas (default)
    pub canvas: FocusHandle,
    /// Focus handle for the command palette
    pub command_palette: FocusHandle,
    /// Focus handle for the preview panel
    pub preview: FocusHandle,
    /// Focus handle for the landing page inputs
    pub landing: FocusHandle,
    /// Focus handle for modal dialogs
    pub modal: FocusHandle,
    /// The currently active focus context
    active_context: FocusContext,
    /// Whether focus needs to be restored to canvas on next render
    needs_focus_restore: bool,
}

impl FocusManager {
    /// Create a new FocusManager with all FocusHandles initialized
    pub fn new(cx: &mut App) -> Self {
        Self {
            canvas: cx.focus_handle(),
            command_palette: cx.focus_handle(),
            preview: cx.focus_handle(),
            landing: cx.focus_handle(),
            modal: cx.focus_handle(),
            active_context: FocusContext::Canvas,
            needs_focus_restore: false,
        }
    }

    /// Get the currently active focus context
    pub fn active_context(&self) -> FocusContext {
        self.active_context
    }

    /// Get the FocusHandle for a specific context
    pub fn handle_for(&self, context: FocusContext) -> &FocusHandle {
        match context {
            FocusContext::Canvas => &self.canvas,
            FocusContext::CommandPalette => &self.command_palette,
            FocusContext::Preview => &self.preview,
            FocusContext::Landing => &self.landing,
            FocusContext::Modal => &self.modal,
        }
    }

    /// Check if a higher-priority context is currently active
    ///
    /// This is used to prevent lower-priority contexts from stealing focus.
    /// For example, canvas clicks shouldn't steal focus when command palette is open.
    pub fn has_higher_priority_focus(&self, than: FocusContext) -> bool {
        self.active_context.priority() > than.priority()
    }

    /// Check if any input-capturing context is active
    ///
    /// When true, keyboard shortcuts that would conflict with text input
    /// (like arrow keys, backspace) should be suppressed on the canvas.
    pub fn is_input_active(&self) -> bool {
        matches!(
            self.active_context,
            FocusContext::CommandPalette | FocusContext::Preview | FocusContext::Landing | FocusContext::Modal
        )
    }

    /// Transition focus to a new context
    ///
    /// This is the primary method for changing focus. It:
    /// 1. Updates the active context
    /// 2. Focuses the appropriate FocusHandle
    pub fn focus(&mut self, context: FocusContext, window: &mut Window) {
        self.active_context = context;
        self.handle_for(context).focus(window);
    }

    /// Try to focus a context, but only if no higher-priority context is active
    ///
    /// Returns true if focus was granted, false if blocked by higher priority.
    pub fn try_focus(&mut self, context: FocusContext, window: &mut Window) -> bool {
        if self.has_higher_priority_focus(context) {
            false
        } else {
            self.focus(context, window);
            true
        }
    }

    /// Release focus from a context and return to canvas
    ///
    /// This should be called when closing modals, command palette, etc.
    pub fn release(&mut self, context: FocusContext, window: &mut Window) {
        // Only release if this context is currently active
        if self.active_context == context {
            self.focus(FocusContext::Canvas, window);
        }
    }

    /// Force release to canvas regardless of current context
    ///
    /// Used when we need to ensure canvas has focus (e.g., after blur events)
    pub fn force_canvas_focus(&mut self, window: &mut Window) {
        self.focus(FocusContext::Canvas, window);
    }

    /// Mark that focus should return to canvas on next opportunity
    /// Used when window isn't available (e.g., Blur callbacks)
    pub fn mark_needs_canvas_focus(&mut self) {
        self.active_context = FocusContext::Canvas;
        self.needs_focus_restore = true;
    }

    /// Check if focus needs to be restored and do so if needed
    /// Call this at the start of render when window is available
    pub fn restore_focus_if_needed(&mut self, window: &mut Window) {
        if self.needs_focus_restore {
            self.canvas.focus(window);
            self.needs_focus_restore = false;
        }
    }

    /// Check if the canvas should handle keyboard events
    ///
    /// Returns false if an input-capturing context is active.
    pub fn canvas_should_handle_keys(&self) -> bool {
        self.active_context == FocusContext::Canvas
    }

    /// Check if a specific context is currently focused
    pub fn is_focused(&self, context: FocusContext) -> bool {
        self.active_context == context
    }

    /// Get the key context string for the currently active context
    pub fn current_key_context(&self) -> &'static str {
        self.active_context.key_context()
    }
}

/// Extension trait for elements that can track focus
pub trait FocusableElement: Sized {
    /// Track focus for a specific context from the FocusManager
    fn track_focus_context(self, manager: &FocusManager, context: FocusContext) -> Self;
}

impl FocusableElement for Div {
    fn track_focus_context(self, manager: &FocusManager, context: FocusContext) -> Self {
        self.track_focus(manager.handle_for(context))
            .key_context(context.key_context())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_focus_context_priority() {
        assert!(FocusContext::Modal.priority() > FocusContext::CommandPalette.priority());
        assert!(FocusContext::CommandPalette.priority() > FocusContext::Preview.priority());
        assert!(FocusContext::Preview.priority() > FocusContext::Landing.priority());
        assert!(FocusContext::Landing.priority() > FocusContext::Canvas.priority());
    }

    #[test]
    fn test_key_context_strings() {
        assert_eq!(FocusContext::Canvas.key_context(), "Canvas");
        assert_eq!(FocusContext::CommandPalette.key_context(), "CommandPalette");
        assert_eq!(FocusContext::Modal.key_context(), "Modal");
    }
}
