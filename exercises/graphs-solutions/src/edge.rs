//! Shared scaffolding for bellman_ford/mst_kruskal — a plain weighted
//! edge, for algorithms that work over a flat edge list instead of an
//! adjacency structure. The tutorial's own two articles define this
//! twice under different field names (bellman-ford's own `Edge { from,
//! to, weight }`, minimum-spanning-trees' Kruskal implementation's
//! `Edge { u, v, weight }`) — unified here under one name so both
//! exercises share the same type instead of each declaring their own.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge {
    pub from: i32,
    pub to: i32,
    pub weight: i32,
}

impl Edge {
    pub fn new(from: i32, to: i32, weight: i32) -> Self {
        Self { from, to, weight }
    }
}
