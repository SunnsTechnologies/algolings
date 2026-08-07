use crate::graph::Graph;
use algolings_trace::{disable, enable, mark_inserted, take_events, Event};
use std::collections::HashSet;

/// Reference solution for the `topological_sort_dfs` exercise.
pub fn topological_sort_dfs(graph: &Graph) -> Option<Vec<i32>> {
    let mut nodes: Vec<i32> = graph.nodes().collect();
    nodes.sort_unstable();

    let mut visited = HashSet::new();
    let mut visiting = HashSet::new();
    let mut stack = Vec::new();

    for &node in &nodes {
        if !visited.contains(&node)
            && !dfs(graph, node, &mut visited, &mut visiting, &mut stack)
        {
            return None;
        }
    }

    stack.reverse();
    let mut order = Vec::new();
    for &node in &stack {
        mark_inserted(order.len(), node);
        order.push(node);
    }
    Some(order)
}

fn dfs(
    graph: &Graph,
    node: i32,
    visited: &mut HashSet<i32>,
    visiting: &mut HashSet<i32>,
    stack: &mut Vec<i32>,
) -> bool {
    if visiting.contains(&node) {
        return false;
    }
    if visited.contains(&node) {
        return true;
    }

    visiting.insert(node);
    for &neighbor in graph.neighbors(node) {
        if !dfs(graph, neighbor, visited, visiting, stack) {
            return false;
        }
    }
    visiting.remove(&node);
    visited.insert(node);
    stack.push(node);
    true
}

#[cfg(test)]
include!("../../tests-shared/topological_sort_recursive_tests.rs");
