use crate::edge::Edge;
use crate::union_find::UnionFind;

/// Reference solution for the `mst_kruskal` exercise.
pub fn kruskal_mst(vertices: &[i32], edges: &[Edge]) -> Vec<Edge> {
    let mut sorted_edges = edges.to_vec();
    sorted_edges.sort_by_key(|e| e.weight);

    let mut uf = UnionFind::new(vertices);
    let mut mst = Vec::new();

    for edge in sorted_edges {
        if uf.union(edge.from, edge.to) {
            mst.push(edge);
        }
    }

    mst
}

#[cfg(test)]
include!("../../tests-shared/mst_kruskal_tests.rs");
