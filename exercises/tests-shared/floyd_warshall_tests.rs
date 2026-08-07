// Shared between exercises/graphs/src/floyd_warshall.rs (the
// learner-facing skeleton) and
// exercises/graphs-solutions/src/floyd_warshall.rs (the reference
// solution), via `include!()`.
//
// Same row-major flattening idiom as grid_paths (mark_set(row * n + col,
// value)), except the SAME cell can improve more than once across the
// algorithm's k iterations, not just once — every improvement gets its
// own Set, same as dijkstra/bellman_ford's repeated distance updates.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_single_vertex_has_distance_zero_to_itself() {
        let dist = vec![vec![0]];
        let result = floyd_warshall(dist);
        assert_eq!(result, vec![vec![0]]);
    }

    #[test]
    fn a_single_vertex_produces_no_trace_events() {
        enable();
        let _ = floyd_warshall(vec![vec![0]]);
        let events = take_events();
        assert_eq!(events, Vec::new());
    }

    #[test]
    fn finds_all_pairs_shortest_paths() {
        let dist = vec![
            vec![0, 3, INF, 5],
            vec![INF, 0, 2, INF],
            vec![8, INF, 0, 1],
            vec![INF, INF, INF, 0],
        ];
        let result = floyd_warshall(dist);
        assert_eq!(
            result,
            vec![
                vec![0, 3, 5, 5],
                vec![10, 0, 2, 3],
                vec![8, 11, 0, 1],
                vec![INF, INF, INF, 0],
            ]
        );
    }

    #[test]
    fn a_vertex_with_no_path_to_another_stays_unreachable() {
        let dist = vec![
            vec![0, 3, INF, 5],
            vec![INF, 0, 2, INF],
            vec![8, INF, 0, 1],
            vec![INF, INF, INF, 0],
        ];
        let result = floyd_warshall(dist);
        // Nothing reaches vertex 3 in this graph, and vertex 3 reaches
        // nothing else either.
        assert_eq!(result[0][3], 5);
        assert_eq!(result[3][0], INF);
        assert_eq!(result[3][1], INF);
        assert_eq!(result[3][2], INF);
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        let dist = vec![
            vec![0, 3, INF, 5],
            vec![INF, 0, 2, INF],
            vec![8, INF, 0, 1],
            vec![INF, INF, INF, 0],
        ];
        disable();
        let result = floyd_warshall(dist);
        assert_eq!(result[1][0], 10);
    }

    #[test]
    fn tracing_enabled_captures_the_exact_sequence_of_improvements() {
        let dist = vec![
            vec![0, 3, INF, 5],
            vec![INF, 0, 2, INF],
            vec![8, INF, 0, 1],
            vec![INF, INF, INF, 0],
        ];
        enable();
        let result = floyd_warshall(dist);
        let events = take_events();
        assert_eq!(result[1][0], 10);
        assert_eq!(
            events,
            vec![
                Event::Set { i: 9, value: 11 },
                Event::Set { i: 2, value: 5 },
                Event::Set { i: 4, value: 10 },
                Event::Set { i: 7, value: 3 },
            ]
        );
    }
}
