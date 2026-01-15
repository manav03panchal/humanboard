//! Unit tests for focus module.

use humanboard::focus::{FocusChangeEvent, FocusContext};

#[test]
fn test_focus_context_priority() {
    assert!(FocusContext::Modal.priority() > FocusContext::CommandPalette.priority());
    assert!(FocusContext::CommandPalette.priority() > FocusContext::TextboxEditing.priority());
    assert!(FocusContext::TextboxEditing.priority() > FocusContext::CodeEditor.priority());
    assert!(FocusContext::CodeEditor.priority() > FocusContext::Preview.priority());
    assert!(FocusContext::Preview.priority() > FocusContext::Landing.priority());
    assert!(FocusContext::Landing.priority() > FocusContext::Canvas.priority());
}

#[test]
fn test_key_context_strings() {
    assert_eq!(FocusContext::Canvas.key_context(), "Canvas");
    assert_eq!(FocusContext::CommandPalette.key_context(), "CommandPalette");
    assert_eq!(FocusContext::TextboxEditing.key_context(), "TextboxEditing");
    assert_eq!(FocusContext::CodeEditor.key_context(), "CodeEditor");
    assert_eq!(FocusContext::Modal.key_context(), "Modal");
}

#[test]
fn test_captures_text_input() {
    assert!(FocusContext::CommandPalette.captures_text_input());
    assert!(FocusContext::TextboxEditing.captures_text_input());
    assert!(FocusContext::CodeEditor.captures_text_input());
    assert!(!FocusContext::Canvas.captures_text_input());
    assert!(!FocusContext::Modal.captures_text_input());
}

#[test]
fn test_focus_change_event() {
    let event = FocusChangeEvent {
        previous: Some(FocusContext::Canvas),
        current: FocusContext::CommandPalette,
    };

    assert!(event.is_focus_in(FocusContext::CommandPalette));
    assert!(!event.is_focus_in(FocusContext::Canvas));
    assert!(event.is_focus_out(FocusContext::Canvas));
    assert!(!event.is_focus_out(FocusContext::CommandPalette));
}

#[test]
fn test_focus_context_all() {
    let all = FocusContext::all();
    assert_eq!(all.len(), 8);
    // Should be in priority order (highest first)
    assert_eq!(all[0], FocusContext::Modal);
    assert_eq!(all[6], FocusContext::Canvas);
}
