use crate::rb::{is_black_balanced, is_bst, no_red_red, tree_contains, Color, RbNode, RbTree};

// Implement delete for a left-leaning red-black tree (LLRB). This
// mirrors bst_deletion's three BST cases (leaf, one child, two children
// via in-order successor), but a red-black delete has one more concern:
// you can only safely delete through a red link. If the path you're
// about to descend into is about to hit a black 2-node, you have to
// borrow a red link from a sibling FIRST, using the GIVEN
// RbNode::move_red_left/move_red_right — then call RbNode::fix_up on
// the way back up to restore the tree's shape (the same three-case
// rebalance you wrote by hand in red_black_tree.rs, given here as a
// building block instead of retaught).
//
// Guard against deleting a value that isn't present — move_red_left/
// move_red_right assume the search path they're on actually continues,
// and will panic on a value that was never there.
//
// One more step Sedgewick's reference implementation includes that
// this project's own tutorial snippet leaves out: prime the root to
// Red before the first recursive call if both its children are black,
// so the very first move_red_left/move_red_right has a red link to
// work with. Force it back to Black afterward, same as insert.
impl RbTree {
    pub fn delete(&mut self, value: i32) {
        todo!("BST-style delete, borrowing red links via move_red_left/move_red_right before descending into a black 2-node, fix_up on the way back up")
    }
}

#[cfg(test)]
include!("../../tests-shared/red_black_tree_delete_tests.rs");
