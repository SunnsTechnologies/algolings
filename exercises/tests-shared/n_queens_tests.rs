// Shared between exercises/recursion-backtracking/src/n_queens.rs (the
// learner-facing skeleton) and
// exercises/recursion-backtracking-solutions/src/n_queens.rs (the
// reference solution), via `include!()`.
#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};
    use std::collections::HashSet;

    #[test]
    fn zero_queens_has_exactly_one_empty_solution() {
        assert_eq!(solve_n_queens(0), vec![Vec::<String>::new()]);
    }

    #[test]
    fn one_queen_has_exactly_one_solution() {
        assert_eq!(solve_n_queens(1), vec![vec!["Q".to_string()]]);
    }

    #[test]
    fn two_and_three_queens_have_no_solutions() {
        // Well-known: the 2-queens and 3-queens boards are too small for
        // any placement to avoid every row/column/diagonal conflict.
        assert_eq!(solve_n_queens(2), Vec::<Vec<String>>::new());
        assert_eq!(solve_n_queens(3), Vec::<Vec<String>>::new());
    }

    #[test]
    fn four_queens_has_exactly_two_solutions() {
        let result = solve_n_queens(4);
        assert_eq!(result.len(), 2);
        let unique: HashSet<Vec<String>> = result.iter().cloned().collect();
        assert_eq!(unique.len(), 2, "the two solutions must be distinct");
    }

    #[test]
    fn every_solution_places_exactly_one_queen_per_row_and_column() {
        for board in solve_n_queens(4) {
            assert_eq!(board.len(), 4);
            let mut columns_used = vec![false; 4];
            for row in &board {
                let queens_in_row = row.chars().filter(|&c| c == 'Q').count();
                assert_eq!(queens_in_row, 1, "each row must have exactly one queen");
                let col = row.chars().position(|c| c == 'Q').unwrap();
                assert!(!columns_used[col], "no column may be reused");
                columns_used[col] = true;
            }
        }
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let a = solve_n_queens(4);
        enable();
        let b = solve_n_queens(4);
        disable();
        assert_eq!(a, b);
    }

    #[test]
    fn tracing_enabled_captures_the_exact_sequence_for_a_solutionless_board() {
        // n=2 has zero valid placements — every attempt eventually
        // conflicts, so this proves the trace stays perfectly balanced
        // (every insert has a matching remove) with NO found() events at
        // all, and that a row where every column conflicts never fires an
        // unpaired remove (mark_inserted/mark_removed only ever happen
        // together, inside the same "column didn't conflict" branch).
        enable();
        solve_n_queens(2);
        let events = take_events();
        disable();

        assert_eq!(
            events,
            vec![
                Event::Insert { i: 0, value: 0 },
                Event::Remove { i: 0 },
                Event::Insert { i: 0, value: 1 },
                Event::Remove { i: 0 },
            ]
        );
    }

    #[test]
    fn tracing_enabled_captures_the_trivial_one_queen_case() {
        enable();
        solve_n_queens(1);
        let events = take_events();
        disable();

        assert_eq!(
            events,
            vec![
                Event::Insert { i: 0, value: 0 },
                Event::Found { i: 0 },
                Event::Remove { i: 0 },
            ]
        );
    }

    #[test]
    fn tracing_balances_every_insert_with_a_remove_for_four_queens() {
        enable();
        solve_n_queens(4);
        let events = take_events();
        disable();

        let inserts = events
            .iter()
            .filter(|e| matches!(e, Event::Insert { .. }))
            .count();
        let removes = events
            .iter()
            .filter(|e| matches!(e, Event::Remove { .. }))
            .count();
        assert_eq!(inserts, removes, "every placed queen must be undone");

        let founds = events
            .iter()
            .filter(|e| matches!(e, Event::Found { .. }))
            .count();
        assert_eq!(founds, 2, "one found() per solution board");
    }
}
