use crate::list::SinglyLinkedList;
use algolings_trace::{found, mark_visited};

/// Reference solution for the `traverse` exercise.
impl SinglyLinkedList {
    pub fn contains(&self, target: i32) -> bool {
        let mut current = self.head.as_deref();
        let mut i = 0;

        while let Some(node) = current {
            mark_visited(i);
            if node.value == target {
                found(i);
                return true;
            }
            current = node.next.as_deref();
            i += 1;
        }

        false
    }
}

#[cfg(test)]
include!("../../tests-shared/traverse_tests.rs");
