use crate::linked_stack::{LinkedStack, Node};
use algolings_trace::{mark_inserted, mark_removed, mark_visited};

// Implement push, pop, and peek for a stack backed by a singly-linked
// list. LIFO maps naturally onto "insert at the head, remove from the
// head" — the newest value is always index 0, exactly like the
// linked-list module's push_front/head-removal, just wrapped in its own
// Stack type here.
//
// Call mark_inserted(0, value) for push, mark_removed(0) for pop, and
// mark_visited(0) for peek. pop/peek on an empty stack should return None
// without recording any event.
impl LinkedStack {
    pub fn push(&mut self, value: i32) {
        todo!("push value onto the head, tracing with mark_inserted(0, value)")
    }

    pub fn pop(&mut self) -> Option<i32> {
        todo!("pop from the head, tracing with mark_removed(0)")
    }

    pub fn peek(&self) -> Option<i32> {
        todo!("read the head's value without removing it, tracing with mark_visited(0)")
    }
}

#[cfg(test)]
include!("../../tests-shared/stack_linked_list_tests.rs");
