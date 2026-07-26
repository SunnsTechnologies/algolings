use algolings_trace::{cmp_lt_values, mark_sorted, set_at};

/// Reference solution for the `insertion_sort` exercise. `key` is copied
/// OUT of `arr[i]` into a local variable before any shifting happens —
/// because `i32` is `Copy`, this is a cheap, independent snapshot, not a
/// borrow. That matters here: as the loop below shifts values right via
/// `set_at`, position `i`'s original slot gets overwritten. Without the
/// copy, the value we're trying to insert would be lost the moment the
/// first shift wrote over it.
pub fn insertion_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 1..n {
        let key = arr[i];
        let mut j = i;
        while j > 0 && cmp_lt_values(&key, &arr[j - 1], i, j - 1) {
            let shifted = arr[j - 1];
            set_at(arr, j, shifted);
            j -= 1;
        }
        set_at(arr, j, key);
    }
    for i in 0..n {
        mark_sorted(i);
    }
}

#[cfg(test)]
include!("../../tests-shared/insertion_tests.rs");
