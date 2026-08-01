use algolings_trace::{found, mark_inserted, mark_removed};

/// Reference solution for the `combinations` exercise.
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
    if current.len() == k {
        result.push(current.clone());
        found(current.len().saturating_sub(1));
        return;
    }

    for i in start..nums.len() {
        mark_inserted(current.len(), nums[i]);
        current.push(nums[i]);
        backtrack(i + 1, nums, k, current, result);
        mark_removed(current.len() - 1);
        current.pop();
    }
}

#[cfg(test)]
include!("../../tests-shared/combinations_tests.rs");
