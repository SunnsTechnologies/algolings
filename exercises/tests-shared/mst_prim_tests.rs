// Shared between exercises/graphs/src/mst_prim.rs (the learner-facing
// skeleton) and exercises/graphs-solutions/src/mst_prim.rs (the
// reference solution), via `include!()`.
//
// Ships untraced: an MST is a growing list of (from, to, weight)
// triples, and the existing trace events only carry one i32 value each
// — there's no truthful way to represent "which two vertices this edge
// connects" with Insert/Set alone. The dispatcher still runs this for
// real (so a broken solution still panics and gets caught), it just
// records zero events.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_single_edge_graph_has_a_one_edge_mst() {
        let graph = WeightedGraph::from_undirected_edges(&[(1, 2, 5)]);
        let mst = prim_mst(&graph, 1);
        assert_eq!(mst, vec![(1, 2, 5)]);
    }

    #[test]
    fn builds_the_minimum_spanning_tree() {
        let graph = WeightedGraph::from_undirected_edges(&[
            (1, 2, 1),
            (1, 3, 4),
            (2, 3, 2),
            (2, 4, 5),
            (3, 4, 3),
        ]);
        let mst = prim_mst(&graph, 1);
        assert_eq!(mst, vec![(1, 2, 1), (2, 3, 2), (3, 4, 3)]);
        let total_weight: i32 = mst.iter().map(|&(_, _, w)| w).sum();
        assert_eq!(total_weight, 6);
    }

    #[test]
    fn the_mst_always_has_exactly_vertex_count_minus_one_edges() {
        let graph = WeightedGraph::from_undirected_edges(&[
            (1, 2, 1),
            (1, 3, 4),
            (2, 3, 2),
            (2, 4, 5),
            (3, 4, 3),
        ]);
        let mst = prim_mst(&graph, 1);
        assert_eq!(mst.len(), graph.len() - 1);
    }

    #[test]
    fn skips_a_more_expensive_edge_when_a_cheaper_one_already_connects_the_same_vertex() {
        // A triangle: connecting all three via the two cheapest edges
        // is correct; including the third would form a cycle.
        let graph = WeightedGraph::from_undirected_edges(&[(1, 2, 1), (2, 3, 1), (1, 3, 10)]);
        let mst = prim_mst(&graph, 1);
        assert_eq!(mst.len(), 2);
        let total_weight: i32 = mst.iter().map(|&(_, _, w)| w).sum();
        assert_eq!(total_weight, 2);
    }
}
