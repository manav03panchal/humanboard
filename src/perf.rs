//! Performance monitoring utilities.
//!
//! Provides comprehensive performance tracking and profiling instrumentation
//! to identify bottlenecks in the Humanboard application.
//!
//! ## Features
//!
//! - **Frame timing**: Track render frame times with rolling averages
//! - **Scoped timers**: RAII-style timing for code blocks
//! - **Hierarchical profiling**: Nested timing with parent-child relationships
//! - **Aggregated statistics**: Per-operation timing histograms
//! - **Conditional compilation**: Zero-cost when profiling disabled
//!
//! ## Usage
//!
//! Enable profiling with the `profiling` feature flag:
//! ```toml
//! [dependencies]
//! humanboard = { features = ["profiling"] }
//! ```
//!
//! Use the profiling macros for zero-cost instrumentation:
//! ```ignore
//! use humanboard::perf::{profile_scope, profile_function};
//!
//! fn expensive_operation() {
//!     profile_function!();  // Times entire function
//!
//!     {
//!         profile_scope!("inner_work");  // Times just this block
//!         // ... work ...
//!     }
//! }
//! ```

use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Instant;
use tracing::{debug, warn};
#[cfg(feature = "profiling")]
use tracing::trace;

// ============================================================================
// Constants
// ============================================================================

/// Target frame time for 60 FPS
pub const TARGET_FRAME_MS: f64 = 16.67;

/// Number of samples to keep for rolling averages
const SAMPLE_COUNT: usize = 60;

/// Threshold multiplier for warning (e.g., 2.0 = warn if frame takes 2x target)
const WARN_THRESHOLD: f64 = 2.0;

/// Number of samples to keep for operation statistics
const STATS_SAMPLE_COUNT: usize = 100;

/// Global flag to enable/disable profiling at runtime
static PROFILING_ENABLED: AtomicBool = AtomicBool::new(cfg!(feature = "profiling"));

/// Global counter for unique timer IDs (for hierarchical profiling)
static TIMER_COUNTER: AtomicU64 = AtomicU64::new(0);

// ============================================================================
// Profiling Macros (zero-cost when disabled)
// ============================================================================

/// Profile a scope with the given name. Zero-cost when profiling is disabled.
///
/// # Example
/// ```ignore
/// use humanboard::perf::profile_scope;
///
/// fn render_items() {
///     profile_scope!("render_items");
///     // ... rendering code ...
/// }
/// ```
#[macro_export]
macro_rules! profile_scope {
    ($name:expr) => {
        #[cfg(feature = "profiling")]
        let _timer = $crate::perf::ScopedTimer::for_profiling($name);
        #[cfg(not(feature = "profiling"))]
        let _ = $name; // Suppress unused variable warning
    };
    ($name:expr, $threshold_ms:expr) => {
        #[cfg(feature = "profiling")]
        let _timer = $crate::perf::ScopedTimer::new($name, $threshold_ms);
        #[cfg(not(feature = "profiling"))]
        let _ = ($name, $threshold_ms);
    };
}

/// Profile the current function. Zero-cost when profiling is disabled.
///
/// # Example
/// ```ignore
/// use humanboard::perf::profile_function;
///
/// fn handle_mouse_down() {
///     profile_function!();
///     // ... event handling code ...
/// }
/// ```
#[macro_export]
macro_rules! profile_function {
    () => {
        $crate::profile_scope!(concat!(module_path!(), "::", $crate::function_name!()));
    };
}

/// Helper macro to get function name (requires nightly or workaround)
#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        // Strip the trailing "::f" from the function name
        &name[..name.len() - 3]
    }};
}

// Re-export macros at crate root
pub use profile_function;
pub use profile_scope;

// ============================================================================
// Runtime Profiling Control
// ============================================================================

/// Enable or disable profiling at runtime.
/// Note: This only affects code compiled with the `profiling` feature.
pub fn set_profiling_enabled(enabled: bool) {
    PROFILING_ENABLED.store(enabled, Ordering::Relaxed);
}

/// Check if profiling is currently enabled.
#[inline]
pub fn is_profiling_enabled() -> bool {
    PROFILING_ENABLED.load(Ordering::Relaxed)
}

// ============================================================================
// Frame Performance Monitor
// ============================================================================

