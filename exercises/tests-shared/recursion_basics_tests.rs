// Shared between exercises/recursion-backtracking/src/recursion_basics.rs
// (the learner-facing skeleton) and
// exercises/recursion-backtracking-solutions/src/recursion_basics.rs (the
// reference solution), via `include!()`.
#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    #[test]
    fn factorial_of_zero_is_one() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_computes_correctly() {
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn fibonacci_base_cases() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn fibonacci_computes_correctly() {
        // 0, 1, 1, 2, 3, 5, 8
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(6), 8);
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let a = factorial(5);
        enable();
        let b = factorial(5);
        disable();
        assert_eq!(a, b);
    }

    #[test]
    fn tracing_enabled_captures_factorials_linear_call_stack() {
        enable();
        factorial(3);
        let events = take_events();
        disable();

        // A linear chain: each call is one level deeper than the last, so
        // the stack grows to depth 3 then unwinds back to 0 in order.
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
    fn tracing_enabled_captures_fibonaccis_branching_call_stack() {
        enable();
        fibonacci(3);
        let events = take_events();
        disable();

        // fib(3) = fib(2) + fib(1). fib(2) = fib(1) + fib(0). Each branch
        // fully unwinds (insert then remove at that depth) before the
        // next branch begins at the same depth — proving two branches
        // never collide at the same array position.
        assert_eq!(
            events,
            vec![
                Event::Insert { i: 0, value: 3 },
                Event::Insert { i: 1, value: 2 },
                Event::Insert { i: 2, value: 1 },
                Event::Remove { i: 2 },
                Event::Insert { i: 2, value: 0 },
                Event::Remove { i: 2 },
                Event::Remove { i: 1 },
                Event::Insert { i: 1, value: 1 },
                Event::Remove { i: 1 },
                Event::Remove { i: 0 },
            ]
        );
    }

    #[test]
    fn every_insert_has_a_matching_remove() {
        enable();
        fibonacci(6);
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
