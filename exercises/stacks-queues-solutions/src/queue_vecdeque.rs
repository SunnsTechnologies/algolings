use crate::vec_queue::VecQueue;
use algolings_trace::{mark_inserted, mark_removed, mark_visited};

/// Reference solution for the `queue_vecdeque` exercise.
impl VecQueue {
    pub fn enqueue(&mut self, value: i32) {
        mark_inserted(self.len(), value);
        self.data.push_back(value);
    }

    pub fn dequeue(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }
        mark_removed(0);
        self.data.pop_front()
    }

    pub fn peek(&self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }
        mark_visited(0);
        self.data.front().copied()
    }
}

#[cfg(test)]
include!("../../tests-shared/queue_vecdeque_tests.rs");
