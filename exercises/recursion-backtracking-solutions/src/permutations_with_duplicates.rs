use algolings_trace::{found, mark_inserted, mark_removed};

/// Reference solution for the `permutations_with_duplicates` exercise.
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
    if current.len() == nums.len() {
        result.push(current.clone());
        found(current.len().saturating_sub(1));
        return;
    }

    for i in 0..nums.len() {
        if used[i] {
            continue;
        }

        if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
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
include!("../../tests-shared/permutations_with_duplicates_tests.rs");
