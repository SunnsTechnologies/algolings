// Shared between exercises/dynamic-programming/src/house_robber.rs (the
// learner-facing skeleton) and
// exercises/dynamic-programming-solutions/src/house_robber.rs (the
// reference solution), via `include!()`.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_houses_means_nothing_to_rob() {
        assert_eq!(rob(&[]), 0);
    }

    #[test]
    fn a_single_house_gets_robbed_outright() {
        assert_eq!(rob(&[5]), 5);
    }

    #[test]
    fn two_houses_takes_the_larger_one() {
        assert_eq!(rob(&[2, 7]), 7);
    }

    #[test]
    fn adjacent_houses_cannot_both_be_robbed() {
        // Robbing every other house (1, 3) beats robbing the single
        // biggest house (2) or any adjacent pair.
        assert_eq!(rob(&[1, 2, 3]), 4);
    }

    #[test]
    fn the_classic_five_house_example() {
        assert_eq!(rob(&[2, 7, 9, 3, 1]), 12);
    }

    #[test]
    fn all_zero_value_houses_yields_nothing() {
        assert_eq!(rob(&[0, 0, 0]), 0);
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        assert_eq!(rob(&[2, 7, 9, 3, 1]), 12);
    }

    #[test]
    fn tracing_enabled_captures_the_exact_sequence() {
        enable();
        let result = rob(&[2, 7, 9, 3, 1]);
        let events = take_events();
        assert_eq!(result, 12);
        assert_eq!(
            events,
            vec![
                Event::Set { i: 0, value: 2 },
                Event::Set { i: 1, value: 7 },
                Event::Set { i: 2, value: 11 },
                Event::Set { i: 3, value: 11 },
                Event::Set { i: 4, value: 12 },
            ]
        );
    }

    #[test]
    fn tracing_a_base_case_emits_no_events() {
        // n <= 1 returns before the dp table is ever built — same
        // accepted gap as climbing_stairs's base cases.
        enable();
        let result = rob(&[5]);
        let events = take_events();
        assert_eq!(result, 5);
        assert_eq!(events, Vec::new());
    }
}
