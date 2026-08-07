// Shared between exercises/graphs/src/bellman_ford.rs (the
// learner-facing skeleton) and
// exercises/graphs-solutions/src/bellman_ford.rs (the reference
// solution), via `include!()`.
//
// Same stable-position-map tracing idiom as dijkstra. `vertices` must be
// a strict superset of every edge's `from`/`to` — an edge endpoint
// missing from `vertices` panics on a missing HashMap key, the same way
// dijkstra would panic on a start node outside the graph.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_single_vertex_has_distance_zero_to_itself() {
        let distances = bellman_ford(&[1], &[], 1).unwrap();
        assert_eq!(distances[&1], 0);
    }

    #[test]
    fn an_unreachable_vertex_stays_at_i32_max() {
        let edges = [Edge::new(1, 2, 1)];
        let distances = bellman_ford(&[1, 2, 3], &edges, 1).unwrap();
        assert_eq!(distances[&2], 1);
        assert_eq!(distances[&3], i32::MAX);
    }

    #[test]
    fn finds_the_shortest_path_even_when_a_longer_one_is_seen_first() {
        let edges = [
            Edge::new(1, 2, 1),
            Edge::new(1, 3, 4),
            Edge::new(2, 3, 1),
            Edge::new(2, 4, 2),
            Edge::new(3, 4, 3),
        ];
        let distances = bellman_ford(&[1, 2, 3, 4], &edges, 1).unwrap();
        assert_eq!(distances[&1], 0);
        assert_eq!(distances[&2], 1);
        assert_eq!(distances[&3], 2);
        assert_eq!(distances[&4], 3);
    }

    #[test]
    fn handles_negative_edge_weights_without_a_cycle() {
        let edges = [Edge::new(1, 2, 4), Edge::new(2, 3, -2), Edge::new(1, 3, 3)];
        let distances = bellman_ford(&[1, 2, 3], &edges, 1).unwrap();
        assert_eq!(distances[&3], 2); // via 1->2->3 = 4 + -2 = 2, cheaper than direct 3
    }

    #[test]
    fn detects_a_negative_weight_cycle() {
        let edges = [Edge::new(1, 2, 1), Edge::new(2, 3, -3), Edge::new(3, 1, 1)];
        let result = bellman_ford(&[1, 2, 3], &edges, 1);
        assert!(result.is_err());
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        let edges = [
            Edge::new(1, 2, 1),
            Edge::new(1, 3, 4),
            Edge::new(2, 3, 1),
            Edge::new(2, 4, 2),
            Edge::new(3, 4, 3),
        ];
        disable();
        let distances = bellman_ford(&[1, 2, 3, 4], &edges, 1).unwrap();
        assert_eq!(distances[&4], 3);
    }

    #[test]
    fn tracing_enabled_captures_the_exact_sequence_including_an_improvement() {
        let edges = [
            Edge::new(1, 2, 1),
            Edge::new(1, 3, 4),
            Edge::new(2, 3, 1),
            Edge::new(2, 4, 2),
            Edge::new(3, 4, 3),
        ];
        enable();
        let distances = bellman_ford(&[1, 2, 3, 4], &edges, 1).unwrap();
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
