// Shared between exercises/trees/src/red_black_tree_delete.rs (the
// learner-facing skeleton) and
// exercises/trees-solutions/src/red_black_tree_delete.rs (the reference
// solution), via `include!()`.
//
// Untraced, same reasoning as red_black_tree. Structural invariants get
// checked after every delete in a sequence, not just once at the end,
// paired with explicit membership/count checks — a bug that silently
// drops or duplicates a subtree can still look like "a valid red-black
// tree" to the invariant checks alone.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deleting_a_leaf_removes_it() {
        let mut tree = RbTree::from_values(&[10, 5, 15]);
        tree.delete(5);
        assert!(!tree_contains(&tree.root, 5));
        assert!(tree_contains(&tree.root, 10));
        assert!(tree_contains(&tree.root, 15));
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn deleting_the_root_keeps_the_remaining_values() {
        let mut tree = RbTree::from_values(&[10, 5, 15]);
        tree.delete(10);
        assert!(!tree_contains(&tree.root, 10));
        assert!(tree_contains(&tree.root, 5));
        assert!(tree_contains(&tree.root, 15));
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn deleting_a_missing_value_does_not_change_the_tree() {
        let mut tree = RbTree::from_values(&[10, 5, 15]);
        tree.delete(99);
        assert_eq!(tree.len(), 3);
        for value in [10, 5, 15] {
            assert!(tree_contains(&tree.root, value));
        }
    }

    #[test]
    fn deleting_from_an_empty_tree_does_nothing() {
        let mut tree = RbTree::new();
        tree.delete(10);
        assert!(tree.is_empty());
    }

    #[test]
    fn deleting_the_only_node_empties_the_tree() {
        let mut tree = RbTree::from_values(&[10]);
        tree.delete(10);
        assert!(tree.is_empty());
    }

    #[test]
    fn root_is_always_black_after_delete() {
        let mut tree = RbTree::from_values(&[10, 5, 15, 3, 7, 12, 18]);
        tree.delete(3);
        assert_eq!(tree.root.as_ref().unwrap().color, Color::Black);
        tree.delete(18);
        assert_eq!(tree.root.as_ref().unwrap().color, Color::Black);
    }

    #[test]
    fn deletion_never_breaks_bst_ordering() {
        let values: Vec<i32> = (1..=30).collect();
        let mut tree = RbTree::from_values(&values);
        for value in 1..=30 {
            tree.delete(value);
            assert!(is_bst(&tree.root, None, None), "BST ordering broke deleting {value}");
        }
        assert!(tree.is_empty());
    }

    #[test]
    fn deletion_never_produces_two_consecutive_red_links() {
        let values: Vec<i32> = (1..=30).collect();
        let mut tree = RbTree::from_values(&values);
        for value in 1..=30 {
            tree.delete(value);
            assert!(no_red_red(&tree.root), "red-red violation deleting {value}");
        }
    }

    #[test]
    fn deletion_never_breaks_black_height_balance() {
        let values: Vec<i32> = (1..=30).collect();
        let mut tree = RbTree::from_values(&values);
        for value in 1..=30 {
            tree.delete(value);
            assert!(is_black_balanced(&tree.root), "black-height mismatch deleting {value}");
        }
    }

    #[test]
    fn deleting_one_value_preserves_every_other_value() {
        let values = [50, 30, 70, 20, 40, 60, 80, 10, 90, 25, 35, 45];
        let mut tree = RbTree::from_values(&values);
        tree.delete(30);
        assert!(!tree_contains(&tree.root, 30));
        for &value in values.iter().filter(|&&v| v != 30) {
            assert!(tree_contains(&tree.root, value), "{value} should survive");
        }
        assert_eq!(tree.len(), values.len() - 1);
    }

    #[test]
    fn deleting_every_value_in_a_random_order_empties_the_tree_cleanly() {
        let values = [8, 3, 15, 1, 6, 20, 4, 7, 13, 17, 25, 22, 27];
        let delete_order = [15, 1, 25, 8, 4, 20, 3, 27, 6, 13, 22, 7, 17];
        let mut tree = RbTree::from_values(&values);
        for &value in &delete_order {
            tree.delete(value);
            assert!(is_bst(&tree.root, None, None));
            assert!(no_red_red(&tree.root));
            assert!(is_black_balanced(&tree.root));
        }
        assert!(tree.is_empty());
    }
}
