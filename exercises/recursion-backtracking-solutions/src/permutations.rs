use algolings_trace::{found, mark_inserted, mark_removed};

/// Reference solution for the `permutations` exercise.
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
    if current.len() == nums.len() {
        result.push(current.clone());
        found(current.len().saturating_sub(1));
        return;
    }

    for i in 0..nums.len() {
        if used[i] {
            continue;
        }

        used[i] = true;
        mark_inserted(current.len(), nums[i]);
        current.push(nums[i]);

        backtrack(nums, used, current, result);

        mark_removed(current.len() - 1);
        current.pop();
        used[i] = false;
    }
}

#[cfg(test)]
include!("../../tests-shared/permutations_tests.rs");
