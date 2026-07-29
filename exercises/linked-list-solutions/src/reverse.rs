use crate::list::{Node, SinglyLinkedList};
use algolings_trace::{mark_inserted, mark_removed};

/// Reference solution for the `reverse` exercise.
impl SinglyLinkedList {
    pub fn reverse(&mut self) {
        let mut remaining_len = self.len();
        let mut prev: Option<Box<Node>> = None;
        let mut current = self.head.take();

        while let Some(mut node) = current {
            let next = node.next.take();
            remaining_len -= 1;
            mark_removed(0);
            mark_inserted(remaining_len, node.value);
            node.next = prev;
            prev = Some(node);
            current = next;
        }

        self.head = prev;
    }
}

#[cfg(test)]
include!("../../tests-shared/reverse_tests.rs");
