use algolings_trace::{cmp_lt, mark_sorted, swap};

// Implement selection sort. For each position i, find the index of the
// smallest value in arr[i..], then swap it into position i.
//
// Track the INDEX of the smallest value you've found so far, not the value
// itself — use `cmp_lt(arr, j, min_index)` to compare candidates. Only call
// `swap(arr, i, min_index)` if `min_index != i` (skip the swap if position
// i is already the minimum). Call `mark_sorted(i)` once position i holds
// its final value.
pub fn selection_sort(arr: &mut [i32]) {
    todo!("implement selection sort using cmp_lt, swap, and mark_sorted")
}

#[cfg(test)]
include!("../../tests-shared/selection_tests.rs");
