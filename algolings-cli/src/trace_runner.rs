//! Runs a command with a real, enforceable timeout: unlike a thread (which
//! can't be safely force-killed on stable Rust — the limitation the step-1
//! spike surfaced), a child process CAN be killed outright. This is the
//! generic polling+kill mechanism; `run_trace` below wraps it with the
//! specific `cargo run --bin trace` invocation and JSON parsing.

use std::io::Read;
use std::process::{Command, Output};
use std::thread;
use std::time::{Duration, Instant};

const POLL_INTERVAL: Duration = Duration::from_millis(20);

#[derive(Debug, PartialEq, Eq)]
pub enum TimeoutOrIo {
    TimedOut,
    Io(String),
}

/// Spawns `command`, polling for completion. If it hasn't finished by
/// `timeout`, it is killed and `Err(TimedOut)` is returned. Returns the
/// process's output (regardless of exit status) if it completes in time —
/// callers decide what a non-zero exit means for their use case.
pub fn run_with_timeout(mut command: Command, timeout: Duration) -> Result<Output, TimeoutOrIo> {
    use std::process::Stdio;
    // Never inherit stdin: this subprocess doesn't read input, and an
    // inherited stdin could contend with the CLI's hint listener (a
    // background thread reading real stdin) for keystrokes.
    command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = command.spawn().map_err(|e| TimeoutOrIo::Io(e.to_string()))?;
    let start = Instant::now();

    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let mut stdout = Vec::new();
                let mut stderr = Vec::new();
                if let Some(mut out) = child.stdout.take() {
                    let _ = out.read_to_end(&mut stdout);
                }
                if let Some(mut err) = child.stderr.take() {
                    let _ = err.read_to_end(&mut stderr);
                }
                return Ok(Output {
                    status,
                    stdout,
                    stderr,
                });
            }
            Ok(None) => {
                if start.elapsed() >= timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(TimeoutOrIo::TimedOut);
                }
                thread::sleep(POLL_INTERVAL);
            }
            Err(e) => return Err(TimeoutOrIo::Io(e.to_string())),
        }
    }
}

/// Errors from running the trace subprocess against the current on-disk
/// exercise code.
#[derive(Debug, PartialEq)]
pub enum TraceError {
    /// The solution panicked while running (e.g. `todo!()`, or a genuine
    /// bug). Carries the subprocess's stderr.
    Panicked(String),
    TimedOut,
    Io(String),
}

/// Runs `exercise_name`'s solution in `package` against `fixture`, with
/// tracing enabled, as a fresh subprocess (`cargo run --bin
/// <package>-trace`) — so it always reflects the CURRENT on-disk code, not
/// whatever was linked into the long-lived `algolings watch` process when
/// it started. Each exercise package declares its own `<package>-trace`
/// binary target (by convention) rather than all sharing a `trace` name,
/// since cargo warns that colliding output filenames across packages may
/// become a hard error.
/// Builds the trailing `cargo run -- <these args>` arguments for the trace
/// binary: exercise name, fixture JSON, and (only for search exercises) the
/// target value. Sort exercises pass `target: None` and keep the exact
/// 2-arg shape every existing trace binary already expects.
fn build_trace_args(exercise_name: &str, fixture_json: &str, target: Option<i32>) -> Vec<String> {
    let mut args = vec![exercise_name.to_string(), fixture_json.to_string()];
    if let Some(t) = target {
        args.push(t.to_string());
    }
    args
}

pub fn run_trace(
    workspace_root: &std::path::Path,
    package: &str,
    exercise_name: &str,
    fixture: &[i32],
    target: Option<i32>,
    timeout: Duration,
) -> Result<Vec<algolings_trace::Event>, TraceError> {
    let fixture_json =
        serde_json::to_string(fixture).map_err(|e| TraceError::Io(e.to_string()))?;
    let bin_name = format!("{package}-trace");
    let trace_args = build_trace_args(exercise_name, &fixture_json, target);

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "-q", "-p", package, "--bin", &bin_name, "--"])
        .args(&trace_args)
        .current_dir(workspace_root);

    let output = run_with_timeout(cmd, timeout).map_err(|e| match e {
        TimeoutOrIo::TimedOut => TraceError::TimedOut,
        TimeoutOrIo::Io(msg) => TraceError::Io(msg),
    })?;

    if output.status.success() {
        serde_json::from_slice(&output.stdout).map_err(|e| TraceError::Io(e.to_string()))
    } else {
        Err(TraceError::Panicked(
            String::from_utf8_lossy(&output.stderr).into_owned(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use std::time::{Duration, Instant};

    #[test]
    fn trace_args_omit_the_target_when_none() {
        assert_eq!(
            build_trace_args("bubble", "[5,1,4]", None),
            vec!["bubble", "[5,1,4]"]
        );
    }

    #[test]
    fn trace_args_append_the_target_when_some() {
        assert_eq!(
            build_trace_args("linear", "[3,7,2,9,5]", Some(9)),
            vec!["linear", "[3,7,2,9,5]", "9"]
        );
    }

    #[test]
    fn a_quick_command_completes_before_the_timeout() {
        let cmd = Command::new("true");
        let output = run_with_timeout(cmd, Duration::from_secs(2))
            .expect("`true` should run and exit within the timeout");
        assert!(output.status.success());
    }

    #[test]
    fn a_hanging_command_is_killed_at_the_timeout_not_after() {
        let mut cmd = Command::new("sleep");
        cmd.arg("5");
        let start = Instant::now();
        let result = run_with_timeout(cmd, Duration::from_millis(200));
        let elapsed = start.elapsed();

        assert_eq!(result, Err(TimeoutOrIo::TimedOut));
        assert!(
            elapsed < Duration::from_secs(2),
            "expected a prompt kill near the 200ms timeout, took {elapsed:?}"
        );
    }

    #[test]
    fn a_failing_command_still_returns_its_output_not_an_error() {
        let cmd = Command::new("false");
        let output = run_with_timeout(cmd, Duration::from_secs(2)).unwrap();
        assert!(!output.status.success());
    }

    fn workspace_root() -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf()
    }

    #[test]
    fn unsolved_exercise_reports_a_panic_not_a_trace() {
        let result = run_trace(
            &workspace_root(),
            "exercises-sort",
            "bubble",
            &[5, 1, 4, 2, 8],
            None,
            Duration::from_secs(30),
        );
        match result {
            Err(TraceError::Panicked(msg)) => {
                assert!(msg.contains("not yet implemented"));
            }
            other => panic!("expected Panicked, got {other:?}"),
        }
    }

    #[test]
    fn correct_solution_returns_its_real_trace() {
        let result = run_trace(
            &workspace_root(),
            "sort-solutions",
            "bubble",
            &[5, 1, 4, 2, 8],
            None,
            Duration::from_secs(30),
        );
        let events = result.expect("the reference solution should trace successfully");
        assert!(!events.is_empty());
        assert!(events
            .iter()
            .any(|e| matches!(e, algolings_trace::Event::Compare { .. })));
        assert!(events
            .iter()
            .any(|e| matches!(e, algolings_trace::Event::Swap { .. })));
    }
}
