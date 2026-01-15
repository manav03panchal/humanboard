//! Focus Management Module
//!
//! This module provides a structured approach to focus management in Humanboard,
//! following Zed's focus handling patterns. The key principles are:
//!
//! 1. **Multiple FocusHandles**: Each focusable context gets its own FocusHandle
//! 2. **Focus Contexts**: Clear hierarchy of focus contexts (Modal > Palette > Preview > Canvas)
//! 3. **No Focus Stealing**: Clicking on canvas doesn't steal focus from active inputs
//! 4. **Explicit Transitions**: Focus changes are explicit and intentional
//! 5. **Focus Events**: Proper on_focus_in/on_focus_out callbacks
//!
//! ## Focus Hierarchy (highest to lowest priority)
//!
//! 1. **Modal** - Settings modal, delete confirmation dialogs
//! 2. **CommandPalette** - Command palette input and results
//! 3. **CodeEditor** - Code editor in edit mode
//! 4. **Preview** - Markdown editor in preview panel
//! 5. **Landing** - Board name editing input
//! 6. **Canvas** - Default canvas focus for keyboard shortcuts
//!
//! ## Usage Example
//!
//! ```ignore
//! // Create focus manager in your app initialization
//! let focus = FocusManager::new(cx);
//!
//! // In your render method, track focus on elements
//! div()
//!     .track_focus(&focus.canvas)
//!     .key_context("Canvas")
//!     .on_action(|action: &SomeAction, window, cx| { ... })
//!
//! // Transition focus explicitly
//! focus.focus(FocusContext::CommandPalette, window);
//!
//! // Release focus back to canvas
//! focus.release(FocusContext::CommandPalette, window);
//! ```

use gpui::*;
use std::collections::HashMap;
use tracing::{debug, trace};

// ============================================================================
// Focus Context - Enum of all focusable regions
// ============================================================================

/// Represents the different focus contexts in the application.
/// Listed in priority order (highest first).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FocusContext {
    /// Modal dialogs (settings, confirmations) - highest priority
    Modal,
    /// Command palette - captures keyboard input
    CommandPalette,
    /// Textbox editing on canvas - captures text input
    TextboxEditing,
    /// Code editor in edit mode - captures undo/redo/save
    CodeEditor,
    /// Preview panel (markdown editor)
    Preview,
    /// Landing page (board name editing)
    Landing,
    /// Canvas - default context for keyboard shortcuts
    Canvas,
    /// Canvas with active input (textbox editing) - subset of shortcuts active
    CanvasInputActive,
}

/// Key context string constants for use in keybindings.
///
/// These constants ensure type-safe key context references across the codebase,
/// preventing typos and enabling compile-time checking.
impl FocusContext {
    /// Key context for Modal dialogs
    pub const KEY_MODAL: &'static str = "Modal";
    /// Key context for Command palette
    pub const KEY_COMMAND_PALETTE: &'static str = "CommandPalette";
    /// Key context for Textbox editing
    pub const KEY_TEXTBOX_EDITING: &'static str = "TextboxEditing";
    /// Key context for Code editor
    pub const KEY_CODE_EDITOR: &'static str = "CodeEditor";
    /// Key context for Preview panel
    pub const KEY_PREVIEW: &'static str = "Preview";
    /// Key context for Landing page
    pub const KEY_LANDING: &'static str = "Landing";
    /// Key context for Canvas
    pub const KEY_CANVAS: &'static str = "Canvas";
    /// Key context for Canvas with active input
    pub const KEY_CANVAS_INPUT_ACTIVE: &'static str = "CanvasInputActive";
}

