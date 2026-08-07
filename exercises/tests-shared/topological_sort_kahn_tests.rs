// Shared between exercises/graphs/src/topological_sort_kahn.rs (the
// learner-facing skeleton) and
// exercises/graphs-solutions/src/topological_sort_kahn.rs (the
// reference solution), via `include!()`.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_single_node_graph_sorts_to_itself() {
        let graph = Graph::from_directed_edges(&[(1, 2)]);
        let order = topological_sort_kahn(&graph);
        assert!(order.is_some());
    }

    #[test]
    fn sorts_a_diamond_shaped_dag() {
        let graph = Graph::from_directed_edges(&[(1, 2), (1, 3), (2, 4), (3, 4)]);
        let order = topological_sort_kahn(&graph).unwrap();
        assert_eq!(order.len(), 4);
        assert_before(&order, 1, 2);
        assert_before(&order, 1, 3);
        assert_before(&order, 2, 4);
        assert_before(&order, 3, 4);
    }

    #[test]
    fn returns_none_for_a_graph_with_a_cycle() {
        let graph = Graph::from_directed_edges(&[(1, 2), (2, 1)]);
        assert_eq!(topological_sort_kahn(&graph), None);
    }

    #[test]
    fn returns_none_for_a_longer_cycle() {
        let graph = Graph::from_directed_edges(&[(1, 2), (2, 3), (3, 1)]);
        assert_eq!(topological_sort_kahn(&graph), None);
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        let graph = Graph::from_directed_edges(&[(1, 2), (1, 3), (2, 4), (3, 4)]);
        disable();
        let order = topological_sort_kahn(&graph).unwrap();
        assert_eq!(order.len(), 4);
    }

    #[test]
    fn tracing_enabled_captures_the_exact_order() {
        let graph = Graph::from_directed_edges(&[(1, 2), (1, 3), (2, 4), (3, 4)]);
        enable();
        let order = topological_sort_kahn(&graph).unwrap();
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

    fn assert_before(order: &[i32], earlier: i32, later: i32) {
        let earlier_pos = order.iter().position(|&n| n == earlier).unwrap();
        let later_pos = order.iter().position(|&n| n == later).unwrap();
        assert!(
            earlier_pos < later_pos,
            "{earlier} should come before {later} in {order:?}"
        );
    }
}
