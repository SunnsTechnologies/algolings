// Shared between exercises/recursion-backtracking/src/permutations.rs
// (the learner-facing skeleton) and
// exercises/recursion-backtracking-solutions/src/permutations.rs (the
// reference solution), via `include!()`.
#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};
    use std::collections::HashSet;

    #[test]
    fn empty_input_produces_one_empty_permutation() {
        assert_eq!(permutations(&[]), vec![Vec::<i32>::new()]);
    }

    #[test]
    fn two_elements_produce_both_orderings() {
        assert_eq!(permutations(&[1, 2]), vec![vec![1, 2], vec![2, 1]]);
    }

    #[test]
    fn produces_n_factorial_permutations() {
        let result = permutations(&[1, 2, 3]);
        assert_eq!(result.len(), 6, "3! = 6");
        let unique: HashSet<Vec<i32>> = result.iter().cloned().collect();
        assert_eq!(unique.len(), 6, "every permutation must be distinct");
        assert!(result.iter().all(|p| p.len() == 3));
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let a = permutations(&[1, 2, 3]);
        enable();
        let b = permutations(&[1, 2, 3]);
        disable();
        assert_eq!(a, b);
    }

    #[test]
    fn tracing_enabled_captures_the_exact_sequence_for_two_elements() {
        enable();
        permutations(&[1, 2]);
        let events = take_events();
        disable();

        assert_eq!(
            events,
            vec![
                Event::Insert { i: 0, value: 1 },
                Event::Insert { i: 1, value: 2 },
                Event::Found { i: 1 },
                Event::Remove { i: 1 },
                Event::Remove { i: 0 },
                Event::Insert { i: 0, value: 2 },
                Event::Insert { i: 1, value: 1 },
                Event::Found { i: 1 },
                Event::Remove { i: 1 },
                Event::Remove { i: 0 },
            ]
        );
    }

    #[test]
    fn tracing_balances_every_insert_with_a_remove() {
        enable();
        permutations(&[1, 2, 3]);
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
        assert_eq!(founds, 6, "one found() per generated permutation, 3!=6");
    }
}
