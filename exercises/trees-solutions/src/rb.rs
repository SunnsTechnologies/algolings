//! Shared scaffolding for red_black_tree/red_black_tree_delete — a
//! left-leaning red-black tree (LLRB) following Sedgewick's formulation,
//! which this project's own tutorial's Rust code closely tracks (the
//! `mut h` parameter naming is straight from that source).
//!
//! `is_red`/`rotate_left`/`rotate_right`/`flip_colors` are given here,
//! fully implemented — mechanical pointer/color surgery, not where
//! either exercise's lesson lives. red_black_tree.rs's lesson is
//! composing them into the three-case insert fix-up (fix a
//! right-leaning red, fix two consecutive left reds, split a 4-node via
//! flip_colors) — written fresh in that exercise, not reused from here.
//!
//! `fix_up`/`move_red_left`/`move_red_right` are ALSO given — deletion
//! calls the exact same three-case fix-up insertion uses (Sedgewick
//! reuses it unchanged), and move_red_left/move_red_right have no
//! case-decision of their own to teach; they always do the same thing.
//! red_black_tree_delete.rs's lesson is `delete`/`delete_min`: knowing
//! WHEN to call move_red_left/move_red_right while descending, so a red
//! link is never lost by deleting through a black node.
//!
//! `flip_colors` here toggles all three colors, matching Sedgewick's
//! canonical `flipColors` (used unchanged by both insert and delete).
//! The tutorial's insertion article shows an absolute-set variant
//! instead (h becomes Red, children become Black) — the two agree at
//! every call site insertion actually uses it (by the time both of a
//! node's children are red, the node itself is provably still Black:
//! a single insert only ever recurses into one child, so the untouched
//! sibling can't have gone red, and if the node itself were red its own
//! parent's earlier fix-up would already have resolved it) — but only
//! the toggle form is also correct for delete's move_red_left/
//! move_red_right, so this module keeps one shared implementation
//! rather than two variants that happen to coincide in one exercise.
//!
//! Values are bare `i32`, matching every other exercise in this
//! project. No tracing: like an AVL rotation, a red-black rotation or
//! color flip restructures/recolors multiple nodes at once, which no
//! existing trace event can represent — this module ships untraced.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

fn flip(color: Color) -> Color {
    match color {
        Color::Red => Color::Black,
        Color::Black => Color::Red,
    }
}

pub struct RbNode {
    pub value: i32,
    pub color: Color,
    pub left: Option<Box<RbNode>>,
    pub right: Option<Box<RbNode>>,
}

impl RbNode {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            color: Color::Red,
            left: None,
            right: None,
        }
    }

    pub fn is_red(node: &Option<Box<RbNode>>) -> bool {
        matches!(node.as_ref().map(|n| n.color), Some(Color::Red))
    }

    pub fn rotate_left(mut h: Box<RbNode>) -> Box<RbNode> {
        let mut x = h.right.take().unwrap();
        h.right = x.left.take();
        x.color = h.color;
        h.color = Color::Red;
        x.left = Some(h);
        x
    }

    pub fn rotate_right(mut h: Box<RbNode>) -> Box<RbNode> {
        let mut x = h.left.take().unwrap();
        h.left = x.right.take();
        x.color = h.color;
        h.color = Color::Red;
        x.right = Some(h);
        x
    }

    pub fn flip_colors(h: &mut Box<RbNode>) {
        h.color = flip(h.color);
        if let Some(ref mut left) = h.left {
            left.color = flip(left.color);
        }
        if let Some(ref mut right) = h.right {
            right.color = flip(right.color);
        }
    }

    /// The same three-case rebalance insertion performs, reused
    /// unchanged at the end of every delete/delete_min step.
    pub fn fix_up(mut h: Box<RbNode>) -> Box<RbNode> {
        if Self::is_red(&h.right) {
            h = Self::rotate_left(h);
        }
        if Self::is_red(&h.left) && Self::is_red(&h.left.as_ref().unwrap().left) {
            h = Self::rotate_right(h);
        }
        if Self::is_red(&h.left) && Self::is_red(&h.right) {
            Self::flip_colors(&mut h);
        }
        h
    }

    /// Borrows a red link from the right sibling so a red-black delete
    /// can safely descend into `h.left`, which is about to become a
    /// black 2-node otherwise.
    pub fn move_red_left(mut h: Box<RbNode>) -> Box<RbNode> {
        Self::flip_colors(&mut h);
        if Self::is_red(&h.right.as_ref().unwrap().left) {
            h.right = Some(Self::rotate_right(h.right.take().unwrap()));
            h = Self::rotate_left(h);
            Self::flip_colors(&mut h);
        }
        h
    }

    /// Mirror image of `move_red_left`, for descending into `h.right`.
    pub fn move_red_right(mut h: Box<RbNode>) -> Box<RbNode> {
        Self::flip_colors(&mut h);
        if Self::is_red(&h.left.as_ref().unwrap().left) {
            h = Self::rotate_right(h);
            Self::flip_colors(&mut h);
        }
        h
    }

    pub fn min_value(node: &RbNode) -> i32 {
        match node.left {
            Some(ref left) => Self::min_value(left),
            None => node.value,
        }
    }
}