impl FocusContext {
    /// Returns the key_context string for this focus context.
    /// Used for binding keyboard shortcuts to specific contexts.
    pub fn key_context(&self) -> &'static str {
        match self {
            FocusContext::Modal => Self::KEY_MODAL,
            FocusContext::CommandPalette => Self::KEY_COMMAND_PALETTE,
            FocusContext::TextboxEditing => Self::KEY_TEXTBOX_EDITING,
            FocusContext::CodeEditor => Self::KEY_CODE_EDITOR,
            FocusContext::Preview => Self::KEY_PREVIEW,
            FocusContext::Landing => Self::KEY_LANDING,
            FocusContext::Canvas => Self::KEY_CANVAS,
            FocusContext::CanvasInputActive => Self::KEY_CANVAS_INPUT_ACTIVE,
        }
    }

    /// Returns the priority of this context (higher = more important).
    /// Used to prevent lower-priority contexts from stealing focus.
    pub fn priority(&self) -> u8 {
        match self {
            FocusContext::Modal => 8,
            FocusContext::CommandPalette => 7,
            FocusContext::TextboxEditing => 6,
            FocusContext::CodeEditor => 5,
            FocusContext::Preview => 4,
            FocusContext::Landing => 3,
            FocusContext::Canvas => 2,
            FocusContext::CanvasInputActive => 1, // Same level as Canvas but distinct
        }
    }

    /// Returns all focus contexts in priority order (highest first).
    pub fn all() -> &'static [FocusContext] {
        &[
            FocusContext::Modal,
            FocusContext::CommandPalette,
            FocusContext::TextboxEditing,
            FocusContext::CodeEditor,
            FocusContext::Preview,
            FocusContext::Landing,
            FocusContext::Canvas,
            FocusContext::CanvasInputActive,
        ]
    }

    /// Check if this context captures text input.
    /// When true, single-key shortcuts should be disabled.
    pub fn captures_text_input(&self) -> bool {
        matches!(
            self,
            FocusContext::CommandPalette
                | FocusContext::TextboxEditing
                | FocusContext::CodeEditor
                | FocusContext::Preview
                | FocusContext::Landing
                | FocusContext::CanvasInputActive
        )
    }
}

// ============================================================================
// Focus Event - Events fired on focus changes
// ============================================================================

/// Event fired when focus changes between contexts.
#[derive(Debug, Clone)]
pub struct FocusChangeEvent {
    /// The context that lost focus (if any)
    pub previous: Option<FocusContext>,
    /// The context that gained focus
    pub current: FocusContext,
}

impl FocusChangeEvent {
    /// Check if a specific context gained focus
    pub fn is_focus_in(&self, context: FocusContext) -> bool {
        self.current == context && self.previous != Some(context)
    }

    /// Check if a specific context lost focus
    pub fn is_focus_out(&self, context: FocusContext) -> bool {
        self.previous == Some(context) && self.current != context
    }
}

/// Callback type for focus change listeners.
pub type FocusChangeCallback = Box<dyn Fn(&FocusChangeEvent) + Send + Sync>;

// ============================================================================
// Focus Manager - Central manager for application focus
// ============================================================================

/// Manages focus across the application.
///
/// This struct holds FocusHandles for each context and provides methods
/// to transition focus between contexts in a controlled manner.
///
/// ## Focus Handle Lifecycle
///
/// FocusHandles are created once during FocusManager initialization and
/// remain valid for the lifetime of the application. They use reference
/// counting internally, so cloning is cheap.
///
/// ## Focus Paths
///
/// The focus path is the chain from the focused element to the root.
/// GPUI dispatches keyboard events along this path, enabling hierarchical
/// action handling (e.g., Escape closes modal, then command palette, then
/// deselects on canvas).
pub struct FocusManager {
    /// Focus handle for the canvas (default)
    pub canvas: FocusHandle,
    /// Focus handle for the command palette
    pub command_palette: FocusHandle,
    /// Focus handle for textbox editing on canvas
    pub textbox_editing: FocusHandle,
    /// Focus handle for the code editor
    pub code_editor: FocusHandle,
    /// Focus handle for the preview panel
    pub preview: FocusHandle,
    /// Focus handle for the landing page inputs
    pub landing: FocusHandle,
    /// Focus handle for modal dialogs
    pub modal: FocusHandle,
    /// The currently active focus context
    active_context: FocusContext,
    /// Previous focus context (for focus restoration)
    previous_context: Option<FocusContext>,
    /// Whether focus needs to be restored to canvas on next render
    needs_focus_restore: bool,
    /// Tab order for focus contexts (lower = earlier in tab order)
    tab_order: HashMap<FocusContext, i32>,
}

