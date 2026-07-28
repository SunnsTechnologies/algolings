// Shared between exercises/linked-list/src/remove.rs (the learner-facing
// skeleton) and exercises/linked-list-solutions/src/remove.rs (the
// reference solution), via `include!()` — same convention as sort/search.
#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    #[test]
    fn removes_an_existing_value() {
        let mut list = SinglyLinkedList::from_values(&[10, 20, 30]);
        assert!(list.remove(20));
        assert_eq!(list.to_vec(), vec![10, 30]);
    }

    #[test]
    fn returns_false_for_a_missing_value() {
        let mut list = SinglyLinkedList::from_values(&[10, 20, 30]);
        assert!(!list.remove(99));
        assert_eq!(list.to_vec(), vec![10, 20, 30]);
    }

    #[test]
    fn removes_the_head() {
        let mut list = SinglyLinkedList::from_values(&[10, 20, 30]);
        assert!(list.remove(10));
        assert_eq!(list.to_vec(), vec![20, 30]);
    }

    #[test]
    fn handles_an_empty_list() {
        let mut list = SinglyLinkedList::new();
        assert!(!list.remove(1));
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let mut a = SinglyLinkedList::from_values(&[10, 20, 30]);
        let removed_a = a.remove(20);
        enable();
        let mut b = SinglyLinkedList::from_values(&[10, 20, 30]);
        let removed_b = b.remove(20);
        disable();
        assert_eq!(removed_a, removed_b);
        assert_eq!(a.to_vec(), b.to_vec());
    }

    #[test]
    fn tracing_enabled_captures_a_remove_event() {
        enable();
        let mut list = SinglyLinkedList::from_values(&[10, 20, 30]);
        list.remove(20);
        let events = take_events();
        disable();
        assert_eq!(events, vec![Event::Remove { i: 1 }]);
    }

    #[test]
    fn tracing_emits_no_event_when_the_value_is_missing() {
        enable();
        let mut list = SinglyLinkedList::from_values(&[10, 20, 30]);
        list.remove(99);
        let events = take_events();
        disable();
        assert!(events.is_empty());
    }

    #[test]
    fn replaying_recorded_events_does_not_panic_and_matches_final_state() {
        // Regression test: same reasoning as insert_tests.rs — replay the
        // recorded Remove events against a growable Vec exactly like the
        // CLI's renderer does (arr.remove), confirm no panic and the
        // result matches to_vec()'s real final state.
        enable();
        let mut list = SinglyLinkedList::from_values(&[10, 20, 30]);
        list.remove(20);
        let events = take_events();
        disable();

        let mut replayed = vec![10, 20, 30];
        for event in &events {
            if let Event::Remove { i } = event {
                replayed.remove(*i);
            }
        }
        assert_eq!(replayed, list.to_vec());
    }
}
