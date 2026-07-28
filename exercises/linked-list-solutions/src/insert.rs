use crate::list::{Node, SinglyLinkedList};
use algolings_trace::mark_inserted;

/// Reference solution for the `insert` exercise.
impl SinglyLinkedList {
    pub fn push_front(&mut self, value: i32) {
        mark_inserted(0, value);
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn push_back(&mut self, value: i32) {
        mark_inserted(self.len(), value);
        let new_node = Box::new(Node { value, next: None });

        let mut current = &mut self.head;
        while let Some(node) = current {
            current = &mut node.next;
        }
        *current = Some(new_node);
    }
}

#[cfg(test)]
include!("../../tests-shared/insert_tests.rs");
