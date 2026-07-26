use algolings_trace::{cmp_lt_values, mark_sorted, set_at};

// Implement insertion sort. Grow a sorted region on the left by taking the
// next element and shifting larger elements right to make room for it.
//
// Copy the value at position i into a local `key` variable FIRST (before
// touching the array) — you're about to overwrite that position via
// shifting, so if you don't save the value out, it's gone. Use
// `cmp_lt_values(&key, &arr[j - 1], i, j - 1)` to check whether the
// previous element is bigger than `key` (reporting positions i and j-1),
// and `set_at(arr, j, shifted_value)` to perform the shift. Once the loop
// finds key's correct spot, `set_at(arr, j, key)` places it there.
pub fn insertion_sort(arr: &mut [i32]) {
    todo!("implement insertion sort using cmp_lt_values, set_at, and mark_sorted")
}

#[cfg(test)]
include!("../../tests-shared/insertion_tests.rs");
