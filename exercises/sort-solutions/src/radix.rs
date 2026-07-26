use algolings_trace::{mark_sorted, set_at};

/// Reference solution for the `radix_sort` exercise (LSD radix sort using
/// counting sort as the stable per-digit subroutine). Operates on `i32`
/// and assumes non-negative values — true for every algolings fixture —
/// since radix sort's digit-extraction trick doesn't generalize to
/// negative numbers without extra preprocessing (e.g. offsetting them into
/// a non-negative range first), which real-world signed-integer radix
/// sorts have to add explicitly.
pub fn radix_sort(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }
    let max_value = *arr.iter().max().unwrap();
    let mut exp: i32 = 1;
    while max_value / exp > 0 {
        counting_sort_by_digit(arr, exp);
        exp *= 10;
    }
    for i in 0..arr.len() {
        mark_sorted(i);
    }
}

fn counting_sort_by_digit(arr: &mut [i32], exp: i32) {
    let mut output = vec![0i32; arr.len()];
    let mut count = [0usize; 10];

    for &value in arr.iter() {
        let digit = ((value / exp) % 10) as usize;
        count[digit] += 1;
    }
    for i in 1..10 {
        count[i] += count[i - 1];
    }
    for &value in arr.iter().rev() {
        let digit = ((value / exp) % 10) as usize;
        count[digit] -= 1;
        set_at(&mut output, count[digit], value);
    }
    for (i, &value) in output.iter().enumerate() {
        set_at(arr, i, value);
    }
}

#[cfg(test)]
include!("../../tests-shared/radix_tests.rs");
