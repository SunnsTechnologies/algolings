use algolings_trace::{disable, enable, mark_set, take_events, Event};

// House robber: adjacent houses can't both be robbed (a security system
// links them) — maximize total value across the whole street. Genuinely
// different recurrence from climbing_stairs' pure sum: at each house you
// DECIDE, dp[i] = max(skip this house, rob it and add to what you had
// two houses back).
//
// Same mark_set(i, dp[i]) tracing idiom as climbing_stairs — call it for
// the base cases (dp[0], dp[1]) too, not just inside the loop.
pub fn rob(nums: &[i32]) -> i32 {
    todo!("dp[i] = dp[i-1].max(dp[i-2] + nums[i]) — mark_set every position you fill, including the two base cases")
}

#[cfg(test)]
include!("../../tests-shared/house_robber_tests.rs");
