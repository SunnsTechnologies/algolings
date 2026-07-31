use crate::linked_queue::{LinkedQueue, Node};
use algolings_trace::{mark_inserted, mark_removed, mark_visited};
use std::cell::RefCell;
use std::rc::Rc;

/// Reference solution for the `queue_linked_list` exercise.
impl LinkedQueue {
    pub fn enqueue(&mut self, value: i32) {
        mark_inserted(self.len(), value);
        let new_node = Rc::new(RefCell::new(Node { value, next: None }));
        match &self.tail {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
            }
            None => {
                self.head = Some(new_node.clone());
            }
        }
        self.tail = Some(new_node);
        self.len += 1;
    }

    pub fn dequeue(&mut self) -> Option<i32> {
        self.head.take().map(|old_head| {
            mark_removed(0);
            let next = old_head.borrow().next.clone();
            match next {
                Some(new_head) => {
                    self.head = Some(new_head);
                }
                None => {
                    self.tail = None;
                }
            }
            self.len -= 1;
            old_head.borrow().value
        })
    }

    pub fn peek(&self) -> Option<i32> {
        self.head.as_ref().map(|node| {
            mark_visited(0);
            node.borrow().value
        })
    }
}

#[cfg(test)]
include!("../../tests-shared/queue_linked_list_tests.rs");
