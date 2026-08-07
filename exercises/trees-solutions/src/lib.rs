mod bst;
mod binary_search_tree;
mod bst_deletion;
mod tree_traversals;
mod binary_tree;
mod binary_tree_insert;
mod min_heap;
mod heap;
mod avl;
mod self_balancing_bst;
mod self_balancing_bst_delete;
mod rb;
mod red_black_tree;
mod red_black_tree_delete;

pub use bst::{Bst, Node};
pub use tree_traversals::{inorder, level_order, postorder, preorder};
pub use binary_tree::BinaryTree;
pub use min_heap::MinHeap;
pub use avl::AvlTree;
pub use rb::RbTree;

#[cfg(test)]
mod sync_tests {
    /// Regression guard: scaffolding files are hand-duplicated between
    /// this crate and exercises/trees (there's no shared dependency
    /// between skeleton and solution crates), with nothing else enforcing
    /// they stay in sync. Pure scaffolding — no todo!()s — so
    /// byte-identical is exactly what "in sync" means.
    #[test]
    fn bst_rs_matches_the_skeleton_crate() {
        let solutions = include_str!("bst.rs");
        let skeleton = include_str!("../../trees/src/bst.rs");
        assert_eq!(
            solutions, skeleton,
            "bst.rs has diverged between trees-solutions and trees"
        );
    }

    #[test]
    fn binary_tree_rs_matches_the_skeleton_crate() {
        let solutions = include_str!("binary_tree.rs");
        let skeleton = include_str!("../../trees/src/binary_tree.rs");
        assert_eq!(
            solutions, skeleton,
            "binary_tree.rs has diverged between trees-solutions and trees"
        );
    }

    #[test]
    fn min_heap_rs_matches_the_skeleton_crate() {
        let solutions = include_str!("min_heap.rs");
        let skeleton = include_str!("../../trees/src/min_heap.rs");
        assert_eq!(
            solutions, skeleton,
            "min_heap.rs has diverged between trees-solutions and trees"
        );
    }

    #[test]
    fn avl_rs_matches_the_skeleton_crate() {
        let solutions = include_str!("avl.rs");
        let skeleton = include_str!("../../trees/src/avl.rs");
        assert_eq!(
            solutions, skeleton,
            "avl.rs has diverged between trees-solutions and trees"
        );
    }

    #[test]
    fn rb_rs_matches_the_skeleton_crate() {
        let solutions = include_str!("rb.rs");
        let skeleton = include_str!("../../trees/src/rb.rs");
        assert_eq!(
            solutions, skeleton,
            "rb.rs has diverged between trees-solutions and trees"
        );
    }
}
