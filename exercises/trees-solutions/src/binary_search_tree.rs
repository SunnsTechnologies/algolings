use crate::bst::{tree_contains, Bst, Node};
use algolings_trace::{found, mark_inserted, mark_visited};

/// Reference solution for the `binary_search_tree` exercise.
impl Bst {
    pub fn insert(&mut self, value: i32) {
        insert_at(&mut self.root, value, 0);
    }

    pub fn contains(&self, value: i32) -> bool {
        contains_at(&self.root, value, 0)
    }
}

fn insert_at(node: &mut Option<Box<Node>>, value: i32, depth: usize) {
    match node {
        Some(n) => {
            mark_visited(depth);
            if value < n.value {
                insert_at(&mut n.left, value, depth + 1);
            } else if value > n.value {
                insert_at(&mut n.right, value, depth + 1);
            }
            // Equal: duplicate, ignored.
        }
        None => {
            mark_inserted(depth, value);
            *node = Some(Box::new(Node {
                value,
                left: None,
                right: None,
            }));
        }
    }
}

fn contains_at(node: &Option<Box<Node>>, value: i32, depth: usize) -> bool {
    match node {
        Some(n) => {
            mark_visited(depth);
            if value == n.value {
                found(depth);
                true
            } else if value < n.value {
                contains_at(&n.left, value, depth + 1)
            } else {
                contains_at(&n.right, value, depth + 1)
            }
        }
        None => false,
    }
}

#[cfg(test)]
include!("../../tests-shared/binary_search_tree_tests.rs");