#[derive(Default)]
pub struct RbTree {
    pub root: Option<Box<RbNode>>,
}

impl RbTree {
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Builds a tree directly from `values` using a private, fully
    /// correct LLRB insert — without going through the learner's own
    /// insert, so red_black_tree_delete's tests don't depend on
    /// red_black_tree being solved correctly.
    pub fn from_values(values: &[i32]) -> Self {
        let mut tree = Self::new();
        for &value in values {
            tree.root = Some(scaffold_insert(tree.root.take(), value));
            if let Some(ref mut root) = tree.root {
                root.color = Color::Black;
            }
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

fn scaffold_insert(node: Option<Box<RbNode>>, value: i32) -> Box<RbNode> {
    let mut node = match node {
        None => return Box::new(RbNode::new(value)),
        Some(node) => node,
    };

    if value < node.value {
        node.left = Some(scaffold_insert(node.left.take(), value));
    } else if value > node.value {
        node.right = Some(scaffold_insert(node.right.take(), value));
    }

    if RbNode::is_red(&node.right) && !RbNode::is_red(&node.left) {
        node = RbNode::rotate_left(node);
    }
    if RbNode::is_red(&node.left) && RbNode::is_red(&node.left.as_ref().unwrap().left) {
        node = RbNode::rotate_right(node);
    }
    if RbNode::is_red(&node.left) && RbNode::is_red(&node.right) {
        RbNode::flip_colors(&mut node);
    }
    node
}

fn scaffold_len(node: &Option<Box<RbNode>>) -> usize {
    match node {
        Some(n) => 1 + scaffold_len(&n.left) + scaffold_len(&n.right),
        None => 0,
    }
}

pub fn tree_contains(node: &Option<Box<RbNode>>, target: i32) -> bool {
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

pub fn is_bst(node: &Option<Box<RbNode>>, min: Option<i32>, max: Option<i32>) -> bool {
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

/// No red node may have a red child — one of the two core red-black
/// invariants.
pub fn no_red_red(node: &Option<Box<RbNode>>) -> bool {
    match node {
        Some(n) => {
            if n.color == Color::Red && (RbNode::is_red(&n.left) || RbNode::is_red(&n.right)) {
                return false;
            }
            no_red_red(&n.left) && no_red_red(&n.right)
        }
        None => true,
    }
}

/// Every root-to-null-leaf path must cross the same number of black
/// links — the other core red-black invariant.
pub fn is_black_balanced(node: &Option<Box<RbNode>>) -> bool {
    black_height(node).is_some()
}

fn black_height(node: &Option<Box<RbNode>>) -> Option<usize> {
    match node {
        None => Some(0),
        Some(n) => {
            let left = black_height(&n.left)?;
            let right = black_height(&n.right)?;
            if left != right {
                return None;
            }
            Some(left + usize::from(n.color == Color::Black))
        }
    }
}
