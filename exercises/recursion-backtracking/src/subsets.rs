use algolings_trace::{found, mark_inserted, mark_removed};

// Generate every subset of `nums` via backtracking: explore by pushing a
// candidate onto `current`, recurse, then pop it back off (undo) before
// trying the next candidate. Every state along the way — including the
// empty one — is a valid subset, so capture `current` into `result` at
// the START of every call, not just at some base case.
//
// Call found(...) right when you capture a subset, mark_inserted(i,
// value) before pushing (i = current.len() beforehand), and
// mark_removed(i) before popping (i = current.len() - 1).
pub fn subsets(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    backtrack(0, nums, &mut current, &mut result);
    result
}

fn backtrack(index: usize, nums: &[i32], current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    todo!("capture current into result (tracing with found), then try each remaining candidate")
}

#[cfg(test)]
include!("../../tests-shared/subsets_tests.rs");
