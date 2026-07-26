//! Debounce and stale-run-cancellation primitives for the file watcher
//! (Architecture review issues 1-2, eng review). Kept decoupled from the
//! actual `notify` filesystem events so the timing/coalescing logic is
//! unit-testable without touching a real filesystem.

use crate::test_runner::{run_package_tests, TestOutcome};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::time::Duration;

/// Tracks which "generation" of file-change is current, so a background
/// test/replay run started for an older generation can tell it's been
/// superseded and its result should be discarded rather than displayed.
pub struct Debouncer {
    generation: AtomicU64,
}

impl Debouncer {
    pub fn new() -> Self {
        Self {
            generation: AtomicU64::new(0),
        }
    }

    /// Call when a settled (debounced) file change is observed. Returns a
    /// token identifying this generation.
    pub fn bump(&self) -> u64 {
        self.generation.fetch_add(1, Ordering::SeqCst) + 1
    }

    /// True if `token` is still the most recent generation.
    pub fn is_current(&self, token: u64) -> bool {
        self.generation.load(Ordering::SeqCst) == token
    }
}

impl Default for Debouncer {
    fn default() -> Self {
        Self::new()
    }
}

/// Blocks until `rx` has been silent for `quiet_period`, draining and
/// coalescing any events that arrive in the meantime — so several rapid
/// saves (common with editors that fire multiple filesystem events per
/// save) settle into a single trigger instead of one per event.
pub fn wait_for_quiet(rx: &Receiver<()>, quiet_period: Duration) -> Result<(), RecvTimeoutError> {
    // Block for at least one event before starting the quiet-period clock.
    rx.recv().map_err(|_| RecvTimeoutError::Disconnected)?;
    loop {
        match rx.recv_timeout(quiet_period) {
            Ok(()) => continue,
            // Timeout means genuinely quiet. Disconnected-after-an-event
            // means the sender is done (e.g. the watcher shut down) — from
            // the caller's perspective both mean "no more events coming,
            // safe to act now", so both settle successfully.
            Err(RecvTimeoutError::Timeout) => return Ok(()),
            Err(RecvTimeoutError::Disconnected) => return Ok(()),
        }
    }
}

/// Watches `path` for filesystem changes, forwarding a `()` signal for each
/// notify event. The returned watcher must be kept alive for as long as
/// watching should continue — dropping it stops the underlying OS watch.
pub fn watch_path(path: &Path) -> notify::Result<(RecommendedWatcher, Receiver<()>)> {
    let (tx, rx) = mpsc::channel();
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
        if res.is_ok() {
            let _ = tx.send(());
        }
    })?;
    watcher.watch(path, RecursiveMode::NonRecursive)?;
    Ok((watcher, rx))
}

