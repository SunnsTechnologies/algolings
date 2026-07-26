use algolings_trace::{cmp_lt_values, mark_sorted, set_at};

/// Reference solution. Recurses on INDEX RANGES into the one original
/// slice (never `split_at_mut` into owned sub-slices) so every comparison
/// and write reports a true global array position — resolving the
/// recursion/sub-slice-offset question from the design review without
/// needing any offset bookkeeping at the call site.
pub fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len < 2 {
        return;
    }
    merge_sort_range(arr, 0, len);
    for i in 0..len {
        mark_sorted(i);
    }
}

fn merge_sort_range(arr: &mut [i32], lo: usize, hi: usize) {
    if hi - lo < 2 {
        return;
    }
    let mid = lo + (hi - lo) / 2;
    merge_sort_range(arr, lo, mid);
    merge_sort_range(arr, mid, hi);
    merge(arr, lo, mid, hi);
}

/// Merges the two already-sorted runs `arr[lo..mid]` and `arr[mid..hi]`.
/// Snapshots the segment first so comparisons read frozen values while the
/// write cursor overwrites `arr` in place — `cmp_lt_values` still reports
/// the true global indices (`left`, `right`) even though the actual values
/// being compared live in `snapshot`, not `arr`.
fn merge(arr: &mut [i32], lo: usize, mid: usize, hi: usize) {
    let snapshot: Vec<i32> = arr[lo..hi].to_vec();
    let mut left = lo;
    let mut right = mid;
    let mut out = lo;

    while left < mid && right < hi {
        let a = snapshot[left - lo];
        let b = snapshot[right - lo];
        if cmp_lt_values(&b, &a, right, left) {
            // right < left, i.e. NOT (a <= b): take from the right run.
            set_at(arr, out, b);
            right += 1;
        } else {
            set_at(arr, out, a);
            left += 1;
        }
        out += 1;
    }
    while left < mid {
        set_at(arr, out, snapshot[left - lo]);
        left += 1;
        out += 1;
    }
    while right < hi {
        set_at(arr, out, snapshot[right - lo]);
        right += 1;
        out += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    #[test]
    fn sorts_correctly() {
        let mut input = vec![5, 1, 4, 2, 8];
        merge_sort(&mut input);
        assert_eq!(input, vec![1, 2, 4, 5, 8]);
    }

    #[test]
    fn handles_empty_and_single_element() {
        let mut empty: Vec<i32> = vec![];
        merge_sort(&mut empty);
        assert_eq!(empty, Vec::<i32>::new());

        let mut single = vec![1];
        merge_sort(&mut single);
        assert_eq!(single, vec![1]);
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let mut a = vec![5, 1, 4, 2, 8, 7, 3, 6];
        let mut b = a.clone();
        merge_sort(&mut a);
        enable();
        merge_sort(&mut b);
        disable();
        assert_eq!(a, b);
    }

    #[test]
    fn tracing_reports_true_global_indices_across_recursion() {
        enable();
        let mut input = vec![5, 1, 4, 2, 8, 7, 3, 6];
        merge_sort(&mut input);
        let events = take_events();
        disable();

        // Every recorded index must be a valid position in the ORIGINAL
        // 8-element array — proves the offset-free recursion scheme reports
        // global, not sub-slice-local, indices even for comparisons deep in
        // the recursion (e.g. within arr[4..6]).
        for event in &events {
            match event {
                Event::Compare { i, j } => {
                    assert!(*i < 8, "Compare index {i} out of global range");
                    assert!(*j < 8, "Compare index {j} out of global range");
                }
                Event::Set { i, .. } => assert!(*i < 8, "Set index {i} out of global range"),
                Event::MarkSorted { i } => assert!(*i < 8),
                Event::Swap { .. } => panic!("merge sort should never emit Swap events"),
            }
        }
        assert!(events.iter().any(|e| matches!(e, Event::Compare { .. })));
        assert!(events.iter().any(|e| matches!(e, Event::Set { .. })));
    }
}
