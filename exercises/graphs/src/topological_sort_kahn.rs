use crate::graph::Graph;
use algolings_trace::{disable, enable, mark_inserted, take_events, Event};
use std::collections::{HashMap, VecDeque};

// Topological sort (Kahn's algorithm, BFS-based): repeatedly peel off
// any vertex with no remaining unprocessed dependencies (in-degree 0).
// Start by counting every vertex's in-degree, seed a queue with
// whichever ones start at 0, then each time you process a vertex,
// "remove" it by decrementing its neighbors' in-degrees — any neighbor
// that drops to 0 becomes newly available.
//
// Unlike the DFS-based version, `order` gets built DIRECTLY in final
// topological order as vertices are processed — no reversal needed, so
// mark_inserted(order.len(), node) maps straight onto the real return
// value with no extra pass required.
//
// If fewer vertices get processed than exist in the graph, some are
// stuck waiting on each other forever — a cycle.
pub fn topological_sort_kahn(graph: &Graph) -> Option<Vec<i32>> {
    todo!("count in-degrees, seed a queue with in-degree-0 vertices (sorted, for deterministic order), pop+trace+decrement neighbors, requeue anything that hits 0 — None if the final order is shorter than the vertex count")
}

#[cfg(test)]
include!("../../tests-shared/topological_sort_kahn_tests.rs");
