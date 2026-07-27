use algolings_trace::probe;

// Implement binary search. The array is SORTED — check the middle element
// and eliminate half the remaining range each time.
//
// Use `probe(arr, mid, &target)` instead of `arr[mid] == target`,
// `narrow_range(left, right)` after adjusting your bounds, and `found(mid)`
// right before returning `Some(mid)`.
pub fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    todo!("implement binary search using probe, narrow_range, and found")
}

#[cfg(test)]
include!("../../tests-shared/binary_search_tests.rs");
