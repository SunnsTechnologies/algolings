use crate::weighted_graph::WeightedGraph;
use algolings_trace::{disable, enable, mark_set, take_events, Event};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

// Dijkstra's algorithm: single-source shortest paths on a graph with
// non-negative weights. Track the best known distance to every vertex,
// always expand the closest not-yet-finalized vertex next (a min-heap),
// and RELAX every edge out of it — if going through the vertex you just
// popped gives a shorter path to a neighbor than what you had before,
// update it.
//
// Trace position is a STABLE map, assigned ONCE up front from the
// graph's sorted node list (not visit order — a node's distance can
// improve more than once over the call). Call mark_set(position, dist)
// every time a distance actually improves, including the very first
// time it's set for the start node itself (distance 0).
pub fn dijkstra(graph: &WeightedGraph, start: i32) -> HashMap<i32, i32> {
    todo!("track distances in a HashMap, use a BinaryHeap<Reverse<(dist, node)>> as the frontier, relax every outgoing edge on pop — mark_set(position_of[node], new_dist) on every improvement")
}

#[cfg(test)]
include!("../../tests-shared/dijkstra_tests.rs");
