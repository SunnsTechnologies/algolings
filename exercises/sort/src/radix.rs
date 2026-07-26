use algolings_trace::{mark_sorted, set_at};

// Implement LSD radix sort: repeatedly counting-sort the array by each
// decimal digit, starting from the ones place, using an increasing power
// of 10 (`exp`) to extract each digit until `exp` exceeds the largest value.
//
// Per digit pass: count how many values have each digit 0-9, turn those
// counts into prefix sums, then walk the array IN REVERSE (for stability),
// placing each value into an `output` buffer at `set_at(&mut output,
// index, value)`, then copy `output` back into `arr` (also via `set_at`).
// This assumes non-negative values, which every algolings fixture is.
// Call `mark_sorted(i)` for every position once fully done.
pub fn radix_sort(arr: &mut [i32]) {
    todo!("implement radix sort using set_at and mark_sorted (no comparisons needed)")
}

#[cfg(test)]
include!("../../tests-shared/radix_tests.rs");
