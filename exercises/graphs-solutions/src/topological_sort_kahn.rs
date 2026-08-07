use crate::graph::Graph;
use algolings_trace::{disable, enable, mark_inserted, take_events, Event};
use std::collections::{HashMap, VecDeque};

/// Reference solution for the `topological_sort_kahn` exercise.
pub fn topological_sort_kahn(graph: &Graph) -> Option<Vec<i32>> {
    let mut nodes: Vec<i32> = graph.nodes().collect();
    nodes.sort_unstable();

    let mut in_degree: HashMap<i32, i32> = HashMap::new();
    for &node in &nodes {
        in_degree.entry(node).or_insert(0);
        for &neighbor in graph.neighbors(node) {
            *in_degree.entry(neighbor).or_insert(0) += 1;
        }
    }

    let mut queue: VecDeque<i32> = nodes
        .iter()
        .copied()
        .filter(|node| in_degree[node] == 0)
        .collect();

    let mut order = Vec::new();
    while let Some(node) = queue.pop_front() {
        mark_inserted(order.len(), node);
        order.push(node);

        for &neighbor in graph.neighbors(node) {
            let degree = in_degree.get_mut(&neighbor).unwrap();
            *degree -= 1;
            if *degree == 0 {
                queue.push_back(neighbor);
            }
        }
    }

    if order.len() == nodes.len() {
        Some(order)
    } else {
        None
    }
}

#[cfg(test)]
include!("../../tests-shared/topological_sort_kahn_tests.rs");
