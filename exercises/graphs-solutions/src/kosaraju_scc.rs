use crate::graph::{reverse_graph, Graph};
use algolings_trace::{disable, enable, mark_inserted, take_events, Event};
use std::collections::HashSet;

/// Reference solution for the `strongly_connected_components` exercise.
pub fn strongly_connected_components(graph: &Graph) -> Vec<Vec<i32>> {
    let mut nodes: Vec<i32> = graph.nodes().collect();
    nodes.sort_unstable();

    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    for &node in &nodes {
        if !visited.contains(&node) {
            fill_order(graph, node, &mut visited, &mut stack);
        }
    }

    let reversed = reverse_graph(graph);
    visited.clear();
    let mut sccs = Vec::new();
    let mut position = 0usize;

    while let Some(node) = stack.pop() {
        if !visited.contains(&node) {
            let mut component = Vec::new();
            dfs_reversed(&reversed, node, &mut visited, &mut component, &mut position);
            sccs.push(component);
        }
    }

    sccs
}

fn fill_order(graph: &Graph, node: i32, visited: &mut HashSet<i32>, stack: &mut Vec<i32>) {
    visited.insert(node);
    for &neighbor in graph.neighbors(node) {
        if !visited.contains(&neighbor) {
            fill_order(graph, neighbor, visited, stack);
        }
    }
    stack.push(node);
}

fn dfs_reversed(
    graph: &Graph,
    node: i32,
    visited: &mut HashSet<i32>,
    component: &mut Vec<i32>,
    position: &mut usize,
) {
    visited.insert(node);
    mark_inserted(*position, node);
    *position += 1;
    component.push(node);

    for &neighbor in graph.neighbors(node) {
        if !visited.contains(&neighbor) {
            dfs_reversed(graph, neighbor, visited, component, position);
        }
    }
}

#[cfg(test)]
include!("../../tests-shared/kosaraju_scc_tests.rs");
