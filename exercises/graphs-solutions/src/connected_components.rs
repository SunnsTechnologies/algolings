use crate::graph::Graph;
use algolings_trace::{disable, enable, mark_inserted, take_events, Event};
use std::collections::HashSet;

/// Reference solution for the `connected_components` exercise.
pub fn connected_components(graph: &Graph) -> Vec<Vec<i32>> {
    let mut visited = HashSet::new();
    let mut components = Vec::new();
    let mut position = 0usize;

    for node in graph.nodes() {
        if !visited.contains(&node) {
            let mut component = Vec::new();
            dfs(graph, node, &mut visited, &mut component, &mut position);
            components.push(component);
        }
    }

    components
}

fn dfs(
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
            dfs(graph, neighbor, visited, component, position);
        }
    }
}

#[cfg(test)]
include!("../../tests-shared/connected_components_tests.rs");
