// Shared between exercises/linked-list/src/doubly_push.rs (the
// learner-facing skeleton) and exercises/linked-list-solutions/src/doubly_push.rs
// (the reference solution), via `include!()` — same convention as the
// singly-linked exercises.
#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    #[test]
    fn push_front_attaches_at_the_head() {
        let mut list = DoublyLinkedList::new();
        list.push_front(2);
        list.push_front(1);
        assert_eq!(list.to_vec(), vec![1, 2]);
    }

    #[test]
    fn push_back_attaches_at_the_tail() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.to_vec(), vec![1, 2]);
    }

    #[test]
    fn push_front_and_push_back_together() {
        let mut list = DoublyLinkedList::new();
        list.push_front(2);
        list.push_front(1);
        list.push_back(3);
        assert_eq!(list.to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn backward_links_are_wired_correctly() {
        // The one thing that actually makes this exercise doubly-linked:
        // push_front/push_back must ALSO set prev, not just next. This is
        // the only test that would fail if a learner skipped prev
        // entirely, wired it backward, or reused Rc where Weak belongs.
        let mut list = DoublyLinkedList::new();
        list.push_front(2);
        list.push_front(1);
        list.push_back(3);
        assert_eq!(
            list.to_vec_backward(),
            vec![3, 2, 1],
            "to_vec_backward() should be the exact reverse of to_vec() — if it \
             isn't, prev/Weak wasn't wired correctly"
        );
    }

    #[test]
    fn len_matches_the_real_number_of_nodes_after_every_push() {
        // Regression guard: len is a manually-maintained counter here, not
        // computed by walking (see doubly_list.rs's module docs) — a
        // missed or doubled increment would silently make len() lie about
        // the list's real size while still passing tests that only check
        // to_vec()'s final values.
        let mut list = DoublyLinkedList::new();
        assert_eq!(list.len(), list.to_vec().len());
        list.push_front(1);
        assert_eq!(list.len(), list.to_vec().len());
        list.push_back(2);
        assert_eq!(list.len(), list.to_vec().len());
        list.push_front(3);
        assert_eq!(list.len(), list.to_vec().len());
    }

    #[test]
    fn handles_pushing_onto_an_empty_list() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        assert_eq!(list.to_vec(), vec![1]);
        assert_eq!(list.to_vec_backward(), vec![1]);
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let mut a = DoublyLinkedList::new();
        a.push_back(1);
        a.push_back(2);
        enable();
        let mut b = DoublyLinkedList::new();
        b.push_back(1);
        b.push_back(2);
        disable();
        assert_eq!(a.to_vec(), b.to_vec());
    }

    #[test]
    fn tracing_enabled_captures_insert_events() {
        enable();
        let mut list = DoublyLinkedList::new();
        list.push_back(10);
        list.push_back(20);
        let events = take_events();
        disable();
        assert_eq!(
            events,
            vec![
                Event::Insert { i: 0, value: 10 },
                Event::Insert { i: 1, value: 20 },
            ]
        );
    }

    #[test]
    fn replaying_recorded_events_does_not_panic_and_matches_final_state() {
        // Same reasoning as the singly-linked insert exercise's equivalent
        // test — this exercise's insertion index comes from the manually
        // maintained len field, not a walk, so it's worth confirming the
        // recorded index is still tied to reality.
        enable();
        let mut list = DoublyLinkedList::new();
        list.push_front(2);
        list.push_front(1);
        list.push_back(3);
        let events = take_events();
        disable();

        let mut replayed: Vec<i32> = Vec::new();
        for event in &events {
            if let Event::Insert { i, value } = event {
                replayed.insert(*i, *value as i32);
            }
        }
        assert_eq!(replayed, list.to_vec());
    }
}
