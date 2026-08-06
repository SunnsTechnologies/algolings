mod graph;
mod bfs;
mod dfs;
mod cycle_detection;
mod connected_components;

pub use graph::Graph;
pub use bfs::bfs;
pub use dfs::dfs;
pub use cycle_detection::{has_cycle_directed, has_cycle_undirected};
pub use connected_components::connected_components;

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
}
