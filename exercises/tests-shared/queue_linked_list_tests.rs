// Shared between exercises/stacks-queues/src/queue_linked_list.rs (the
// learner-facing skeleton) and
// exercises/stacks-queues-solutions/src/queue_linked_list.rs (the
// reference solution), via `include!()`.
//
// All assertions go through to_vec()/is_empty()/len() rather than poking
// `head`/`tail` directly — at a single-element queue, `head` and `tail`
// are the SAME Rc<RefCell<Node>>, and holding overlapping borrow()/
// borrow_mut() calls across both fields panics with BorrowMutError.
#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    #[test]
    fn enqueue_on_an_empty_queue_sets_up_head_and_tail() {
        let mut queue = LinkedQueue::new();
        queue.enqueue(1);
        assert_eq!(queue.to_vec(), vec![1]);
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn enqueue_adds_to_the_back() {
        let mut queue = LinkedQueue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(queue.to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn dequeue_on_an_empty_queue_returns_none() {
        let mut queue = LinkedQueue::new();
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn dequeue_removes_and_returns_the_front_value_fifo() {
        let mut queue = LinkedQueue::from_values(&[1, 2, 3]);
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn dequeuing_the_only_element_empties_the_queue_completely() {
        let mut queue = LinkedQueue::from_values(&[1]);
        assert_eq!(queue.dequeue(), Some(1));
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.to_vec(), Vec::<i32>::new());
    }

    #[test]
    fn enqueue_after_emptying_the_queue_sets_up_a_fresh_head_and_tail() {
        // Regression test: if dequeue forgets to null `tail` when the
        // queue becomes empty, a later enqueue's "wire onto the old tail"
        // branch links the new node onto an orphaned node that self.head
        // no longer points to — the new value never actually reaches the
        // queue. This only shows up on a SECOND enqueue after emptying,
        // not on the dequeue itself, which is why it needs its own test.
        let mut queue = LinkedQueue::from_values(&[1]);
        assert_eq!(queue.dequeue(), Some(1));
        assert!(queue.is_empty());

        queue.enqueue(2);
        assert_eq!(queue.to_vec(), vec![2]);

        queue.enqueue(3);
        assert_eq!(queue.to_vec(), vec![2, 3]);
    }

    #[test]
    fn enqueue_and_dequeue_interleave_correctly_fifo() {
        let mut queue = LinkedQueue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        assert_eq!(queue.dequeue(), Some(1));
        queue.enqueue(3);
        assert_eq!(queue.to_vec(), vec![2, 3]);
    }

    #[test]
    fn len_matches_the_real_number_of_nodes_after_every_operation() {
        let mut queue = LinkedQueue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        assert_eq!(queue.len(), 2);
        queue.dequeue();
        assert_eq!(queue.len(), 1);
        queue.dequeue();
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn peek_on_an_empty_queue_returns_none() {
        let queue = LinkedQueue::new();
        assert_eq!(queue.peek(), None);
    }

    #[test]
    fn peek_reads_the_front_without_removing_it() {
        let queue = LinkedQueue::from_values(&[1, 2, 3]);
        assert_eq!(queue.peek(), Some(1));
        assert_eq!(queue.to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let mut a = LinkedQueue::from_values(&[1, 2]);
        let dequeued_a = a.dequeue();
        enable();
        let mut b = LinkedQueue::from_values(&[1, 2]);
        let dequeued_b = b.dequeue();
        disable();
        assert_eq!(dequeued_a, dequeued_b);
    }

    #[test]
    fn tracing_enabled_captures_the_exact_index_of_each_operation() {
        enable();
        let mut queue = LinkedQueue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        let _ = queue.peek();
        let _ = queue.dequeue();
        let events = take_events();
        disable();

        assert_eq!(
            events,
            vec![
                Event::Insert { i: 0, value: 1 },
                Event::Insert { i: 1, value: 2 },
                Event::Probe { i: 0 },
                Event::Remove { i: 0 },
            ]
        );
    }

    #[test]
    fn tracing_emits_no_event_for_dequeue_or_peek_on_an_empty_queue() {
        enable();
        let mut queue = LinkedQueue::new();
        let _ = queue.dequeue();
        let _ = queue.peek();
        let events = take_events();
        disable();
        assert!(events.is_empty());
    }

    #[test]
    fn replaying_recorded_events_matches_the_real_final_state() {
        enable();
        let mut queue = LinkedQueue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        let _ = queue.dequeue();
        let events = take_events();
        disable();

        let mut replayed: Vec<i32> = Vec::new();
        for event in &events {
            match event {
                Event::Insert { i, value } => replayed.insert(*i, *value as i32),
                Event::Remove { i } => {
                    replayed.remove(*i);
                }
                Event::Probe { .. } => {}
                other => panic!("queue_linked_list should never emit {other:?}"),
            }
        }
        assert_eq!(replayed, queue.to_vec());
    }
}
