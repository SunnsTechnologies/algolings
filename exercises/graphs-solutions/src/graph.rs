//! Shared scaffolding for bfs/dfs/cycle_detection/connected_components — an
//! unweighted graph, stored as an adjacency list. The tutorial's own Rust
//! source represents this as a bare `type Graph = HashMap<i32, Vec<i32>>`
//! with no wrapper at all; this project wraps it the same way every other
//! module wraps its tutorial data structure (`Bst`, `AvlTree`, `RbTree`),
//! so `new`/`len`/`is_empty` stay consistent across the whole curriculum.
//!
//! One struct serves bfs/dfs/cycle_detection/connected_components/
//! topological_sort_dfs/topological_sort_kahn/strongly_connected_components
//! — directed vs undirected is a choice of constructor (`from_directed_edges`
//! inserts one direction per edge, `from_undirected_edges` inserts both), not
//! a type-level distinction. Weighted algorithms (Dijkstra, Bellman-Ford,
//! MST, ...) use the separate `WeightedGraph` in `weighted_graph.rs` instead.
//!
//! Values are bare `i32` vertex IDs, matching every other exercise in this
//! project.

use std::collections::HashMap;

#[derive(Default)]
pub struct Graph {
    adjacency: HashMap<i32, Vec<i32>>,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Each `(from, to)` pair becomes one directed edge. `to` also gets an
    /// entry (possibly empty) if it doesn't already have one — without
    /// this, a pure "sink" vertex (never a `from`) would be invisible to
    /// `nodes()`/`len()`, even though it's still reachable via traversal
    /// from whichever vertex points to it.
    pub fn from_directed_edges(edges: &[(i32, i32)]) -> Self {
        let mut graph = Self::new();
        for &(from, to) in edges {
            graph.adjacency.entry(from).or_default().push(to);
            graph.adjacency.entry(to).or_default();
        }
        graph
    }

    /// Each `(a, b)` pair becomes an edge in both directions.
    pub fn from_undirected_edges(edges: &[(i32, i32)]) -> Self {
        let mut graph = Self::new();
        for &(a, b) in edges {
            graph.adjacency.entry(a).or_default().push(b);
            graph.adjacency.entry(b).or_default().push(a);
        }
        graph
    }

    pub fn neighbors(&self, node: i32) -> &[i32] {
        self.adjacency.get(&node).map_or(&[], |v| v.as_slice())
    }

    /// Iteration order is whatever the underlying `HashMap` gives —
    /// unordered and not stable across runs. Callers that need a
    /// deterministic result (e.g. `connected_components`'s tests) must
    /// sort before comparing.
    pub fn nodes(&self) -> impl Iterator<Item = i32> + '_ {
        self.adjacency.keys().copied()
    }

    pub fn len(&self) -> usize {
        self.adjacency.len()
    }

    pub fn is_empty(&self) -> bool {
        self.adjacency.is_empty()
    }
}

/// Flips every edge's direction — given scaffolding, used by
/// `strongly_connected_components`'s Kosaraju's algorithm (the graded
/// lesson is the two-DFS-pass orchestration, not this mechanical
/// transformation). Every node in the original graph gets an entry in
/// the reversed graph too, even ones with no incoming edges of their
/// own — the same "sink stays visible" guarantee `from_directed_edges`
/// already makes, kept here for consistency even though Kosaraju's own
/// traversal doesn't strictly need it (`neighbors()` already degrades a
/// missing key to an empty slice).
pub fn reverse_graph(graph: &Graph) -> Graph {
    let mut reversed = Graph::new();
    for from in graph.nodes() {
        reversed.adjacency.entry(from).or_default();
        for &to in graph.neighbors(from) {
            reversed.adjacency.entry(to).or_default().push(from);
        }
    }
    reversed
}
