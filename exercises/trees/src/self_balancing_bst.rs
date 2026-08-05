use crate::avl::{is_balanced, is_bst, tree_contains, AvlNode, AvlTree};

// Implement insert for an AVL tree — a self-balancing BST that keeps
// every node's balance factor (left subtree height minus right subtree
// height) within [-1, 1] by rotating after each insertion. rotate_left/
// rotate_right/update_height/balance_factor are already given (in
// avl.rs) — the lesson here is recognizing WHICH of the four cases
// applies after a normal BST-style insert:
//
// LL (balance > 1, new value went into the left child's LEFT subtree):
//   rotate_right(node)
// RR (balance < -1, new value went into the right child's RIGHT subtree):
//   rotate_left(node)
// LR (balance > 1, new value went into the left child's RIGHT subtree):
//   rotate_left the left child first, THEN rotate_right(node)
// RL (balance < -1, new value went into the right child's LEFT subtree):
//   rotate_right the right child first, THEN rotate_left(node)
//
// After the normal BST descent (recursing left or right, or creating a
// new leaf), call update_height() and check balance_factor() at every
// level on the way back up — a rotation at a low level can change
// whether a rotation is needed higher up.
impl AvlTree {
    pub fn insert(&mut self, value: i32) {
        todo!("descend like a normal BST insert, then rebalance on the way back up")
    }
}

#[cfg(test)]
include!("../../tests-shared/self_balancing_bst_tests.rs");
