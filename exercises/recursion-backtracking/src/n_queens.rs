use algolings_trace::{found, mark_inserted, mark_removed};

// Place N queens on an N×N board so none attacks another — no two in the
// same row, column, or diagonal. Since queens go one per row, in row
// order, `positions: Vec<usize>` (the column chosen for each row so far)
// fully captures the search state — a simplification of the tutorial's
// literal 2D board that fits the same push/recurse/pop idiom every other
// exercise in this module uses.
//
// `cols`/`diag1`/`diag2` track which columns and diagonals are already
// occupied — `diag1` uses `row + n - col` and `diag2` uses `row + col` to
// turn "same diagonal" into "same array index" without going negative.
//
// For each row (= positions.len()), try every column that isn't
// conflicting. Call mark_inserted(row, col as i64) and push ONLY inside
// the "didn't conflict" branch — a row where every column conflicts
// should never call mark_inserted or mark_removed at all, since nothing
// was ever placed there. Call found(...) once every row has a queen
// (positions.len() == n). Note: mark_inserted needs a value that
// implements Into<i64> — col is usize, so it needs an explicit `as i64`.
pub fn solve_n_queens(n: usize) -> Vec<Vec<String>> {
    let mut positions: Vec<usize> = Vec::new();
    let mut cols = vec![false; n];
    let mut diag1 = vec![false; 2 * n];
    let mut diag2 = vec![false; 2 * n];
    let mut results = Vec::new();
    backtrack(n, &mut positions, &mut cols, &mut diag1, &mut diag2, &mut results);
    results
}

fn backtrack(
    n: usize,
    positions: &mut Vec<usize>,
    cols: &mut [bool],
    diag1: &mut [bool],
    diag2: &mut [bool],
    results: &mut Vec<Vec<String>>,
) {
    todo!("if positions.len() == n, build the board and capture it (tracing with found); otherwise try each non-conflicting column")
}

#[cfg(test)]
include!("../../tests-shared/n_queens_tests.rs");
