// Shared between exercises/trees/src/red_black_tree.rs (the
// learner-facing skeleton) and
// exercises/trees-solutions/src/red_black_tree.rs (the reference
// solution), via `include!()`.
//
// Untraced, same reasoning as self_balancing_bst: a rotation or color
// flip restructures/recolors multiple nodes at once, which no existing
// trace event can represent. Structural invariants (is_bst, no_red_red,
// is_black_balanced) get checked after every insert in a sequence, not
// just once at the end, so a violation introduced partway through can't
// hide behind a later insert that happens to look fine on its own.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_into_an_empty_tree_places_the_root() {
        let mut tree = RbTree::new();
        tree.insert(10);
        assert_eq!(tree.len(), 1);
        assert!(tree_contains(&tree.root, 10));
    }

    #[test]
    fn root_is_always_black_after_insert() {
        let mut tree = RbTree::new();
        for value in [10, 20, 30, 5, 15, 25, 35] {
            tree.insert(value);
            assert_eq!(tree.root.as_ref().unwrap().color, Color::Black);
        }
    }

    #[test]
    fn insert_maintains_the_bst_property() {
        let mut tree = RbTree::new();
        for value in [50, 30, 70, 20, 40, 60, 80, 10, 90, 25] {
            tree.insert(value);
            assert!(is_bst(&tree.root, None, None), "BST ordering broke inserting {value}");
        }
    }

    #[test]
    fn insert_never_produces_two_consecutive_red_links() {
        let mut tree = RbTree::new();
        for value in 1..=30 {
            tree.insert(value);
            assert!(no_red_red(&tree.root), "red-red violation inserting {value}");
        }
    }

    #[test]
    fn insert_keeps_every_path_the_same_black_height() {
        let mut tree = RbTree::new();
        for value in [7, 3, 18, 10, 22, 8, 11, 26, 2, 6, 13] {
            tree.insert(value);
            assert!(is_black_balanced(&tree.root), "black-height mismatch inserting {value}");
        }
    }

    #[test]
    fn ascending_insertion_stays_logarithmic_not_a_linked_list() {
        // A plain unbalanced BST degenerates to a linked list on
        // ascending input; a red-black tree must not.
        let mut tree = RbTree::new();
        for value in 1..=200 {
            tree.insert(value);
        }
        assert!(is_bst(&tree.root, None, None));
        assert!(no_red_red(&tree.root));
        assert!(is_black_balanced(&tree.root));
        assert_eq!(tree.len(), 200);
    }

    #[test]
    fn inserting_a_duplicate_does_not_grow_the_tree() {
        let mut tree = RbTree::new();
        tree.insert(10);
        tree.insert(10);
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn insert_preserves_every_previously_inserted_value() {
        let mut tree = RbTree::new();
        let values = [50, 30, 70, 20, 40, 60, 80, 10, 90, 25, 35, 45];
        for &value in &values {
            tree.insert(value);
        }
        for &value in &values {
            assert!(tree_contains(&tree.root, value), "{value} should be findable");
        }
        assert_eq!(tree.len(), values.len());
    }

    #[test]
    fn descending_insertion_also_stays_balanced() {
        let mut tree = RbTree::new();
        for value in (1..=100).rev() {
            tree.insert(value);
        }
        assert!(is_bst(&tree.root, None, None));
        assert!(no_red_red(&tree.root));
        assert!(is_black_balanced(&tree.root));
    }
}
