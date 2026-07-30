use crate::doubly_list::DoublyLinkedList;
use algolings_trace::mark_inserted;

// Implement push_front and push_back for a DOUBLY-linked list. Unlike the
// singly-linked version, every node must also point BACKWARD via `prev` —
// a `Weak` reference (via `Rc::downgrade`), not a strong `Rc`, or you'd
// create a reference cycle that never gets freed.
//
// push_back doesn't need to walk to find the end — the list keeps a `tail`
// pointer for exactly that reason, which is what makes it O(1) here
// (unlike the singly-linked list's push_back). Call `mark_inserted(i,
// value)` where `i` is 0 for push_front, or the list's length (before
// inserting) for push_back.
//
// Note: `RefCell` panics at runtime — "already borrowed" — if you try to
// `borrow_mut()` a node you're already borrowing. That's `RefCell` doing
// its job: catching an aliasing mistake `Box` couldn't even represent.
impl DoublyLinkedList {
    pub fn push_front(&mut self, value: i32) {
        todo!("implement push_front using mark_inserted, wiring prev via Weak")
    }

    pub fn push_back(&mut self, value: i32) {
        todo!("implement push_back using mark_inserted, wiring prev via Weak")
    }
}

#[cfg(test)]
include!("../../tests-shared/doubly_push_tests.rs");
