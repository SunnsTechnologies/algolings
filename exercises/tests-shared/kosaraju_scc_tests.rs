// Shared between exercises/graphs/src/strongly_connected_components.rs
// (the learner-facing skeleton) and
// exercises/graphs-solutions/src/strongly_connected_components.rs (the
// reference solution), via `include!()`.
//
// Kosaraju's algorithm runs two DFS passes — pass 1 (fill_order, over
// the original graph) is untraced bookkeeping; pass 2 (the actual
// group-building DFS, over the reversed graph) is traced with the same
// monotonic-position-counter idiom connected_components already
// established: mark_inserted(*position, node), never reset per group.
#[cfg(test)]
mod tests {
    use super::*;

    fn sorted_components(mut components: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for component in &mut components {
            component.sort_unstable();
        }
        components.sort_unstable();
        components
    }

    #[test]
    fn a_single_node_is_its_own_component() {
        let graph = Graph::from_directed_edges(&[(1, 1)]);
        let sccs = sorted_components(strongly_connected_components(&graph));
        assert_eq!(sccs, vec![vec![1]]);
    }

    #[test]
    fn a_cycle_forms_one_strongly_connected_component() {
        let graph = Graph::from_directed_edges(&[(1, 2), (2, 3), (3, 1)]);
        let sccs = sorted_components(strongly_connected_components(&graph));
        assert_eq!(sccs, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn a_dag_with_no_cycles_makes_every_node_its_own_component() {
        let graph = Graph::from_directed_edges(&[(1, 2), (2, 3)]);
        let sccs = sorted_components(strongly_connected_components(&graph));
        assert_eq!(sccs, vec![vec![1], vec![2], vec![3]]);
    }

    #[test]
    fn a_cycle_plus_a_trailing_chain_separates_correctly() {
        // 1,2,3 form a cycle; 4 and 5 hang off the cycle one-way, with
        // no path back — each stays its own singleton component.
        let graph = Graph::from_directed_edges(&[(1, 2), (2, 3), (3, 1), (3, 4), (4, 5)]);
        let sccs = sorted_components(strongly_connected_components(&graph));
        assert_eq!(sccs, vec![vec![1, 2, 3], vec![4], vec![5]]);
    }

    #[test]
    fn every_vertex_appears_in_exactly_one_component() {
        let graph = Graph::from_directed_edges(&[(1, 2), (2, 3), (3, 1), (3, 4), (4, 5)]);
        let sccs = strongly_connected_components(&graph);
        let total: usize = sccs.iter().map(Vec::len).sum();
        assert_eq!(total, graph.len());
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        let graph = Graph::from_directed_edges(&[(1, 2), (2, 3), (3, 1)]);
        disable();
        let sccs = sorted_components(strongly_connected_components(&graph));
        assert_eq!(sccs, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn tracing_never_resets_the_position_counter_across_components() {
        let graph = Graph::from_directed_edges(&[(1, 2), (2, 3), (3, 1), (3, 4), (4, 5)]);
        enable();
        let _ = strongly_connected_components(&graph);
        let events = take_events();

        assert_eq!(events.len(), graph.len(), "one Insert per vertex, no repeats");
        let mut positions: Vec<usize> = events
            .iter()
            .map(|event| match event {
                Event::Insert { i, .. } => *i,
                other => panic!("expected only Insert events, got {other:?}"),
            })
            .collect();
        positions.sort_unstable();
        assert_eq!(
            positions,
            (0..graph.len()).collect::<Vec<_>>(),
            "position counter must run 0..n once across the whole call, never reset"
        );
    }

    #[test]
    fn tracing_enabled_captures_the_exact_sequence_for_a_single_incoming_edge_chain() {
        // Every node here has at most one incoming edge, so the
        // reversed graph's neighbor lists have no ordering ambiguity —
        // safe to assert an exact sequence.
        let graph = Graph::from_directed_edges(&[(1, 2), (2, 3), (3, 1), (3, 4), (4, 5)]);
        enable();
        let sccs = strongly_connected_components(&graph);
        let events = take_events();
        assert_eq!(sorted_components(sccs), vec![vec![1, 2, 3], vec![4], vec![5]]);
        assert_eq!(
            events,
            vec![
                Event::Insert { i: 0, value: 1 },
                Event::Insert { i: 1, value: 3 },
                Event::Insert { i: 2, value: 2 },
                Event::Insert { i: 3, value: 4 },
                Event::Insert { i: 4, value: 5 },
            ]
        );
    }
}
