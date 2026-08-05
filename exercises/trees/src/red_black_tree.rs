use crate::rb::{is_black_balanced, is_bst, no_red_red, tree_contains, Color, RbNode, RbTree};

// Implement insert for a left-leaning red-black tree (LLRB): BST descent
// to place the new node (always red, per RbNode::new), then on the way
// back up, fix three things in order using the GIVEN is_red/rotate_left/
// rotate_right/flip_colors:
//
// 1. A right-leaning red link (right child red, left child not) —
//    rotate_left to lean it left.
// 2. Two consecutive red links leaning left (left child red AND left
//    child's left child red) — rotate_right to balance them.
// 3. Both children red (a temporary 4-node) — flip_colors to split it
//    and push the red link up to the parent.
//
// This is Sedgewick's `balance` step, done fresh here — the version
// reused by red_black_tree_delete lives in rb.rs as `RbNode::fix_up`,
// but you're writing the same three checks yourself so the logic isn't
// just imported.
impl RbTree {
    pub fn insert(&mut self, value: i32) {
        self.root = Some(insert_at(self.root.take(), value));
        if let Some(ref mut root) = self.root {
            root.color = Color::Black;
        }
    }
}

fn insert_at(node: Option<Box<RbNode>>, value: i32) -> Box<RbNode> {
    todo!("BST descent, then fix a right-leaning red, fix two consecutive left reds, split a 4-node")
}

#[cfg(test)]
include!("../../tests-shared/red_black_tree_tests.rs");
