#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    #[test]
    fn sorts_correctly() {
        let mut input = vec![12, 11, 13, 5, 6];
        insertion_sort(&mut input);
        assert_eq!(input, vec![5, 6, 11, 12, 13]);
    }

    #[test]
    fn handles_empty_and_single_element() {
        let mut empty: Vec<i32> = vec![];
        insertion_sort(&mut empty);
        assert_eq!(empty, Vec::<i32>::new());

        let mut single = vec![1];
        insertion_sort(&mut single);
        assert_eq!(single, vec![1]);
    }

    #[test]
    fn already_sorted_is_unaffected() {
        let mut input = vec![1, 2, 3, 4];
        insertion_sort(&mut input);
        assert_eq!(input, vec![1, 2, 3, 4]);
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let mut a = vec![12, 11, 13, 5, 6];
        let mut b = a.clone();
        insertion_sort(&mut a);
        enable();
        insertion_sort(&mut b);
        disable();
        assert_eq!(a, b);
    }

    #[test]
    fn tracing_enabled_captures_compare_and_set_events() {
        enable();
        let mut input = vec![2, 1];
        insertion_sort(&mut input);
        let events = take_events();
        disable();
        assert!(events.iter().any(|e| matches!(e, Event::Compare { .. })));
        assert!(events.iter().any(|e| matches!(e, Event::Set { .. })));
    }
}
