// Shared between exercises/graphs/src/dijkstra.rs (the learner-facing
// skeleton) and exercises/graphs-solutions/src/dijkstra.rs (the
// reference solution), via `include!()`.
//
// Trace position is a STABLE map assigned once up front (sorted node
// list, index = position), not visit order — a node's distance can
// improve multiple times over one call, and mark_set replaying at the
// same position twice is exactly how the trace shows that improvement.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_single_node_graph_has_distance_zero_to_itself() {
        let graph = WeightedGraph::from_directed_edges(&[(1, 2, 3)]);
        let distances = dijkstra(&graph, 1);
        assert_eq!(distances[&1], 0);
    }

    #[test]
    fn an_unreachable_node_stays_at_i32_max() {
        let graph = WeightedGraph::from_directed_edges(&[(1, 2, 1), (3, 4, 1)]);
        let distances = dijkstra(&graph, 1);
        assert_eq!(distances[&1], 0);
        assert_eq!(distances[&2], 1);
        assert_eq!(distances[&3], i32::MAX);
        assert_eq!(distances[&4], i32::MAX);
    }

    #[test]
    fn a_sink_start_with_no_outgoing_edges_only_reaches_itself() {
        let graph = WeightedGraph::from_directed_edges(&[(1, 2, 5)]);
        let distances = dijkstra(&graph, 2);
        assert_eq!(distances[&2], 0);
        assert_eq!(distances[&1], i32::MAX);
    }

    #[test]
    fn finds_the_shortest_path_even_when_a_longer_one_is_seen_first() {
        // 1->3 direct costs 4, but 1->2->3 costs 1+1=2 — dijkstra must
        // find and keep the cheaper path even though the direct edge
        // to 3 is relaxed first.
        let graph =
            WeightedGraph::from_directed_edges(&[(1, 2, 1), (1, 3, 4), (2, 3, 1), (2, 4, 2), (3, 4, 3)]);
        let distances = dijkstra(&graph, 1);
        assert_eq!(distances[&1], 0);
        assert_eq!(distances[&2], 1);
        assert_eq!(distances[&3], 2);
        assert_eq!(distances[&4], 3);
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        let graph =
            WeightedGraph::from_directed_edges(&[(1, 2, 1), (1, 3, 4), (2, 3, 1), (2, 4, 2), (3, 4, 3)]);
        disable();
        let distances = dijkstra(&graph, 1);
        assert_eq!(distances[&4], 3);
    }

    #[test]
    fn tracing_enabled_captures_the_exact_sequence_including_an_improvement() {
        let graph =
            WeightedGraph::from_directed_edges(&[(1, 2, 1), (1, 3, 4), (2, 3, 1), (2, 4, 2), (3, 4, 3)]);
        enable();
        let distances = dijkstra(&graph, 1);
        let events = take_events();
        assert_eq!(distances[&4], 3);
        assert_eq!(
            events,
            vec![
                Event::Set { i: 0, value: 0 },
                Event::Set { i: 1, value: 1 },
                Event::Set { i: 2, value: 4 },
                Event::Set { i: 2, value: 2 },
                Event::Set { i: 3, value: 3 },
            ]
        );
    }
}
