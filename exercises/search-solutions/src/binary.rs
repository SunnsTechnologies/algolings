use algolings_trace::{found, narrow_range, probe};

/// Reference solution for the `binary_search` exercise. Uses
/// `left + (right - left) / 2` for the midpoint rather than
/// `(left + right) / 2` — the latter can overflow for large bounds even
/// when the true midpoint is nowhere close.
pub fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if probe(arr, mid, &target) {
            found(mid);
            return Some(mid);
        }
        if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
        narrow_range(left, right);
    }

    None
}

#[cfg(test)]
include!("../../tests-shared/binary_search_tests.rs");
