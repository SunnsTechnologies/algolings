// Shared between exercises/trees/src/binary_tree_insert.rs (the
// learner-facing skeleton) and
// exercises/trees-solutions/src/binary_tree_insert.rs (the reference
// solution), via `include!()`.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_tree::level_order_values;
    use algolings_trace::{disable, enable, take_events, Event};

    #[test]
    fn insert_into_an_empty_tree_places_the_root() {
        let mut tree = BinaryTree::new();
        tree.insert(5);
        assert_eq!(level_order_values(&tree.root), vec![5]);
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn repeated_inserts_fill_slots_in_level_order() {
        let mut tree = BinaryTree::new();
        for value in [1, 2, 3, 4, 5] {
            tree.insert(value);
        }
        // Level-order placement fills every slot in the same order values
        // were inserted, so the two sequences always match exactly.
        assert_eq!(level_order_values(&tree.root), vec![1, 2, 3, 4, 5]);
        assert_eq!(tree.len(), 5);
    }

    #[test]
    fn insert_ignores_value_ordering_unlike_a_bst() {
        let mut tree = BinaryTree::new();
        for value in [1, 2, 3] {
            tree.insert(value);
        }
        // A BST would chain 2 and 3 both into the right subtree (both
        // greater than 1). A plain binary tree has no ordering rule at
        // all: 2 fills the root's LEFT slot simply because it's empty and
        // comes first in level order.
        let root = tree.root.as_ref().unwrap();
        assert_eq!(root.value, 1);
        assert_eq!(root.left.as_ref().unwrap().value, 2);
        assert_eq!(root.right.as_ref().unwrap().value, 3);
    }

    #[test]
    fn insert_fills_a_missing_right_slot_before_descending_further() {
        let mut tree = BinaryTree::from_values(&[1, 2]);
        tree.insert(3);
        // Root already has a left child (2) but no right child, so 3
        // belongs at the root's right slot, not one level deeper.
        let root = tree.root.as_ref().unwrap();
        assert_eq!(root.right.as_ref().unwrap().value, 3);
        assert!(root.left.as_ref().unwrap().left.is_none());
    }

    #[test]
    fn from_values_matches_repeated_inserts() {
        let tree = BinaryTree::from_values(&[1, 2, 3, 4, 5]);
        assert_eq!(level_order_values(&tree.root), vec![1, 2, 3, 4, 5]);
        assert_eq!(tree.len(), 5);
    }

    #[test]
    fn is_empty_reflects_tree_state() {
        let mut tree = BinaryTree::new();
        assert!(tree.is_empty());
        tree.insert(1);
        assert!(!tree.is_empty());
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let mut a = BinaryTree::new();
        for value in [1, 2, 3] {
            a.insert(value);
        }
        enable();
        let mut b = BinaryTree::new();
        for value in [1, 2, 3] {
            b.insert(value);
        }
        disable();
        assert_eq!(level_order_values(&a.root), level_order_values(&b.root));
    }

    #[test]
    fn tracing_the_first_insert_emits_only_an_insert_event() {
        let mut tree = BinaryTree::new();

        enable();
        tree.insert(5);
        let events = take_events();
        disable();

        assert_eq!(events, vec![Event::Insert { i: 0, value: 5 }]);
    }

    #[test]
    fn tracing_captures_a_root_right_slot_fill() {
        let mut tree = BinaryTree::from_values(&[1, 2]);

        enable();
        tree.insert(3);
        let events = take_events();
        disable();

        // Dequeue root (index 0): its left slot is occupied (enqueue
        // left), its right slot is empty -> insert there directly.
        assert_eq!(
            events,
            vec![Event::Probe { i: 0 }, Event::Insert { i: 2, value: 3 }]
        );
    }

    #[test]
    fn tracing_captures_a_second_level_insert() {
        let mut tree = BinaryTree::from_values(&[1, 2, 3]);

        enable();
        tree.insert(4);
        let events = take_events();
        disable();

        // Dequeue root (index 0, both slots full) -> dequeue its left
        // child (index 1) -> left slot empty -> insert at index 3.
        assert_eq!(
            events,
            vec![
                Event::Probe { i: 0 },
                Event::Probe { i: 1 },
                Event::Insert { i: 3, value: 4 },
            ]
        );
    }
}
