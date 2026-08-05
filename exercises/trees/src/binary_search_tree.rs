use crate::bst::{tree_contains, Bst, Node};
use algolings_trace::{found, mark_inserted, mark_visited};

// Implement insert and contains for a binary search tree — smaller values
// go left, larger go right, so a search never has to look at more than
// one subtree at each level.
//
// There's no flat array here, so trace the DESCENT itself: each recursive
// call is one level deeper, exactly like recursion_basics' call stack.
// Call mark_visited(depth) at every existing node you compare against,
// mark_inserted(depth, value) when insert places a NEW node at an empty
// slot, and found(depth) when contains locates the value. A single
// insert/contains call only ever walks ONE path from the root, so depth
// never collides with anything else happening at the same time.
impl Bst {
    pub fn insert(&mut self, value: i32) {
        todo!("descend comparing at each level (mark_visited), place a new node at the first empty slot (mark_inserted)")
    }

    pub fn contains(&self, value: i32) -> bool {
        todo!("descend comparing at each level (mark_visited), call found(depth) and return true if located")
    }
}

#[cfg(test)]
include!("../../tests-shared/binary_search_tree_tests.rs");
