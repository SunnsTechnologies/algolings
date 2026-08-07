// Shared between exercises/graphs/src/mst_kruskal.rs (the
// learner-facing skeleton) and
// exercises/graphs-solutions/src/mst_kruskal.rs (the reference
// solution), via `include!()`.
//
// Ships untraced, same reasoning as mst_prim.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_single_edge_graph_has_a_one_edge_mst() {
        let edges = [Edge::new(1, 2, 5)];
        let mst = kruskal_mst(&[1, 2], &edges);
        assert_eq!(mst, vec![Edge::new(1, 2, 5)]);
    }

    #[test]
    fn builds_the_same_minimum_spanning_tree_as_prims() {
        let edges = [
            Edge::new(1, 2, 1),
            Edge::new(1, 3, 4),
            Edge::new(2, 3, 2),
            Edge::new(2, 4, 5),
            Edge::new(3, 4, 3),
        ];
        let mst = kruskal_mst(&[1, 2, 3, 4], &edges);
        assert_eq!(
            mst,
            vec![Edge::new(1, 2, 1), Edge::new(2, 3, 2), Edge::new(3, 4, 3)]
        );
        let total_weight: i32 = mst.iter().map(|e| e.weight).sum();
        assert_eq!(total_weight, 6);
    }

    #[test]
    fn the_mst_always_has_exactly_vertex_count_minus_one_edges() {
        let edges = [
            Edge::new(1, 2, 1),
            Edge::new(1, 3, 4),
            Edge::new(2, 3, 2),
            Edge::new(2, 4, 5),
            Edge::new(3, 4, 3),
        ];
        let vertices = [1, 2, 3, 4];
        let mst = kruskal_mst(&vertices, &edges);
        assert_eq!(mst.len(), vertices.len() - 1);
    }

    #[test]
    fn skips_an_edge_that_would_close_a_cycle() {
        let edges = [
            Edge::new(1, 2, 1),
            Edge::new(2, 3, 1),
            Edge::new(1, 3, 10),
        ];
        let mst = kruskal_mst(&[1, 2, 3], &edges);
        assert_eq!(mst.len(), 2);
        assert!(!mst.contains(&Edge::new(1, 3, 10)));
        let total_weight: i32 = mst.iter().map(|e| e.weight).sum();
        assert_eq!(total_weight, 2);
    }
}
