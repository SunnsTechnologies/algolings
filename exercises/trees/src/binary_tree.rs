//! Shared scaffolding for binary_tree_insert — a plain (unordered) binary
//! tree. Unlike bst.rs's `Bst`, nothing here compares values: `insert` is
//! the lesson (in binary_tree_insert.rs) and always places a new node in
//! the first empty left/right slot found by breadth-first search, since a
//! plain binary tree has no ordering rule to guide placement.
//!
//! Reuses `crate::bst::Node` as-is — the node SHAPE (value/left/right)
//! isn't what makes a tree a BST; the insertion policy is. `new`/
//! `from_values`/`len`/`is_empty` are always-implemented infrastructure so
//! binary_tree_insert's tests can build starting state without depending
//! on the learner's own (possibly unsolved) insert.

use crate::bst::Node;
use std::collections::VecDeque;

#[derive(Default)]
pub struct BinaryTree {
    pub root: Option<Box<Node>>,
}

impl BinaryTree {
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Builds a tree directly from `values`, without going through
    /// insert — same level-order placement a real insert would produce,
    /// so binary_tree_insert's own tests don't depend on insert being
    /// solved correctly.
    pub fn from_values(values: &[i32]) -> Self {
        let mut tree = Self::new();
        for &value in values {
            scaffold_insert(&mut tree.root, value);
        }
        tree
    }

    pub fn len(&self) -> usize {
        scaffold_len(&self.root)
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}

fn scaffold_insert(root: &mut Option<Box<Node>>, value: i32) {
    if root.is_none() {
        *root = Some(Box::new(Node {
            value,
            left: None,
            right: None,
        }));
        return;
    }
    let mut queue: VecDeque<&mut Box<Node>> = VecDeque::new();
    queue.push_back(root.as_mut().unwrap());
    while let Some(node) = queue.pop_front() {
        if node.left.is_some() {
            queue.push_back(node.left.as_mut().unwrap());
        } else {
            node.left = Some(Box::new(Node {
                value,
                left: None,
                right: None,
            }));
            return;
        }
        if node.right.is_some() {
            queue.push_back(node.right.as_mut().unwrap());
        } else {
            node.right = Some(Box::new(Node {
                value,
                left: None,
                right: None,
            }));
            return;
        }
    }
}

fn scaffold_len(node: &Option<Box<Node>>) -> usize {
    match node {
        Some(n) => 1 + scaffold_len(&n.left) + scaffold_len(&n.right),
        None => 0,
    }
}

/// Free function (not tied to tree_traversals.rs's own lesson) so
/// binary_tree_insert's tests can check shape without depending on
/// another exercise's (possibly unsolved) traversal. Collects values
/// breadth-first, skipping empty slots — for a tree built purely by
/// level-order insertion, this always equals the original insertion
/// order.
pub fn level_order_values(root: &Option<Box<Node>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut queue: VecDeque<&Box<Node>> = VecDeque::new();
    if let Some(node) = root {
        queue.push_back(node);
    }
    while let Some(node) = queue.pop_front() {
        result.push(node.value);
        if let Some(left) = &node.left {
            queue.push_back(left);
        }
        if let Some(right) = &node.right {
            queue.push_back(right);
        }
    }
    result
}
