use crate::linked_queue::LinkedQueue;
use algolings_trace::{mark_inserted, mark_removed, mark_visited};

// Implement enqueue, dequeue, and peek for a queue backed by a
// singly-linked list. Unlike a stack (which only ever touches ONE end), a
// queue's enqueue and dequeue touch OPPOSITE ends — so you need a stored
// `tail` reference, or finding the end to enqueue onto would mean walking
// the whole list every time.
//
// enqueue: if the list is empty, the new node becomes BOTH head and tail.
// Otherwise, wire the OLD tail's `next` to the new node, then make the new
// node the tail.
//
// dequeue: take the head. If there's no next node, the list is now empty —
// null `tail` too, or it'll dangle (and the next enqueue would wire onto
// an orphaned node nothing points to anymore).
//
// Call mark_inserted(self.len(), value) for enqueue, mark_removed(0) for
// dequeue, and mark_visited(0) for peek. dequeue/peek on an empty queue
// should return None without recording any event.
impl LinkedQueue {
    pub fn enqueue(&mut self, value: i32) {
        todo!("wire the new node onto the tail (or set head+tail if empty)")
    }

    pub fn dequeue(&mut self) -> Option<i32> {
        todo!("take the head, nulling tail too if the queue is now empty")
    }

    pub fn peek(&self) -> Option<i32> {
        todo!("read the head's value without removing it")
    }
}

#[cfg(test)]
include!("../../tests-shared/queue_linked_list_tests.rs");
