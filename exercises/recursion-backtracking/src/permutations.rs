use algolings_trace::{found, mark_inserted, mark_removed};

// Generate every permutation of `nums` via backtracking. Unlike subsets/
// combinations (which only ever move FORWARD through the input),
// permutations needs every element available at every position — so
// track which indices are already in `current` with a `used: Vec<bool>`,
// and try every UNUSED index at each step, not just the ones after where
// you started.
//
// Call found(...) once current reaches full length, mark_inserted(i,
// value) before pushing (i = current.len() beforehand), and
// mark_removed(i) before popping (i = current.len() - 1) — remember to
// also flip `used[idx]` back to false on the way out.
pub fn permutations(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    let mut used = vec![false; nums.len()];
    backtrack(nums, &mut used, &mut current, &mut result);
    result
}

fn backtrack(
    nums: &[i32],
    used: &mut [bool],
    current: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    todo!("if current.len() == nums.len(), capture it (tracing with found) and return; otherwise try each unused index")
}

#[cfg(test)]
include!("../../tests-shared/permutations_tests.rs");
