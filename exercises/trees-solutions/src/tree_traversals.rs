use crate::bst::{Bst, Node};
use algolings_trace::mark_inserted;
use std::collections::VecDeque;

/// Reference solution for the `tree_traversals` exercise.
pub fn inorder(node: &Option<Box<Node>>, result: &mut Vec<i32>) {
    if let Some(n) = node {
        inorder(&n.left, result);
        mark_inserted(result.len(), n.value);
        result.push(n.value);
        inorder(&n.right, result);
    }
}

pub fn preorder(node: &Option<Box<Node>>, result: &mut Vec<i32>) {
    if let Some(n) = node {
        mark_inserted(result.len(), n.value);
        result.push(n.value);
        preorder(&n.left, result);
        preorder(&n.right, result);
    }
}

pub fn postorder(node: &Option<Box<Node>>, result: &mut Vec<i32>) {
    if let Some(n) = node {
        postorder(&n.left, result);
        postorder(&n.right, result);
        mark_inserted(result.len(), n.value);
        result.push(n.value);
    }
}

pub fn level_order(root: &Option<Box<Node>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut queue = VecDeque::new();

    if let Some(node) = root {
        queue.push_back(node);
    }

    while let Some(node) = queue.pop_front() {
        mark_inserted(result.len(), node.value);
        result.push(node.value);

        if let Some(ref left) = node.left {
            queue.push_back(left);
        }
        if let Some(ref right) = node.right {
            queue.push_back(right);
        }
    }

    result
}

#[cfg(test)]
include!("../../tests-shared/tree_traversals_tests.rs");
