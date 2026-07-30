// Shared between exercises/linked-list/src/doubly_contains.rs (the
// learner-facing skeleton) and exercises/linked-list-solutions/src/doubly_contains.rs
// (the reference solution), via `include!()` — same convention as
// traverse_tests.rs (the singly-linked list's equivalent exercise).
#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    #[test]
    fn finds_an_existing_value() {
        let list = DoublyLinkedList::from_values(&[10, 20, 30]);
        assert!(list.contains(20));
    }

    #[test]
    fn returns_false_for_a_missing_value() {
        let list = DoublyLinkedList::from_values(&[10, 20, 30]);
        assert!(!list.contains(99));
    }

    #[test]
    fn handles_an_empty_list() {
        let list = DoublyLinkedList::new();
        assert!(!list.contains(1));
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let list = DoublyLinkedList::from_values(&[10, 20, 30]);
        let a = list.contains(20);
        enable();
        let b = list.contains(20);
        disable();
        assert_eq!(a, b);
    }

    #[test]
    fn tracing_enabled_captures_probe_and_found_events() {
        enable();
        let list = DoublyLinkedList::from_values(&[10, 20, 30]);
        let result = list.contains(20);
        let events = take_events();
        disable();
        assert!(result);
        assert!(events.contains(&Event::Probe { i: 1 }));
        assert!(events.contains(&Event::Found { i: 1 }));
    }

    #[test]
    fn tracing_emits_no_found_event_when_the_value_is_missing() {
        enable();
        let list = DoublyLinkedList::from_values(&[10, 20, 30]);
        let result = list.contains(99);
        let events = take_events();
        disable();
        assert!(!result);
        assert!(!events.iter().any(|e| matches!(e, Event::Found { .. })));
    }
}
