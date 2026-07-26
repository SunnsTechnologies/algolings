use algolings_trace::{cmp_lt, mark_sorted, swap};

/// Reference solution for the `bubble_sort` exercise. CI runs this crate's
/// tests to prove a correct answer exists for the shared test suite in
/// `exercises/tests-shared/bubble_tests.rs` — the same suite the learner's
/// skeleton in `exercises/sort/src/bubble.rs` must pass.
pub fn bubble_sort(arr: &mut [i32]) {
    let mut n = arr.len();
    if n < 2 {
        return;
    }

    loop {
        let mut swapped = false;

        for i in 1..n {
            if cmp_lt(arr, i, i - 1) {
                swap(arr, i - 1, i);
                swapped = true;
            }
        }

        n -= 1;
        mark_sorted(n);

        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
include!("../../tests-shared/bubble_tests.rs");
