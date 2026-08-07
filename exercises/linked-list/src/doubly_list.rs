//! Shared scaffolding for the doubly-linked-list exercises — the same role
//! `list.rs` plays for the singly-linked ones, but for a genuinely
//! different ownership pattern: `Rc<RefCell<Node>>` for forward links and
//! `Weak<RefCell<Node>>` for backward links, so a node can be reached (and
//! mutated) from two directions without `Box`'s exclusive-ownership rule
//! getting in the way, and without the strong-reference cycle `Rc` alone
//! would create if `prev` were also `Rc`.
//!
//! `len` is a real field, incremented/decremented alongside real
//! insertions/deletions rather than computed by walking — `push_back` is
//! supposed to be O(1) here (a stored tail pointer, no walk needed to find
//! the end), and a length-walk just to compute the tracer's index would
//! silently undo that the moment tracing was added.
//!
//! `to_vec_backward` is scaffolding only, not a learner exercise — it
//! exists so the shared tests can verify a learner's push_front/push_back
//! actually wired `prev`/`Weak` correctly, since nothing else here reads
//! backward links yet (that arrives with a later exercise).

use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Node {
    pub value: i32,
    pub prev: Option<Weak<RefCell<Node>>>,
    pub next: Option<Rc<RefCell<Node>>>,
}

#[derive(Default)]
pub struct DoublyLinkedList {
    pub head: Option<Rc<RefCell<Node>>>,
    pub tail: Option<Rc<RefCell<Node>>>,
    /// Maintained directly by push/pop exercises (`self.len += 1`, etc.),
    /// the same way they already mutate `head`/`tail` directly — not
    /// walked, see the module docs for why.
    pub len: usize,
}

impl DoublyLinkedList {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    /// Builds a list directly from `values`, wiring both `next` (`Rc`) and
    /// `prev` (`Weak`, via `Rc::downgrade`) correctly — without going
    /// through push_front/push_back, so a learner's own (possibly
    /// unsolved) implementation never affects this.
    pub fn from_values(values: &[i32]) -> Self {
        let mut head: Option<Rc<RefCell<Node>>> = None;
        let mut tail: Option<Rc<RefCell<Node>>> = None;

        for &value in values {
            let new_node = Rc::new(RefCell::new(Node {
                value,
                prev: tail.as_ref().map(Rc::downgrade),
                next: None,
            }));
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

    /// Builds a list from `values`, then — if `cycle_to_index` is `Some` —
    /// makes the LAST node's `next` point back to the node at that index
    /// instead of terminating at `None`, deliberately constructing a
    /// genuine cycle. Only `floyds_cycle_detection` uses this: `Box`
    /// (this project's plain singly-linked list) can't represent a real
    /// cycle at all — exclusive ownership forbids a node having two
    /// owners or pointing back to an ancestor — but `Rc`'s shared
    /// ownership can. `cycle_to_index` must be `< values.len()`; this
    /// panics on an out-of-bounds index the same way `Vec` indexing
    /// would, since it's only ever called with hardcoded literals from
    /// this exercise's own tests, never learner-facing input.
    pub fn from_values_with_cycle(values: &[i32], cycle_to_index: Option<usize>) -> Self {
        if values.is_empty() {
            return Self::new();
        }

        let nodes: Vec<Rc<RefCell<Node>>> = values
            .iter()
            .map(|&value| {
                Rc::new(RefCell::new(Node {
                    value,
                    prev: None,
                    next: None,
                }))
            })
            .collect();

        for i in 0..nodes.len() - 1 {
            nodes[i].borrow_mut().next = Some(nodes[i + 1].clone());
            nodes[i + 1].borrow_mut().prev = Some(Rc::downgrade(&nodes[i]));
        }

        if let Some(index) = cycle_to_index {
            nodes.last().unwrap().borrow_mut().next = Some(nodes[index].clone());
        }

        Self {
            head: nodes.first().cloned(),
            tail: nodes.last().cloned(),
            len: values.len(),
        }
    }

    /// Reads the list's contents head-to-tail via `next`, without going
    /// through a learner's own traversal code. NEVER call this on a list
    /// built with a real cycle (`from_values_with_cycle(_, Some(_))`) —
    /// it walks until `next` is `None`, which a cyclic list never reaches.
    pub fn to_vec(&self) -> Vec<i32> {
        let mut values = Vec::new();
        let mut current = self.head.clone();
        while let Some(node) = current {
            values.push(node.borrow().value);
            current = node.borrow().next.clone();
        }
        values
    }

    /// Reads the list's contents tail-to-head via `prev`/`Weak`. Not used
    /// by any learner-facing exercise yet — exists purely so shared tests
    /// can confirm push_front/push_back wired backward links correctly.
    pub fn to_vec_backward(&self) -> Vec<i32> {
        let mut values = Vec::new();
        let mut current = self.tail.clone();
        while let Some(node) = current {
            values.push(node.borrow().value);
            current = node.borrow().prev.as_ref().and_then(Weak::upgrade);
        }
        values
    }

    /// O(1): reads the stored counter rather than walking the list. Named
    /// to match the `len()` convention every other collection in this
    /// codebase (and `std`) follows, even though the field itself (`self.len`)
    /// is what push/pop exercises actually mutate.
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_values_builds_a_list_in_forward_order() {
        let list = DoublyLinkedList::from_values(&[1, 2, 3]);
        assert_eq!(list.to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn from_values_wires_backward_links_correctly() {
        let list = DoublyLinkedList::from_values(&[1, 2, 3]);
        assert_eq!(list.to_vec_backward(), vec![3, 2, 1]);
    }

    #[test]
    fn from_values_with_no_values_is_empty() {
        let list = DoublyLinkedList::from_values(&[]);
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
        assert_eq!(list.to_vec_backward(), Vec::<i32>::new());
    }

    #[test]
    fn len_matches_the_number_of_values() {
        let list = DoublyLinkedList::from_values(&[1, 2, 3]);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn new_list_is_empty() {
        let list = DoublyLinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.to_vec(), Vec::<i32>::new());
        assert_eq!(list.to_vec_backward(), Vec::<i32>::new());
    }
}
