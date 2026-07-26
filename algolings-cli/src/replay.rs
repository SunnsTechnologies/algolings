//! Runs a learner's (or reference) solution in isolation, with tracing
//! enabled, catching panics and enforcing a timeout — the "replay
//! invocation, separate from cargo test" piece the design doc's step 1
//! spike was scoped to resolve.

use algolings_trace::Event;
use std::panic::{self, AssertUnwindSafe};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Eq)]
pub enum ReplayError {
    Panicked(String),
    TimedOut,
}

/// Runs `f` on a fresh thread with tracing enabled, returning the recorded
/// events. `f` is expected to call into `algolings_trace`'s helpers
/// (`cmp_lt`, `swap`, `set_at`, `mark_sorted`) as part of running the
/// exercise solution.
///
/// Running on a dedicated thread serves two purposes at once: it gives
/// panic-catch and timeout detection via a channel + `recv_timeout`, AND it
/// isolates the thread-local trace buffer per invocation for free (no
/// manual reset needed between replays).
///
/// KNOWN LIMITATION (surfaced by this spike, not solved by it): on timeout,
/// there is no safe stable-Rust way to force-kill the spawned thread. A
/// genuinely infinite-looping solution's thread keeps running in the
/// background after this function returns `TimedOut` — it just no longer
/// blocks the caller. Repeated timeouts (e.g. a learner keeps saving a
/// still-broken infinite loop) accumulate orphaned background threads
/// consuming CPU until the whole `algolings` process exits. A true kill
/// would require running the replay in a child OS process instead of a
/// thread, which is a bigger design change deferred past this spike.
pub fn replay<F>(timeout: Duration, f: F) -> Result<Vec<Event>, ReplayError>
where
    F: FnOnce() + Send + 'static,
{
    let (tx, rx) = mpsc::channel();

    let _handle = thread::spawn(move || {
        algolings_trace::enable();
        let result = panic::catch_unwind(AssertUnwindSafe(f));
        let events = algolings_trace::take_events();
        algolings_trace::disable();
        // Ignore send errors: if the receiver already timed out and moved
        // on, there's nothing left to deliver the result to.
        let _ = tx.send((result, events));
    });

    match rx.recv_timeout(timeout) {
        Ok((Ok(()), events)) => Ok(events),
        Ok((Err(panic_payload), _events)) => Err(ReplayError::Panicked(panic_message(
            &panic_payload,
        ))),
        Err(_) => Err(ReplayError::TimedOut),
    }
}

fn panic_message(payload: &Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "unknown panic".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{cmp_lt, mark_sorted, swap};
    use std::time::Duration;

    fn short_timeout() -> Duration {
        Duration::from_millis(300)
    }

    #[test]
    fn successful_solution_returns_its_trace_events() {
        let result = replay(short_timeout(), || {
            let mut arr = [2, 1];
            if cmp_lt(&arr, 1, 0) {
                swap(&mut arr, 0, 1);
            }
            mark_sorted(0);
            mark_sorted(1);
        });

        let events = result.expect("well-behaved solution should not error");
        assert!(!events.is_empty());
    }

    #[test]
    fn panicking_solution_is_caught_not_propagated() {
        let result = replay(short_timeout(), || {
            panic!("learner's solution has a bug: index out of bounds");
        });

        assert_eq!(
            result,
            Err(ReplayError::Panicked(
                "learner's solution has a bug: index out of bounds".to_string()
            ))
        );
    }

    #[test]
    fn infinite_looping_solution_times_out_instead_of_hanging() {
        let result = replay(short_timeout(), || {
            loop {
                std::hint::spin_loop();
            }
        });

        assert_eq!(result, Err(ReplayError::TimedOut));
    }

    #[test]
    fn replay_does_not_block_caller_past_the_timeout() {
        let start = std::time::Instant::now();
        let _ = replay(short_timeout(), || loop {
            std::hint::spin_loop();
        });
        let elapsed = start.elapsed();
        assert!(
            elapsed < Duration::from_secs(2),
            "replay took {elapsed:?}, should return promptly at the timeout boundary"
        );
    }
}
