use crate::vec_queue::VecQueue;
use algolings_trace::{mark_inserted, mark_removed, mark_visited};

// Implement enqueue, dequeue, and peek for a VecDeque-backed queue. FIFO
// (first in, first out) needs O(1) access at BOTH ends — enqueue adds to
// the back, dequeue removes from the front — exactly what VecDeque's ring
// buffer gives you, unlike Vec (whose remove(0) would shift every
// remaining element down by one).
//
// Call mark_inserted(self.len(), value) for enqueue (read the length
// BEFORE enqueuing), mark_removed(0) for dequeue (always the front), and
// mark_visited(0) for peek. dequeue/peek on an empty queue should return
// None without recording any event.
impl VecQueue {
    pub fn enqueue(&mut self, value: i32) {
        todo!("push value onto the back of self.data, tracing with mark_inserted")
    }

    pub fn dequeue(&mut self) -> Option<i32> {
        todo!("pop from the front of self.data, tracing with mark_removed(0)")
    }

    pub fn peek(&self) -> Option<i32> {
        todo!("read the front value without removing it, tracing with mark_visited(0)")
    }
}

#[cfg(test)]
include!("../../tests-shared/queue_vecdeque_tests.rs");
