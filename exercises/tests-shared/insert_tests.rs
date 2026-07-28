// Shared between exercises/linked-list/src/insert.rs (the learner-facing
// skeleton) and exercises/linked-list-solutions/src/insert.rs (the
// reference solution), via `include!()` — same convention as sort/search.
#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    #[test]
    fn push_front_attaches_at_the_head() {
        let mut list = SinglyLinkedList::new();
        list.push_front(2);
        list.push_front(1);
        assert_eq!(list.to_vec(), vec![1, 2]);
    }

    #[test]
    fn push_back_attaches_at_the_end() {
        let mut list = SinglyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.to_vec(), vec![1, 2]);
    }

    #[test]
    fn push_front_and_push_back_together() {
        let mut list = SinglyLinkedList::new();
        list.push_front(2);
        list.push_front(1);
        list.push_back(3);
        assert_eq!(list.to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let mut a = SinglyLinkedList::new();
        a.push_back(1);
        a.push_back(2);
        enable();
        let mut b = SinglyLinkedList::new();
        b.push_back(1);
        b.push_back(2);
        disable();
        assert_eq!(a.to_vec(), b.to_vec());
    }

    #[test]
    fn tracing_enabled_captures_insert_events() {
        enable();
        let mut list = SinglyLinkedList::new();
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
        // Regression test: a passing exercise could still emit an
        // out-of-bounds insert index if the index isn't actually tied to
        // the list's real length. Replay the recorded events against a
        // growable Vec exactly like the CLI's renderer does (arr.insert),
        // and confirm it doesn't panic and matches to_vec()'s real result.
        enable();
        let mut list = SinglyLinkedList::new();
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
