use crate::edge::Edge;
use crate::union_find::UnionFind;

// Kruskal's algorithm: sort every edge cheapest-first, then add each one
// UNLESS it would connect two vertices that are already in the same
// component (which would close a cycle — a tree never has one). The
// given UnionFind does the "already connected?" check in near-O(1):
// union() returns false exactly when the edge would form a cycle.
//
// Ships untraced, same reasoning as mst_prim.
pub fn kruskal_mst(vertices: &[i32], edges: &[Edge]) -> Vec<Edge> {
    todo!("sort edges by weight, then for each one (cheapest first) call UnionFind::union(edge.from, edge.to) — keep the edge only if union() returns true")
}

#[cfg(test)]
include!("../../tests-shared/mst_kruskal_tests.rs");
