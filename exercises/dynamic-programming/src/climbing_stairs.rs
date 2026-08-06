use algolings_trace::{disable, enable, mark_set, take_events, Event};

// Climbing stairs: taking 1 or 2 steps at a time, how many distinct ways
// are there to reach the top of an n-step staircase? Same recurrence as
// Fibonacci (dp[i] = dp[i-1] + dp[i-2]) — the DP idiom this exercise
// teaches is bottom-up tabulation: build the answer to every smaller
// subproblem first, in order, so each one only ever gets computed once.
//
// dp[i] is computed internally as usize (it grows Fibonacci-fast), then
// cast to i32 for mark_set — algolings-trace's tracing primitives require
// Into<i64>, which usize doesn't implement but i32 does. This does cap
// this exercise's valid range at n < 46: dp[46] already exceeds i32::MAX,
// and the cast would silently wrap instead of panicking. No fixture,
// hint, or test in this project asks for anything close to that — it's
// documented here rather than designed around, the same way this
// project accepts (but doesn't specifically test for) i32 overflow
// elsewhere.
//
// Trace with mark_set(i, dp[i] as i32) — same idiom counting_sort and
// radix_sort already use for a value landing in a fixed-size output
// array. Call it for the base cases (dp[0], dp[1]) too, not just inside
// the loop.
pub fn climb_stairs(n: usize) -> i32 {
    todo!("dp[i] = dp[i-1] + dp[i-2], starting from dp[0] = dp[1] = 1 — mark_set every position you fill, including the two base cases")
}

#[cfg(test)]
include!("../../tests-shared/climbing_stairs_tests.rs");
