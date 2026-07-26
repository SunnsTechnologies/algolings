use algolings_trace::{mark_sorted, set_at};

// Implement counting sort. This algorithm has no comparisons at all — it
// counts how many times each value appears, turns those counts into
// prefix sums (so each count becomes "how many values are <= this one"),
// then places every value directly into its final position using those
// counts.
//
// Steps:
//   1. Find min/max to size a `counts` array covering the value range.
//   2. Count occurrences of each value.
//   3. Turn counts into prefix sums.
//   4. Walk the input in REVERSE (for stability), decrementing the count
//      for each value and using it as the index to place that value into
//      an `output` buffer — use `set_at(&mut output, index, value)`.
//   5. Copy `output` back into `arr` (also via `set_at`), and call
//      `mark_sorted(i)` for every position once it holds its final value.
pub fn counting_sort(arr: &mut [i32]) {
    todo!("implement counting sort using set_at and mark_sorted (no comparisons needed)")
}

#[cfg(test)]
include!("../../tests-shared/counting_tests.rs");
