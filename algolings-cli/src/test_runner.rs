//! Runs `cargo test` for a given package/filter and captures pass/fail —
//! drives the watch loop's State 2 (failure) / State 3 (trace replay)
//! transition.

use std::path::Path;
use std::process::{Command, Stdio};

pub struct TestOutcome {
    pub passed: bool,
    pub output: String,
}

/// Runs `cargo test -p <package> <filter>` from `workspace_root`, capturing
/// combined stdout+stderr. Returns `Ok` even when the tests fail — `passed`
/// carries that outcome; `Err` is reserved for the `cargo` process itself
/// failing to start.
pub fn run_package_tests(
    workspace_root: &Path,
    package: &str,
    filter: &str,
) -> std::io::Result<TestOutcome> {
    let output = Command::new("cargo")
        .args(["test", "-p", package, "--lib", filter])
        .current_dir(workspace_root)
        // This subprocess never reads input. Explicitly null stdin rather
        // than rely on Command::output()'s default — the CLI's hint
        // listener reads real stdin on a background thread, and an
        // inherited stdin here could contend with it for keystrokes.
        .stdin(Stdio::null())
        .output()?;

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
        let outcome = run_package_tests(&workspace_root(), "exercises-sort", "bubble").unwrap();
        assert!(!outcome.passed);
        assert!(outcome.output.contains("bubble"));
    }

    #[test]
    fn correct_reference_solution_reports_success() {
        let outcome = run_package_tests(&workspace_root(), "sort-solutions", "bubble").unwrap();
        assert!(outcome.passed);
    }

    #[test]
    fn a_filter_matching_zero_tests_is_not_reported_as_a_pass() {
        // Regression test: found via manual end-to-end testing of the real
        // CLI. `exercise.name` ("bubble_sort") was being used as the test
        // filter, but the test module is named `bubble` — the filter
        // matched zero tests, and cargo reports that as a trivial success,
        // so a completely unsolved exercise silently showed PASSED.
        let outcome =
            run_package_tests(&workspace_root(), "exercises-sort", "bubble_sort").unwrap();
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
            )
            .unwrap();
            assert!(
                outcome.passed,
                "{:?}'s test_filter ({:?}) matched zero tests against {:?}",
                exercise.name, exercise.test_filter, module.solutions_package
            );
        }
    }
}
