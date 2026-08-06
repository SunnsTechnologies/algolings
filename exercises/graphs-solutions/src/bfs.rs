use crate::graph::Graph;
use algolings_trace::{disable, enable, mark_inserted, take_events, Event};
use std::collections::{HashSet, VecDeque};

/// Reference solution for the `bfs` exercise.
pub fn bfs(graph: &Graph, start: i32) -> Vec<i32> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut order = Vec::new();

    visited.insert(start);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        mark_inserted(order.len(), node);
        order.push(node);

        for &neighbor in graph.neighbors(node) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }

    order
}

#[cfg(test)]
include!("../../tests-shared/bfs_tests.rs");
