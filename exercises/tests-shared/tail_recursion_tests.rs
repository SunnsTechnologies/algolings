// Shared between exercises/recursion-backtracking/src/tail_recursion.rs
// (the learner-facing skeleton) and
// exercises/recursion-backtracking-solutions/src/tail_recursion.rs (the
// reference solution), via `include!()`.
#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    #[test]
    fn factorial_tail_of_zero_is_one() {
        assert_eq!(factorial_tail(0, 1), 1);
    }

    #[test]
    fn factorial_tail_computes_correctly() {
        assert_eq!(factorial_tail(5, 1), 120);
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let a = factorial_tail(5, 1);
        enable();
        let b = factorial_tail(5, 1);
        disable();
        assert_eq!(a, b);
    }

    #[test]
    fn tracing_enabled_still_shows_a_growing_and_shrinking_stack() {
        // Even though the accumulator carries the result forward instead
        // of computing on the way back up, Rust doesn't guarantee tail
        // call optimization — the trace shows the SAME depth-4 stack
        // factorial's plain recursive version does. The rewrite changes
        // what's computed at each step, not whether frames pile up.
        enable();
        factorial_tail(3, 1);
        let events = take_events();
        disable();

        assert_eq!(
            events,
            vec![
                Event::Insert { i: 0, value: 3 },
                Event::Insert { i: 1, value: 2 },
                Event::Insert { i: 2, value: 1 },
                Event::Insert { i: 3, value: 0 },
                Event::Remove { i: 3 },
                Event::Remove { i: 2 },
                Event::Remove { i: 1 },
                Event::Remove { i: 0 },
            ]
        );
    }

    #[test]
    fn every_insert_has_a_matching_remove() {
        enable();
        factorial_tail(6, 1);
        let events = take_events();
        disable();

        let inserts = events
            .iter()
            .filter(|e| matches!(e, Event::Insert { .. }))
            .count();
        let removes = events
            .iter()
            .filter(|e| matches!(e, Event::Remove { .. }))
            .count();
        assert_eq!(inserts, removes, "the call stack must fully unwind");
    }
}
