use algolings_trace::{mark_inserted, mark_removed};

/// Reference solution for the `tail_recursion` exercise.
pub fn factorial_tail(n: u64, acc: u64) -> u64 {
    factorial_tail_helper(n, acc, 0)
}

fn factorial_tail_helper(n: u64, acc: u64, depth: usize) -> u64 {
    mark_inserted(depth, n as i64);
    let result = if n == 0 {
        acc
    } else {
        factorial_tail_helper(n - 1, n * acc, depth + 1)
    };
    mark_removed(depth);
    result
}

#[cfg(test)]
include!("../../tests-shared/tail_recursion_tests.rs");
