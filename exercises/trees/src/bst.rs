//! Shared scaffolding for binary_search_tree/bst_deletion/tree_traversals —
//! a plain (unbalanced) binary search tree. `insert`/`contains` (in
//! binary_search_tree.rs), `delete` (in bst_deletion.rs), and the four
//! traversals (in tree_traversals.rs) are the lesson; `new`/`from_values`/
//! `len`/`is_empty` are always-implemented infrastructure so each
//! exercise's tests can build or read tree state without depending on
//! another exercise's (possibly unsolved) implementation.
//!
//! Values are bare `i32`, not a generic `T: Ord`, matching every other
//! exercise in this project.
//!
//! `tree_contains` is a free function, not a `Bst` method — `contains` is
//! itself the binary_search_tree exercise's own lesson, so scaffolding
//! can't define a method with that name without colliding with the
//! learner's `impl Bst` block. bst_deletion's tests use it to verify a
//! value survived or is gone without depending on binary_search_tree
//! being solved.

pub struct Node {
    pub value: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

#[derive(Default)]
pub struct Bst {
    pub root: Option<Box<Node>>,
}

impl Bst {
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Builds a tree directly from `values`, without going through
    /// insert — so bst_deletion/tree_traversals's tests don't depend on
    /// insert being solved correctly. Inserts in the given order, same as
    /// repeated real inserts would.
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

fn scaffold_insert(node: &mut Option<Box<Node>>, value: i32) {
    match node {
        Some(n) => {
            if value < n.value {
                scaffold_insert(&mut n.left, value);
            } else if value > n.value {
                scaffold_insert(&mut n.right, value);
            }
        }
        None => {
            *node = Some(Box::new(Node {
                value,
                left: None,
                right: None,
            }));
        }
    }
}

fn scaffold_len(node: &Option<Box<Node>>) -> usize {
    match node {
        Some(n) => 1 + scaffold_len(&n.left) + scaffold_len(&n.right),
        None => 0,
    }
}

/// Free function (not a `Bst` method — see module docs) for tests to
/// verify a value is present, independent of the learner's own contains.
pub fn tree_contains(node: &Option<Box<Node>>, target: i32) -> bool {
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