/// Comprehensive performance monitor for tracking frame times and operation statistics.
pub struct PerfMonitor {
    /// Recent frame times in milliseconds
    frame_times: VecDeque<f64>,
    /// When the current frame started
    frame_start: Option<Instant>,
    /// Count of frames that exceeded the warning threshold
    slow_frame_count: u64,
    /// Total frames tracked
    total_frames: u64,
    /// Per-operation timing statistics
    operation_stats: HashMap<&'static str, OperationStats>,
    /// Current frame's operation timings (for hierarchical display)
    current_frame_ops: Vec<OperationTiming>,
}

/// Statistics for a specific operation type.
#[derive(Debug, Clone)]
pub struct OperationStats {
    /// Recent timing samples in milliseconds
    samples: VecDeque<f64>,
    /// Total invocation count
    count: u64,
    /// Minimum observed time
    min_ms: f64,
    /// Maximum observed time
    max_ms: f64,
    /// Running sum for average calculation
    sum_ms: f64,
}

impl Default for OperationStats {
    fn default() -> Self {
        Self {
            samples: VecDeque::with_capacity(STATS_SAMPLE_COUNT),
            count: 0,
            min_ms: f64::MAX,
            max_ms: 0.0,
            sum_ms: 0.0,
        }
    }
}

impl OperationStats {
    /// Record a new timing sample.
    pub fn record(&mut self, ms: f64) {
        if self.samples.len() >= STATS_SAMPLE_COUNT {
            if let Some(old) = self.samples.pop_front() {
                self.sum_ms -= old;
            }
        }
        self.samples.push_back(ms);
        self.sum_ms += ms;
        self.count += 1;
        self.min_ms = self.min_ms.min(ms);
        self.max_ms = self.max_ms.max(ms);
    }

    /// Get the average time over recent samples.
    pub fn average(&self) -> f64 {
        if self.samples.is_empty() {
            0.0
        } else {
            self.sum_ms / self.samples.len() as f64
        }
    }

    /// Get the p95 (95th percentile) time.
    pub fn p95(&self) -> f64 {
        if self.samples.is_empty() {
            return 0.0;
        }
        let mut sorted: Vec<f64> = self.samples.iter().copied().collect();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let idx = ((sorted.len() as f64) * 0.95).floor() as usize;
        sorted.get(idx.min(sorted.len() - 1)).copied().unwrap_or(0.0)
    }
}

