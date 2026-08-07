//! Shared scaffolding for dijkstra/mst_prim — a weighted graph, stored as
//! an adjacency list of `(neighbor, weight)` pairs, matching the
//! tutorial's own `type Graph = HashMap<i32, Vec<(i32, i32)>>`. Wrapped
//! the same way the unweighted `Graph` wraps its bare `HashMap<i32,
//! Vec<i32>>` — a separate type rather than adding a weight parameter to
//! `Graph` itself, since every exercise that uses one never needs the
//! other.
//!
//! `from_directed_edges`'s "ensure `to` has an entry" step is NOT merely
//! cosmetic here the way it is for the unweighted `Graph` (there, it only
//! affected `len()`/`nodes()`, since `bfs`/`dfs` discover vertices purely
//! by walking edges). `dijkstra` pre-enumerates `graph.nodes()` up front
//! to build a stable trace-position map before it ever starts relaxing
//! edges — a sink vertex missing from that enumeration means its
//! distance is never tracked at all, and relaxing an edge INTO it panics
//! on a missing key. Keep this step; it's load-bearing for correctness,
//! not just accessor completeness.
//!
//! Values are bare `i32`, matching every other exercise in this project.

use std::collections::HashMap;

#[derive(Default)]
pub struct WeightedGraph {
    adjacency: HashMap<i32, Vec<(i32, i32)>>,
}

impl WeightedGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Each `(from, to, weight)` triple becomes one directed edge.
    pub fn from_directed_edges(edges: &[(i32, i32, i32)]) -> Self {
        let mut graph = Self::new();
        for &(from, to, weight) in edges {
            graph.adjacency.entry(from).or_default().push((to, weight));
            graph.adjacency.entry(to).or_default();
        }
        graph
    }

    /// Each `(a, b, weight)` triple becomes an edge in both directions.
    pub fn from_undirected_edges(edges: &[(i32, i32, i32)]) -> Self {
        let mut graph = Self::new();
        for &(a, b, weight) in edges {
            graph.adjacency.entry(a).or_default().push((b, weight));
            graph.adjacency.entry(b).or_default().push((a, weight));
        }
        graph
    }

    pub fn neighbors(&self, node: i32) -> &[(i32, i32)] {
        self.adjacency.get(&node).map_or(&[], |v| v.as_slice())
    }

    /// Iteration order is whatever the underlying `HashMap` gives —
    /// unordered and not stable across runs. `dijkstra` sorts this before
    /// using it to build a stable trace-position map.
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
