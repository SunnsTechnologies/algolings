use crate::doubly_list::DoublyLinkedList;
use algolings_trace::{found, mark_converging};

// A singly-linked list can only search forward, one node at a time. A
// doubly-linked list can search from BOTH ends at once, walking `left`
// forward via `next` and `right` backward via `prev`, until they meet in
// the middle — this is the thing `prev` genuinely buys you over a
// singly-linked list, beyond what doubly_push/doubly_pop already showed.
//
// Track position with two index counters, `li` starting at 0 and `ri`
// starting at `self.len() - 1` (guard the empty-list case first, or that
// subtraction underflows). Call `mark_converging(li, ri)` once per step,
// checking BOTH nodes against `target` — except when `li == ri`, which
// means they're the SAME node (an odd-length list's middle element): check
// it once, not twice, and stop there rather than decrementing `ri` past 0.
impl DoublyLinkedList {
    pub fn contains_converging(&self, target: i32) -> bool {
        todo!("walk li forward via next and ri backward via prev until they meet")
    }
}

#[cfg(test)]
include!("../../tests-shared/doubly_converge_tests.rs");
