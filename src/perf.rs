//! Performance monitoring utilities.
//!
//! Provides simple frame time tracking and performance warnings
//! to help identify bottlenecks.

use std::collections::VecDeque;
use std::time::Instant;

#[cfg(test)]
use std::time::Duration;
use tracing::warn;

/// Target frame time for 60 FPS
const TARGET_FRAME_MS: f64 = 16.67;

/// Number of samples to keep for running average
const SAMPLE_COUNT: usize = 60;

/// Threshold multiplier for warning (e.g., 2.0 = warn if frame takes 2x target)
const WARN_THRESHOLD: f64 = 2.0;

/// Simple performance monitor for tracking frame times.
pub struct PerfMonitor {
    /// Recent frame times in milliseconds
    frame_times: VecDeque<f64>,
    /// When the current frame started
    frame_start: Option<Instant>,
    /// Count of frames that exceeded the warning threshold
    slow_frame_count: u64,
    /// Total frames tracked
    total_frames: u64,
}

impl Default for PerfMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl PerfMonitor {
    /// Create a new performance monitor.
    pub fn new() -> Self {
        Self {
            frame_times: VecDeque::with_capacity(SAMPLE_COUNT),
            frame_start: None,
            slow_frame_count: 0,
            total_frames: 0,
        }
    }

    /// Mark the start of a frame.
    pub fn begin_frame(&mut self) {
        self.frame_start = Some(Instant::now());
    }

    /// Mark the end of a frame and record timing.
    /// Returns the frame time in milliseconds.
    pub fn end_frame(&mut self) -> Option<f64> {
        let start = self.frame_start.take()?;
        let elapsed = start.elapsed();
        let ms = elapsed.as_secs_f64() * 1000.0;

        // Track the sample
        if self.frame_times.len() >= SAMPLE_COUNT {
            self.frame_times.pop_front();
        }
        self.frame_times.push_back(ms);
        self.total_frames += 1;

        // Check for slow frame
        if ms > TARGET_FRAME_MS * WARN_THRESHOLD {
            self.slow_frame_count += 1;
            warn!(
                frame_time_ms = format!("{:.2}", ms),
                target_ms = format!("{:.2}", TARGET_FRAME_MS),
                "Slow frame detected"
            );
        }

        Some(ms)
    }

    /// Get the average frame time over recent samples.
    pub fn average_frame_time(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        self.frame_times.iter().sum::<f64>() / self.frame_times.len() as f64
    }

    /// Get the maximum frame time in recent samples.
    pub fn max_frame_time(&self) -> f64 {
        self.frame_times.iter().copied().fold(0.0, f64::max)
    }

    /// Get the percentage of frames that were slow.
    pub fn slow_frame_percentage(&self) -> f64 {
        if self.total_frames == 0 {
            return 0.0;
        }
        (self.slow_frame_count as f64 / self.total_frames as f64) * 100.0
    }

    /// Get estimated FPS based on average frame time.
    pub fn estimated_fps(&self) -> f64 {
        let avg = self.average_frame_time();
        if avg <= 0.0 {
            return 0.0;
        }
        1000.0 / avg
    }

    /// Log a performance summary if there are issues.
    pub fn log_summary_if_slow(&self) {
        let avg = self.average_frame_time();
        if avg > TARGET_FRAME_MS {
            warn!(
                avg_frame_ms = format!("{:.2}", avg),
                max_frame_ms = format!("{:.2}", self.max_frame_time()),
                slow_percentage = format!("{:.1}%", self.slow_frame_percentage()),
                estimated_fps = format!("{:.1}", self.estimated_fps()),
                "Performance below target"
            );
        }
    }
}

/// A simple scoped timer that logs duration on drop.
pub struct ScopedTimer {
    name: &'static str,
    start: Instant,
    threshold_ms: f64,
}

impl ScopedTimer {
    /// Create a new scoped timer with a warning threshold.
    pub fn new(name: &'static str, threshold_ms: f64) -> Self {
        Self {
            name,
            start: Instant::now(),
            threshold_ms,
        }
    }

    /// Create a timer with the default threshold (16ms).
    pub fn with_default_threshold(name: &'static str) -> Self {
        Self::new(name, TARGET_FRAME_MS)
    }
}

impl Drop for ScopedTimer {
    fn drop(&mut self) {
        let elapsed_ms = self.start.elapsed().as_secs_f64() * 1000.0;
        if elapsed_ms > self.threshold_ms {
            warn!(
                operation = self.name,
                elapsed_ms = format!("{:.2}", elapsed_ms),
                threshold_ms = format!("{:.2}", self.threshold_ms),
                "Slow operation"
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
