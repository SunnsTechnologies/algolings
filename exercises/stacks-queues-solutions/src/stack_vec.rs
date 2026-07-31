use crate::vec_stack::VecStack;
use algolings_trace::{mark_inserted, mark_removed, mark_visited};

/// Reference solution for the `stack_vec` exercise.
impl VecStack {
    pub fn push(&mut self, value: i32) {
        mark_inserted(self.len(), value);
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }
        mark_removed(self.len() - 1);
        self.data.pop()
    }

    pub fn peek(&self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }
        mark_visited(self.len() - 1);
        self.data.last().copied()
    }
}

#[cfg(test)]
include!("../../tests-shared/stack_vec_tests.rs");
