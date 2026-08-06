use algolings_trace::{disable, enable, mark_set, take_events, Event};

/// Reference solution for the `house_robber` exercise.
pub fn rob(nums: &[i32]) -> i32 {
    let n = nums.len();
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return nums[0];
    }

    let mut dp = vec![0; n];
    dp[0] = nums[0];
    dp[1] = nums[0].max(nums[1]);
    mark_set(0, dp[0]);
    mark_set(1, dp[1]);

    for i in 2..n {
        dp[i] = dp[i - 1].max(dp[i - 2] + nums[i]);
        mark_set(i, dp[i]);
    }

    dp[n - 1]
}

#[cfg(test)]
include!("../../tests-shared/house_robber_tests.rs");
