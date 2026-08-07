use crate::binary_tree::BinaryTree;
use crate::bst::Node;
use algolings_trace::{mark_inserted, mark_visited};
use std::collections::VecDeque;

// A plain binary tree has no ordering rule, so there's nothing to compare
// against — insert always places the new value in the first empty
// left/right slot found by breadth-first search from the root, one level
// at a time.
//
// There's no flat array here either, so trace the level-order SLOT INDEX
// instead of an array position: the root is index 0, and a node at index
// i has children at 2i+1 (left) and 2i+2 (right) — the same arithmetic
// heap.rs uses for its flat Vec, just computed by walking Box pointers
// with a queue instead of indexing a Vec directly. Call mark_visited(index)
// at every already-occupied node you dequeue, and mark_inserted(index,
// value) at the first empty slot you find.
impl BinaryTree {
    pub fn insert(&mut self, value: i32) {
        todo!(
            "BFS from the root with a queue of (node, level-order index) pairs; \
             mark_visited(index) at each occupied node, mark_inserted(index, value) \
             at the first empty left/right child slot"
        )
    }
}

#[cfg(test)]
include!("../../tests-shared/binary_tree_insert_tests.rs");
