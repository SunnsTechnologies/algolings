#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    #[test]
    fn sorts_correctly() {
        let mut input = vec![29, 10, 14, 37, 13];
        selection_sort(&mut input);
        assert_eq!(input, vec![10, 13, 14, 29, 37]);
    }

    #[test]
    fn handles_empty_and_single_element() {
        let mut empty: Vec<i32> = vec![];
        selection_sort(&mut empty);
        assert_eq!(empty, Vec::<i32>::new());

        let mut single = vec![1];
        selection_sort(&mut single);
        assert_eq!(single, vec![1]);
    }

    #[test]
    fn already_sorted_is_unaffected() {
        let mut input = vec![1, 2, 3, 4];
        selection_sort(&mut input);
        assert_eq!(input, vec![1, 2, 3, 4]);
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let mut a = vec![29, 10, 14, 37, 13];
        let mut b = a.clone();
        selection_sort(&mut a);
        enable();
        selection_sort(&mut b);
        disable();
        assert_eq!(a, b);
    }

    #[test]
    fn tracing_enabled_captures_compare_and_swap_events() {
        enable();
        let mut input = vec![2, 1];
        selection_sort(&mut input);
        let events = take_events();
        disable();
        assert!(events.iter().any(|e| matches!(e, Event::Compare { .. })));
        assert!(events.contains(&Event::Swap { i: 0, j: 1 }));
    }

    #[test]
    fn does_not_emit_a_no_op_swap_when_the_minimum_is_already_in_place() {
        // Regression-style guard: if position i is already the minimum,
        // an implementation that unconditionally calls swap(arr, i, i)
        // would emit a Swap event for something that visibly didn't move —
        // a trace that lies to the viewer. The reference solution guards
        // against this; this test enforces the same discipline on the
        // learner's solution.
        enable();
        let mut input = vec![1, 5, 4, 3, 2];
        selection_sort(&mut input);
        let events = take_events();
        disable();
        assert!(
            !events.contains(&Event::Swap { i: 0, j: 0 }),
            "should not emit a self-swap when position 0 is already the minimum"
        );
    }
}
