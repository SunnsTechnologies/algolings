use algolings_trace::{found, mark_inserted, mark_removed};

/// Reference solution for the `subsets` exercise.
pub fn subsets(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    backtrack(0, nums, &mut current, &mut result);
    result
}

fn backtrack(index: usize, nums: &[i32], current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    result.push(current.clone());
    found(current.len().saturating_sub(1));

    for i in index..nums.len() {
        mark_inserted(current.len(), nums[i]);
        current.push(nums[i]);
        backtrack(i + 1, nums, current, result);
        mark_removed(current.len() - 1);
        current.pop();
    }
}

#[cfg(test)]
include!("../../tests-shared/subsets_tests.rs");
