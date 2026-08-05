use crate::bst::{tree_contains, Bst, Node};
use algolings_trace::{mark_removed, mark_set, mark_visited};

/// Reference solution for the `bst_deletion` exercise.
impl Bst {
    pub fn delete(&mut self, value: i32) {
        self.root = delete_node(self.root.take(), value, 0);
    }
}

fn delete_node(node: Option<Box<Node>>, value: i32, depth: usize) -> Option<Box<Node>> {
    let mut n = node?;
    mark_visited(depth);

    if value < n.value {
        n.left = delete_node(n.left.take(), value, depth + 1);
        Some(n)
    } else if value > n.value {
        n.right = delete_node(n.right.take(), value, depth + 1);
        Some(n)
    } else {
        match (n.left.take(), n.right.take()) {
            (None, None) => {
                mark_removed(depth);
                None
            }
            (Some(child), None) | (None, Some(child)) => {
                mark_removed(depth);
                Some(child)
            }
            (Some(left), Some(right)) => {
                let (successor_value, new_right) = extract_min(right, depth + 1);
                mark_set(depth, successor_value);
                n.value = successor_value;
                n.left = Some(left);
                n.right = new_right;
                Some(n)
            }
        }
    }
}

fn extract_min(node: Box<Node>, depth: usize) -> (i32, Option<Box<Node>>) {
    let mut n = node;
    mark_visited(depth);
    match n.left.take() {
        Some(left) => {
            let (min, new_left) = extract_min(left, depth + 1);
            n.left = new_left;
            (min, Some(n))
        }
        None => {
            mark_removed(depth);
            (n.value, n.right.take())
        }
    }
}

#[cfg(test)]
include!("../../tests-shared/bst_deletion_tests.rs");
