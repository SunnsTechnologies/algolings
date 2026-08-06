// Shared between exercises/dynamic-programming/src/grid_paths.rs (the
// learner-facing skeleton) and
// exercises/dynamic-programming-solutions/src/grid_paths.rs (the
// reference solution), via `include!()`.
//
// Tracing flattens the 2D table row-major: mark_set(row * cols + col,
// value). The base-case first row and first column both need their OWN
// pass — row 0 must be traced in full (it includes the (0,0) corner),
// and column 0 must start at row 1, not row 0, or the corner cell gets
// a duplicate Set event. `every_position_gets_exactly_one_set_event`
// below is the direct regression test for that.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_single_cell_grid_has_exactly_one_path() {
        assert_eq!(unique_paths(1, 1), 1);
    }

    #[test]
    fn a_single_row_has_exactly_one_path() {
        assert_eq!(unique_paths(1, 5), 1);
    }

    #[test]
    fn a_single_column_has_exactly_one_path() {
        assert_eq!(unique_paths(5, 1), 1);
    }

    #[test]
    fn a_three_by_three_grid_has_six_paths() {
        assert_eq!(unique_paths(3, 3), 6);
    }

    #[test]
    fn a_three_by_four_grid_has_ten_paths() {
        assert_eq!(unique_paths(3, 4), 10);
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        assert_eq!(unique_paths(3, 4), 10);
    }

    #[test]
    fn every_position_gets_exactly_one_set_event() {
        // Regression test for the corner-cell double-write an earlier
        // draft of this exercise's tracing hit: tracing row 0 in full
        // AND column 0 starting from row 0 (instead of row 1) sets
        // (0,0) twice. Every position 0..rows*cols must appear in the
        // trace EXACTLY once, for every grid shape, including the
        // degenerate single-row/single-column cases.
        for (rows, cols) in [(1, 1), (1, 5), (5, 1), (3, 3), (3, 4), (4, 3)] {
            enable();
            let _ = unique_paths(rows, cols);
            let events = take_events();
            assert_eq!(
                events.len(),
                rows * cols,
                "expected exactly one Set per cell for a {rows}x{cols} grid"
            );
            let mut positions: Vec<usize> = events
                .iter()
                .map(|event| match event {
                    Event::Set { i, .. } => *i,
                    other => panic!("expected only Set events, got {other:?}"),
                })
                .collect();
            positions.sort_unstable();
            assert_eq!(
                positions,
                (0..rows * cols).collect::<Vec<_>>(),
                "every position must be set exactly once for a {rows}x{cols} grid, no duplicates or gaps"
            );
        }
    }

    #[test]
    fn tracing_enabled_captures_the_exact_sequence_for_a_three_by_four_grid() {
        enable();
        let result = unique_paths(3, 4);
        let events = take_events();
        assert_eq!(result, 10);
        assert_eq!(
            events,
            vec![
                Event::Set { i: 0, value: 1 },
                Event::Set { i: 1, value: 1 },
                Event::Set { i: 2, value: 1 },
                Event::Set { i: 3, value: 1 },
                Event::Set { i: 4, value: 1 },
                Event::Set { i: 8, value: 1 },
                Event::Set { i: 5, value: 2 },
                Event::Set { i: 6, value: 3 },
                Event::Set { i: 7, value: 4 },
                Event::Set { i: 9, value: 3 },
                Event::Set { i: 10, value: 6 },
                Event::Set { i: 11, value: 10 },
            ]
        );
    }
}
