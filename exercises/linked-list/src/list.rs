//! Shared scaffolding for the linked-list exercises. `Node`/`SinglyLinkedList`
//! and the helpers below are always-implemented infrastructure, not the
//! lesson — `insert.rs`, `remove.rs`, and `traverse.rs` each add their own
//! `impl SinglyLinkedList` block with just the one behavior being exercised.
//! `to_vec`/`from_values`/`len` exist so each exercise's tests and trace can
//! read or build list state without depending on whether the OTHER
//! exercises are solved.

pub struct Node {
    pub value: i32,
    pub next: Option<Box<Node>>,
}

#[derive(Default)]
pub struct SinglyLinkedList {
    pub head: Option<Box<Node>>,
}

impl SinglyLinkedList {
    pub fn new() -> Self {
        Self { head: None }
    }

    /// Builds a list directly from `values`, without going through
    /// push_front/push_back — so remove/traverse's tests and trace don't
    /// depend on `insert` being solved correctly.
    pub fn from_values(values: &[i32]) -> Self {
        let mut head = None;
        for &value in values.iter().rev() {
            head = Some(Box::new(Node { value, next: head }));
        }
        Self { head }
    }

    /// Reads the list's current contents into a `Vec`, without going
    /// through the learner's own `traverse`/`contains` — used by test
    /// assertions and the trace dispatcher.
    pub fn to_vec(&self) -> Vec<i32> {
        let mut values = Vec::new();
        let mut current = self.head.as_deref();
        while let Some(node) = current {
            values.push(node.value);
            current = node.next.as_deref();
        }
        values
    }

    /// The number of nodes, computed by walking the list — there's no
    /// stored length. Used to compute `push_back`'s insertion index for
    /// the tracer.
    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = self.head.as_deref();
        while let Some(node) = current {
            count += 1;
            current = node.next.as_deref();
        }
        count
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_values_builds_a_list_in_order() {
        let list = SinglyLinkedList::from_values(&[1, 2, 3]);
        assert_eq!(list.to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn from_values_with_no_values_is_empty() {
        let list = SinglyLinkedList::from_values(&[]);
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn len_counts_the_nodes() {
        let list = SinglyLinkedList::from_values(&[1, 2, 3]);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn new_list_is_empty() {
        let list = SinglyLinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.to_vec(), Vec::<i32>::new());
    }
}
