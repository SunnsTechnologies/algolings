//! Shared scaffolding for the queue_linked_list exercise — a queue backed
//! by a singly-linked list with BOTH `head` and `tail` references, using
//! `Rc<RefCell<Node>>` the same way the linked-list module's
//! `DoublyLinkedList` does for forward links. No `Weak`/`prev` anywhere: a
//! queue only ever removes from the head and inserts at the tail, so
//! nothing ever needs to walk backward or worry about the reference-cycle
//! risk a doubly-linked list's `pop_back` has to guard against.
//!
//! `len` is a real field, incremented/decremented alongside real
//! enqueues/dequeues rather than computed by walking — same reasoning as
//! the doubly-linked list's scaffolding.
//!
//! At `len() == 1`, `head` and `tail` are the SAME `Rc<RefCell<Node>>` —
//! never hold overlapping `borrow()`/`borrow_mut()` calls across both
//! fields at once, or it panics with `BorrowMutError`. `to_vec` avoids
//! this by only ever borrowing one node at a time, releasing each borrow
//! before moving to the next; tests should go through `to_vec()`/
//! `is_empty()` rather than poking `head`/`tail` directly for the same
//! reason.

use std::cell::RefCell;
use std::rc::Rc;

pub struct Node {
    pub value: i32,
    pub next: Option<Rc<RefCell<Node>>>,
}

#[derive(Default)]
pub struct LinkedQueue {
    pub head: Option<Rc<RefCell<Node>>>,
    pub tail: Option<Rc<RefCell<Node>>>,
    /// Maintained directly by the enqueue/dequeue exercise (`self.len +=
    /// 1`, etc.), the same way it already mutates `head`/`tail` directly —
    /// not walked, see the module docs for why.
    pub len: usize,
}

impl LinkedQueue {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    /// Builds a queue directly from `values`, wiring `next` correctly —
    /// without going through enqueue, so a learner's own (possibly
    /// unsolved) implementation never affects this. `values[0]` ends up at
    /// the front, as if each value had been enqueued in that order.
    pub fn from_values(values: &[i32]) -> Self {
        let mut head: Option<Rc<RefCell<Node>>> = None;
        let mut tail: Option<Rc<RefCell<Node>>> = None;

        for &value in values {
            let new_node = Rc::new(RefCell::new(Node { value, next: None }));
            match &tail {
                Some(old_tail) => old_tail.borrow_mut().next = Some(new_node.clone()),
                None => head = Some(new_node.clone()),
            }
            tail = Some(new_node);
        }

        Self {
            head,
            tail,
            len: values.len(),
        }
    }

    /// Reads the queue's contents front-to-back via `next`, without going
    /// through a learner's own enqueue/dequeue code.
    pub fn to_vec(&self) -> Vec<i32> {
        let mut values = Vec::new();
        let mut current = self.head.clone();
        while let Some(node) = current {
            values.push(node.borrow().value);
            current = node.borrow().next.clone();
        }
        values
    }

    /// O(1): reads the stored counter rather than walking the queue.
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}
