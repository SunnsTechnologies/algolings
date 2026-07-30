// Shared between exercises/linked-list/src/doubly_converge.rs (the
// learner-facing skeleton) and exercises/linked-list-solutions/src/doubly_converge.rs
// (the reference solution), via `include!()` — same convention as
// doubly_contains_tests.rs.
#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    #[test]
    fn finds_the_target_via_the_left_pointer() {
        let list = DoublyLinkedList::from_values(&[10, 20, 30, 40, 50]);
        assert!(list.contains_converging(10));
    }

    #[test]
    fn finds_the_target_via_the_right_pointer() {
        let list = DoublyLinkedList::from_values(&[10, 20, 30, 40, 50]);
        assert!(list.contains_converging(50));
    }

    #[test]
    fn finds_the_middle_of_an_odd_length_list() {
        let list = DoublyLinkedList::from_values(&[10, 20, 30]);
        assert!(list.contains_converging(20));
    }

    #[test]
    fn returns_false_for_a_missing_value() {
        let list = DoublyLinkedList::from_values(&[10, 20, 30, 40, 50]);
        assert!(!list.contains_converging(99));
    }

    #[test]
    fn handles_an_empty_list_without_underflowing() {
        let list = DoublyLinkedList::new();

        enable();
        let found_it = list.contains_converging(1);
        let events = take_events();
        disable();

        assert!(!found_it);
        assert!(events.is_empty());
    }

    #[test]
    fn handles_a_single_element_list_checking_it_exactly_once() {
        let list = DoublyLinkedList::from_values(&[42]);

        enable();
        let found_it = list.contains_converging(42);
        let events = take_events();
        disable();

        assert!(found_it);
        assert_eq!(
            events,
            vec![Event::Converge { left: 0, right: 0 }, Event::Found { i: 0 }],
            "a single-element list has li == ri from the start — it must be \
             checked exactly once, not twice"
        );
    }

    #[test]
    fn single_element_list_with_no_match_still_checks_exactly_once() {
        let list = DoublyLinkedList::from_values(&[42]);

        enable();
        let found_it = list.contains_converging(99);
        let events = take_events();
        disable();

        assert!(!found_it);
        assert_eq!(events, vec![Event::Converge { left: 0, right: 0 }]);
    }

    #[test]
    fn even_length_list_checks_every_index_exactly_once() {
        let list = DoublyLinkedList::from_values(&[10, 20, 30, 40]);

        enable();
        let found_it = list.contains_converging(99);
        let events = take_events();
        disable();

        assert!(!found_it);
        let mut visited: Vec<usize> = events
            .iter()
            .flat_map(|e| match *e {
                Event::Converge { left, right } => vec![left, right],
                _ => vec![],
            })
            .collect();
        visited.sort_unstable();
        assert_eq!(
            visited,
            vec![0, 1, 2, 3],
            "every index must be covered exactly once, even when the two \
             pointers cross between an even number of elements"
        );
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let list = DoublyLinkedList::from_values(&[10, 20, 30, 40, 50]);
        let a = list.contains_converging(30);
        enable();
        let b = list.contains_converging(30);
        disable();
        assert_eq!(a, b);
    }

    #[test]
    fn tracing_enabled_captures_the_exact_sequence_of_converge_events() {
        enable();
        let list = DoublyLinkedList::from_values(&[10, 20, 30, 40, 50]);
        let result = list.contains_converging(30);
        let events = take_events();
        disable();

        assert!(result);
        assert_eq!(
            events,
            vec![
                Event::Converge { left: 0, right: 4 },
                Event::Converge { left: 1, right: 3 },
                Event::Converge { left: 2, right: 2 },
                Event::Found { i: 2 },
            ]
        );
    }

    #[test]
    fn tracing_emits_no_found_event_when_the_value_is_missing() {
        enable();
        let list = DoublyLinkedList::from_values(&[10, 20, 30, 40, 50]);
        let result = list.contains_converging(99);
        let events = take_events();
        disable();
        assert!(!result);
        assert!(!events.iter().any(|e| matches!(e, Event::Found { .. })));
    }
}
