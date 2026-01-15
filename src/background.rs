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

use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
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
                    // Check if we should stop - recover from poisoned lock to allow graceful shutdown
                    let should_stop = running
                        .lock()
                        .map(|guard| !*guard)
                        .unwrap_or(true); // Stop if lock is poisoned
                    if should_stop {
                        break;
                    }

                    // Try to get a task - recover from poisoned lock
                    let task = {
                        let rx = match task_rx.lock() {
                            Ok(guard) => guard,
                            Err(poisoned) => poisoned.into_inner(),
                        };
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
                            if let Ok(mut count) = pending_count.lock() {
                                *count = count.saturating_sub(1);
                            }
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
        if let Ok(mut count) = self.pending_count.lock() {
            *count += 1;
        }

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
        self.pending_count
            .lock()
            .map(|count| *count > 0)
            .unwrap_or(false)
    }

    /// Get the number of pending tasks.
    pub fn pending_count(&self) -> usize {
        self.pending_count.lock().map(|c| *c).unwrap_or(0)
    }

    /// Shutdown the executor gracefully.
    pub fn shutdown(&mut self) {
        // Signal workers to stop
        if let Ok(mut running) = self.running.lock() {
            *running = false;
        }

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::Duration;

    #[test]
    fn test_executor_creation() {
        let executor = BackgroundExecutor::new(2);
        assert!(!executor.has_pending());
        assert_eq!(executor.pending_count(), 0);
    }

    #[test]
    fn test_spawn_and_complete() {
        let executor = BackgroundExecutor::new(1);
        let completed = Arc::new(AtomicBool::new(false));
        let completed_clone = Arc::clone(&completed);

        executor.spawn(
            "test_task",
            || Ok(42),
            move |result: TaskResult<i32>| {
                assert_eq!(result.unwrap(), 42);
                completed_clone.store(true, Ordering::SeqCst);
            },
        );

        // Wait a bit for task to complete
        thread::sleep(Duration::from_millis(100));

        // Process results
        let processed = executor.process_results();
        assert_eq!(processed, 1);
        assert!(completed.load(Ordering::SeqCst));
    }

    #[test]
    fn test_error_handling() {
        let executor = BackgroundExecutor::new(1);
        let got_error = Arc::new(AtomicBool::new(false));
        let got_error_clone = Arc::clone(&got_error);

        executor.spawn(
            "failing_task",
            || Err::<(), _>("intentional error".to_string()),
            move |result: TaskResult<()>| {
                assert!(result.is_err());
                got_error_clone.store(true, Ordering::SeqCst);
            },
        );

        thread::sleep(Duration::from_millis(100));
        executor.process_results();
        assert!(got_error.load(Ordering::SeqCst));
    }

    #[test]
    fn test_multiple_tasks() {
        let executor = BackgroundExecutor::new(2);
        let counter = Arc::new(Mutex::new(0));

        for i in 0..5 {
            let counter = Arc::clone(&counter);
            executor.spawn(
                &format!("task_{}", i),
                move || Ok(i),
                move |result: TaskResult<i32>| {
                    if result.is_ok() {
                        *counter.lock().unwrap() += 1;
                    }
                },
            );
        }

        // Wait for all tasks
        thread::sleep(Duration::from_millis(200));
        executor.process_results();

        assert_eq!(*counter.lock().unwrap(), 5);
    }
}