impl FocusManager {
    /// Create a new FocusManager with all FocusHandles initialized.
    ///
    /// This should be called once during application initialization.
    /// The FocusHandles created here will be used throughout the
    /// application's lifetime.
    pub fn new(cx: &mut App) -> Self {
        debug!("Initializing FocusManager with all focus handles");

        // Initialize default tab order
        let mut tab_order = HashMap::new();
        tab_order.insert(FocusContext::Canvas, 0);
        tab_order.insert(FocusContext::Landing, 1);
        tab_order.insert(FocusContext::Preview, 2);
        tab_order.insert(FocusContext::CodeEditor, 3);
        tab_order.insert(FocusContext::TextboxEditing, 4);
        tab_order.insert(FocusContext::CommandPalette, 5);
        tab_order.insert(FocusContext::Modal, 6);

        Self {
            canvas: cx.focus_handle(),
            command_palette: cx.focus_handle(),
            textbox_editing: cx.focus_handle(),
            code_editor: cx.focus_handle(),
            preview: cx.focus_handle(),
            landing: cx.focus_handle(),
            modal: cx.focus_handle(),
            active_context: FocusContext::Canvas,
            previous_context: None,
            needs_focus_restore: false,
            tab_order,
        }
    }

    /// Get the currently active focus context.
    pub fn active_context(&self) -> FocusContext {
        self.active_context
    }

    /// Get the previous focus context (if any).
    pub fn previous_context(&self) -> Option<FocusContext> {
        self.previous_context
    }

    /// Get the FocusHandle for a specific context.
    pub fn handle_for(&self, context: FocusContext) -> &FocusHandle {
        match context {
            FocusContext::Canvas | FocusContext::CanvasInputActive => &self.canvas,
            FocusContext::CommandPalette => &self.command_palette,
            FocusContext::TextboxEditing => &self.textbox_editing,
            FocusContext::CodeEditor => &self.code_editor,
            FocusContext::Preview => &self.preview,
            FocusContext::Landing => &self.landing,
            FocusContext::Modal => &self.modal,
        }
    }

    /// Check if a higher-priority context is currently active.
    ///
    /// This is used to prevent lower-priority contexts from stealing focus.
    /// For example, canvas clicks shouldn't steal focus when command palette is open.
    pub fn has_higher_priority_focus(&self, than: FocusContext) -> bool {
        self.active_context.priority() > than.priority()
    }

    /// Check if any input-capturing context is active.
    ///
    /// When true, keyboard shortcuts that would conflict with text input
    /// (like arrow keys, backspace) should be suppressed on the canvas.
    pub fn is_input_active(&self) -> bool {
        matches!(
            self.active_context,
            FocusContext::CommandPalette
                | FocusContext::CodeEditor
                | FocusContext::Preview
                | FocusContext::Landing
                | FocusContext::Modal
                | FocusContext::TextboxEditing
        )
    }

    /// Transition focus to a new context.
    ///
    /// This is the primary method for changing focus. It:
    /// 1. Stores the previous context for potential restoration
    /// 2. Updates the active context
    /// 3. Focuses the appropriate FocusHandle
    ///
    /// Returns a FocusChangeEvent describing the transition.
    pub fn focus(&mut self, context: FocusContext, window: &mut Window) -> FocusChangeEvent {
        let previous = if self.active_context != context {
            Some(self.active_context)
        } else {
            None
        };

        self.previous_context = previous;
        self.active_context = context;
        self.handle_for(context).focus(window);

        let event = FocusChangeEvent {
            previous,
            current: context,
        };

        trace!(
            "Focus changed: {:?} -> {:?}",
            event.previous,
            event.current
        );

        event
    }

