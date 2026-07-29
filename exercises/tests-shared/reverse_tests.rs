// Shared between exercises/linked-list/src/reverse.rs (the learner-facing
// skeleton) and exercises/linked-list-solutions/src/reverse.rs (the
// reference solution), via `include!()` — same convention as
// insert/remove/traverse.
#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    #[test]
    fn reverses_a_multi_element_list() {
        let mut list = SinglyLinkedList::from_values(&[1, 2, 3]);
        list.reverse();
        assert_eq!(list.to_vec(), vec![3, 2, 1]);
    }

    #[test]
    fn handles_an_empty_list() {
        let mut list = SinglyLinkedList::new();
        list.reverse();
        assert_eq!(list.to_vec(), Vec::<i32>::new());
    }

    #[test]
    fn handles_a_single_element_list() {
        let mut list = SinglyLinkedList::from_values(&[1]);
        list.reverse();
        assert_eq!(list.to_vec(), vec![1]);
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let mut a = SinglyLinkedList::from_values(&[1, 2, 3]);
        a.reverse();
        enable();
        let mut b = SinglyLinkedList::from_values(&[1, 2, 3]);
        b.reverse();
        disable();
        assert_eq!(a.to_vec(), b.to_vec());
    }

    #[test]
    fn tracing_enabled_captures_insert_and_remove_events() {
        // Doesn't assert an exact order — any correct algorithm should
        // record SOME Insert/Remove events, not one specific sequence.
        // Asserting the literal order would fail a correct reverse that's
        // implemented differently than the reference solution.
        enable();
        let mut list = SinglyLinkedList::from_values(&[1, 2, 3]);
        list.reverse();
        let events = take_events();
        disable();
        assert!(events.iter().any(|e| matches!(e, Event::Insert { .. })));
        assert!(events.iter().any(|e| matches!(e, Event::Remove { .. })));
    }

    #[test]
    fn replaying_recorded_events_does_not_panic_and_matches_final_state() {
        // Regression test, same reasoning as insert/remove_tests.rs — and
        // not a formality here: reverse's insertion index is a computed
        // remaining-length, not a hardcoded constant, so a buggy learner
        // implementation can genuinely emit an out-of-bounds index.
        enable();
        let mut list = SinglyLinkedList::from_values(&[1, 2, 3]);
        list.reverse();
        let events = take_events();
        disable();

        let mut replayed = vec![1, 2, 3];
        for event in &events {
            match event {
                Event::Remove { i } => {
                    replayed.remove(*i);
                }
                Event::Insert { i, value } => {
                    replayed.insert(*i, *value as i32);
                }
                _ => {}
            }
        }
        assert_eq!(replayed, list.to_vec());
    }
}
