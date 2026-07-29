use crate::list::SinglyLinkedList;
use algolings_trace::{mark_inserted, mark_removed};

// Implement reverse: reverse the list in place, without allocating a new
// list.
//
// Track how many nodes are still unprocessed as you walk. For each node,
// call `mark_removed(0)` (it's leaving the front of what's left) then
// `mark_inserted(remaining_len, value)` (it's joining the front of what
// you've already reversed) — `remaining_len` is the count still unprocessed
// AFTER this node leaves.
impl SinglyLinkedList {
    pub fn reverse(&mut self) {
        todo!("implement reverse using mark_removed and mark_inserted")
    }
}

#[cfg(test)]
include!("../../tests-shared/reverse_tests.rs");
