use algolings_trace::{cmp_lt, mark_sorted, swap};

// Implement bubble sort. Compare adjacent elements and swap them if they're
// in the wrong order, repeating passes until a full pass makes no swaps.
//
// Use `cmp_lt(arr, i, j)` instead of `arr[i] < arr[j]`, and `swap(arr, i, j)`
// instead of `arr.swap(i, j)` — same behavior, but it lets algolings trace
// the comparisons and swaps your solution actually makes. Call
// `mark_sorted(i)` once position `i` is in its final place.
pub fn bubble_sort(arr: &mut [i32]) {
    todo!("implement bubble sort using cmp_lt, swap, and mark_sorted")
}

#[cfg(test)]
include!("../../tests-shared/bubble_tests.rs");
