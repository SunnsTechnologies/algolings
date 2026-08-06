// Shared between exercises/graphs/src/bfs.rs (the learner-facing skeleton)
// and exercises/graphs-solutions/src/bfs.rs (the reference solution), via
// `include!()`.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bfs_on_a_sink_vertex_with_no_outgoing_edges_visits_just_that_vertex() {
        // 2 is a pure sink in a directed graph: it exists as a vertex (via
        // from_directed_edges' sink-entry guarantee) but has zero outgoing
        // edges of its own.
        let graph = Graph::from_directed_edges(&[(1, 2)]);
        let order = bfs(&graph, 2);
        assert_eq!(order, vec![2]);
    }

    #[test]
    fn bfs_from_an_unknown_start_returns_just_the_start() {
        let graph = Graph::from_undirected_edges(&[(2, 3)]);
        let order = bfs(&graph, 99);
        assert_eq!(order, vec![99]);
    }

    #[test]
    fn bfs_visits_every_reachable_node_in_level_order() {
        // A diamond: 1 connects to 2 and 3, both of which connect to 4.
        let graph = Graph::from_undirected_edges(&[(1, 2), (1, 3), (2, 4), (3, 4)]);
        let order = bfs(&graph, 1);
        assert_eq!(order, vec![1, 2, 3, 4]);
    }

    #[test]
    fn bfs_does_not_visit_an_unreachable_component() {
        let graph = Graph::from_undirected_edges(&[(1, 2), (3, 4)]);
        let order = bfs(&graph, 1);
        assert_eq!(order, vec![1, 2]);
    }

    #[test]
    fn bfs_handles_a_self_loop_without_infinite_looping() {
        let graph = Graph::from_undirected_edges(&[(1, 1), (1, 2)]);
        let order = bfs(&graph, 1);
        assert_eq!(order, vec![1, 2]);
    }

    #[test]
    fn bfs_never_visits_a_node_twice_in_a_cyclic_graph() {
        let graph = Graph::from_undirected_edges(&[(1, 2), (2, 3), (3, 1)]);
        let order = bfs(&graph, 1);
        assert_eq!(order.len(), 3);
        let mut sorted = order.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, vec![1, 2, 3]);
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        let graph = Graph::from_undirected_edges(&[(1, 2), (1, 3), (2, 4), (3, 4)]);
        disable();
        let order = bfs(&graph, 1);
        assert_eq!(order, vec![1, 2, 3, 4]);
    }

    #[test]
    fn tracing_enabled_captures_the_exact_visit_order() {
        let graph = Graph::from_undirected_edges(&[(1, 2), (1, 3), (2, 4), (3, 4)]);
        enable();
        let order = bfs(&graph, 1);
        let events = take_events();
        assert_eq!(order, vec![1, 2, 3, 4]);
        assert_eq!(
            events,
            vec![
                Event::Insert { i: 0, value: 1 },
                Event::Insert { i: 1, value: 2 },
                Event::Insert { i: 2, value: 3 },
                Event::Insert { i: 3, value: 4 },
            ]
        );
    }
}
