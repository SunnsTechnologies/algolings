mod graph;
mod weighted_graph;
mod edge;
mod union_find;
mod bfs;
mod dfs;
mod cycle_detection;
mod connected_components;
mod dijkstra;
mod bellman_ford;
mod floyd_warshall;
mod mst_prim;
mod mst_kruskal;
mod topological_sort_recursive;
mod topological_sort_kahn;
mod kosaraju_scc;

pub use graph::{reverse_graph, Graph};
pub use weighted_graph::WeightedGraph;
pub use edge::Edge;
pub use union_find::UnionFind;
pub use bfs::bfs;
pub use dfs::dfs;
pub use cycle_detection::{has_cycle_directed, has_cycle_undirected};
pub use connected_components::connected_components;
pub use dijkstra::dijkstra;
pub use bellman_ford::bellman_ford;
pub use floyd_warshall::{floyd_warshall, INF};
pub use mst_prim::prim_mst;
pub use mst_kruskal::kruskal_mst;
pub use topological_sort_recursive::topological_sort_dfs;
pub use topological_sort_kahn::topological_sort_kahn;
pub use kosaraju_scc::strongly_connected_components;

#[cfg(test)]
mod sync_tests {
    /// Regression guard: scaffolding files are hand-duplicated between
    /// this crate and exercises/graphs (there's no shared dependency
    /// between skeleton and solution crates), with nothing else enforcing
    /// they stay in sync. Pure scaffolding — no todo!()s — so
    /// byte-identical is exactly what "in sync" means.
    #[test]
    fn graph_rs_matches_the_skeleton_crate() {
        let solutions = include_str!("graph.rs");
        let skeleton = include_str!("../../graphs/src/graph.rs");
        assert_eq!(
            solutions, skeleton,
            "graph.rs has diverged between graphs-solutions and graphs"
        );
    }

    #[test]
    fn weighted_graph_rs_matches_the_skeleton_crate() {
        let solutions = include_str!("weighted_graph.rs");
        let skeleton = include_str!("../../graphs/src/weighted_graph.rs");
        assert_eq!(
            solutions, skeleton,
            "weighted_graph.rs has diverged between graphs-solutions and graphs"
        );
    }

    #[test]
    fn edge_rs_matches_the_skeleton_crate() {
        let solutions = include_str!("edge.rs");
        let skeleton = include_str!("../../graphs/src/edge.rs");
        assert_eq!(
            solutions, skeleton,
            "edge.rs has diverged between graphs-solutions and graphs"
        );
    }

    #[test]
    fn union_find_rs_matches_the_skeleton_crate() {
        let solutions = include_str!("union_find.rs");
        let skeleton = include_str!("../../graphs/src/union_find.rs");
        assert_eq!(
            solutions, skeleton,
            "union_find.rs has diverged between graphs-solutions and graphs"
        );
    }
}
