#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    #[test]
    fn sorts_correctly() {
        let mut input = vec![10, 7, 8, 9, 1, 5];
        quick_sort(&mut input);
        assert_eq!(input, vec![1, 5, 7, 8, 9, 10]);
    }

    #[test]
    fn handles_empty_and_single_element() {
        let mut empty: Vec<i32> = vec![];
        quick_sort(&mut empty);
        assert_eq!(empty, Vec::<i32>::new());

        let mut single = vec![1];
        quick_sort(&mut single);
        assert_eq!(single, vec![1]);
    }

    #[test]
    fn already_sorted_is_unaffected() {
        let mut input = vec![1, 2, 3, 4];
        quick_sort(&mut input);
        assert_eq!(input, vec![1, 2, 3, 4]);
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let mut a = vec![10, 7, 8, 9, 1, 5];
        let mut b = a.clone();
        quick_sort(&mut a);
        enable();
        quick_sort(&mut b);
        disable();
        assert_eq!(a, b);
    }

    #[test]
    fn tracing_enabled_captures_compare_and_swap_events() {
        enable();
        let mut input = vec![2, 1];
        quick_sort(&mut input);
        let events = take_events();
        disable();
        assert!(events.iter().any(|e| matches!(e, Event::Compare { .. })));
        assert!(events.iter().any(|e| matches!(e, Event::Swap { .. })));
    }
}
