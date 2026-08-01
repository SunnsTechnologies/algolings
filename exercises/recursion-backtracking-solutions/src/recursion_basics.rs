use algolings_trace::{mark_inserted, mark_removed};

/// Reference solution for the `recursion_basics` exercise.
pub fn factorial(n: u64) -> u64 {
    factorial_helper(n, 0)
}

fn factorial_helper(n: u64, depth: usize) -> u64 {
    mark_inserted(depth, n as i64);
    let result = if n == 0 {
        1
    } else {
        n * factorial_helper(n - 1, depth + 1)
    };
    mark_removed(depth);
    result
}

pub fn fibonacci(n: u64) -> u64 {
    fibonacci_helper(n, 0)
}

fn fibonacci_helper(n: u64, depth: usize) -> u64 {
    mark_inserted(depth, n as i64);
    let result = match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_helper(n - 1, depth + 1) + fibonacci_helper(n - 2, depth + 1),
    };
    mark_removed(depth);
    result
}

#[cfg(test)]
include!("../../tests-shared/recursion_basics_tests.rs");