/// Timing information for a single operation invocation.
#[derive(Debug, Clone)]
pub struct OperationTiming {
    pub name: &'static str,
    pub elapsed_ms: f64,
    pub depth: usize,
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
            operation_stats: HashMap::new(),
            current_frame_ops: Vec::new(),
        }
    }

    /// Mark the start of a frame.
    pub fn begin_frame(&mut self) {
        self.frame_start = Some(Instant::now());
        self.current_frame_ops.clear();
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

            // Log breakdown of slow frame
            #[cfg(feature = "profiling")]
            self.log_frame_breakdown();
        }

        Some(ms)
    }

    /// Record an operation timing.
    pub fn record_operation(&mut self, name: &'static str, elapsed_ms: f64, depth: usize) {
        // Update per-operation statistics
        self.operation_stats
            .entry(name)
            .or_default()
            .record(elapsed_ms);

        // Record for current frame breakdown
        self.current_frame_ops.push(OperationTiming {
            name,
            elapsed_ms,
            depth,
        });
    }

    /// Log a breakdown of the current frame's operations.
    #[cfg(feature = "profiling")]
    fn log_frame_breakdown(&self) {
        if self.current_frame_ops.is_empty() {
            return;
        }

        debug!("Frame breakdown:");
        for op in &self.current_frame_ops {
            let indent = "  ".repeat(op.depth);
            debug!("{}{}: {:.2}ms", indent, op.name, op.elapsed_ms);
        }
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

    /// Get statistics for a specific operation.
    pub fn get_operation_stats(&self, name: &str) -> Option<&OperationStats> {
        self.operation_stats.get(name)
    }

    /// Get all operation statistics.
    pub fn all_operation_stats(&self) -> &HashMap<&'static str, OperationStats> {
        &self.operation_stats
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

            // Log top slow operations
            self.log_slow_operations();
        }
    }

    /// Log the slowest operations.
    fn log_slow_operations(&self) {
        let mut ops: Vec<_> = self.operation_stats.iter().collect();
        ops.sort_by(|a, b| {
            b.1.average()
                .partial_cmp(&a.1.average())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        debug!("Top slow operations:");
        for (name, stats) in ops.iter().take(5) {
            if stats.average() > 0.1 {
                // Only show ops taking >0.1ms
                debug!(
                    "  {}: avg={:.2}ms, p95={:.2}ms, max={:.2}ms, count={}",
                    name,
                    stats.average(),
                    stats.p95(),
                    stats.max_ms,
                    stats.count
                );
            }
        }
    }

    /// Reset all statistics.
    pub fn reset(&mut self) {
        self.frame_times.clear();
        self.slow_frame_count = 0;
        self.total_frames = 0;
        self.operation_stats.clear();
        self.current_frame_ops.clear();
    }
}

// ============================================================================
// Scoped Timer
// ============================================================================

/// A scoped timer that logs duration on drop.
///
/// When the `profiling` feature is enabled, timers automatically report
/// their results to a thread-local profiler for aggregation.
pub struct ScopedTimer {
    name: &'static str,
    start: Instant,
    threshold_ms: f64,
    #[allow(dead_code)]
    timer_id: u64,
    #[cfg(feature = "profiling")]
    depth: usize,
}

// Thread-local depth tracking for hierarchical profiling
#[cfg(feature = "profiling")]
thread_local! {
    static CURRENT_DEPTH: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

impl ScopedTimer {
    /// Create a new scoped timer with a warning threshold.
    pub fn new(name: &'static str, threshold_ms: f64) -> Self {
        let timer_id = TIMER_COUNTER.fetch_add(1, Ordering::Relaxed);

        #[cfg(feature = "profiling")]
        let depth = CURRENT_DEPTH.with(|d| {
            let current = d.get();
            d.set(current + 1);
            current
        });

        Self {
            name,
            start: Instant::now(),
            threshold_ms,
            timer_id,
            #[cfg(feature = "profiling")]
            depth,
        }
    }

    /// Create a timer with the default threshold (16ms).
    pub fn with_default_threshold(name: &'static str) -> Self {
        Self::new(name, TARGET_FRAME_MS)
    }

    /// Create a timer for profiling (lower threshold, 1ms).
    pub fn for_profiling(name: &'static str) -> Self {
        Self::new(name, 1.0)
    }

    /// Create a timer that always logs (threshold of 0).
    #[allow(dead_code)]
    pub fn always_log(name: &'static str) -> Self {
        Self::new(name, 0.0)
    }

    /// Get elapsed time without stopping the timer.
    #[allow(dead_code)]
    pub fn elapsed_ms(&self) -> f64 {
        self.start.elapsed().as_secs_f64() * 1000.0
    }

    /// Get the timer's name.
    #[allow(dead_code)]
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Get the timer's unique ID.
    #[allow(dead_code)]
    pub fn id(&self) -> u64 {
        self.timer_id
    }
}

impl Drop for ScopedTimer {
    fn drop(&mut self) {
        let elapsed_ms = self.start.elapsed().as_secs_f64() * 1000.0;

        #[cfg(feature = "profiling")]
        {
            // Decrement depth
            CURRENT_DEPTH.with(|d| d.set(d.get().saturating_sub(1)));

            // Log with hierarchy indication
            if elapsed_ms > self.threshold_ms {
                let indent = "  ".repeat(self.depth);
                trace!(
                    "{}[PERF] {}: {:.2}ms",
                    indent,
                    self.name,
                    elapsed_ms
                );
            }
        }

        #[cfg(not(feature = "profiling"))]
        {
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
}

// ============================================================================
// Timing Utilities
// ============================================================================

/// Measure execution time of a closure and return both the result and elapsed time.
///
/// # Example
/// ```ignore
/// let (result, elapsed_ms) = measure(|| expensive_computation());
/// println!("Computed {} in {:.2}ms", result, elapsed_ms);
/// ```
#[inline]
pub fn measure<T, F: FnOnce() -> T>(f: F) -> (T, f64) {
    let start = Instant::now();
    let result = f();
    let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
    (result, elapsed_ms)
}

/// Measure execution time and log if it exceeds the threshold.
///
/// # Example
/// ```ignore
/// let result = measure_and_log("render_canvas", 5.0, || render_all_items());
/// ```
#[inline]
pub fn measure_and_log<T, F: FnOnce() -> T>(name: &str, threshold_ms: f64, f: F) -> T {
    let (result, elapsed_ms) = measure(f);
    if elapsed_ms > threshold_ms {
        warn!(
            operation = name,
            elapsed_ms = format!("{:.2}", elapsed_ms),
            threshold_ms = format!("{:.2}", threshold_ms),
            "Slow operation"
        );
    }
    result
}

/// Measure execution time only when profiling is enabled.
/// Returns the result directly without timing overhead when disabled.
#[inline]
#[allow(dead_code)]
pub fn measure_if_profiling<T, F: FnOnce() -> T>(name: &'static str, f: F) -> T {
    #[cfg(feature = "profiling")]
    {
        let _timer = ScopedTimer::for_profiling(name);
        f()
    }
    #[cfg(not(feature = "profiling"))]
    {
        let _ = name;
        f()
    }
}

// ============================================================================
// Hit Testing Performance Helpers
// ============================================================================

/// Performance-aware hit testing configuration.
/// Tracks hit testing performance and provides optimization hints.
pub struct HitTestProfiler {
    /// Number of items tested in the last hit test
    pub items_tested: usize,
    /// Time taken for the last hit test
    pub last_test_ms: f64,
    /// Average items per hit test
    avg_items: f64,
    /// Total hit tests performed
    #[allow(dead_code)]
    test_count: u64,
}

impl Default for HitTestProfiler {
    fn default() -> Self {
        Self::new()
    }
}

impl HitTestProfiler {
    pub fn new() -> Self {
        Self {
            items_tested: 0,
            last_test_ms: 0.0,
            avg_items: 0.0,
            test_count: 0,
        }
    }

    /// Record a hit test result.
    pub fn record(&mut self, items_tested: usize, elapsed_ms: f64) {
        self.items_tested = items_tested;
        self.last_test_ms = elapsed_ms;
        self.test_count += 1;

        // Update running average
        let alpha = 0.1; // Exponential moving average factor
        self.avg_items = self.avg_items * (1.0 - alpha) + items_tested as f64 * alpha;
    }

    /// Check if hit testing is becoming a bottleneck.
    #[allow(dead_code)]
    pub fn is_bottleneck(&self) -> bool {
        // Consider it a bottleneck if average > 100 items and taking > 1ms
        self.avg_items > 100.0 && self.last_test_ms > 1.0
    }

    /// Get optimization suggestion.
    #[allow(dead_code)]
    pub fn optimization_hint(&self) -> Option<&'static str> {
        if self.avg_items > 500.0 {
            Some("Consider spatial partitioning (quadtree) for hit testing")
        } else if self.avg_items > 100.0 && self.last_test_ms > 2.0 {
            Some("Hit testing taking too long - consider viewport culling")
        } else {
            None
        }
    }
}

// ============================================================================
// Render Performance Helpers
// ============================================================================

/// Performance budget tracker for render operations.
/// Helps ensure individual operations stay within frame budget.
#[allow(dead_code)]
pub struct RenderBudget {
    /// Total budget in milliseconds
    budget_ms: f64,
    /// Time spent so far
    spent_ms: f64,
    /// Start of current budget period
    start: Instant,
}

impl RenderBudget {
    /// Create a new render budget (default: 12ms to leave headroom for GPUI)
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::with_budget(12.0)
    }

    /// Create a render budget with custom limit.
    #[allow(dead_code)]
    pub fn with_budget(budget_ms: f64) -> Self {
        Self {
            budget_ms,
            spent_ms: 0.0,
            start: Instant::now(),
        }
    }

    /// Check remaining budget.
    #[inline]
    #[allow(dead_code)]
    pub fn remaining_ms(&self) -> f64 {
        let elapsed = self.start.elapsed().as_secs_f64() * 1000.0;
        (self.budget_ms - elapsed).max(0.0)
    }

    /// Check if we've exceeded the budget.
    #[inline]
    #[allow(dead_code)]
    pub fn is_exceeded(&self) -> bool {
        self.remaining_ms() <= 0.0
    }

    /// Record time spent on an operation.
    #[allow(dead_code)]
    pub fn record(&mut self, operation: &str, elapsed_ms: f64) {
        self.spent_ms += elapsed_ms;

        #[cfg(feature = "profiling")]
        trace!("[BUDGET] {}: {:.2}ms (remaining: {:.2}ms)",
               operation, elapsed_ms, self.remaining_ms());

        #[cfg(not(feature = "profiling"))]
        let _ = operation;
    }

    /// Check if we should skip a non-critical operation to stay in budget.
    #[inline]
    #[allow(dead_code)]
    pub fn should_skip(&self, estimated_ms: f64) -> bool {
        self.remaining_ms() < estimated_ms
    }
}

impl Default for RenderBudget {
    fn default() -> Self {
        Self::new()
    }
}
