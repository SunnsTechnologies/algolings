// Shared between exercises/trees/src/tree_traversals.rs (the
// learner-facing skeleton) and
// exercises/trees-solutions/src/tree_traversals.rs (the reference
// solution), via `include!()`.
#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    #[test]
    fn inorder_visits_in_sorted_order() {
        let tree = Bst::from_values(&[5, 3, 8]);
        let mut result = Vec::new();
        inorder(&tree.root, &mut result);
        assert_eq!(result, vec![3, 5, 8]);
    }

    #[test]
    fn preorder_visits_root_first() {
        let tree = Bst::from_values(&[5, 3, 8]);
        let mut result = Vec::new();
        preorder(&tree.root, &mut result);
        assert_eq!(result, vec![5, 3, 8]);
    }

    #[test]
    fn postorder_visits_root_last() {
        let tree = Bst::from_values(&[5, 3, 8]);
        let mut result = Vec::new();
        postorder(&tree.root, &mut result);
        assert_eq!(result, vec![3, 8, 5]);
    }

    #[test]
    fn level_order_visits_by_depth() {
        let tree = Bst::from_values(&[5, 3, 8]);
        assert_eq!(level_order(&tree.root), vec![5, 3, 8]);
    }

    #[test]
    fn empty_tree_produces_empty_traversals() {
        let tree = Bst::new();
        let mut result = Vec::new();
        inorder(&tree.root, &mut result);
        assert!(result.is_empty());
        assert!(level_order(&tree.root).is_empty());
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let tree = Bst::from_values(&[5, 3, 8]);
        let mut a = Vec::new();
        inorder(&tree.root, &mut a);
        enable();
        let mut b = Vec::new();
        inorder(&tree.root, &mut b);
        disable();
        assert_eq!(a, b);
    }

    #[test]
    fn tracing_enabled_captures_inorder_as_it_appends() {
        let tree = Bst::from_values(&[5, 3, 8]);

        enable();
        let mut result = Vec::new();
        inorder(&tree.root, &mut result);
        let events = take_events();
        disable();

        assert_eq!(
            events,
            vec![
                Event::Insert { i: 0, value: 3 },
                Event::Insert { i: 1, value: 5 },
                Event::Insert { i: 2, value: 8 },
            ]
        );
    }

    #[test]
    fn tracing_enabled_captures_preorder_as_it_appends() {
        let tree = Bst::from_values(&[5, 3, 8]);

        enable();
        let mut result = Vec::new();
        preorder(&tree.root, &mut result);
        let events = take_events();
        disable();

        assert_eq!(
            events,
            vec![
                Event::Insert { i: 0, value: 5 },
                Event::Insert { i: 1, value: 3 },
                Event::Insert { i: 2, value: 8 },
            ]
        );
    }

    #[test]
    fn tracing_enabled_captures_postorder_as_it_appends() {
        let tree = Bst::from_values(&[5, 3, 8]);

        enable();
        let mut result = Vec::new();
        postorder(&tree.root, &mut result);
        let events = take_events();
        disable();

        assert_eq!(
            events,
            vec![
                Event::Insert { i: 0, value: 3 },
                Event::Insert { i: 1, value: 8 },
                Event::Insert { i: 2, value: 5 },
            ]
        );
    }

    #[test]
    fn tracing_enabled_captures_level_order_as_it_appends() {
        let tree = Bst::from_values(&[5, 3, 8]);

        enable();
        let result = level_order(&tree.root);
        let events = take_events();
        disable();

        assert_eq!(result, vec![5, 3, 8]);
        assert_eq!(
            events,
            vec![
                Event::Insert { i: 0, value: 5 },
                Event::Insert { i: 1, value: 3 },
                Event::Insert { i: 2, value: 8 },
            ]
        );
    }
}
