// Shared between exercises/linked-list/src/floyds_cycle_detection.rs
// (the learner-facing skeleton) and
// exercises/linked-list-solutions/src/floyds_cycle_detection.rs (the
// reference solution), via `include!()`.
//
// Two STABLE display positions: position 0 always shows slow's current
// value, position 1 always shows fast's current value — mark_set on
// every real advance, reusing the exact idiom dijkstra/bellman_ford
// established (a stable slot whose value gets replaced repeatedly).
// Never call to_vec()/to_vec_backward() on a list built with a real
// cycle — they walk until next is None, which a cyclic list never
// reaches.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_list_has_no_cycle() {
        let list = DoublyLinkedList::from_values_with_cycle(&[], None);
        assert!(!list.has_cycle());
    }

    #[test]
    fn a_single_node_with_no_cycle_is_fine() {
        let list = DoublyLinkedList::from_values_with_cycle(&[1], None);
        assert!(!list.has_cycle());
    }

    #[test]
    fn a_self_loop_is_detected() {
        let list = DoublyLinkedList::from_values_with_cycle(&[1], Some(0));
        assert!(list.has_cycle());
    }

    #[test]
    fn an_ordinary_acyclic_list_has_no_cycle() {
        let list = DoublyLinkedList::from_values_with_cycle(&[1, 2, 3, 4, 5], None);
        assert!(!list.has_cycle());
    }

    #[test]
    fn a_cycle_partway_through_is_detected() {
        let list = DoublyLinkedList::from_values_with_cycle(&[1, 2, 3, 4], Some(1));
        assert!(list.has_cycle());
    }

    #[test]
    fn a_cycle_back_to_the_head_is_detected() {
        let list = DoublyLinkedList::from_values_with_cycle(&[1, 2, 3, 4, 5], Some(0));
        assert!(list.has_cycle());
    }

    #[test]
    fn a_repeated_value_does_not_look_like_a_cycle() {
        // The critical case proving this compares POINTER identity, not
        // value: 1 appears twice, at two different nodes, with no real
        // back-reference between them.
        let list = DoublyLinkedList::from_values_with_cycle(&[1, 2, 1, 3], None);
        assert!(!list.has_cycle());
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        let list = DoublyLinkedList::from_values_with_cycle(&[1, 2, 3, 4], Some(1));
        disable();
        assert!(list.has_cycle());
    }

    #[test]
    fn tracing_enabled_captures_the_exact_sequence() {
        let list = DoublyLinkedList::from_values_with_cycle(&[1, 2, 3, 4], Some(1));
        enable();
        let result = list.has_cycle();
        let events = take_events();
        assert!(result);
        assert_eq!(
            events,
            vec![
                Event::Set { i: 0, value: 2 },
                Event::Set { i: 1, value: 3 },
                Event::Set { i: 0, value: 3 },
                Event::Set { i: 1, value: 2 },
                Event::Set { i: 0, value: 4 },
                Event::Set { i: 1, value: 4 },
            ]
        );
    }
}
