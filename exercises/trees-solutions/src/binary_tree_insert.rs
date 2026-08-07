use crate::binary_tree::BinaryTree;
use crate::bst::Node;
use algolings_trace::{mark_inserted, mark_visited};
use std::collections::VecDeque;

/// Reference solution for the `binary_tree_insert` exercise.
impl BinaryTree {
    pub fn insert(&mut self, value: i32) {
        if self.root.is_none() {
            mark_inserted(0, value);
            self.root = Some(Box::new(Node {
                value,
                left: None,
                right: None,
            }));
            return;
        }
        let mut queue: VecDeque<(&mut Box<Node>, usize)> = VecDeque::new();
        queue.push_back((self.root.as_mut().unwrap(), 0));
        while let Some((node, index)) = queue.pop_front() {
            mark_visited(index);
            if node.left.is_some() {
                let left_index = 2 * index + 1;
                queue.push_back((node.left.as_mut().unwrap(), left_index));
            } else {
                mark_inserted(2 * index + 1, value);
                node.left = Some(Box::new(Node {
                    value,
                    left: None,
                    right: None,
                }));
                return;
            }
            if node.right.is_some() {
                let right_index = 2 * index + 2;
                queue.push_back((node.right.as_mut().unwrap(), right_index));
            } else {
                mark_inserted(2 * index + 2, value);
                node.right = Some(Box::new(Node {
                    value,
                    left: None,
                    right: None,
                }));
                return;
            }
        }
    }
}

#[cfg(test)]
include!("../../tests-shared/binary_tree_insert_tests.rs");
