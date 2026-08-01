use crate::hash_table::CustomHashTable;
use algolings_trace::{mark_set, mark_visited};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Reference solution for the `custom_hash_table` exercise.
impl CustomHashTable {
    fn bucket_index(&self, key: i32) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.buckets.len()
    }

    pub fn insert(&mut self, key: i32) {
        let index = self.bucket_index(key);
        if !self.buckets[index].contains(&key) {
            self.buckets[index].push(key);
        }
        mark_set(index, key);
    }

    pub fn contains(&self, key: i32) -> bool {
        let index = self.bucket_index(key);
        mark_visited(index);
        self.buckets[index].contains(&key)
    }

    pub fn remove(&mut self, key: i32) -> bool {
        let index = self.bucket_index(key);
        let bucket = &mut self.buckets[index];
        let Some(pos) = bucket.iter().position(|&k| k == key) else {
            return false;
        };
        bucket.remove(pos);
        let display_value = bucket.last().copied().unwrap_or(0);
        mark_set(index, display_value);
        true
    }
}

#[cfg(test)]
include!("../../tests-shared/custom_hash_table_tests.rs");
