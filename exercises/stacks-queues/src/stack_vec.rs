use crate::vec_stack::VecStack;
use algolings_trace::{mark_inserted, mark_removed, mark_visited};

// Implement push, pop, and peek for a Vec-backed stack — the simplest way
// to get LIFO (last in, first out) behavior in Rust: Vec's own push/pop
// already operate on the END of the buffer, exactly what a stack needs.
//
// Call mark_inserted(self.len(), value) for push (read the length BEFORE
// pushing), mark_removed(self.len() - 1) for pop, and
// mark_visited(self.len() - 1) for peek. pop/peek on an empty stack should
// return None without recording any event.
impl VecStack {
    pub fn push(&mut self, value: i32) {
        todo!("push value onto the end of self.data, tracing with mark_inserted")
    }

    pub fn pop(&mut self) -> Option<i32> {
        todo!("pop from the end of self.data, tracing with mark_removed")
    }

    pub fn peek(&self) -> Option<i32> {
        todo!("read the last value without removing it, tracing with mark_visited")
    }
}

#[cfg(test)]
include!("../../tests-shared/stack_vec_tests.rs");
