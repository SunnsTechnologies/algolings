use crate::list::SinglyLinkedList;
use algolings_trace::{found, mark_visited};

// Implement contains: walk the list checking each value against `target`.
//
// Use `.as_deref()` to go from `Option<Box<Node>>` to `Option<&Node>` and
// walk with shared references, not ownership. Call `mark_visited(i)` for
// every position you check, and `found(i)` right before returning true.
impl SinglyLinkedList {
    pub fn contains(&self, target: i32) -> bool {
        todo!("implement contains using mark_visited and found")
    }
}

#[cfg(test)]
include!("../../tests-shared/traverse_tests.rs");
