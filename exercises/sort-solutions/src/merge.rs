use algolings_trace::{cmp_lt_values, mark_sorted, set_at};

/// Reference solution for the `merge_sort` exercise. Recurses on INDEX
/// RANGES into the one original slice (never `split_at_mut` into owned
/// sub-slices) so every comparison and write reports a true global array
/// position — no offset bookkeeping needed at the call site.
pub fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len < 2 {
        return;
    }
    merge_sort_range(arr, 0, len);
    for i in 0..len {
        mark_sorted(i);
    }
}

fn merge_sort_range(arr: &mut [i32], lo: usize, hi: usize) {
    if hi - lo < 2 {
        return;
    }
    let mid = lo + (hi - lo) / 2;
    merge_sort_range(arr, lo, mid);
    merge_sort_range(arr, mid, hi);
    merge(arr, lo, mid, hi);
}

/// Merges the two already-sorted runs `arr[lo..mid]` and `arr[mid..hi]`.
/// Snapshots the segment first so comparisons read frozen values while the
/// write cursor overwrites `arr` in place — `cmp_lt_values` still reports
/// the true global indices (`left`, `right`) even though the actual values
/// being compared live in `snapshot`, not `arr`.
fn merge(arr: &mut [i32], lo: usize, mid: usize, hi: usize) {
    let snapshot: Vec<i32> = arr[lo..hi].to_vec();
    let mut left = lo;
    let mut right = mid;
    let mut out = lo;

    while left < mid && right < hi {
        let a = snapshot[left - lo];
        let b = snapshot[right - lo];
        if cmp_lt_values(&b, &a, right, left) {
            set_at(arr, out, b);
            right += 1;
        } else {
            set_at(arr, out, a);
            left += 1;
        }
        out += 1;
    }
    while left < mid {
        set_at(arr, out, snapshot[left - lo]);
        left += 1;
        out += 1;
    }
    while right < hi {
        set_at(arr, out, snapshot[right - lo]);
        right += 1;
        out += 1;
    }
}

#[cfg(test)]
include!("../../tests-shared/merge_tests.rs");
