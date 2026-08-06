use crate::graph::Graph;
use algolings_trace::{disable, enable, mark_inserted, take_events, Event};
use std::collections::HashSet;

// Implement depth-first search, recursively: visit a node, then fully
// explore one unvisited neighbor (and everything reachable from it)
// before moving on to the next neighbor.
//
// Same append-tracing idiom as bfs: mark_inserted(order.len(), node)
// right where you push into `order`.
pub fn dfs(graph: &Graph, start: i32) -> Vec<i32> {
    todo!("Recursive DFS: a private helper carrying visited + order, mark_inserted(order.len(), node) on every visit")
}

#[cfg(test)]
include!("../../tests-shared/dfs_tests.rs");
