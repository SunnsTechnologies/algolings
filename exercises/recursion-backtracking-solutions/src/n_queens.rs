use algolings_trace::{found, mark_inserted, mark_removed};

/// Reference solution for the `n_queens` exercise.
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
    let row = positions.len();

    if row == n {
        let board: Vec<String> = positions
            .iter()
            .map(|&col| (0..n).map(|c| if c == col { 'Q' } else { '.' }).collect())
            .collect();
        results.push(board);
        found(row.saturating_sub(1));
        return;
    }

    for col in 0..n {
        let d1 = row + n - col;
        let d2 = row + col;

        if cols[col] || diag1[d1] || diag2[d2] {
            continue;
        }

        cols[col] = true;
        diag1[d1] = true;
        diag2[d2] = true;

        mark_inserted(positions.len(), col as i64);
        positions.push(col);

        backtrack(n, positions, cols, diag1, diag2, results);

        mark_removed(positions.len() - 1);
        positions.pop();

        cols[col] = false;
        diag1[d1] = false;
        diag2[d2] = false;
    }
}

#[cfg(test)]
include!("../../tests-shared/n_queens_tests.rs");
