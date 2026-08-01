use crate::hash_table::CustomHashTable;
use algolings_trace::{mark_set, mark_visited};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Implement bucket_index, insert, contains, and remove for a hash table
// built from scratch with separate chaining: each bucket is a Vec of the
// keys that hashed into it, so colliding keys coexist in the same bucket
// instead of overwriting each other.
//
// bucket_index: hash the key with DefaultHasher, then reduce it to a
// valid bucket position with `% self.buckets.len()`.
//
// insert: find the key's bucket. If it's already there, do nothing (this
// is a set, not a map — there's no separate value to update). Otherwise
// push it. Either way, call mark_set(index, key).
//
// contains: check whether the key's bucket contains it, tracing with
// mark_visited(index).
//
// remove: find the key's bucket, remove it if present. Trace with
// mark_set(index, value) where value is 0 if the bucket is now empty, or
// the bucket's remaining key if others are still chained there — the
// trace's row only shows one value per bucket, so it can't show a
// collision surviving directly (the tests verify that part).
impl CustomHashTable {
    fn bucket_index(&self, key: i32) -> usize {
        todo!("hash key with DefaultHasher, then reduce mod self.buckets.len()")
    }

    pub fn insert(&mut self, key: i32) {
        todo!("find key's bucket; push if not already present, tracing with mark_set")
    }

    pub fn contains(&self, key: i32) -> bool {
        todo!("check whether key's bucket contains it, tracing with mark_visited")
    }

    pub fn remove(&mut self, key: i32) -> bool {
        todo!("remove key from its bucket if present, tracing with mark_set")
    }
}

#[cfg(test)]
include!("../../tests-shared/custom_hash_table_tests.rs");
