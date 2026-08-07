use crate::edge::Edge;
use algolings_trace::{disable, enable, mark_set, take_events, Event};
use std::collections::HashMap;

// Bellman-Ford: single-source shortest paths that also work with
// NEGATIVE edge weights (which break Dijkstra's greedy assumption). The
// trick is brute force with a proof behind it: relax every edge, V-1
// times over — any shortest path in a graph with no negative cycle uses
// at most V-1 edges, so V-1 full passes are guaranteed to have
// propagated the true shortest distance everywhere. One more pass that
// STILL finds an improvement means a negative-weight cycle exists (the
// "shortest path" would keep shrinking forever), so that's an error.
//
// `vertices` must include every edge's `from`/`to` — an edge endpoint
// missing from `vertices` panics on a missing key, same failure mode as
// dijkstra's start node needing to already be in its graph.
//
// Same stable-position-map tracing idiom as dijkstra: assign every
// vertex a position ONCE (sorted, not visit order), then mark_set on
// every improvement. If this ends in Err (a negative cycle), the last
// Set at each position does NOT represent a real final answer — the
// concept note explains why the numbers kept moving.
pub fn bellman_ford(
    vertices: &[i32],
    edges: &[Edge],
    source: i32,
) -> Result<HashMap<i32, i32>, &'static str> {
    todo!("relax every edge V-1 times (distance[to] = distance[from] + weight, if smaller), mark_set(position_of[to], new_dist) on every improvement, then one more pass to detect a negative cycle")
}

#[cfg(test)]
include!("../../tests-shared/bellman_ford_tests.rs");
