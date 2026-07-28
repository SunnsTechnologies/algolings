use crate::list::SinglyLinkedList;
use algolings_trace::mark_inserted;

// Implement push_front and push_back.
//
// push_front attaches the new node directly at the head. push_back has to
// walk to the end first — there's no stored tail pointer. Call
// `mark_inserted(i, value)` right where you attach the new node: `i` is 0
// for push_front, or `self.len()` (before inserting) for push_back.
impl SinglyLinkedList {
    pub fn push_front(&mut self, value: i32) {
        todo!("implement push_front using mark_inserted")
    }

    pub fn push_back(&mut self, value: i32) {
        todo!("implement push_back using mark_inserted")
    }
}

#[cfg(test)]
include!("../../tests-shared/insert_tests.rs");
