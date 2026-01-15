//! Unit tests for selection module.

use gpui::{point, px};
use humanboard::selection::{MarqueeState, SelectionManager};

#[test]
fn test_single_selection() {
    let mut mgr = SelectionManager::new();

    mgr.select(1);
    assert!(mgr.is_selected(1));
    assert_eq!(mgr.count(), 1);

    mgr.select(2);
    assert!(!mgr.is_selected(1));
    assert!(mgr.is_selected(2));
    assert_eq!(mgr.count(), 1);
}

#[test]
fn test_toggle_selection() {
    let mut mgr = SelectionManager::new();

    mgr.select(1);
    mgr.toggle(2);
    assert!(mgr.is_selected(1));
    assert!(mgr.is_selected(2));

    mgr.toggle(1);
    assert!(!mgr.is_selected(1));
    assert!(mgr.is_selected(2));
}

#[test]
fn test_marquee_bounds() {
    let marquee = MarqueeState {
        start: point(px(100.0), px(100.0)),
        current: point(px(50.0), px(150.0)),
    };

    let bounds = marquee.bounds();
    assert_eq!(f32::from(bounds.origin.x), 50.0);
    assert_eq!(f32::from(bounds.origin.y), 100.0);
    assert_eq!(f32::from(bounds.size.width), 50.0);
    assert_eq!(f32::from(bounds.size.height), 50.0);
}
