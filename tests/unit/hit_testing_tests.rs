//! Unit tests for hit_testing module.

use gpui::{point, px, size};
use humanboard::hit_testing::{HitTestResult, HitTester};

#[test]
fn test_hit_test_header() {
    let tester = HitTester::new();
    let result = tester.hit_test(
        point(px(100.0), px(20.0)), // In header area
        std::iter::empty(),
        point(px(0.0), px(0.0)),
        1.0,
        size(px(800.0), px(600.0)),
        None,
    );
    assert_eq!(result, HitTestResult::Header);
}

#[test]
fn test_hit_test_dock() {
    let tester = HitTester::new();
    let result = tester.hit_test(
        point(px(20.0), px(100.0)), // In dock area
        std::iter::empty(),
        point(px(0.0), px(0.0)),
        1.0,
        size(px(800.0), px(600.0)),
        None,
    );
    assert_eq!(result, HitTestResult::Dock);
}

#[test]
fn test_hit_test_canvas() {
    let tester = HitTester::new();
    let result = tester.hit_test(
        point(px(400.0), px(300.0)), // In canvas area
        std::iter::empty(),
        point(px(0.0), px(0.0)),
        1.0,
        size(px(800.0), px(600.0)),
        None,
    );
    assert_eq!(result, HitTestResult::Canvas);
}

#[test]
fn test_screen_to_canvas() {
    let tester = HitTester::new();
    let screen_pos = point(px(144.0), px(140.0)); // 44 (DOCK_WIDTH) + 100, 40 (header) + 100
    let canvas_pos = tester.screen_to_canvas(screen_pos, point(px(0.0), px(0.0)), 1.0);
    assert_eq!(f32::from(canvas_pos.x), 100.0);
    assert_eq!(f32::from(canvas_pos.y), 100.0);
}
