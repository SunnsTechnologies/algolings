use crate::graph::{reverse_graph, Graph};
use algolings_trace::{disable, enable, mark_inserted, take_events, Event};
use std::collections::HashSet;

// Strongly connected components (Kosaraju's algorithm): find every
// maximal group of vertices that can all reach each other. Two passes:
//
// 1. DFS from every not-yet-visited vertex over the ORIGINAL graph,
//    pushing each one onto a stack once everything reachable from it is
//    fully explored (finish order) — untraced, same status as
//    topological_sort_dfs's finish-order pass.
// 2. DFS over the REVERSED graph (use the given `reverse_graph`),
//    popping vertices off that finish-order stack — each DFS this pass
//    starts collects exactly one strongly connected component.
//
// Trace ONLY pass 2, same idiom connected_components already
// established: ONE running position counter across the WHOLE call,
// mark_inserted(*position, node) as each node joins ANY component,
// never reset per group.
pub fn strongly_connected_components(graph: &Graph) -> Vec<Vec<i32>> {
    todo!("pass 1: untraced DFS building a finish-order stack over the original graph. pass 2: DFS over reverse_graph(graph), popping the stack, tracing each node's FIRST visit with a running position counter that never resets")
}

#[cfg(test)]
include!("../../tests-shared/kosaraju_scc_tests.rs");
