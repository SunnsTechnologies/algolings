use crate::graph::Graph;
use algolings_trace::{disable, enable, mark_inserted, take_events, Event};
use std::collections::HashSet;

/// Reference solution for the `dfs` exercise.
pub fn dfs(graph: &Graph, start: i32) -> Vec<i32> {
    let mut visited = HashSet::new();
    let mut order = Vec::new();
    dfs_helper(graph, start, &mut visited, &mut order);
    order
}

fn dfs_helper(graph: &Graph, node: i32, visited: &mut HashSet<i32>, order: &mut Vec<i32>) {
    visited.insert(node);
    mark_inserted(order.len(), node);
    order.push(node);

    for &neighbor in graph.neighbors(node) {
        if !visited.contains(&neighbor) {
            dfs_helper(graph, neighbor, visited, order);
        }
    }
}

#[cfg(test)]
include!("../../tests-shared/dfs_tests.rs");
