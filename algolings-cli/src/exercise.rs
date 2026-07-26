//! The exercise-harness registry: one place declaring what an exercise is
//! (skeleton path, trace fixture, concept-note text) so the watch loop,
//! the CLI, and future exercise-porting (design doc Next Steps step 4)
//! all read from a single source instead of copy-pasted per-exercise glue.

pub struct Exercise {
    /// Display name (e.g. "bubble_sort") — shown to the learner, used in
    /// the concept note.
    pub name: &'static str,
    /// The `cargo test` filter that actually matches this exercise's test
    /// module (e.g. "bubble", matching `bubble::tests::*`). Deliberately a
    /// SEPARATE field from `name`: they look similar but are not
    /// interchangeable — a filter that matches zero tests reports a
    /// trivial pass, so getting this wrong silently shows an unsolved
    /// exercise as PASSED (a real bug caught during end-to-end testing).
    pub test_filter: &'static str,
    pub skeleton_path: &'static str,
    pub fixture: &'static [i32],
    pub concept_note: &'static str,
}

pub const EXERCISES: &[Exercise] = &[Exercise {
    name: "bubble_sort",
    test_filter: "bubble",
    skeleton_path: "exercises/sort/src/bubble.rs",
    fixture: &[5, 1, 4, 2, 8],
    concept_note: "Why `cmp_lt`/`swap` instead of `<`/`.swap()`? They're normal \
        function calls with the same behavior — the only difference is they let \
        algolings record which comparisons and swaps your solution actually made, \
        so the trace you just watched was your own reasoning, not a canned replay.",
}];

pub fn find_by_skeleton_path(path: &str) -> Option<&'static Exercise> {
    EXERCISES.iter().find(|e| e.skeleton_path == path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_a_registered_exercise_by_skeleton_path() {
        let exercise = find_by_skeleton_path("exercises/sort/src/bubble.rs");
        assert_eq!(exercise.map(|e| e.name), Some("bubble_sort"));
    }

    #[test]
    fn returns_none_for_an_unregistered_path() {
        assert!(find_by_skeleton_path("exercises/sort/src/nonexistent.rs").is_none());
    }

    #[test]
    fn registered_exercise_carries_a_non_empty_fixture_and_concept_note() {
        let exercise = find_by_skeleton_path("exercises/sort/src/bubble.rs").unwrap();
        assert!(!exercise.fixture.is_empty());
        assert!(!exercise.concept_note.is_empty());
    }
}
