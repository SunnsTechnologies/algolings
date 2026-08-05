// Shared between exercises/trees/src/bst_deletion.rs (the learner-facing
// skeleton) and exercises/trees-solutions/src/bst_deletion.rs (the
// reference solution), via `include!()`.
//
// The two-children case is the one that actually matters here: naively
// restarting the depth counter for the in-order-successor search would
// collide with an unrelated sibling sharing that same depth, and never
// showing the target's value actually changing to the successor's value
// would misrepresent what happened. Both are covered by dedicated
// exact-sequence tests below, hand-verified against the real algorithm
// before this file was written.
#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    #[test]
    fn deleting_a_leaf_removes_it() {
        let mut tree = Bst::from_values(&[10, 5, 15]);
        tree.delete(5);
        assert!(!tree_contains(&tree.root, 5));
        assert!(tree_contains(&tree.root, 10));
        assert!(tree_contains(&tree.root, 15));
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn deleting_a_node_with_one_child_splices_the_child_in() {
        let mut tree = Bst::from_values(&[10, 5, 3]);
        tree.delete(5);
        assert!(!tree_contains(&tree.root, 5));
        assert!(tree_contains(&tree.root, 3));
        assert!(tree_contains(&tree.root, 10));
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn deleting_a_node_with_two_children_keeps_both_subtrees_intact() {
        let mut tree = Bst::from_values(&[10, 5, 15]);
        tree.delete(10);
        assert!(!tree_contains(&tree.root, 10));
        assert!(tree_contains(&tree.root, 5), "the untouched sibling must survive");
        assert!(tree_contains(&tree.root, 15), "the successor must still be findable");
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn deleting_a_missing_value_does_not_change_the_tree() {
        let mut tree = Bst::from_values(&[10, 5, 15]);
        tree.delete(99);
        assert_eq!(tree.len(), 3);
        assert!(tree_contains(&tree.root, 10));
        assert!(tree_contains(&tree.root, 5));
        assert!(tree_contains(&tree.root, 15));
    }

    #[test]
    fn deleting_the_only_node_empties_the_tree() {
        let mut tree = Bst::from_values(&[10]);
        tree.delete(10);
        assert!(tree.is_empty());
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let mut a = Bst::from_values(&[10, 5, 15]);
        a.delete(5);
        enable();
        let mut b = Bst::from_values(&[10, 5, 15]);
        b.delete(5);
        disable();
        assert_eq!(tree_contains(&a.root, 5), tree_contains(&b.root, 5));
    }

    #[test]
    fn tracing_enabled_captures_the_exact_sequence_for_a_leaf() {
        let mut tree = Bst::from_values(&[10, 5, 15]);

        enable();
        tree.delete(5);
        let events = take_events();
        disable();

        assert_eq!(
            events,
            vec![Event::Probe { i: 0 }, Event::Probe { i: 1 }, Event::Remove { i: 1 }]
        );
    }

    #[test]
    fn tracing_enabled_captures_the_exact_sequence_for_one_child() {
        let mut tree = Bst::from_values(&[10, 5, 3]);

        enable();
        tree.delete(5);
        let events = take_events();
        disable();

        assert_eq!(
            events,
            vec![Event::Probe { i: 0 }, Event::Probe { i: 1 }, Event::Remove { i: 1 }]
        );
    }

    #[test]
    fn tracing_enabled_captures_the_exact_sequence_for_two_children() {
        // root=10, left=5, right=15. Deleting the root's value gets
        // overwritten by the in-order successor (15, the leftmost node
        // of the right subtree), and 15's own node gets spliced out —
        // NOT 5, which is an unrelated sibling that happens to share the
        // same depth (1) as the successor.
        let mut tree = Bst::from_values(&[10, 5, 15]);

        enable();
        tree.delete(10);
        let events = take_events();
        disable();

        assert_eq!(
            events,
            vec![
                Event::Probe { i: 0 },
                Event::Probe { i: 1 },
                Event::Remove { i: 1 },
                Event::Set { i: 0, value: 15 },
            ]
        );
    }

    #[test]
    fn tracing_emits_no_event_for_deleting_a_missing_value() {
        let mut tree = Bst::from_values(&[10, 5, 15]);

        enable();
        tree.delete(99);
        let events = take_events();
        disable();

        assert!(!events.iter().any(|e| matches!(e, Event::Remove { .. } | Event::Set { .. })));
    }
}
