// Shared between exercises/graphs/src/connected_components.rs (the
// learner-facing skeleton) and
// exercises/graphs-solutions/src/connected_components.rs (the reference
// solution), via `include!()`.
//
// Graph::nodes() iterates a HashMap, so which component is discovered
// first — and which of its vertices becomes the DFS entry point — isn't
// stable across runs. Tests compare components as SETS (sort each inner
// Vec, then sort the outer Vec) rather than asserting exact order. The
// tracing test checks the POSITION COUNTER never resets across
// components (extract every `i`, confirm it's exactly 0..n with no
// repeats) rather than asserting exact node values at exact positions —
// this is the direct regression test for the bug an outside-voice review
// caught before implementation: resetting position per component made
// Event::Insert (a real `Vec::insert`, not an overwrite) shove every
// earlier component's display sideways on every single run.
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
    fn an_empty_graph_has_no_components() {
        let graph = Graph::new();
        assert_eq!(connected_components(&graph), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn a_fully_connected_graph_is_one_component() {
        let graph = Graph::from_undirected_edges(&[(1, 2), (2, 3), (3, 1)]);
        let components = sorted_components(connected_components(&graph));
        assert_eq!(components, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn finds_multiple_disconnected_components() {
        let graph = Graph::from_undirected_edges(&[(1, 2), (3, 4), (4, 5)]);
        let components = sorted_components(connected_components(&graph));
        assert_eq!(components, vec![vec![1, 2], vec![3, 4, 5]]);
    }

    #[test]
    fn a_self_loop_forms_its_own_single_vertex_component() {
        let graph = Graph::from_undirected_edges(&[(1, 1), (2, 3)]);
        let components = sorted_components(connected_components(&graph));
        assert_eq!(components, vec![vec![1], vec![2, 3]]);
    }

    #[test]
    fn every_vertex_appears_in_exactly_one_component() {
        let graph = Graph::from_undirected_edges(&[(1, 2), (3, 4), (4, 5), (6, 6)]);
        let components = connected_components(&graph);
        let total: usize = components.iter().map(Vec::len).sum();
        assert_eq!(total, graph.len());

        let mut all_values: Vec<i32> = components.into_iter().flatten().collect();
        all_values.sort_unstable();
        assert_eq!(all_values, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        let graph = Graph::from_undirected_edges(&[(1, 2), (3, 4)]);
        disable();
        let components = sorted_components(connected_components(&graph));
        assert_eq!(components, vec![vec![1, 2], vec![3, 4]]);
    }

    #[test]
    fn tracing_enabled_captures_the_exact_sequence_for_a_single_vertex() {
        let graph = Graph::from_undirected_edges(&[(1, 1)]);
        enable();
        let components = connected_components(&graph);
        let events = take_events();
        assert_eq!(components, vec![vec![1]]);
        assert_eq!(events, vec![Event::Insert { i: 0, value: 1 }]);
    }

    #[test]
    fn tracing_never_resets_the_position_counter_across_components() {
        let graph = Graph::from_undirected_edges(&[(1, 2), (3, 4), (4, 5)]);
        enable();
        let _ = connected_components(&graph);
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
}
