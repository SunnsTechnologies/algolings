use algolings_trace::{cmp_lt_values, mark_sorted, set_at};

/// Reference solution for the `shell_sort` exercise — insertion sort
/// generalized with a gap. The `j >= gap` guard isn't just a logic nicety:
/// `j` and `gap` are both `usize`, so `j - gap` when `j < gap` would
/// underflow and panic in a debug build (or silently wrap in release).
/// The guard is load-bearing for correctness, not style.
pub fn shell_sort(arr: &mut [i32]) {
    let mut gap = arr.len() / 2;
    while gap > 0 {
        for i in gap..arr.len() {
            let temp = arr[i];
            let mut j = i;
            while j >= gap && cmp_lt_values(&temp, &arr[j - gap], i, j - gap) {
                let shifted = arr[j - gap];
                set_at(arr, j, shifted);
                j -= gap;
            }
            set_at(arr, j, temp);
        }
        gap /= 2;
    }
    for i in 0..arr.len() {
        mark_sorted(i);
    }
}

#[cfg(test)]
include!("../../tests-shared/shell_tests.rs");
