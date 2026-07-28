use crate::list::SinglyLinkedList;
use algolings_trace::mark_removed;

// Implement remove: walk the list looking for a node whose value matches
// `target`, and unlink it if found.
//
// Use `current.take()` to move a node out of the list temporarily — you
// can't decide whether to keep it or splice around it through a shared
// reference. Call `mark_removed(i)` right before removing a matching node.
impl SinglyLinkedList {
    pub fn remove(&mut self, target: i32) -> bool {
        todo!("implement remove using mark_removed")
    }
}

#[cfg(test)]
include!("../../tests-shared/remove_tests.rs");
