// Shared between exercises/recursion-backtracking/src/subsets.rs (the
// learner-facing skeleton) and
// exercises/recursion-backtracking-solutions/src/subsets.rs (the
// reference solution), via `include!()`.
//
// subsets fires found() on EVERY recursive call (every prefix is a valid
// subset) — 2^n events for an n-element input. Exact-sequence assertions
// only make sense for the smallest inputs; everything else uses
// structural checks (counts, containment, balance) instead.
#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};
    use std::collections::HashSet;

    #[test]
    fn empty_input_produces_only_the_empty_subset() {
        assert_eq!(subsets(&[]), vec![Vec::<i32>::new()]);
    }

    #[test]
    fn single_element_produces_the_empty_and_full_subset() {
        assert_eq!(subsets(&[1]), vec![vec![], vec![1]]);
    }

    #[test]
    fn produces_exactly_two_to_the_n_subsets() {
        let result = subsets(&[1, 2, 3]);
        assert_eq!(result.len(), 8);
    }

    #[test]
    fn every_subset_is_distinct_and_the_full_set_is_included() {
        let result = subsets(&[1, 2, 3]);
        let unique: HashSet<Vec<i32>> = result.iter().cloned().collect();
        assert_eq!(unique.len(), 8, "every generated subset must be distinct");
        assert!(result.contains(&vec![]));
        assert!(result.contains(&vec![1, 2, 3]));
        assert!(result.contains(&vec![2]));
        assert!(result.contains(&vec![1, 3]));
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let a = subsets(&[1, 2]);
        enable();
        let b = subsets(&[1, 2]);
        disable();
        assert_eq!(a, b);
    }

    #[test]
    fn tracing_enabled_captures_the_exact_sequence_for_one_element() {
        enable();
        subsets(&[1]);
        let events = take_events();
        disable();

        assert_eq!(
            events,
            vec![
                Event::Found { i: 0 },
                Event::Insert { i: 0, value: 1 },
                Event::Found { i: 0 },
                Event::Remove { i: 0 },
            ]
        );
    }

    #[test]
    fn tracing_balances_every_insert_with_a_remove() {
        enable();
        subsets(&[1, 2, 3]);
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
        assert_eq!(inserts, removes, "every pushed candidate must be undone");

        let founds = events
            .iter()
            .filter(|e| matches!(e, Event::Found { .. }))
            .count();
        assert_eq!(founds, 8, "one found() per generated subset (2^3)");
    }
}
