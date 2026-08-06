use algolings_trace::{disable, enable, mark_set, take_events, Event};

/// Reference solution for the `climbing_stairs` exercise.
///
/// Computed internally as `usize` (Fibonacci-fast growth), cast to `i32`
/// for `mark_set` — valid for `n < 46`; `dp[46]` already exceeds
/// `i32::MAX` and the cast would silently wrap rather than panic. No
/// fixture or test in this project asks for anything close to that.
pub fn climb_stairs(n: usize) -> i32 {
    if n <= 1 {
        return 1;
    }

    let mut dp = vec![0usize; n + 1];
    dp[0] = 1;
    dp[1] = 1;
    mark_set(0, dp[0] as i32);
    mark_set(1, dp[1] as i32);

    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
        mark_set(i, dp[i] as i32);
    }

    dp[n] as i32
}

#[cfg(test)]
include!("../../tests-shared/climbing_stairs_tests.rs");
