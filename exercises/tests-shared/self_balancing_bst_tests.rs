// Shared between exercises/trees/src/self_balancing_bst.rs (the
// learner-facing skeleton) and
// exercises/trees-solutions/src/self_balancing_bst.rs (the reference
// solution), via `include!()`.
//
// Untraced — a rotation restructures multiple nodes' pointers at once,
// which no existing trace event can represent. Correctness is verified
// structurally instead: every value present, BST ordering preserved,
// and the balance factor within [-1, 1] at every node after every
// insert.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_into_an_empty_tree_places_the_root() {
        let mut tree = AvlTree::new();
        tree.insert(5);
        assert!(tree_contains(&tree.root, 5));
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn insert_maintains_every_value() {
        let mut tree = AvlTree::new();
        for value in [5, 3, 8, 1, 4, 7, 9] {
            tree.insert(value);
        }
        for value in [5, 3, 8, 1, 4, 7, 9] {
            assert!(tree_contains(&tree.root, value));
        }
        assert_eq!(tree.len(), 7);
    }

    #[test]
    fn insert_never_produces_an_unbalanced_node() {
        let mut tree = AvlTree::new();
        for value in 1..=15 {
            tree.insert(value);
            assert!(
                is_balanced(&tree.root),
                "balance factor exceeded [-1, 1] after inserting {value}"
            );
        }
    }

    #[test]
    fn insert_preserves_bst_ordering_through_rotations() {
        let mut tree = AvlTree::new();
        for value in [5, 3, 8, 1, 4, 7, 9, 2, 6] {
            tree.insert(value);
        }
        assert!(is_bst(&tree.root, None, None));
    }

    #[test]
    fn sequential_insertion_stays_logarithmic_height() {
        // A plain BST inserting 1..=100 in order degrades to a straight
        // line (height 100). AVL's rotations must keep it near log2(100)
        // ~= 7 — this is the entire reason AVL trees exist.
        let mut tree = AvlTree::new();
        for value in 1..=100 {
            tree.insert(value);
        }
        assert!(
            tree.height() <= 10,
            "height {} is too tall for a balanced 100-node tree",
            tree.height()
        );
    }

    #[test]
    fn left_left_case_rotates_right() {
        // Textbook LL: inserting in descending order forces a
        // left-heavy imbalance, fixed by a single right rotation. The
        // middle value ends up as the new root.
        let mut tree = AvlTree::new();
        for value in [30, 20, 10] {
            tree.insert(value);
        }
        assert_eq!(tree.root.as_ref().unwrap().value, 20);
    }

    #[test]
    fn right_right_case_rotates_left() {
        let mut tree = AvlTree::new();
        for value in [10, 20, 30] {
            tree.insert(value);
        }
        assert_eq!(tree.root.as_ref().unwrap().value, 20);
    }

    #[test]
    fn left_right_case_rotates_left_then_right() {
        let mut tree = AvlTree::new();
        for value in [30, 10, 20] {
            tree.insert(value);
        }
        assert_eq!(tree.root.as_ref().unwrap().value, 20);
    }

    #[test]
    fn right_left_case_rotates_right_then_left() {
        let mut tree = AvlTree::new();
        for value in [10, 30, 20] {
            tree.insert(value);
        }
        assert_eq!(tree.root.as_ref().unwrap().value, 20);
    }

    #[test]
    fn inserting_a_duplicate_does_not_grow_the_tree() {
        let mut tree = AvlTree::from_values(&[5, 3, 8]);
        tree.insert(5);
        assert_eq!(tree.len(), 3);
    }
}
