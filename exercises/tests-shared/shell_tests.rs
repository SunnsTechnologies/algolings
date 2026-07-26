#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    #[test]
    fn sorts_correctly() {
        let mut input = vec![12, 34, 54, 2, 3];
        shell_sort(&mut input);
        assert_eq!(input, vec![2, 3, 12, 34, 54]);
    }

    #[test]
    fn handles_empty_and_single_element() {
        let mut empty: Vec<i32> = vec![];
        shell_sort(&mut empty);
        assert_eq!(empty, Vec::<i32>::new());

        let mut single = vec![1];
        shell_sort(&mut single);
        assert_eq!(single, vec![1]);
    }

    #[test]
    fn already_sorted_is_unaffected() {
        let mut input = vec![1, 2, 3, 4];
        shell_sort(&mut input);
        assert_eq!(input, vec![1, 2, 3, 4]);
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let mut a = vec![12, 34, 54, 2, 3];
        let mut b = a.clone();
        shell_sort(&mut a);
        enable();
        shell_sort(&mut b);
        disable();
        assert_eq!(a, b);
    }

    #[test]
    fn tracing_enabled_captures_compare_and_set_events() {
        enable();
        let mut input = vec![2, 1];
        shell_sort(&mut input);
        let events = take_events();
        disable();
        assert!(events.iter().any(|e| matches!(e, Event::Compare { .. })));
        assert!(events.iter().any(|e| matches!(e, Event::Set { .. })));
    }

    #[test]
    fn never_panics_on_a_gap_underflow() {
        // Regression-style guard: j - gap must never be attempted when
        // j < gap. A solution missing the `j >= gap` guard would panic
        // (usize underflow) on inputs that reach a small enough gap.
        let mut input = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        shell_sort(&mut input);
        assert_eq!(input, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
