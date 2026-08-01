// Shared between exercises/recursion-backtracking/src/combinations.rs
// (the learner-facing skeleton) and
// exercises/recursion-backtracking-solutions/src/combinations.rs (the
// reference solution), via `include!()`.
#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};
    use std::collections::HashSet;

    #[test]
    fn k_of_zero_produces_only_the_empty_combination() {
        assert_eq!(combinations(&[1, 2, 3], 0), vec![Vec::<i32>::new()]);
    }

    #[test]
    fn k_equal_to_length_produces_only_the_full_set() {
        assert_eq!(combinations(&[1, 2, 3], 3), vec![vec![1, 2, 3]]);
    }

    #[test]
    fn produces_every_combination_of_size_k() {
        let result = combinations(&[1, 2, 3], 2);
        assert_eq!(result.len(), 3, "C(3, 2) = 3");
        let unique: HashSet<Vec<i32>> = result.iter().cloned().collect();
        assert_eq!(unique.len(), 3, "every combination must be distinct");
        assert!(result.contains(&vec![1, 2]));
        assert!(result.contains(&vec![1, 3]));
        assert!(result.contains(&vec![2, 3]));
        assert!(result.iter().all(|c| c.len() == 2), "every result must have exactly k elements");
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let a = combinations(&[1, 2, 3], 2);
        enable();
        let b = combinations(&[1, 2, 3], 2);
        disable();
        assert_eq!(a, b);
    }

    #[test]
    fn tracing_enabled_captures_the_exact_sequence_for_k_of_one() {
        enable();
        combinations(&[1, 2], 1);
        let events = take_events();
        disable();

        assert_eq!(
            events,
            vec![
                Event::Insert { i: 0, value: 1 },
                Event::Found { i: 0 },
                Event::Remove { i: 0 },
                Event::Insert { i: 0, value: 2 },
                Event::Found { i: 0 },
                Event::Remove { i: 0 },
            ]
        );
    }

    #[test]
    fn tracing_balances_every_insert_with_a_remove() {
        enable();
        combinations(&[1, 2, 3], 2);
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
        assert_eq!(founds, 3, "one found() per generated combination, C(3,2)=3");
    }
}