/// The core `algolings watch` loop: watch `skeleton_path`, debounce saves,
/// run `package`'s tests (filtered by `test_filter`) once settled, and
/// report the outcome — discarding results superseded by a newer save
/// before they complete.
///
/// `max_iterations`: `None` runs forever (the real CLI); `Some(n)` stops
/// after `n` settled runs (used by tests, which can't block forever).
#[allow(clippy::too_many_arguments)]
pub fn run_watch_loop(
    workspace_root: &Path,
    skeleton_path: &Path,
    package: &str,
    test_filter: &str,
    quiet_period: Duration,
    max_iterations: Option<u32>,
    mut on_settled: impl FnMut(),
    mut on_result: impl FnMut(&TestOutcome),
) -> notify::Result<()> {
    let absolute_path = if skeleton_path.is_absolute() {
        skeleton_path.to_path_buf()
    } else {
        workspace_root.join(skeleton_path)
    };
    let (_watcher, rx) = watch_path(&absolute_path)?;
    let debouncer = Debouncer::new();

    let mut ran = 0;
    loop {
        if wait_for_quiet(&rx, quiet_period).is_err() {
            break;
        }
        on_settled();
        let token = debouncer.bump();
        if let Ok(outcome) = run_package_tests(workspace_root, package, test_filter)
            && debouncer.is_current(token)
        {
            on_result(&outcome);
        }
        ran += 1;
        if max_iterations.is_some_and(|max| ran >= max) {
            break;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn fresh_token_is_current() {
        let debouncer = Debouncer::new();
        let token = debouncer.bump();
        assert!(debouncer.is_current(token));
    }

    #[test]
    fn superseded_token_is_no_longer_current() {
        let debouncer = Debouncer::new();
        let stale_token = debouncer.bump();
        let _fresh_token = debouncer.bump();
        assert!(!debouncer.is_current(stale_token));
    }

    #[test]
    fn wait_for_quiet_returns_after_a_single_event_settles() {
        let (tx, rx) = mpsc::channel();
        tx.send(()).unwrap();
        let result = wait_for_quiet(&rx, Duration::from_millis(30));
        assert!(result.is_ok());
    }

    #[test]
    fn wait_for_quiet_coalesces_rapid_events_into_one_settle() {
        let (tx, rx) = mpsc::channel();
        // Keep a sender alive in this thread for the whole test, so the
        // channel disconnecting (once the spawned thread finishes) doesn't
        // interfere with measuring the pure quiet-period debounce behavior.
        let _tx_keep_alive = tx.clone();
        thread::spawn(move || {
            for _ in 0..5 {
                tx.send(()).unwrap();
                thread::sleep(Duration::from_millis(5));
            }
        });
        let start = std::time::Instant::now();
        wait_for_quiet(&rx, Duration::from_millis(40)).unwrap();
        let elapsed = start.elapsed();
        // 5 events, 5ms apart (~20ms total), then a 40ms quiet window: the
        // settle should land around ~60ms, not fire on the first event
        // (~40ms) or take drastically longer.
        assert!(elapsed >= Duration::from_millis(40));
        assert!(elapsed < Duration::from_millis(200));
    }

    #[test]
    fn wait_for_quiet_errors_when_sender_is_dropped_before_any_event() {
        let (tx, rx) = mpsc::channel::<()>();
        drop(tx);
        assert!(wait_for_quiet(&rx, Duration::from_millis(10)).is_err());
    }

    #[test]
    fn wait_for_quiet_settles_promptly_if_sender_disconnects_after_an_event() {
        // Models the watcher shutting down mid-debounce: once at least one
        // event has been seen, a disconnect should settle immediately
        // rather than waiting out the full quiet period or erroring.
        let (tx, rx) = mpsc::channel();
        tx.send(()).unwrap();
        drop(tx);
        let start = std::time::Instant::now();
        let result = wait_for_quiet(&rx, Duration::from_millis(200));
        assert!(result.is_ok());
        assert!(start.elapsed() < Duration::from_millis(200));
    }

    #[test]
    fn watch_path_reports_a_real_file_write() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("watched.rs");
        std::fs::write(&file_path, "initial").unwrap();

        let (_watcher, rx) = watch_path(&file_path).unwrap();
        // Give the OS watch a moment to fully register before writing.
        thread::sleep(Duration::from_millis(50));
        std::fs::write(&file_path, "changed").unwrap();

        let result = rx.recv_timeout(Duration::from_secs(2));
        assert!(result.is_ok(), "expected a filesystem event within 2s");
    }

    #[test]
    fn run_watch_loop_runs_tests_once_settled_and_reports_the_result() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("watched.rs");
        std::fs::write(&file_path, "initial").unwrap();

        let received = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let received_clone = received.clone();

        let workspace_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();

        let settled_count = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
        let settled_count_clone = settled_count.clone();

        let watched_path = file_path.clone();
        let handle = thread::spawn(move || {
            run_watch_loop(
                &workspace_root,
                &watched_path,
                "exercises-sort",
                "bubble",
                Duration::from_millis(30),
                Some(1),
                move || {
                    settled_count_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                },
                |outcome| received_clone.lock().unwrap().push(outcome.passed),
            )
        });

        thread::sleep(Duration::from_millis(100));
        std::fs::write(&file_path, "changed").unwrap();

        handle.join().unwrap().unwrap();
        assert_eq!(received.lock().unwrap().len(), 1);
        assert_eq!(settled_count.load(std::sync::atomic::Ordering::SeqCst), 1);
    }
}
