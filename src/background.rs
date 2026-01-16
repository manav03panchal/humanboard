//! Background task system for async operations.
//!
//! Provides a simple way to run CPU-intensive or blocking operations
//! off the main thread, with results delivered back via callbacks.
//!
//! ## Usage
//!
//! ```ignore
//! // Queue a background task
//! background.spawn("load_pdf", move || {
//!     // This runs on a background thread
//!     load_pdf_pages(&path)
//! }, |result, cx| {
//!     // This runs on the main thread when complete
//!     match result {
//!         Ok(pages) => { /* update state */ }
//!         Err(e) => { /* show error */ }
//!     }
//! });
//! ```

use parking_lot::Mutex;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use tracing::{debug, error, info_span, warn};

/// Result of a background task
pub type TaskResult<T> = Result<T, String>;

/// A pending task result waiting to be processed on the main thread
struct PendingResult {
    task_name: String,
    result: Box<dyn std::any::Any + Send>,
    callback: Box<dyn FnOnce(Box<dyn std::any::Any + Send>) + Send>,
}

/// Background task executor.
///
/// Manages a pool of worker threads and provides a way to run tasks
/// off the main thread with results delivered back via callbacks.
pub struct BackgroundExecutor {
    /// Sender for queuing tasks
    task_tx: Sender<BackgroundTask>,
    /// Receiver for completed task results
    result_rx: Receiver<PendingResult>,
    /// Worker threads
    workers: Vec<JoinHandle<()>>,
    /// Whether the executor is running
    running: Arc<Mutex<bool>>,
    /// Count of pending tasks
    pending_count: Arc<Mutex<usize>>,
}

/// A task to be executed in the background
struct BackgroundTask {
    name: String,
    work: Box<dyn FnOnce() -> Box<dyn std::any::Any + Send> + Send>,
    callback: Box<dyn FnOnce(Box<dyn std::any::Any + Send>) + Send>,
}

impl BackgroundExecutor {
    /// Create a new background executor with the specified number of worker threads.
    pub fn new(num_workers: usize) -> Self {
        let (task_tx, task_rx) = mpsc::channel::<BackgroundTask>();
        let (result_tx, result_rx) = mpsc::channel::<PendingResult>();
        let running = Arc::new(Mutex::new(true));
        let pending_count = Arc::new(Mutex::new(0usize));

        let task_rx = Arc::new(Mutex::new(task_rx));
        let mut workers = Vec::with_capacity(num_workers);

        for i in 0..num_workers {
            let task_rx = Arc::clone(&task_rx);
            let result_tx = result_tx.clone();
            let running = Arc::clone(&running);
            let pending_count = Arc::clone(&pending_count);

            let handle = thread::spawn(move || {
                loop {
                    // Check if we should stop
                    if !*running.lock() {
                        break;
                    }

                    // Try to get a task
                    let task = {
                        let rx = task_rx.lock();
                        rx.recv_timeout(std::time::Duration::from_millis(100))
                    };

                    match task {
                        Ok(task) => {
                            let _span = info_span!("background_task",
                                name = %task.name,
                                worker = i
                            ).entered();

                            debug!("Starting task: {}", task.name);
                            let start = std::time::Instant::now();

                            // Execute the work
                            let result = (task.work)();

                            let elapsed = start.elapsed();
                            debug!("Task {} completed in {:?}", task.name, elapsed);

                            // Send result back
                            let pending = PendingResult {
                                task_name: task.name,
                                result,
                                callback: task.callback,
                            };

                            if result_tx.send(pending).is_err() {
                                warn!("Failed to send task result - receiver dropped");
                            }

                            // Decrement pending count
                            let mut count = pending_count.lock();
                            *count = count.saturating_sub(1);
                        }
                        Err(mpsc::RecvTimeoutError::Timeout) => {
                            // No task available, continue waiting
                        }
                        Err(mpsc::RecvTimeoutError::Disconnected) => {
                            // Channel closed, exit
                            break;
                        }
                    }
                }
                debug!("Worker {} shutting down", i);
            });

            workers.push(handle);
        }

        Self {
            task_tx,
            result_rx,
            workers,
            running,
            pending_count,
        }
    }

    /// Create an executor with a default number of workers (2).
    pub fn with_default_workers() -> Self {
        Self::new(2)
    }

    /// Spawn a background task.
    ///
    /// The `work` closure runs on a background thread.
    /// The `on_complete` closure runs on the main thread when you call `process_results`.
    pub fn spawn<T, F, C>(&self, name: &str, work: F, on_complete: C)
    where
        T: Send + 'static,
        F: FnOnce() -> TaskResult<T> + Send + 'static,
        C: FnOnce(TaskResult<T>) + Send + 'static,
    {
        // Increment pending count
        *self.pending_count.lock() += 1;

        let task = BackgroundTask {
            name: name.to_string(),
            work: Box::new(move || {
                let result = work();
                Box::new(result) as Box<dyn std::any::Any + Send>
            }),
            callback: Box::new(move |boxed_result| {
                if let Ok(result) = boxed_result.downcast::<TaskResult<T>>() {
                    on_complete(*result);
                } else {
                    error!("Failed to downcast task result");
                }
            }),
        };

        if self.task_tx.send(task).is_err() {
            error!("Failed to queue background task: {}", name);
        } else {
            debug!("Queued background task: {}", name);
        }
    }

    /// Process any completed task results on the main thread.
    ///
    /// Call this periodically (e.g., in your render loop) to handle
    /// completed background tasks.
    ///
    /// Returns the number of results processed.
    pub fn process_results(&self) -> usize {
        let mut processed = 0;

        // Process all available results without blocking
        while let Ok(pending) = self.result_rx.try_recv() {
            debug!("Processing result for task: {}", pending.task_name);
            (pending.callback)(pending.result);
            processed += 1;
        }

        processed
    }

    /// Check if there are any pending tasks.
    pub fn has_pending(&self) -> bool {
        *self.pending_count.lock() > 0
    }

    /// Get the number of pending tasks.
    pub fn pending_count(&self) -> usize {
        *self.pending_count.lock()
    }

    /// Shutdown the executor gracefully.
    pub fn shutdown(&mut self) {
        // Signal workers to stop
        *self.running.lock() = false;

        // Wait for workers to finish (with timeout)
        for handle in self.workers.drain(..) {
            let _ = handle.join();
        }

        debug!("Background executor shut down");
    }
}

impl Drop for BackgroundExecutor {
    fn drop(&mut self) {
        self.shutdown();
    }
}

impl Default for BackgroundExecutor {
    fn default() -> Self {
        Self::with_default_workers()
    }
}

// ============================================================================
// Convenience functions for common background tasks
// ============================================================================

/// Load file contents in background
pub fn load_file_async<F>(
    executor: &BackgroundExecutor,
    path: std::path::PathBuf,
    on_complete: F,
)
where
    F: FnOnce(TaskResult<Vec<u8>>) + Send + 'static,
{
    let path_str = path.display().to_string();
    executor.spawn(
        &format!("load_file:{}", path_str),
        move || {
            std::fs::read(&path)
                .map_err(|e| format!("Failed to read {}: {}", path.display(), e))
        },
        on_complete,
    );
}

/// Load text file in background
pub fn load_text_async<F>(
    executor: &BackgroundExecutor,
    path: std::path::PathBuf,
    on_complete: F,
)
where
    F: FnOnce(TaskResult<String>) + Send + 'static,
{
    let path_str = path.display().to_string();
    executor.spawn(
        &format!("load_text:{}", path_str),
        move || {
            std::fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read {}: {}", path.display(), e))
        },
        on_complete,
    );
}
