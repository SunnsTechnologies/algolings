use algolings_trace::{disable, enable, mark_set, take_events, Event};

/// Reference solution for the `grid_paths` exercise.
pub fn unique_paths(rows: usize, cols: usize) -> i32 {
    let mut dp = vec![vec![1i32; cols]; rows];

    for j in 0..cols {
        mark_set(j, dp[0][j]);
    }
    for i in 1..rows {
        mark_set(i * cols, dp[i][0]);
    }

    for i in 1..rows {
        for j in 1..cols {
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            mark_set(i * cols + j, dp[i][j]);
        }
    }

    dp[rows - 1][cols - 1]
}

#[cfg(test)]
include!("../../tests-shared/grid_paths_tests.rs");
