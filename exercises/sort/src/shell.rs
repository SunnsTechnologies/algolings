use algolings_trace::{cmp_lt_values, mark_sorted, set_at};

// Implement Shell sort: insertion sort, but comparing/shifting elements
// `gap` positions apart instead of adjacent ones, starting with a large
// gap and halving it each pass down to 1.
//
// Guard every `j - gap` with `j >= gap` first — `j` and `gap` are `usize`,
// and `j - gap` when `j < gap` would underflow and panic. Use
// `cmp_lt_values(&temp, &arr[j - gap], i, j - gap)` for the comparison and
// `set_at(arr, j, shifted_value)` for the shift, same pattern as insertion
// sort. Call `mark_sorted(i)` for every position once fully done.
pub fn shell_sort(arr: &mut [i32]) {
    todo!("implement shell sort using cmp_lt_values, set_at, and mark_sorted")
}

#[cfg(test)]
include!("../../tests-shared/shell_tests.rs");
