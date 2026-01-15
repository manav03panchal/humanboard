//! Unit tests for perf module.

use humanboard::perf::{PerfMonitor, ScopedTimer};
use std::time::Duration;

#[test]
fn test_perf_monitor_basic() {
    let mut monitor = PerfMonitor::new();

    monitor.begin_frame();
    std::thread::sleep(Duration::from_millis(1));
    let time = monitor.end_frame();

    assert!(time.is_some());
    assert!(time.unwrap() >= 1.0);
}

#[test]
fn test_average_calculation() {
    let mut monitor = PerfMonitor::new();

    // Simulate some frames
    for _ in 0..5 {
        monitor.begin_frame();
        std::thread::sleep(Duration::from_millis(1));
        monitor.end_frame();
    }

    assert!(monitor.average_frame_time() >= 1.0);
    assert!(monitor.estimated_fps() > 0.0);
}

#[test]
fn test_scoped_timer() {
    // This should not warn (threshold is high)
    let _timer = ScopedTimer::new("test_op", 1000.0);
    std::thread::sleep(Duration::from_millis(1));
    // Timer drops here, no warning expected
}
