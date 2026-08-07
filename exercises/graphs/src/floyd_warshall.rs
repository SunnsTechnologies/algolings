use algolings_trace::{disable, enable, mark_set, take_events, Event};

// Floyd-Warshall: ALL-pairs shortest paths, not just single-source —
// works directly on a dense adjacency matrix instead of an adjacency
// list. The core idea: for every possible "via" vertex k, check whether
// routing through k gives a shorter i->j path than whatever's already
// known. Three nested loops (k outermost, then i, then j) is the whole
// algorithm.
//
// INF stands in for "no direct edge" — deliberately i32::MAX / 2, not
// i32::MAX itself, so that INF + INF (an attempt to route through two
// unreachable legs) still fits in an i32 without overflowing, and
// safely stays larger than any real finite distance.
//
// Trace flattened row-major, same idiom as grid_paths:
// mark_set(row * n + col, value) — but unlike grid_paths, the SAME cell
// can improve more than once as k increases, and each improvement gets
// its own Set call, not just the final value.
pub const INF: i32 = i32::MAX / 2;

pub fn floyd_warshall(mut dist: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    todo!("for k, then i, then j: if dist[i][k] + dist[k][j] improves on dist[i][j], update it and mark_set(i * n + j, dist[i][j])")
}

#[cfg(test)]
include!("../../tests-shared/floyd_warshall_tests.rs");
