// Shared between exercises/search/src/linear.rs (the learner-facing
// skeleton) and exercises/search-solutions/src/linear.rs (the reference
// solution), via `include!()` — same convention as the sort module.
#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    #[test]
    fn finds_an_existing_element() {
        let input = vec![3, 7, 2, 9, 5];
        assert_eq!(linear_search(&input, 9), Some(3));
    }

    #[test]
    fn returns_none_for_a_missing_element() {
        let input = vec![3, 7, 2, 9, 5];
        assert_eq!(linear_search(&input, 4), None);
    }

    #[test]
    fn handles_an_empty_slice() {
        let input: Vec<i32> = vec![];
        assert_eq!(linear_search(&input, 1), None);
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let input = vec![3, 7, 2, 9, 5];
        let a = linear_search(&input, 9);
        enable();
        let b = linear_search(&input, 9);
        disable();
        assert_eq!(a, b);
    }

    #[test]
    fn tracing_enabled_captures_probe_and_found_events() {
        enable();
        let input = vec![3, 7, 2, 9, 5];
        let result = linear_search(&input, 9);
        let events = take_events();
        disable();
        assert_eq!(result, Some(3));
        assert!(events.contains(&Event::Probe { i: 3 }));
        assert!(events.contains(&Event::Found { i: 3 }));
    }

    #[test]
    fn tracing_emits_no_found_event_when_the_target_is_missing() {
        enable();
        let input = vec![3, 7, 2, 9, 5];
        let result = linear_search(&input, 4);
        let events = take_events();
        disable();
        assert_eq!(result, None);
        assert!(!events.iter().any(|e| matches!(e, Event::Found { .. })));
    }
}