    /// Set the active context without actually focusing the FocusHandle.
    ///
    /// This is useful when another component (like an Input) should have the actual
    /// focus, but we want to track that we're in a specific context for keybinding
    /// and state purposes.
    pub fn set_context_without_focus(&mut self, context: FocusContext) {
        let previous = if self.active_context != context {
            Some(self.active_context)
        } else {
            None
        };

        self.previous_context = previous;
        self.active_context = context;

        trace!(
            "Context set (no focus change): {:?} -> {:?}",
            previous,
            context
        );
    }

    /// Try to focus a context, but only if no higher-priority context is active.
    ///
    /// Returns `Some(FocusChangeEvent)` if focus was granted, `None` if blocked.
    pub fn try_focus(
        &mut self,
        context: FocusContext,
        window: &mut Window,
    ) -> Option<FocusChangeEvent> {
        if self.has_higher_priority_focus(context) {
            trace!(
                "Focus to {:?} blocked by higher priority {:?}",
                context,
                self.active_context
            );
            None
        } else {
            Some(self.focus(context, window))
        }
    }

    /// Release focus from a context and return to the previous context or canvas.
    ///
    /// This should be called when closing modals, command palette, etc.
    /// Returns the FocusChangeEvent if focus was actually released.
    pub fn release(
        &mut self,
        context: FocusContext,
        window: &mut Window,
    ) -> Option<FocusChangeEvent> {
        // Only release if this context is currently active
        if self.active_context == context {
            // Restore to previous context if it's still valid, otherwise canvas
            let target = self
                .previous_context
                .filter(|&prev| prev.priority() < context.priority())
                .unwrap_or(FocusContext::Canvas);

            Some(self.focus(target, window))
        } else {
            None
        }
    }

    /// Force release to canvas regardless of current context.
    ///
    /// Used when we need to ensure canvas has focus (e.g., after blur events).
    pub fn force_canvas_focus(&mut self, window: &mut Window) -> FocusChangeEvent {
        self.focus(FocusContext::Canvas, window)
    }

    /// Mark that focus should return to canvas on next opportunity.
    /// Used when window isn't available (e.g., Blur callbacks).
    pub fn mark_needs_canvas_focus(&mut self) {
        self.previous_context = Some(self.active_context);
        self.active_context = FocusContext::Canvas;
        self.needs_focus_restore = true;
    }

    /// Check if focus needs to be restored and do so if needed.
    /// Call this at the start of render when window is available.
    pub fn restore_focus_if_needed(&mut self, window: &mut Window) -> Option<FocusChangeEvent> {
        if self.needs_focus_restore {
            self.canvas.focus(window);
            self.needs_focus_restore = false;
            Some(FocusChangeEvent {
                previous: self.previous_context,
                current: FocusContext::Canvas,
            })
        } else {
            None
        }
    }

    /// Check if the canvas should handle keyboard events.
    ///
    /// Returns false if an input-capturing context is active.
    pub fn canvas_should_handle_keys(&self) -> bool {
        self.active_context == FocusContext::Canvas
    }

    /// Check if a specific context is currently focused.
    pub fn is_focused(&self, context: FocusContext) -> bool {
        self.active_context == context
    }

    /// Check if a specific context contains the focused context.
    ///
    /// This is useful for hierarchical focus checks. For example,
    /// a parent container might want to know if any of its children
    /// have focus.
    ///
    /// In our flat hierarchy, this is equivalent to is_focused,
    /// but the API mirrors Zed's for future extensibility.
    pub fn contains_focused(&self, context: FocusContext) -> bool {
        // In a flat hierarchy, contains_focused == is_focused
        // If we add nested contexts later, this would check the focus path
        self.is_focused(context)
    }

