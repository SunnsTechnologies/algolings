use algolings_trace::{cmp_lt, mark_sorted, swap};

/// Reference solution for the `quick_sort` exercise (Lomuto partition
/// scheme). `low`/`high` are deliberately `isize`, not `usize`: the
/// partition step needs `low - 1` as a starting sentinel for "no elements
/// placed yet", and that can legitimately go to `-1`. `usize` has no
/// negative values — `0usize - 1` panics in debug builds — so `isize` is
/// the correct choice here, not just a style preference.
pub fn quick_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len > 1 {
        quick_sort_recursive(arr, 0, (len - 1) as isize);
    }
    for i in 0..len {
        mark_sorted(i);
    }
}

fn quick_sort_recursive(arr: &mut [i32], low: isize, high: isize) {
    if low < high {
        let pivot_index = partition(arr, low, high);
        quick_sort_recursive(arr, low, pivot_index - 1);
        quick_sort_recursive(arr, pivot_index + 1, high);
    }
}

fn partition(arr: &mut [i32], low: isize, high: isize) -> isize {
    let mut i = low - 1;
    for j in low..high {
        // arr[j] <= pivot (arr[high], unchanged until the final swap below)
        if !cmp_lt(arr, high as usize, j as usize) {
            i += 1;
            swap(arr, i as usize, j as usize);
        }
    }
    swap(arr, (i + 1) as usize, high as usize);
    i + 1
}

#[cfg(test)]
include!("../../tests-shared/quick_tests.rs");
