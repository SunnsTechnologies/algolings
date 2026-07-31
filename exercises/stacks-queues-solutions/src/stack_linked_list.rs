use crate::linked_stack::{LinkedStack, Node};
use algolings_trace::{mark_inserted, mark_removed, mark_visited};

/// Reference solution for the `stack_linked_list` exercise.
impl LinkedStack {
    pub fn push(&mut self, value: i32) {
        mark_inserted(0, value);
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            mark_removed(0);
            self.head = node.next;
            node.value
        })
    }

    pub fn peek(&self) -> Option<i32> {
        self.head.as_ref().map(|node| {
            mark_visited(0);
            node.value
        })
    }
}

#[cfg(test)]
include!("../../tests-shared/stack_linked_list_tests.rs");
