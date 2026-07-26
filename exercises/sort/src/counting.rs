use algolings_trace::{mark_sorted, set_at};

/// Reference solution. Counting sort has no comparisons or swaps at all —
/// this is the deliberate test case for whether the event vocabulary
/// (Compare/Swap/Set/MarkSorted) covers a non-comparison sort. It does: the
/// whole algorithm is representable as `Set` events placing each value into
/// its final output position, with zero `Compare`/`Swap` events emitted.
/// The counting/bucketing phase itself is not traced — only the placement
/// into the output array is, since that's the part that maps onto the
/// same "array with positions" visualization the compare/swap sorts use.
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
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    #[test]
    fn sorts_correctly() {
        let mut input = vec![5, 1, 4, 2, 8, 1, 4];
        counting_sort(&mut input);
        assert_eq!(input, vec![1, 1, 2, 4, 4, 5, 8]);
    }

    #[test]
    fn handles_empty_and_single_element() {
        let mut empty: Vec<i32> = vec![];
        counting_sort(&mut empty);
        assert_eq!(empty, Vec::<i32>::new());

        let mut single = vec![7];
        counting_sort(&mut single);
        assert_eq!(single, vec![7]);
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let mut a = vec![5, 1, 4, 2, 8, 1, 4];
        let mut b = a.clone();
        counting_sort(&mut a);
        enable();
        counting_sort(&mut b);
        disable();
        assert_eq!(a, b);
    }

    #[test]
    fn tracing_emits_only_set_and_mark_sorted_no_compare_or_swap() {
        enable();
        let mut input = vec![5, 1, 4, 2, 8];
        counting_sort(&mut input);
        let events = take_events();
        disable();

        assert!(
            events
                .iter()
                .all(|e| matches!(e, Event::Set { .. } | Event::MarkSorted { .. })),
            "counting sort must not emit Compare or Swap events: {events:?}"
        );
        assert!(events.iter().any(|e| matches!(e, Event::Set { .. })));
    }
}
