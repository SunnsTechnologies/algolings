use crate::min_heap::MinHeap;
use algolings_trace::{cmp_lt, swap};

/// Reference solution for the `heap` exercise.
impl MinHeap {
    pub fn insert(&mut self, value: i32) {
        self.data.push(value);
        self.heapify_up(self.data.len() - 1);
    }

    pub fn extract_min(&mut self) -> Option<i32> {
        if self.data.is_empty() {
            return None;
        }
        let last = self.data.pop().unwrap();
        if self.data.is_empty() {
            return Some(last);
        }
        let min = std::mem::replace(&mut self.data[0], last);
        self.heapify_down(0);
        Some(min)
    }

    fn heapify_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = (index - 1) / 2;
            if cmp_lt(&self.data, index, parent) {
                swap(&mut self.data, index, parent);
                index = parent;
            } else {
                break;
            }
        }
    }

    fn heapify_down(&mut self, mut index: usize) {
        let len = self.data.len();
        loop {
            let left = 2 * index + 1;
            let right = 2 * index + 2;
            let mut smallest = index;

            if left < len && cmp_lt(&self.data, left, smallest) {
                smallest = left;
            }
            if right < len && cmp_lt(&self.data, right, smallest) {
                smallest = right;
            }
            if smallest == index {
                break;
            }
            swap(&mut self.data, index, smallest);
            index = smallest;
        }
    }
}

#[cfg(test)]
include!("../../tests-shared/heap_tests.rs");
