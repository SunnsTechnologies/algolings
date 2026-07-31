//! Shared scaffolding for the stack_linked_list exercise — LIFO via a
//! `Box<Node>`-backed stack, structurally identical to how the linked-list
//! module's `SinglyLinkedList` works, but its own self-contained type
//! since this module doesn't depend on that crate.

pub struct Node {
    pub value: i32,
    pub next: Option<Box<Node>>,
}

#[derive(Default)]
pub struct LinkedStack {
    pub head: Option<Box<Node>>,
}

impl LinkedStack {
    pub fn new() -> Self {
        Self { head: None }
    }

    /// Builds a stack directly from `values`, without going through push —
    /// so pop/peek's tests don't depend on push being solved correctly.
    /// The LAST value in `values` ends up on top, as if each value had
    /// been pushed in that order — so this iterates FORWARD, prepending
    /// each value as it goes (the opposite of a list's from_values, which
    /// iterates in reverse to keep head as the FIRST value).
    pub fn from_values(values: &[i32]) -> Self {
        let mut head = None;
        for &value in values {
            head = Some(Box::new(Node { value, next: head }));
        }
        Self { head }
    }

    pub fn to_vec(&self) -> Vec<i32> {
        let mut values = Vec::new();
        let mut current = self.head.as_deref();
        while let Some(node) = current {
            values.push(node.value);
            current = node.next.as_deref();
        }
        values
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}
