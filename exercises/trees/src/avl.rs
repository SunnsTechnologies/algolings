//! Shared scaffolding for self_balancing_bst/self_balancing_bst_delete —
//! an AVL tree, height-balanced after every insert and delete.
//!
//! Rotation mechanics (`rotate_left`/`rotate_right`/`height_of`/
//! `update_height`/`balance_factor`) are given here, fully implemented —
//! they're mechanical pointer surgery once understood, not where either
//! exercise's real lesson lives. The lesson in BOTH exercises is knowing
//! WHICH of the four cases (LL/RR/LR/RL) applies and when to call these:
//! self_balancing_bst.rs for insertion's version of that decision,
//! self_balancing_bst_delete.rs for deletion's (genuinely different
//! conditions — deletion can require a rotation even when a child's own
//! balance factor is already 0, which insertion never triggers).
//!
//! Values are bare `i32`, matching every other exercise in this project.
//! No tracing here at all: unlike a stack push/pop, a rotation
//! restructures multiple nodes' parent/child pointers simultaneously,
//! which no existing trace event (or reasonable approximation) can
//! represent — this whole module ships untraced.

pub struct AvlNode {
    pub value: i32,
    pub height: i32,
    pub left: Option<Box<AvlNode>>,
    pub right: Option<Box<AvlNode>>,
}

impl AvlNode {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            height: 1,
            left: None,
            right: None,
        }
    }

    pub fn height_of(node: &Option<Box<AvlNode>>) -> i32 {
        node.as_ref().map_or(0, |n| n.height)
    }

    pub fn update_height(&mut self) {
        self.height = 1 + std::cmp::max(Self::height_of(&self.left), Self::height_of(&self.right));
    }

    pub fn balance_factor(&self) -> i32 {
        Self::height_of(&self.left) - Self::height_of(&self.right)
    }

    pub fn rotate_right(mut y: Box<AvlNode>) -> Box<AvlNode> {
        let mut x = y.left.take().unwrap();
        y.left = x.right.take();
        y.update_height();
        x.right = Some(y);
        x.update_height();
        x
    }

    pub fn rotate_left(mut x: Box<AvlNode>) -> Box<AvlNode> {
        let mut y = x.right.take().unwrap();
        x.right = y.left.take();
        x.update_height();
        y.left = Some(x);
        y.update_height();
        y
    }
}

#[derive(Default)]
pub struct AvlTree {
    pub root: Option<Box<AvlNode>>,
}

impl AvlTree {
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Builds a tree directly from `values` using a private, fully
    /// correct AVL insert — without going through the learner's own
    /// insert, so self_balancing_bst_delete's tests don't depend on
    /// self_balancing_bst being solved correctly.
    pub fn from_values(values: &[i32]) -> Self {
        let mut tree = Self::new();
        for &value in values {
            tree.root = Some(match tree.root.take() {
                Some(node) => scaffold_insert(node, value),
                None => Box::new(AvlNode::new(value)),
            });
        }
        tree
    }

    pub fn len(&self) -> usize {
        scaffold_len(&self.root)
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// The tree's height, for verifying AVL's balance guarantee stays
    /// O(log n) rather than degrading toward a linked list.
    pub fn height(&self) -> i32 {
        AvlNode::height_of(&self.root)
    }
}

fn scaffold_insert(mut node: Box<AvlNode>, value: i32) -> Box<AvlNode> {
    if value < node.value {
        node.left = Some(match node.left.take() {
            Some(left) => scaffold_insert(left, value),
            None => Box::new(AvlNode::new(value)),
        });
    } else if value > node.value {
        node.right = Some(match node.right.take() {
            Some(right) => scaffold_insert(right, value),
            None => Box::new(AvlNode::new(value)),
        });
    } else {
        return node;
    }

    node.update_height();
    let balance = node.balance_factor();

    if balance > 1 && value < node.left.as_ref().unwrap().value {
        return AvlNode::rotate_right(node);
    }
    if balance < -1 && value > node.right.as_ref().unwrap().value {
        return AvlNode::rotate_left(node);
    }
    if balance > 1 && value > node.left.as_ref().unwrap().value {
        node.left = Some(AvlNode::rotate_left(node.left.take().unwrap()));
        return AvlNode::rotate_right(node);
    }
    if balance < -1 && value < node.right.as_ref().unwrap().value {
        node.right = Some(AvlNode::rotate_right(node.right.take().unwrap()));
        return AvlNode::rotate_left(node);
    }

    node
}

fn scaffold_len(node: &Option<Box<AvlNode>>) -> usize {
    match node {
        Some(n) => 1 + scaffold_len(&n.left) + scaffold_len(&n.right),
        None => 0,
    }
}

/// Verifies every node's balance factor is within [-1, 1] — the AVL
/// invariant tests check after every insert/delete.
pub fn is_balanced(node: &Option<Box<AvlNode>>) -> bool {
    match node {
        Some(n) => n.balance_factor().abs() <= 1 && is_balanced(&n.left) && is_balanced(&n.right),
        None => true,
    }
}

/// Verifies BST ordering still holds (rotations must preserve it).
pub fn is_bst(node: &Option<Box<AvlNode>>, min: Option<i32>, max: Option<i32>) -> bool {
    match node {
        Some(n) => {
            if let Some(min) = min {
                if n.value <= min {
                    return false;
                }
            }
            if let Some(max) = max {
                if n.value >= max {
                    return false;
                }
            }
            is_bst(&n.left, min, Some(n.value)) && is_bst(&n.right, Some(n.value), max)
        }
        None => true,
    }
}

pub fn tree_contains(node: &Option<Box<AvlNode>>, target: i32) -> bool {
    match node {
        Some(n) => {
            if target == n.value {
                true
            } else if target < n.value {
                tree_contains(&n.left, target)
            } else {
                tree_contains(&n.right, target)
            }
        }
        None => false,
    }
}
