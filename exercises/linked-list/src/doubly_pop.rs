use crate::doubly_list::DoublyLinkedList;
use algolings_trace::mark_removed;

// Implement pop_front and pop_back for a DOUBLY-linked list. Both remove a
// node and return its value, and both have to fix up the pointer on the
// OTHER end once that node is gone.
//
// pop_front: after removing the head, the NEW head's `prev` still points
// (via `Weak`) at the node you just removed — null it out, or it's
// dangling. If there was no next node, the list is now empty: null `tail`
// too.
//
// pop_back: after removing the tail, the NEW tail's `next` still points
// (via `Rc`) at the node you just removed — null it out. To find the new
// tail, upgrade the removed node's `prev` `Weak` reference. If there's no
// previous node, the list is now empty: null `head` too.
//
// Call `mark_removed(0)` for pop_front, or `mark_removed(self.len() - 1)`
// for pop_back — read the length BEFORE decrementing it. Popping an empty
// list should return `None` without recording any event.
impl DoublyLinkedList {
    pub fn pop_front(&mut self) -> Option<i32> {
        todo!("implement pop_front using mark_removed, nulling the new head's prev")
    }

    pub fn pop_back(&mut self) -> Option<i32> {
        todo!("implement pop_back using mark_removed, nulling the new tail's next")
    }
}

#[cfg(test)]
include!("../../tests-shared/doubly_pop_tests.rs");
