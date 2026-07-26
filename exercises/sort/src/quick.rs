use algolings_trace::{cmp_lt, mark_sorted, swap};

// Implement quick sort using the Lomuto partition scheme: pick the last
// element as the pivot, partition the slice so everything <= pivot ends up
// to its left, then recursively sort each side.
//
// Use `isize`, not `usize`, for your low/high bounds — the partition
// step needs a "nothing placed yet" sentinel of `low - 1`, which can
// legitimately be `-1`. `usize` can't represent that (`0usize - 1` panics).
//
// Inside partition: compare `arr[j]` against the pivot with
// `cmp_lt(arr, high, j)` (this is `pivot < arr[j]`, so negate it for
// `arr[j] <= pivot`), and use `swap(arr, i, j)` to move qualifying
// elements left. Call `mark_sorted(i)` for every position once fully done.
pub fn quick_sort(arr: &mut [i32]) {
    todo!("implement quick sort using cmp_lt, swap, and mark_sorted")
}

#[cfg(test)]
include!("../../tests-shared/quick_tests.rs");
