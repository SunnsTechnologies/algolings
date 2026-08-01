use algolings_trace::{found, mark_inserted, mark_removed};

// Generate every combination of size `k` from `nums`, via the same
// push/recurse/pop backtracking idiom as subsets — but this time, only
// capture `current` into `result` once it reaches exactly length `k`
// (not at every prefix).
//
// Call found(...) right when you capture a combination, mark_inserted(i,
// value) before pushing (i = current.len() beforehand), and
// mark_removed(i) before popping (i = current.len() - 1).
pub fn combinations(nums: &[i32], k: usize) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    backtrack(0, nums, k, &mut current, &mut result);
    result
}

fn backtrack(
    start: usize,
    nums: &[i32],
    k: usize,
    current: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    todo!("if current.len() == k, capture it (tracing with found) and return; otherwise try each remaining candidate")
}

#[cfg(test)]
include!("../../tests-shared/combinations_tests.rs");
