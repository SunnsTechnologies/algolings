// Shared between exercises/search/src/binary.rs (the learner-facing
// skeleton) and exercises/search-solutions/src/binary.rs (the reference
// solution), via `include!()` — same convention as the sort module.
#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    const SORTED: [i32; 10] = [2, 5, 8, 12, 16, 23, 38, 56, 72, 91];

    #[test]
    fn finds_an_existing_element() {
        assert_eq!(binary_search(&SORTED, 23), Some(5));
    }

    #[test]
    fn returns_none_for_a_missing_element() {
        assert_eq!(binary_search(&SORTED, 7), None);
    }

    #[test]
    fn handles_an_empty_slice() {
        let empty: [i32; 0] = [];
        assert_eq!(binary_search(&empty, 1), None);
    }

    #[test]
    fn finds_the_first_and_last_elements() {
        assert_eq!(binary_search(&SORTED, 2), Some(0));
        assert_eq!(binary_search(&SORTED, 91), Some(9));
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let a = binary_search(&SORTED, 23);
        enable();
        let b = binary_search(&SORTED, 23);
        disable();
        assert_eq!(a, b);
    }

    #[test]
    fn tracing_enabled_captures_probe_narrow_range_and_found_events() {
        // 16 sits away from the very first midpoint checked (whichever
        // bounds convention a correct implementation uses), so finding it
        // genuinely requires at least one range narrowing, not just a
        // lucky first probe.
        enable();
        let result = binary_search(&SORTED, 16);
        let events = take_events();
        disable();
        assert_eq!(result, Some(4));
        assert!(events.iter().any(|e| matches!(e, Event::Probe { .. })));
        assert!(events
            .iter()
            .any(|e| matches!(e, Event::NarrowRange { .. })));
        assert!(events.contains(&Event::Found { i: 4 }));
    }

    #[test]
    fn tracing_emits_no_found_event_when_the_target_is_missing() {
        enable();
        let result = binary_search(&SORTED, 7);
        let events = take_events();
        disable();
        assert_eq!(result, None);
        assert!(!events.iter().any(|e| matches!(e, Event::Found { .. })));
    }
}
