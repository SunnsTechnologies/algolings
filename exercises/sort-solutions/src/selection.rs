use algolings_trace::{cmp_lt, mark_sorted, swap};

/// Reference solution for the `selection_sort` exercise. Tracks the INDEX
/// of the smallest remaining value (not the value itself) — tracking the
/// value would still require a second search to find where it lives before
/// swapping, and would misbehave with duplicate values. Only swaps when
/// `min_index != i`, so a position that's already correct never emits a
/// no-op Swap event that would visibly lie to someone watching the trace.
pub fn selection_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        let mut min_index = i;
        for j in (i + 1)..n {
            if cmp_lt(arr, j, min_index) {
                min_index = j;
            }
        }
        if min_index != i {
            swap(arr, i, min_index);
        }
        mark_sorted(i);
    }
}

#[cfg(test)]
include!("../../tests-shared/selection_tests.rs");
