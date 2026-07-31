// Shared between exercises/stacks-queues/src/stack_vec.rs (the
// learner-facing skeleton) and exercises/stacks-queues-solutions/src/stack_vec.rs
// (the reference solution), via `include!()`.
#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    #[test]
    fn push_adds_to_the_top() {
        let mut stack = VecStack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.to_vec(), vec![1, 2]);
    }

    #[test]
    fn pop_on_an_empty_stack_returns_none() {
        let mut stack = VecStack::new();
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn pop_removes_and_returns_the_top_value_lifo() {
        let mut stack = VecStack::from_values(&[1, 2, 3]);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn push_and_pop_interleave_correctly() {
        let mut stack = VecStack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2));
        stack.push(3);
        assert_eq!(stack.to_vec(), vec![1, 3]);
    }

    #[test]
    fn peek_on_an_empty_stack_returns_none() {
        let stack = VecStack::new();
        assert_eq!(stack.peek(), None);
    }

    #[test]
    fn peek_reads_the_top_without_removing_it() {
        let stack = VecStack::from_values(&[1, 2, 3]);
        assert_eq!(stack.peek(), Some(3));
        assert_eq!(stack.to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let mut a = VecStack::from_values(&[1, 2]);
        let popped_a = a.pop();
        enable();
        let mut b = VecStack::from_values(&[1, 2]);
        let popped_b = b.pop();
        disable();
        assert_eq!(popped_a, popped_b);
    }

    #[test]
    fn tracing_enabled_captures_the_exact_index_of_each_operation() {
        enable();
        let mut stack = VecStack::new();
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
                Event::Insert { i: 1, value: 2 },
                Event::Probe { i: 1 },
                Event::Remove { i: 1 },
            ]
        );
    }

    #[test]
    fn tracing_emits_no_event_for_pop_or_peek_on_an_empty_stack() {
        enable();
        let mut stack = VecStack::new();
        let _ = stack.pop();
        let _ = stack.peek();
        let events = take_events();
        disable();
        assert!(events.is_empty());
    }

    #[test]
    fn replaying_recorded_events_matches_the_real_final_state() {
        enable();
        let mut stack = VecStack::new();
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
                other => panic!("stack_vec should never emit {other:?}"),
            }
        }
        assert_eq!(replayed, stack.to_vec());
    }
}
