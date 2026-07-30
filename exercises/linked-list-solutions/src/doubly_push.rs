use crate::doubly_list::{DoublyLinkedList, Node};
use algolings_trace::mark_inserted;
use std::cell::RefCell;
use std::rc::Rc;

/// Reference solution for the `doubly_push` exercise.
impl DoublyLinkedList {
    pub fn push_front(&mut self, value: i32) {
        mark_inserted(0, value);
        let old_head = self.head.take();
        let new_node = Rc::new(RefCell::new(Node {
            value,
            prev: None,
            next: old_head.clone(),
        }));

        match old_head {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
            }
            None => {
                self.tail = Some(new_node.clone());
            }
        }

        self.head = Some(new_node);
        self.len += 1;
    }

    pub fn push_back(&mut self, value: i32) {
        mark_inserted(self.len(), value);
        let old_tail = self.tail.take();
        let new_node = Rc::new(RefCell::new(Node {
            value,
            prev: old_tail.as_ref().map(Rc::downgrade),
            next: None,
        }));

        match &old_tail {
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
}

#[cfg(test)]
include!("../../tests-shared/doubly_push_tests.rs");
