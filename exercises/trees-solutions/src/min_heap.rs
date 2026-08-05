//! Shared scaffolding for the heap exercise — a min-heap backed by
//! `Vec<i32>`, a complete binary tree stored flat: a node at index `i`
//! has children at `2i+1`/`2i+2` and a parent at `(i-1)/2`.
//!
//! `insert`/`extract_min`/`heapify_up`/`heapify_down` are the lesson (in
//! heap.rs); `new`/`from_values`/`peek`/`len`/`is_empty` are
//! always-implemented infrastructure.

pub struct MinHeap {
    pub data: Vec<i32>,
}

impl MinHeap {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Builds a heap directly from `values` (inserting each one, same
    /// ordering a real insert would produce), without going through the
    /// learner's own insert — so extract_min's tests don't depend on
    /// insert being solved correctly.
    pub fn from_values(values: &[i32]) -> Self {
        let mut heap = Self::new();
        for &value in values {
            heap.data.push(value);
            let last = heap.data.len() - 1;
            scaffold_heapify_up(&mut heap.data, last);
        }
        heap
    }

    pub fn peek(&self) -> Option<i32> {
        self.data.first().copied()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Default for MinHeap {
    fn default() -> Self {
        Self::new()
    }
}

fn scaffold_heapify_up(data: &mut [i32], mut index: usize) {
    while index > 0 {
        let parent = (index - 1) / 2;
        if data[index] < data[parent] {
            data.swap(index, parent);
            index = parent;
        } else {
            break;
        }
    }
}
