use crate::graph::Graph;
use algolings_trace::{disable, enable, found, mark_inserted, take_events, Event};
use std::collections::{HashMap, HashSet};

// Two different algorithms, sharing a Graph and a tracing idiom: a cycle
// exists in an UNDIRECTED graph if DFS reaches an already-visited
// neighbor that isn't your immediate parent (parent-tracking). A cycle
// exists in a DIRECTED graph if DFS reaches a node that's still on the
// CURRENT recursion stack (stack-tracking) — a directed graph can safely
// revisit an already-fully-explored node without that being a cycle,
// which an undirected graph's edges can't distinguish.
//
// Trace both the same way: position tracks VISIT ORDER across the WHOLE
// call (never resets, even across disconnected components) — call
// mark_inserted(position, node) the first time a node is visited, and
// found(original_position) the instant you confirm a cycle, pointing
// back at wherever that node already sits in the trace.
//
// Use Option<i32> for "no parent", not a magic sentinel value like -1 —
// vertex IDs are ordinary i32s and a self-loop on a vertex whose ID
// matches your sentinel would silently look like "just the parent edge."
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
    todo!("mark the node visited at *position (then increment it), then check each neighbor: already visited and not the parent means a cycle, unvisited means recurse")
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
    todo!("mark the node visited AND on the stack, then check each neighbor: already visited AND still on the stack means a cycle, unvisited means recurse — remove yourself from the stack before returning")
}

#[cfg(test)]
include!("../../tests-shared/cycle_detection_tests.rs");
