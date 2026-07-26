// Shared between exercises/sort/src/bubble.rs (the learner-facing skeleton)
// and exercises/sort-solutions/src/bubble.rs (the reference solution), via
// `include!()`. One test suite, two implementations to run it against —
// this is the exercise-harness convention that keeps porting future
// exercises (step 4) from becoming copy-pasted boilerplate.
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
