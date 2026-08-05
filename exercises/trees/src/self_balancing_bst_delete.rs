use crate::avl::{is_balanced, is_bst, tree_contains, AvlNode, AvlTree};

// Implement delete for an AVL tree — the same three BST cases as
// bst_deletion (leaf, one child, two children via in-order successor),
// but rebalancing has to happen on the way BACK UP after the structural
// change, same as self_balancing_bst's insert.
//
// The four rebalancing conditions are DIFFERENT from insertion's,
// though — deletion can require a rotation even when the relevant
// child's own balance factor is already 0, which insertion never
// triggers:
//
// LL (balance > 1, left child's balance_factor >= 0): rotate_right(node)
// LR (balance > 1, left child's balance_factor < 0): rotate_left the
//     left child first, then rotate_right(node)
// RR (balance < -1, right child's balance_factor <= 0): rotate_left(node)
// RL (balance < -1, right child's balance_factor > 0): rotate_right the
//     right child first, then rotate_left(node)
//
// Same as bst_deletion: update_height() and check balance_factor() at
// EVERY level on the way back up, not just where the node was removed —
// the imbalance can appear several levels above the actual deletion.
impl AvlTree {
    pub fn delete(&mut self, value: i32) {
        todo!("BST-style delete (leaf/one-child/two-children), rebalancing on the way back up")
    }
}

#[cfg(test)]
include!("../../tests-shared/self_balancing_bst_delete_tests.rs");
