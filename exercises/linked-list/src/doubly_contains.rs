use crate::doubly_list::DoublyLinkedList;
use algolings_trace::{found, mark_visited};

// Implement contains for a DOUBLY-linked list, walking forward only. This is
// almost identical to the singly-linked list's `contains` (in traverse.rs) —
// the difference is entirely in the ownership: instead of `Option<Box<Node>>`
// walked with `.as_deref()`, a node's `next` is `Option<Rc<RefCell<Node>>>`,
// so walking means cloning the `Rc` and reading through `.borrow()`.
//
// Call `mark_visited(i)` for every node you check, and `found(i)` the moment
// you find the target — same convention as the singly-linked version.
impl DoublyLinkedList {
    pub fn contains(&self, target: i32) -> bool {
        todo!("walk forward via next, calling mark_visited and found")
    }
}

#[cfg(test)]
include!("../../tests-shared/doubly_contains_tests.rs");
