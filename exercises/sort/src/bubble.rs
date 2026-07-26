use algolings_trace::{cmp_lt, mark_sorted, swap};

/// Reference solution. Signature is exactly what a learner would write —
/// `arr: &mut [i32]` — the only thing that differs from a from-scratch
/// idiomatic bubble sort is calling `cmp_lt`/`swap` instead of `<`/`.swap()`.
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
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    #[test]
    fn sorts_correctly() {
        let mut input = vec![5, 1, 4, 2, 8];
        bubble_sort(&mut input);
        assert_eq!(input, vec![1, 2, 4, 5, 8]);
    }

    #[test]
    fn handles_empty_and_single_element() {
        let mut empty: Vec<i32> = vec![];
        bubble_sort(&mut empty);
        assert_eq!(empty, Vec::<i32>::new());

        let mut single = vec![1];
        bubble_sort(&mut single);
        assert_eq!(single, vec![1]);
    }

    #[test]
    fn already_sorted_is_unaffected() {
        let mut input = vec![1, 2, 3, 4];
        bubble_sort(&mut input);
        assert_eq!(input, vec![1, 2, 3, 4]);
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let mut a = vec![5, 1, 4, 2, 8];
        let mut b = a.clone();
        bubble_sort(&mut a);
        // A second run with tracing enabled must produce the identical
        // sorted output — proves cmp_lt/swap's tracing hook never changes
        // the actual sort behavior, only whether it records events.
        enable();
        bubble_sort(&mut b);
        disable();
        assert_eq!(a, b);
    }

    #[test]
    fn tracing_enabled_captures_compare_and_swap_events() {
        enable();
        let mut input = vec![2, 1];
        bubble_sort(&mut input);
        let events = take_events();
        disable();
        assert!(events.contains(&Event::Compare { i: 1, j: 0 }));
        assert!(events.contains(&Event::Swap { i: 0, j: 1 }));
    }
}
