// Shared between exercises/dynamic-programming/src/climbing_stairs.rs (the
// learner-facing skeleton) and
// exercises/dynamic-programming-solutions/src/climbing_stairs.rs (the
// reference solution), via `include!()`.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_stairs_has_exactly_one_way_to_stand_still() {
        assert_eq!(climb_stairs(0), 1);
    }

    #[test]
    fn one_stair_has_exactly_one_way() {
        assert_eq!(climb_stairs(1), 1);
    }

    #[test]
    fn two_stairs_has_two_ways() {
        assert_eq!(climb_stairs(2), 2);
    }

    #[test]
    fn three_stairs_has_three_ways() {
        assert_eq!(climb_stairs(3), 3);
    }

    #[test]
    fn five_stairs_has_eight_ways() {
        assert_eq!(climb_stairs(5), 8);
    }

    #[test]
    fn ten_stairs_has_eighty_nine_ways() {
        assert_eq!(climb_stairs(10), 89);
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        assert_eq!(climb_stairs(5), 8);
    }

    #[test]
    fn tracing_enabled_captures_the_exact_sequence() {
        enable();
        let result = climb_stairs(5);
        let events = take_events();
        assert_eq!(result, 8);
        assert_eq!(
            events,
            vec![
                Event::Set { i: 0, value: 1 },
                Event::Set { i: 1, value: 1 },
                Event::Set { i: 2, value: 2 },
                Event::Set { i: 3, value: 3 },
                Event::Set { i: 4, value: 5 },
                Event::Set { i: 5, value: 8 },
            ]
        );
    }

    #[test]
    fn tracing_a_base_case_emits_no_events() {
        // n <= 1 returns before the dp table is ever built — a real,
        // accepted gap: the exercise's animated trace only has something
        // to show for n >= 2, which is why the fixture used for the CLI
        // demo is never this small.
        enable();
        let result = climb_stairs(1);
        let events = take_events();
        assert_eq!(result, 1);
        assert_eq!(events, Vec::new());
    }
}
