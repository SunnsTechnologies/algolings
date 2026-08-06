use crate::graph::Graph;
use algolings_trace::{disable, enable, mark_inserted, take_events, Event};
use std::collections::{HashSet, VecDeque};

// Implement breadth-first search: visit `start`, then every unvisited
// neighbor one "ring" out at a time, using a queue rather than recursion.
//
// Trace it the same way tree_traversals appends to its result: call
// mark_inserted(order.len(), node) right where you push into `order` — the
// position IS the visit order, nothing else to track.
pub fn bfs(graph: &Graph, start: i32) -> Vec<i32> {
    todo!("BFS: a HashSet for visited, a VecDeque as the queue, mark_inserted(order.len(), node) on every visit")
}

#[cfg(test)]
include!("../../tests-shared/bfs_tests.rs");
