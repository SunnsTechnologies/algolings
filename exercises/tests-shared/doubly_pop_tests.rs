// Shared between exercises/linked-list/src/doubly_pop.rs (the
// learner-facing skeleton) and exercises/linked-list-solutions/src/doubly_pop.rs
// (the reference solution), via `include!()` — same convention as the
// other linked-list exercises.
#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    #[test]
    fn pop_front_removes_and_returns_the_first_value() {
        let mut list = DoublyLinkedList::from_values(&[1, 2, 3]);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.to_vec(), vec![2, 3]);
    }

    #[test]
    fn pop_back_removes_and_returns_the_last_value() {
        let mut list = DoublyLinkedList::from_values(&[1, 2, 3]);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.to_vec(), vec![1, 2]);
    }

    #[test]
    fn pop_front_on_a_single_element_list_empties_it_completely() {
        let mut list = DoublyLinkedList::from_values(&[1]);
        assert_eq!(list.pop_front(), Some(1));
        assert!(list.is_empty());
        assert_eq!(list.to_vec(), Vec::<i32>::new());
        assert_eq!(list.to_vec_backward(), Vec::<i32>::new());
    }

    #[test]
    fn pop_back_on_a_single_element_list_empties_it_completely() {
        let mut list = DoublyLinkedList::from_values(&[1]);
        assert_eq!(list.pop_back(), Some(1));
        assert!(list.is_empty());
        assert_eq!(list.to_vec(), Vec::<i32>::new());
        assert_eq!(list.to_vec_backward(), Vec::<i32>::new());
    }

    #[test]
    fn pop_front_then_pop_back_on_a_two_element_list() {
        // Regression test: per-method tests structurally can't catch a
        // stale `prev` left by pop_front. If pop_front forgets to null the
        // new head's prev, to_vec_backward() STILL looks correct right
        // afterward — a dangling Weak upgrades to None at exactly the
        // point a genuinely-empty prev would too, since the freed node's
        // memory is just gone. The bug only surfaces when pop_back runs
        // on the SAME list afterward and tries to upgrade that dangling
        // Weak while it's still Some(..) — which is exactly what this
        // sequence exercises.
        let mut list = DoublyLinkedList::from_values(&[1, 2]);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.to_vec(), vec![2]);
        assert_eq!(
            list.to_vec_backward(),
            vec![2],
            "the remaining node must genuinely be both head and tail"
        );

        assert_eq!(list.pop_back(), Some(2));
        assert!(list.is_empty());
    }

    #[test]
    fn pop_front_and_pop_back_on_an_empty_list_return_none() {
        let mut list = DoublyLinkedList::new();
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn backward_links_stay_correct_after_removal_from_both_ends() {
        // The removal-side equivalent of doubly_push's
        // backward_links_are_wired_correctly.
        let mut list = DoublyLinkedList::from_values(&[1, 2, 3, 4]);
        list.pop_front();
        list.pop_back();
        assert_eq!(list.to_vec(), vec![2, 3]);
        assert_eq!(list.to_vec_backward(), vec![3, 2]);
    }

    #[test]
    fn len_matches_the_real_number_of_nodes_after_every_pop() {
        let mut list = DoublyLinkedList::from_values(&[1, 2, 3, 4]);
        assert_eq!(list.len(), list.to_vec().len());
        list.pop_front();
        assert_eq!(list.len(), list.to_vec().len());
        list.pop_back();
        assert_eq!(list.len(), list.to_vec().len());
        list.pop_front();
        assert_eq!(list.len(), list.to_vec().len());
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let mut a = DoublyLinkedList::from_values(&[1, 2, 3]);
        let popped_a = a.pop_front();
        enable();
        let mut b = DoublyLinkedList::from_values(&[1, 2, 3]);
        let popped_b = b.pop_front();
        disable();
        assert_eq!(popped_a, popped_b);
        assert_eq!(a.to_vec(), b.to_vec());
    }

    #[test]
    fn tracing_enabled_captures_the_exact_index_of_each_removal() {
        // Exact indices, not just "some Remove event exists" — a bug that
        // computes the index at the wrong time (e.g. reading the length
        // AFTER decrementing instead of before) would still emit an
        // in-bounds, non-panicking index and pass a presence-only check.
        enable();
        let mut list = DoublyLinkedList::from_values(&[1, 2, 3]);
        list.pop_front();
        let front_events = take_events();
        list.pop_back();
        let back_events = take_events();
        disable();
        assert_eq!(front_events, vec![Event::Remove { i: 0 }]);
        assert_eq!(back_events, vec![Event::Remove { i: 1 }]);
    }

    #[test]
    fn tracing_emits_no_event_when_popping_an_empty_list() {
        enable();
        let mut list = DoublyLinkedList::new();
        list.pop_front();
        list.pop_back();
        let events = take_events();
        disable();
        assert!(events.is_empty());
    }

    #[test]
    fn replaying_recorded_events_matches_the_real_final_state() {
        // Not just "doesn't panic" — asserts the replayed events actually
        // reconstruct the same final state as the real list, matching
        // remove_tests.rs's established convention. Catches a wrong (but
        // in-bounds) index that a presence-only or no-panic-only check
        // would miss.
        enable();
        let mut list = DoublyLinkedList::from_values(&[1, 2, 3, 4]);
        list.pop_front();
        list.pop_back();
        let events = take_events();
        disable();

        let mut replayed = vec![1, 2, 3, 4];
        for event in &events {
            if let Event::Remove { i } = event {
                replayed.remove(*i);
            }
        }
        assert_eq!(replayed, list.to_vec());
    }
}
