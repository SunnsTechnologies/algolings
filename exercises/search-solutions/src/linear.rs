use algolings_trace::{found, probe};

/// Reference solution for the `linear_search` exercise. CI runs this
/// crate's tests to prove a correct answer exists for the shared test
/// suite in `exercises/tests-shared/linear_search_tests.rs` — the same
/// suite the learner's skeleton in `exercises/search/src/linear.rs` must
/// pass.
pub fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    for i in 0..arr.len() {
        if probe(arr, i, &target) {
            found(i);
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
include!("../../tests-shared/linear_search_tests.rs");
