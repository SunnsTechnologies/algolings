// Shared between exercises/stacks-queues/src/stack_linked_list.rs (the
// learner-facing skeleton) and
// exercises/stacks-queues-solutions/src/stack_linked_list.rs (the
// reference solution), via `include!()`.
#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    #[test]
    fn push_adds_to_the_top() {
        let mut stack = LinkedStack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.to_vec(), vec![2, 1]);
    }

    #[test]
    fn pop_on_an_empty_stack_returns_none() {
        let mut stack = LinkedStack::new();
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn pop_removes_and_returns_the_top_value_lifo() {
        let mut stack = LinkedStack::from_values(&[1, 2, 3]);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn push_and_pop_interleave_correctly() {
        let mut stack = LinkedStack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2));
        stack.push(3);
        assert_eq!(stack.to_vec(), vec![3, 1]);
    }

    #[test]
    fn peek_on_an_empty_stack_returns_none() {
        let stack = LinkedStack::new();
        assert_eq!(stack.peek(), None);
    }

    #[test]
    fn peek_reads_the_top_without_removing_it() {
        let stack = LinkedStack::from_values(&[1, 2, 3]);
        assert_eq!(stack.peek(), Some(3));
        assert_eq!(stack.to_vec(), vec![3, 2, 1]);
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let mut a = LinkedStack::from_values(&[1, 2]);
        let popped_a = a.pop();
        enable();
        let mut b = LinkedStack::from_values(&[1, 2]);
        let popped_b = b.pop();
        disable();
        assert_eq!(popped_a, popped_b);
    }

    #[test]
    fn tracing_enabled_captures_every_operation_at_index_zero() {
        enable();
        let mut stack = LinkedStack::new();
        stack.push(1);
        stack.push(2);
        let _ = stack.peek();
        let _ = stack.pop();
        let events = take_events();
        disable();

        assert_eq!(
            events,
            vec![
                Event::Insert { i: 0, value: 1 },
                Event::Insert { i: 0, value: 2 },
                Event::Probe { i: 0 },
                Event::Remove { i: 0 },
            ]
        );
    }

    #[test]
    fn tracing_emits_no_event_for_pop_or_peek_on_an_empty_stack() {
        enable();
        let mut stack = LinkedStack::new();
        let _ = stack.pop();
        let _ = stack.peek();
        let events = take_events();
        disable();
        assert!(events.is_empty());
    }

    #[test]
    fn replaying_recorded_events_matches_the_real_final_state() {
        enable();
        let mut stack = LinkedStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        let _ = stack.pop();
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
                other => panic!("stack_linked_list should never emit {other:?}"),
            }
        }
        assert_eq!(replayed, stack.to_vec());
    }
}
