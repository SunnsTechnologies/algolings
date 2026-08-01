use algolings_trace::{mark_inserted, mark_removed};

// Rewrite factorial with an accumulator: instead of computing `n *
// factorial(n - 1)` AFTER the recursive call returns, `acc` carries the
// running product FORWARD, so the recursive call is the very last thing
// each frame does — no work left to do once it returns.
//
// Trace it the same way as recursion_basics: mark_inserted(depth, n as
// i64) on entry, mark_removed(depth) right before returning.
// factorial_tail_helper carries the depth; factorial_tail just starts it
// at 0.
pub fn factorial_tail(n: u64, acc: u64) -> u64 {
    factorial_tail_helper(n, acc, 0)
}

fn factorial_tail_helper(n: u64, acc: u64, depth: usize) -> u64 {
    todo!("mark_inserted(depth, n as i64), recurse with the accumulator, mark_removed(depth) before returning")
}

#[cfg(test)]
include!("../../tests-shared/tail_recursion_tests.rs");
