use std::collections::HashMap;

// Implement count_frequencies using std HashMap's entry() API — the
// idiomatic way to "update if present, insert if not" in one lookup
// instead of two (a naive version would call contains_key then get_mut or
// insert separately, hashing the key twice for every value).
//
// entry(key) returns an Entry, which is either Occupied or Vacant.
// or_insert(default) gives you a &mut to the value either way — a fresh
// `default` if the key was vacant, the existing value if occupied — so
// you can just increment through the returned reference.
//
// No tracing calls here: std HashMap has no stable index or iteration
// order to trace against, unlike every other exercise's array/list model.
pub fn count_frequencies(values: &[i32]) -> HashMap<i32, usize> {
    todo!("build a HashMap<i32, usize>, using entry(value).or_insert(0) to count each value")
}

#[cfg(test)]
include!("../../tests-shared/hash_entry_tests.rs");
