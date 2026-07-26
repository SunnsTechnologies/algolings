use algolings_trace::{cmp_lt_values, mark_sorted, set_at};

// Implement merge sort. Split the slice in half, recursively sort each
// half, then merge the two sorted halves back together.
//
// Recurse on INDEX RANGES into the SAME slice (e.g. a helper function
// taking `lo`/`hi` bounds) rather than splitting into sub-slices with
// `split_at_mut` — that way every position you touch is already a true
// global index, with no offset math needed.
//
// When merging, snapshot the segment into a `Vec` first (you can't safely
// read from `arr` while also writing into it at an overlapping position).
// Use `cmp_lt_values(&a, &b, i, j)` to compare two snapshotted values while
// still reporting their true global positions `i`/`j`, and `set_at(arr, i,
// value)` to write the merged result back. Call `mark_sorted(i)` once the
// whole array is in its final order.
pub fn merge_sort(arr: &mut [i32]) {
    todo!("implement merge sort using cmp_lt_values, set_at, and mark_sorted")
}

#[cfg(test)]
include!("../../tests-shared/merge_tests.rs");
