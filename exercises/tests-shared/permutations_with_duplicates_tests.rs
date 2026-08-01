// Shared between
// exercises/recursion-backtracking/src/permutations_with_duplicates.rs
// (the learner-facing skeleton) and
// exercises/recursion-backtracking-solutions/src/permutations_with_duplicates.rs
// (the reference solution), via `include!()`.
#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};
    use std::collections::HashSet;

    #[test]
    fn no_duplicates_behaves_like_plain_permutations() {
        let result = permute_unique(vec![1, 2]);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&vec![1, 2]));
        assert!(result.contains(&vec![2, 1]));
    }

    #[test]
    fn two_identical_elements_produce_exactly_one_permutation() {
        // Naive permutations would produce [1,1] twice (once per which
        // "1" is considered first) — the whole point of this exercise is
        // skipping that duplicate.
        assert_eq!(permute_unique(vec![1, 1]), vec![vec![1, 1]]);
    }

    #[test]
    fn duplicates_are_not_double_counted() {
        let result = permute_unique(vec![1, 1, 2]);
        // Naive would be 3! = 6; with one pair of duplicates it's 3!/2! = 3.
        assert_eq!(result.len(), 3);
        let unique: HashSet<Vec<i32>> = result.iter().cloned().collect();
        assert_eq!(unique.len(), 3, "every permutation must be distinct");
        assert!(result.contains(&vec![1, 1, 2]));
        assert!(result.contains(&vec![1, 2, 1]));
        assert!(result.contains(&vec![2, 1, 1]));
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let a = permute_unique(vec![1, 1, 2]);
        enable();
        let b = permute_unique(vec![1, 1, 2]);
        disable();
        assert_eq!(a, b);
    }

    #[test]
    fn tracing_enabled_captures_the_exact_sequence_for_two_duplicates() {
        enable();
        permute_unique(vec![1, 1]);
        let events = take_events();
        disable();

        // Only one branch of the top-level loop ever runs — the other is
        // skipped as a duplicate, which is why there are only 5 events
        // instead of the 10 a duplicate-blind version would produce.
        assert_eq!(
            events,
            vec![
                Event::Insert { i: 0, value: 1 },
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
        permute_unique(vec![1, 1, 2]);
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
        assert_eq!(founds, 3, "one found() per unique permutation");
    }
}