    /// Check if a specific context is within the focused context.
    ///
    /// This is the inverse of contains_focused and is useful for
    /// determining if an element should respond to focus-related styling.
    pub fn within_focused(&self, context: FocusContext) -> bool {
        // In a flat hierarchy, this is also equivalent to is_focused
        self.is_focused(context)
    }

    /// Get the key context string for the currently active context.
    pub fn current_key_context(&self) -> &'static str {
        self.active_context.key_context()
    }

    /// Get the tab index for a context.
    pub fn tab_index(&self, context: FocusContext) -> i32 {
        *self.tab_order.get(&context).unwrap_or(&0)
    }

    /// Set the tab index for a context.
    pub fn set_tab_index(&mut self, context: FocusContext, index: i32) {
        self.tab_order.insert(context, index);
    }

    /// Move focus to the next context in tab order.
    ///
    /// Returns the FocusChangeEvent if focus changed.
    pub fn focus_next(&mut self, window: &mut Window) -> Option<FocusChangeEvent> {
        let current_index = self.tab_index(self.active_context);

        // Find the next context with a higher tab index
        let mut next_context = None;
        let mut min_index = i32::MAX;

        for &context in FocusContext::all() {
            let index = self.tab_index(context);
            if index > current_index && index < min_index {
                min_index = index;
                next_context = Some(context);
            }
        }

        // If no higher index found, wrap to the lowest
        if next_context.is_none() {
            let mut min_index = i32::MAX;
            for &context in FocusContext::all() {
                let index = self.tab_index(context);
                if index < min_index {
                    min_index = index;
                    next_context = Some(context);
                }
            }
        }

        next_context.map(|ctx| self.focus(ctx, window))
    }

    /// Move focus to the previous context in tab order.
    ///
    /// Returns the FocusChangeEvent if focus changed.
    pub fn focus_previous(&mut self, window: &mut Window) -> Option<FocusChangeEvent> {
        let current_index = self.tab_index(self.active_context);

        // Find the previous context with a lower tab index
        let mut prev_context = None;
        let mut max_index = i32::MIN;

        for &context in FocusContext::all() {
            let index = self.tab_index(context);
            if index < current_index && index > max_index {
                max_index = index;
                prev_context = Some(context);
            }
        }

        // If no lower index found, wrap to the highest
        if prev_context.is_none() {
            let mut max_index = i32::MIN;
            for &context in FocusContext::all() {
                let index = self.tab_index(context);
                if index > max_index {
                    max_index = index;
                    prev_context = Some(context);
                }
            }
        }

        prev_context.map(|ctx| self.focus(ctx, window))
    }
}

// ============================================================================
// FocusableElement - Extension trait for elements
// ============================================================================

/// Extension trait for elements that can track focus.
pub trait FocusableElement: Sized {
    /// Track focus for a specific context from the FocusManager.
    fn track_focus_context(self, manager: &FocusManager, context: FocusContext) -> Self;
}

impl FocusableElement for Div {
    fn track_focus_context(self, manager: &FocusManager, context: FocusContext) -> Self {
        self.track_focus(manager.handle_for(context))
            .key_context(context.key_context())
    }
}

// ============================================================================
// Focus Query Helpers
// ============================================================================

/// Check if a FocusHandle is currently focused within a window.
pub fn is_handle_focused(handle: &FocusHandle, window: &Window) -> bool {
    handle.is_focused(window)
}

/// Check if a FocusHandle contains the currently focused element.
pub fn handle_contains_focused(handle: &FocusHandle, window: &Window, cx: &App) -> bool {
    handle.contains_focused(window, cx)
}

/// Check if a FocusHandle is within the focus path of the currently focused element.
pub fn handle_within_focused(handle: &FocusHandle, window: &Window, cx: &mut App) -> bool {
    handle.within_focused(window, cx)
}
