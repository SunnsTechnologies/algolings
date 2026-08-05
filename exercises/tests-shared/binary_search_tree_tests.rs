// Shared between exercises/trees/src/binary_search_tree.rs (the
// learner-facing skeleton) and
// exercises/trees-solutions/src/binary_search_tree.rs (the reference
// solution), via `include!()`.
#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    #[test]
    fn insert_into_an_empty_tree_places_the_root() {
        let mut tree = Bst::new();
        tree.insert(5);
        assert!(tree_contains(&tree.root, 5));
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn insert_maintains_the_bst_property() {
        let mut tree = Bst::new();
        for value in [5, 3, 8, 1, 4] {
            tree.insert(value);
        }
        for value in [5, 3, 8, 1, 4] {
            assert!(tree_contains(&tree.root, value));
        }
        assert_eq!(tree.len(), 5);
    }

    #[test]
    fn inserting_a_duplicate_does_not_grow_the_tree() {
        let mut tree = Bst::from_values(&[5, 3, 8]);
        tree.insert(5);
        assert_eq!(tree.len(), 3);
    }

    #[test]
    fn contains_finds_an_existing_value() {
        let tree = Bst::from_values(&[5, 3, 8, 1]);
        assert!(tree.contains(1));
    }

    #[test]
    fn contains_on_an_empty_tree_returns_false() {
        let tree = Bst::new();
        assert!(!tree.contains(5));
    }

    #[test]
    fn contains_returns_false_for_a_missing_value() {
        let tree = Bst::from_values(&[5, 3, 8]);
        assert!(!tree.contains(99));
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let mut a = Bst::new();
        a.insert(5);
        let contains_a = a.contains(5);
        enable();
        let mut b = Bst::new();
        b.insert(5);
        let contains_b = b.contains(5);
        disable();
        assert_eq!(contains_a, contains_b);
    }

    #[test]
    fn tracing_enabled_captures_the_exact_descent_for_insert() {
        let mut tree = Bst::from_values(&[5, 3, 8]);

        enable();
        tree.insert(1);
        let events = take_events();
        disable();

        // 1 < 5 (depth 0) -> 1 < 3 (depth 1) -> empty slot at depth 2.
        assert_eq!(
            events,
            vec![
                Event::Probe { i: 0 },
                Event::Probe { i: 1 },
                Event::Insert { i: 2, value: 1 },
            ]
        );
    }

    #[test]
    fn tracing_enabled_captures_the_exact_descent_for_contains() {
        let tree = Bst::from_values(&[5, 3, 8, 1]);

        enable();
        let found = tree.contains(1);
        let events = take_events();
        disable();

        assert!(found);
        // 1 < 5 (depth 0) -> 1 < 3 (depth 1) -> 1 == 1 (depth 2).
        assert_eq!(
            events,
            vec![
                Event::Probe { i: 0 },
                Event::Probe { i: 1 },
                Event::Probe { i: 2 },
                Event::Found { i: 2 },
            ]
        );
    }

    #[test]
    fn tracing_emits_no_found_event_when_the_value_is_missing() {
        let tree = Bst::from_values(&[5, 3, 8]);

        enable();
        let found = tree.contains(99);
        let events = take_events();
        disable();

        assert!(!found);
        assert!(!events.iter().any(|e| matches!(e, Event::Found { .. })));
    }
}
