use algolings_trace::{mark_sorted, set_at};

/// Reference solution for the `counting_sort` exercise. Counting sort has
/// no comparisons or swaps at all: the whole algorithm is representable as
/// `Set` events placing each value into its final output position. The
/// counting/bucketing phase itself isn't traced — only the placement into
/// the output array is, since that's the part that maps onto the same
/// "array with positions" visualization the compare/swap sorts use.
pub fn counting_sort(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }
    let min = *arr.iter().min().unwrap();
    let max = *arr.iter().max().unwrap();
    let range = (max - min + 1) as usize;

    let mut counts = vec![0usize; range];
    for &v in arr.iter() {
        counts[(v - min) as usize] += 1;
    }
    for i in 1..range {
        counts[i] += counts[i - 1];
    }

    let mut output = vec![0i32; arr.len()];
    for &v in arr.iter().rev() {
        let bucket = (v - min) as usize;
        counts[bucket] -= 1;
        set_at(&mut output, counts[bucket], v);
    }

    for (i, &v) in output.iter().enumerate() {
        set_at(arr, i, v);
        mark_sorted(i);
    }
}

#[cfg(test)]
include!("../../tests-shared/counting_tests.rs");
