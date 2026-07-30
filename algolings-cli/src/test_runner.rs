//! Runs `cargo test` for a given package/filter and captures pass/fail —
//! drives the watch loop's State 2 (failure) / State 3 (trace replay)
//! transition.

use crate::trace_runner::{run_with_timeout, TimeoutOrIo};
use std::path::Path;
use std::process::Command;
use std::time::Duration;

pub struct TestOutcome {
    pub passed: bool,
    pub output: String,
}

/// Runs `cargo test -p <package> <filter>` from `workspace_root`, capturing
/// combined stdout+stderr. Returns `Ok` even when the tests fail — `passed`
/// carries that outcome; `Err` is reserved for the `cargo` process itself
/// failing to start.
///
/// `timeout` guards against a genuine hang, not just a slow compile — rare
/// (the exercise code being tested is normal `cargo test` 99% of the time),
/// but not impossible: e.g. a linked-list exercise where a learner's
/// mistake creates a real reference cycle (`Rc`, unlike `Box`, doesn't
/// prevent this), and a shared test walks the cycle forever. A timeout is
/// reported as `passed: false` with an explanatory message, rather than
/// blocking the watch loop forever.
pub fn run_package_tests(
    workspace_root: &Path,
    package: &str,
    filter: &str,
    timeout: Duration,
) -> std::io::Result<TestOutcome> {
    let mut cmd = Command::new("cargo");
    cmd.args(["test", "-p", package, "--lib", filter])
        .current_dir(workspace_root);
    // run_with_timeout nulls stdin itself — this subprocess never reads
    // input, and an inherited stdin could contend with the CLI's hint
    // listener (a background thread reading real stdin) for keystrokes.

    let output = match run_with_timeout(cmd, timeout) {
        Ok(output) => output,
        Err(TimeoutOrIo::TimedOut) => {
            return Ok(TestOutcome {
                passed: false,
                output: format!("tests timed out after {timeout:?} (possible infinite loop)"),
            });
        }
        Err(TimeoutOrIo::Io(msg)) => {
            return Err(std::io::Error::other(msg));
        }
    };

    let mut combined = String::from_utf8_lossy(&output.stdout).into_owned();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));

    // A filter matching zero tests makes `cargo test` exit successfully
    // (vacuously — nothing failed because nothing ran). That is NEVER a
    // legitimate pass for an exercise: it means the filter/module name is
    // wrong, not that the learner solved anything. Guard against it
    // explicitly rather than trusting the exit code alone.
    let matched_zero_tests = combined.contains("running 0 tests");
    let passed = output.status.success() && !matched_zero_tests;

    Ok(TestOutcome {
        passed,
        output: combined,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    const A_GENEROUS_TIMEOUT: Duration = Duration::from_secs(30);

    fn workspace_root() -> PathBuf {
        // algolings-cli's manifest dir is <workspace>/algolings-cli; the
        // workspace root is one level up.
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf()
    }

    #[test]
    fn unsolved_skeleton_reports_failure() {
        let outcome =
            run_package_tests(&workspace_root(), "exercises-sort", "bubble", A_GENEROUS_TIMEOUT)
                .unwrap();
        assert!(!outcome.passed);
        assert!(outcome.output.contains("bubble"));
    }

    #[test]
    fn correct_reference_solution_reports_success() {
        let outcome = run_package_tests(
            &workspace_root(),
            "sort-solutions",
            "bubble",
            A_GENEROUS_TIMEOUT,
        )
        .unwrap();
        assert!(outcome.passed);
    }

    #[test]
    fn a_filter_matching_zero_tests_is_not_reported_as_a_pass() {
        // Regression test: found via manual end-to-end testing of the real
        // CLI. `exercise.name` ("bubble_sort") was being used as the test
        // filter, but the test module is named `bubble` — the filter
        // matched zero tests, and cargo reports that as a trivial success,
        // so a completely unsolved exercise silently showed PASSED.
        let outcome = run_package_tests(
            &workspace_root(),
            "exercises-sort",
            "bubble_sort",
            A_GENEROUS_TIMEOUT,
        )
        .unwrap();
        assert!(
            !outcome.passed,
            "a filter matching zero tests must never be treated as a pass"
        );
    }

    #[test]
    fn exercise_registry_module_name_actually_matches_its_test_filter() {
        // Regression test: this is the check that should have caught the
        // bug above — that an exercise's test_filter field is something the
        // shared test suite's module path actually contains. Checked per
        // module (not just sort) so a search exercise with a typo'd filter
        // wouldn't slip through undetected.
        for module in crate::exercise::MODULES {
            let exercise = &module.exercises[0];
            let outcome = run_package_tests(
                &workspace_root(),
                module.solutions_package,
                exercise.test_filter,
                A_GENEROUS_TIMEOUT,
            )
            .unwrap();
            assert!(
                outcome.passed,
                "{:?}'s test_filter ({:?}) matched zero tests against {:?}",
                exercise.name, exercise.test_filter, module.solutions_package
            );
        }
    }

    /// A standalone throwaway crate whose one test hangs forever —
    /// modeling a real reference-cycle bug in a learner's exercise code,
    /// which `Rc` (unlike `Box`) makes possible.
    fn build_hanging_crate() -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("Cargo.toml"),
            "[package]\nname = \"hangpkg\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )
        .unwrap();
        std::fs::create_dir(dir.path().join("src")).unwrap();
        std::fs::write(
            dir.path().join("src/lib.rs"),
            "#[cfg(test)]\nmod tests {\n    #[test]\n    fn hangs_forever() {\n        \
             loop {}\n    }\n}\n",
        )
        .unwrap();
        dir
    }

    #[test]
    fn a_genuinely_hanging_test_suite_is_reported_as_failed_not_left_hanging() {
        let dir = build_hanging_crate();
        let outcome = run_package_tests(
            dir.path(),
            "hangpkg",
            "hangs_forever",
            Duration::from_millis(300),
        )
        .unwrap();
        assert!(!outcome.passed);
        assert!(
            outcome.output.contains("timed out"),
            "expected a timeout message, got: {}",
            outcome.output
        );
    }
}
