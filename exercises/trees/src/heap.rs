use crate::min_heap::MinHeap;
use algolings_trace::{cmp_lt, swap};

// Unlike everything else in this module, a heap is a genuine value
// SEQUENCE (stored flat in a Vec), so it traces exactly like a sort
// exercise: cmp_lt/swap are the SAME helpers bubble/selection/insertion
// sort already use.
//
// insert: push onto the end, then "bubble up" — while the new value is
// smaller than its parent ((index - 1) / 2), swap them and move up.
//
// extract_min: the minimum is always at index 0. Pop the LAST element,
// put it at index 0 (std::mem::replace to get the old minimum back),
// then "bubble down" — while a child (2*index+1 or 2*index+2) is
// smaller, swap with the smaller child and move down.
impl MinHeap {
    pub fn insert(&mut self, value: i32) {
        todo!("push onto the end, then heapify_up from the last index")
    }

    pub fn extract_min(&mut self) -> Option<i32> {
        todo!("swap the last element into index 0, pop the old last slot, heapify_down from 0")
    }

    fn heapify_up(&mut self, index: usize) {
        todo!("while index > 0 and data[index] < data[parent], swap and move up")
    }

    fn heapify_down(&mut self, index: usize) {
        todo!("while a child is smaller than data[index], swap with the smaller child and move down")
    }
}

#[cfg(test)]
include!("../../tests-shared/heap_tests.rs");
