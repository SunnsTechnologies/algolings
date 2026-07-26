#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    #[test]
    fn sorts_correctly() {
        let mut input = vec![170, 45, 75, 90, 802, 24, 2, 66];
        radix_sort(&mut input);
        assert_eq!(input, vec![2, 24, 45, 66, 75, 90, 170, 802]);
    }

    #[test]
    fn handles_empty_and_single_element() {
        let mut empty: Vec<i32> = vec![];
        radix_sort(&mut empty);
        assert_eq!(empty, Vec::<i32>::new());

        let mut single = vec![7];
        radix_sort(&mut single);
        assert_eq!(single, vec![7]);
    }

    #[test]
    fn handles_all_same_digit_count() {
        let mut input = vec![5, 1, 4, 2, 8];
        radix_sort(&mut input);
        assert_eq!(input, vec![1, 2, 4, 5, 8]);
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let mut a = vec![170, 45, 75, 90, 802, 24, 2, 66];
        let mut b = a.clone();
        radix_sort(&mut a);
        enable();
        radix_sort(&mut b);
        disable();
        assert_eq!(a, b);
    }

    #[test]
    fn tracing_emits_only_set_and_mark_sorted_no_compare_or_swap() {
        enable();
        let mut input = vec![170, 45, 75, 90, 802, 24, 2, 66];
        radix_sort(&mut input);
        let events = take_events();
        disable();
        assert!(
            events
                .iter()
                .all(|e| matches!(e, Event::Set { .. } | Event::MarkSorted { .. })),
            "radix sort must not emit Compare or Swap events: {events:?}"
        );
        assert!(events.iter().any(|e| matches!(e, Event::Set { .. })));
    }
}
