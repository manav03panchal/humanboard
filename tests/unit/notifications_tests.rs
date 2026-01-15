//! Unit tests for notifications module.

use humanboard::notifications::{Toast, ToastManager, ToastVariant};
use std::time::Duration;

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
