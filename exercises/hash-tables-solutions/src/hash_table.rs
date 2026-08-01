//! Shared scaffolding for the custom_hash_table exercise — a hash table
//! built from scratch with separate chaining: each bucket is a small Vec
//! of the keys that hashed into it, so multiple colliding keys coexist
//! (unlike open addressing, where a collision has to look elsewhere for a
//! free slot).
//!
//! `insert`/`contains`/`remove` and the hash-and-modulo `bucket_index`
//! computation are the lesson (in `custom_hash_table.rs`); `new`/
//! `from_keys`/`len`/`is_empty` are always-implemented infrastructure so
//! the exercise's tests can build or read table state without depending
//! on the learner's own insert being correct.
//!
//! Keys are bare `i32` here, not a generic `K: Eq + Hash` paired with a
//! separate stored value — this project's trace/render system only
//! understands flat i32 sequences, so this is a simplified "hash set,"
//! not a full hash map. The idiom being taught (DefaultHasher + modulo,
//! chaining for collisions) survives fully intact without a separate
//! value.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub const DEFAULT_BUCKETS: usize = 8;

pub struct CustomHashTable {
    pub buckets: Vec<Vec<i32>>,
}

impl CustomHashTable {
    pub fn new() -> Self {
        Self {
            buckets: vec![Vec::new(); DEFAULT_BUCKETS],
        }
    }

    /// Builds a table directly from `keys`, without going through insert —
    /// so contains/remove's tests don't depend on insert being solved
    /// correctly. Duplicates the hash-and-modulo computation the exercise
    /// itself teaches, deliberately kept separate (not exposed as a public
    /// method) from the learner's own implementation.
    pub fn from_keys(keys: &[i32]) -> Self {
        let mut table = Self::new();
        for &key in keys {
            let index = scaffold_bucket_index(key, table.buckets.len());
            if !table.buckets[index].contains(&key) {
                table.buckets[index].push(key);
            }
        }
        table
    }

    pub fn len(&self) -> usize {
        self.buckets.iter().map(Vec::len).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for CustomHashTable {
    fn default() -> Self {
        Self::new()
    }
}

fn scaffold_bucket_index(key: i32, bucket_count: usize) -> usize {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    (hasher.finish() as usize) % bucket_count
}
