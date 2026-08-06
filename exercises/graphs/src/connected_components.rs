use crate::graph::Graph;
use algolings_trace::{disable, enable, mark_inserted, take_events, Event};
use std::collections::HashSet;

// Find every connected component: run DFS from each not-yet-visited
// vertex, collecting everything it reaches into one group, until every
// vertex belongs to exactly one group.
//
// Own private DFS helper here, same as cycle_detection — don't call the
// public dfs() from dfs.rs, so this exercise's tests don't depend on
// that one being solved.
//
// Trace with ONE running position counter across the WHOLE call —
// mark_inserted(position, node) the first time each node is visited,
// incrementing once per visit, regardless of which component it belongs
// to. Do NOT restart the counter at 0 for each new component: Insert
// replays as a real array insert, not an overwrite, so resetting would
// make every later component shove every earlier one sideways in the
// animation.
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
    todo!("mark_inserted(*position, node) then increment it, push node into component, recurse into every unvisited neighbor")
}

#[cfg(test)]
include!("../../tests-shared/connected_components_tests.rs");
