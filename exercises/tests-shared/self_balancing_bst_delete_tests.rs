// Shared between exercises/trees/src/self_balancing_bst_delete.rs (the
// learner-facing skeleton) and
// exercises/trees-solutions/src/self_balancing_bst_delete.rs (the
// reference solution), via `include!()`.
//
// Untraced, same reasoning as self_balancing_bst. Deletion's rebalancing
// conditions are genuinely different from insertion's (a rotation can be
// required even when the relevant child's OWN balance factor is already
// 0, which insertion never triggers) — is_balanced/is_bst get checked
// after every delete in a sequence, not just once at the end, to catch a
// rebalance that only breaks on a LATER operation.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deleting_a_leaf_removes_it() {
        let mut tree = AvlTree::from_values(&[10, 5, 15]);
        tree.delete(5);
        assert!(!tree_contains(&tree.root, 5));
        assert!(tree_contains(&tree.root, 10));
        assert!(tree_contains(&tree.root, 15));
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn deleting_a_node_with_two_children_keeps_both_subtrees_intact() {
        let mut tree = AvlTree::from_values(&[10, 5, 15]);
        tree.delete(10);
        assert!(!tree_contains(&tree.root, 10));
        assert!(tree_contains(&tree.root, 5));
        assert!(tree_contains(&tree.root, 15));
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn deleting_a_missing_value_does_not_change_the_tree() {
        let mut tree = AvlTree::from_values(&[10, 5, 15]);
        tree.delete(99);
        assert_eq!(tree.len(), 3);
    }

    #[test]
    fn deleting_the_only_node_empties_the_tree() {
        let mut tree = AvlTree::from_values(&[10]);
        tree.delete(10);
        assert!(tree.is_empty());
    }

    #[test]
    fn deletion_never_produces_an_unbalanced_node() {
        let values: Vec<i32> = (1..=20).collect();
        let mut tree = AvlTree::from_values(&values);
        for value in 1..=20 {
            tree.delete(value);
            assert!(
                is_balanced(&tree.root),
                "balance factor exceeded [-1, 1] after deleting {value}"
            );
            assert!(is_bst(&tree.root, None, None), "BST ordering broke after deleting {value}");
        }
        assert!(tree.is_empty());
    }

    #[test]
    fn deletion_preserves_every_remaining_value() {
        let mut tree = AvlTree::from_values(&[50, 30, 70, 20, 40, 60, 80, 10]);
        tree.delete(30);
        for value in [50, 70, 20, 40, 60, 80, 10] {
            assert!(tree_contains(&tree.root, value), "{value} should survive");
        }
        assert!(!tree_contains(&tree.root, 30));
        assert!(is_balanced(&tree.root));
        assert!(is_bst(&tree.root, None, None));
    }

    #[test]
    fn deleting_causes_a_rebalancing_rotation() {
        // Building a left-heavy shape and deleting from the right forces
        // a rotation during deletion, not just insertion.
        let mut tree = AvlTree::from_values(&[30, 20, 40, 10, 25, 35, 45, 5]);
        let height_before = tree.height();
        tree.delete(45);
        tree.delete(40);
        tree.delete(35);
        assert!(is_balanced(&tree.root));
        assert!(is_bst(&tree.root, None, None));
        assert!(tree.height() <= height_before);
    }
}
