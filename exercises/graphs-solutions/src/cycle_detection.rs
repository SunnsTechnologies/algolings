use crate::graph::Graph;
use algolings_trace::{disable, enable, found, mark_inserted, take_events, Event};
use std::collections::{HashMap, HashSet};

/// Reference solution for the `cycle_detection` exercise.
pub fn has_cycle_undirected(graph: &Graph) -> bool {
    let mut visited: HashMap<i32, usize> = HashMap::new();
    let mut position = 0usize;
    for node in graph.nodes() {
        if !visited.contains_key(&node)
            && dfs_undirected(graph, node, None, &mut visited, &mut position)
        {
            return true;
        }
    }
    false
}

fn dfs_undirected(
    graph: &Graph,
    node: i32,
    parent: Option<i32>,
    visited: &mut HashMap<i32, usize>,
    position: &mut usize,
) -> bool {
    let my_pos = *position;
    visited.insert(node, my_pos);
    mark_inserted(my_pos, node);
    *position += 1;

    for &neighbor in graph.neighbors(node) {
        if let Some(&neighbor_pos) = visited.get(&neighbor) {
            if Some(neighbor) != parent {
                found(neighbor_pos);
                return true;
            }
        } else if dfs_undirected(graph, neighbor, Some(node), visited, position) {
            return true;
        }
    }
    false
}

pub fn has_cycle_directed(graph: &Graph) -> bool {
    let mut visited: HashMap<i32, usize> = HashMap::new();
    let mut on_stack: HashSet<i32> = HashSet::new();
    let mut position = 0usize;
    for node in graph.nodes() {
        if !visited.contains_key(&node)
            && dfs_directed(graph, node, &mut visited, &mut on_stack, &mut position)
        {
            return true;
        }
    }
    false
}

fn dfs_directed(
    graph: &Graph,
    node: i32,
    visited: &mut HashMap<i32, usize>,
    on_stack: &mut HashSet<i32>,
    position: &mut usize,
) -> bool {
    let my_pos = *position;
    visited.insert(node, my_pos);
    mark_inserted(my_pos, node);
    *position += 1;
    on_stack.insert(node);

    for &neighbor in graph.neighbors(node) {
        if let Some(&neighbor_pos) = visited.get(&neighbor) {
            if on_stack.contains(&neighbor) {
                found(neighbor_pos);
                return true;
            }
        } else if dfs_directed(graph, neighbor, visited, on_stack, position) {
            return true;
        }
    }

    on_stack.remove(&node);
    false
}

#[cfg(test)]
include!("../../tests-shared/cycle_detection_tests.rs");
