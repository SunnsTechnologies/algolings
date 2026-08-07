use algolings_trace::{disable, enable, mark_set, take_events, Event};

pub const INF: i32 = i32::MAX / 2;

/// Reference solution for the `floyd_warshall` exercise.
pub fn floyd_warshall(mut dist: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = dist.len();

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let through_k = dist[i][k] + dist[k][j];
                if through_k < dist[i][j] {
                    dist[i][j] = through_k;
                    mark_set(i * n + j, dist[i][j]);
                }
            }
        }
    }

    dist
}

#[cfg(test)]
include!("../../tests-shared/floyd_warshall_tests.rs");
