use algolings_trace::{found, mark_inserted, mark_removed};

// Same backtracking shape as permutations, but `nums` may contain
// duplicate values — without a skip check, [1, 1] would produce [1, 1]
// TWICE (once per which "1" gets picked first), which are the same
// permutation. Sorting first groups equal values together, so the fix is:
// among equal values, only ever start with the FIRST unused one at this
// position. If nums[i] == nums[i - 1] and the earlier one ISN'T currently
// used (meaning we're about to pick the later duplicate before the
// earlier one at this branch), skip it — that ordering was already
// covered by a different branch.
//
// Call found(...) once current reaches full length, mark_inserted(i,
// value) before pushing (i = current.len() beforehand), and
// mark_removed(i) before popping (i = current.len() - 1).
pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    let mut result = Vec::new();
    let mut current = Vec::new();
    let mut used = vec![false; nums.len()];
    backtrack(&nums, &mut used, &mut current, &mut result);
    result
}

fn backtrack(
    nums: &[i32],
    used: &mut [bool],
    current: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    todo!("same as permutations, plus: skip nums[i] if it equals nums[i-1] and nums[i-1] isn't used")
}

#[cfg(test)]
include!("../../tests-shared/permutations_with_duplicates_tests.rs");
