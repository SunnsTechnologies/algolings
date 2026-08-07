use crate::weighted_graph::WeightedGraph;
use algolings_trace::{disable, enable, mark_set, take_events, Event};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

/// Reference solution for the `dijkstra` exercise.
pub fn dijkstra(graph: &WeightedGraph, start: i32) -> HashMap<i32, i32> {
    let mut sorted_nodes: Vec<i32> = graph.nodes().collect();
    sorted_nodes.sort_unstable();
    let position_of: HashMap<i32, usize> = sorted_nodes
        .iter()
        .enumerate()
        .map(|(i, &n)| (n, i))
        .collect();

    let mut distances = HashMap::new();
    let mut heap = BinaryHeap::new();
    for &node in &sorted_nodes {
        distances.insert(node, i32::MAX);
    }
    distances.insert(start, 0);
    mark_set(position_of[&start], 0);
    heap.push(Reverse((0, start)));

    while let Some(Reverse((current_dist, node))) = heap.pop() {
        if current_dist > distances[&node] {
            continue;
        }

        for &(neighbor, weight) in graph.neighbors(node) {
            let new_dist = current_dist + weight;
            if new_dist < distances[&neighbor] {
                distances.insert(neighbor, new_dist);
                mark_set(position_of[&neighbor], new_dist);
                heap.push(Reverse((new_dist, neighbor)));
            }
        }
    }

    distances
}

#[cfg(test)]
include!("../../tests-shared/dijkstra_tests.rs");
