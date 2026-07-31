//! Shared scaffolding for the queue_vecdeque exercise — a queue backed by
//! `VecDeque<i32>`, a ring buffer that supports O(1) push/pop at BOTH
//! ends (unlike `Vec`, whose `remove(0)` is O(n) since everything after
//! index 0 has to shift down).

use std::collections::VecDeque;

#[derive(Default)]
pub struct VecQueue {
    pub data: VecDeque<i32>,
}

impl VecQueue {
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }

    /// Builds a queue directly from `values`, without going through
    /// enqueue — so dequeue/peek's tests don't depend on enqueue being
    /// solved correctly. `values[0]` is at the front, as if each value had
    /// been enqueued in that order.
    pub fn from_values(values: &[i32]) -> Self {
        Self {
            data: values.iter().copied().collect(),
        }
    }

    pub fn to_vec(&self) -> Vec<i32> {
        self.data.iter().copied().collect()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
