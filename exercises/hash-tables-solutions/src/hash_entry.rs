use std::collections::HashMap;

/// Reference solution for the `hash_entry` exercise.
pub fn count_frequencies(values: &[i32]) -> HashMap<i32, usize> {
    let mut counts = HashMap::new();
    for &value in values {
        *counts.entry(value).or_insert(0) += 1;
    }
    counts
}

#[cfg(test)]
include!("../../tests-shared/hash_entry_tests.rs");
