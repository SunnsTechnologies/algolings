use algolings_trace::{mark_inserted, mark_removed};

// Implement factorial and fibonacci recursively. There's no array here —
// the thing worth tracing is the CALL STACK itself, growing one frame per
// recursive call and shrinking as each call returns.
//
// Call mark_inserted(depth, n as i64) the moment a call begins (before
// recursing further), and mark_removed(depth) right before it returns.
// This never collides: a call at depth d always fully finishes —
// including everything it recurses into — before anything else at depth d
// begins, so whatever was there is always gone before something new
// lands.
//
// factorial_helper/fibonacci_helper carry the depth down through the
// recursion; factorial/fibonacci just start it at 0.
pub fn factorial(n: u64) -> u64 {
    factorial_helper(n, 0)
}

fn factorial_helper(n: u64, depth: usize) -> u64 {
    todo!("mark_inserted(depth, n as i64), compute recursively, mark_removed(depth) before returning")
}

pub fn fibonacci(n: u64) -> u64 {
    fibonacci_helper(n, 0)
}

fn fibonacci_helper(n: u64, depth: usize) -> u64 {
    todo!("same call-stack tracing pattern as factorial_helper")
}

#[cfg(test)]
include!("../../tests-shared/recursion_basics_tests.rs");
