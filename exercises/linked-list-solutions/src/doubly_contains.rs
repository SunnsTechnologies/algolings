use crate::doubly_list::DoublyLinkedList;
use algolings_trace::{found, mark_visited};

/// Reference solution for the `doubly_contains` exercise.
impl DoublyLinkedList {
    pub fn contains(&self, target: i32) -> bool {
        let mut current = self.head.clone();
        let mut i = 0;

        while let Some(node) = current {
            mark_visited(i);
            if node.borrow().value == target {
                found(i);
                return true;
            }
            current = node.borrow().next.clone();
            i += 1;
        }

        false
    }
}

#[cfg(test)]
include!("../../tests-shared/doubly_contains_tests.rs");
