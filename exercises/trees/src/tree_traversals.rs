use crate::bst::{Bst, Node};
use algolings_trace::mark_inserted;
use std::collections::VecDeque;

// Implement the four classic tree traversals. Each one visits every node
// exactly once, in a different order, and collects values into `result`
// as it goes — trace that append the same way insert.rs did:
// mark_inserted(result.len(), value) right before pushing.
//
// inorder:  left, then this node, then right  (sorted order, for a BST)
// preorder: this node, then left, then right
// postorder: left, then right, then this node
// level_order: breadth-first, one level at a time (needs a VecDeque queue,
//              not recursion)
pub fn inorder(node: &Option<Box<Node>>, result: &mut Vec<i32>) {
    todo!("left, then this node (mark_inserted before pushing), then right")
}

pub fn preorder(node: &Option<Box<Node>>, result: &mut Vec<i32>) {
    todo!("this node (mark_inserted before pushing), then left, then right")
}

pub fn postorder(node: &Option<Box<Node>>, result: &mut Vec<i32>) {
    todo!("left, then right, then this node (mark_inserted before pushing)")
}

pub fn level_order(root: &Option<Box<Node>>) -> Vec<i32> {
    todo!("breadth-first with a VecDeque queue, mark_inserted before each push")
}

#[cfg(test)]
include!("../../tests-shared/tree_traversals_tests.rs");
