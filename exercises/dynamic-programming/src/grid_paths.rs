use algolings_trace::{disable, enable, mark_set, take_events, Event};

// Unique paths through a grid: starting at the top-left corner, moving
// only right or down, how many distinct paths reach the bottom-right
// corner? A genuinely 2D DP table this time — dp[i][j] = dp[i-1][j] +
// dp[i][j-1] (the number of ways to reach a cell is the sum of the ways
// to reach the cell above it and the cell to its left).
//
// Trace flattened row-major: mark_set(row * cols + col, value) — this
// project's trace events only understand a flat array, not a 2D table,
// so index math stands in for the grid shape.
//
// The first row and first column are both base cases (exactly one way
// to reach any of them: keep moving in the same direction). Trace row 0
// IN FULL first (that includes the (0, 0) corner), THEN trace column 0
// starting from row 1 — starting column 0's loop at row 0 instead would
// mark_set the corner cell TWICE.
pub fn unique_paths(rows: usize, cols: usize) -> i32 {
    todo!("dp[row][col] = dp[row-1][col] + dp[row][col-1], base case 1 along the first row and first column — mark_set(row * cols + col, value) for every cell, row 0 in full then column 0 starting at row 1")
}

#[cfg(test)]
include!("../../tests-shared/grid_paths_tests.rs");
