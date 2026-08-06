// Shared between exercises/graphs/src/cycle_detection.rs (the
// learner-facing skeleton) and exercises/graphs-solutions/src/cycle_detection.rs
// (the reference solution), via `include!()`.
//
// Graph::nodes() iterates a HashMap, so its order isn't stable across
// runs — the exact TRACE tests below deliberately use single-vertex
// self-loop fixtures (no ambiguity about which vertex the outer loop
// picks first) rather than asserting an exact sequence on a multi-vertex
// graph, which would be flaky. Every other test asserts only the
// boolean result, which is correct regardless of traversal order.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undirected_empty_graph_has_no_cycle() {
        let graph = Graph::new();
        assert!(!has_cycle_undirected(&graph));
    }

    #[test]
    fn undirected_a_single_edge_is_not_a_cycle() {
        // The critical case proving parent-skipping works: without it, a
        // naive "any visited neighbor means a cycle" check would flag
        // this, since 2's only neighbor (1) is already visited.
        let graph = Graph::from_undirected_edges(&[(1, 2)]);
        assert!(!has_cycle_undirected(&graph));
    }

    #[test]
    fn undirected_a_star_shape_is_not_a_cycle() {
        let graph = Graph::from_undirected_edges(&[(1, 2), (1, 3), (1, 4)]);
        assert!(!has_cycle_undirected(&graph));
    }

    #[test]
    fn undirected_a_triangle_has_a_cycle() {
        let graph = Graph::from_undirected_edges(&[(1, 2), (2, 3), (3, 1)]);
        assert!(has_cycle_undirected(&graph));
    }

    #[test]
    fn undirected_detects_a_cycle_in_one_of_several_disconnected_components() {
        let graph = Graph::from_undirected_edges(&[(1, 2), (3, 4), (4, 5), (5, 3)]);
        assert!(has_cycle_undirected(&graph));
    }

    #[test]
    fn undirected_no_disconnected_component_has_a_cycle() {
        let graph = Graph::from_undirected_edges(&[(1, 2), (3, 4)]);
        assert!(!has_cycle_undirected(&graph));
    }

    #[test]
    fn undirected_detects_a_self_loop_on_a_vertex_id_that_would_collide_with_a_naive_sentinel() {
        // Regression test: a parent-tracking implementation that uses a
        // magic i32 sentinel (e.g. -1) for "no parent" instead of
        // Option<i32> would silently miss this — vertex -1's self-loop
        // neighbor (-1) would compare equal to the sentinel and get
        // treated as "just the parent edge."
        let graph = Graph::from_undirected_edges(&[(-1, -1)]);
        assert!(has_cycle_undirected(&graph));
    }

    #[test]
    fn directed_empty_graph_has_no_cycle() {
        let graph = Graph::new();
        assert!(!has_cycle_directed(&graph));
    }

    #[test]
    fn directed_a_simple_chain_is_not_a_cycle() {
        let graph = Graph::from_directed_edges(&[(1, 2), (2, 3)]);
        assert!(!has_cycle_directed(&graph));
    }

    #[test]
    fn directed_a_dag_with_a_shared_descendant_is_not_a_cycle() {
        // 4 is reachable from both 2 and 3 but is never an ANCESTOR of
        // either — proves the algorithm distinguishes "already fully
        // explored" from "currently on the recursion stack."
        let graph = Graph::from_directed_edges(&[(1, 2), (1, 3), (2, 4), (3, 4)]);
        assert!(!has_cycle_directed(&graph));
    }

    #[test]
    fn directed_a_cycle_is_detected() {
        let graph = Graph::from_directed_edges(&[(1, 2), (2, 3), (3, 1)]);
        assert!(has_cycle_directed(&graph));
    }

    #[test]
    fn directed_a_self_loop_is_a_cycle() {
        let graph = Graph::from_directed_edges(&[(1, 1)]);
        assert!(has_cycle_directed(&graph));
    }

    #[test]
    fn directed_detects_a_cycle_in_one_of_several_disconnected_components() {
        let graph = Graph::from_directed_edges(&[(1, 2), (3, 4), (4, 5), (5, 3)]);
        assert!(has_cycle_directed(&graph));
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        let graph = Graph::from_undirected_edges(&[(1, 2), (2, 3), (3, 1)]);
        disable();
        assert!(has_cycle_undirected(&graph));
    }

    #[test]
    fn tracing_enabled_captures_the_exact_sequence_for_an_undirected_self_loop() {
        let graph = Graph::from_undirected_edges(&[(1, 1)]);
        enable();
        let result = has_cycle_undirected(&graph);
        let events = take_events();
        assert!(result);
        assert_eq!(
            events,
            vec![
                Event::Insert { i: 0, value: 1 },
                Event::Found { i: 0 },
            ]
        );
    }

    #[test]
    fn tracing_enabled_captures_the_exact_sequence_for_a_directed_self_loop() {
        let graph = Graph::from_directed_edges(&[(1, 1)]);
        enable();
        let result = has_cycle_directed(&graph);
        let events = take_events();
        assert!(result);
        assert_eq!(
            events,
            vec![
                Event::Insert { i: 0, value: 1 },
                Event::Found { i: 0 },
            ]
        );
    }
}
