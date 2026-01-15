//! Unit tests for background module.

use humanboard::background::{BackgroundExecutor, TaskResult};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
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
