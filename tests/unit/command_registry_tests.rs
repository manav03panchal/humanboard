//! Unit tests for command_registry module.

use humanboard::command_registry::{create_default_registry, CommandRegistry, GoHomeCommand};

#[test]
fn test_registry_creation() {
    let registry = CommandRegistry::new();
    assert!(registry.is_empty());
}

#[test]
fn test_register_and_get() {
    let mut registry = CommandRegistry::new();
    registry.register(GoHomeCommand);

    assert_eq!(registry.len(), 1);
    assert!(registry.get("navigation:home").is_some());
    assert!(registry.get("nonexistent").is_none());
}

#[test]
fn test_default_registry() {
    let registry = create_default_registry();
    assert!(registry.len() > 0);
    assert!(registry.get("navigation:home").is_some());
    assert!(registry.get("edit:undo").is_some());
    assert!(registry.get("view:zoom_in").is_some());
}

#[test]
fn test_categories() {
    let registry = create_default_registry();
    let categories: Vec<_> = registry.categories().collect();
    assert!(categories.contains(&&"Navigation".to_string()));
    assert!(categories.contains(&&"Edit".to_string()));
    assert!(categories.contains(&&"View".to_string()));
}
