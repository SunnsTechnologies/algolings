//! The exercise-harness registry: one place declaring what an exercise is
//! (skeleton path, trace fixture, concept-note text) so the watch loop,
//! the CLI, and future exercise-porting (design doc Next Steps step 4)
//! all read from a single source instead of copy-pasted per-exercise glue.
//!
//! Order matters: this is the curriculum sequence learners progress
//! through (design doc Constraints — bubble, selection, insertion, merge,
//! quick, shell, counting, radix).

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
    /// Also used as the dispatch key in the `<package>-trace` binaries.
    pub test_filter: &'static str,
    pub skeleton_path: &'static str,
    pub fixture: &'static [i32],
    pub concept_note: &'static str,
}

pub const EXERCISES: &[Exercise] = &[
    Exercise {
        name: "bubble_sort",
        test_filter: "bubble",
        skeleton_path: "exercises/sort/src/bubble.rs",
        fixture: &[5, 1, 4, 2, 8],
        concept_note: "Why `cmp_lt`/`swap` instead of `<`/`.swap()`? They're normal \
            function calls with the same behavior — the only difference is they let \
            algolings record which comparisons and swaps your solution actually made, \
            so the trace you just watched was your own reasoning, not a canned replay.",
    },
    Exercise {
        name: "selection_sort",
        test_filter: "selection",
        skeleton_path: "exercises/sort/src/selection.rs",
        fixture: &[29, 10, 14, 37, 13],
        concept_note: "Why track `min_index` instead of the minimum value itself? \
            Tracking the value would still leave you needing a second search to find \
            WHERE it lives before you can swap it — and it breaks with duplicate \
            values. Tracking the index sidesteps both problems for free.",
    },
    Exercise {
        name: "insertion_sort",
        test_filter: "insertion",
        skeleton_path: "exercises/sort/src/insertion.rs",
        fixture: &[12, 11, 13, 5, 6],
        concept_note: "Why copy `arr[i]` into a local `key` before touching the array? \
            Because `i32` is `Copy`, that's a cheap, independent snapshot — and it has \
            to happen first, since the shifting that follows overwrites position i. \
            Skip the copy and the value you're inserting is gone before you place it.",
    },
    Exercise {
        name: "merge_sort",
        test_filter: "merge",
        skeleton_path: "exercises/sort/src/merge.rs",
        fixture: &[5, 1, 4, 2, 8, 7, 3, 6],
        concept_note: "Why recurse on index ranges into ONE slice instead of splitting \
            into sub-slices with `split_at_mut`? Sub-slices would make every recorded \
            position relative to that smaller slice, not the original array — you'd \
            need offset math everywhere. Passing the same slice down with `lo`/`hi` \
            bounds means every index you touch is already a true global position.",
    },
    Exercise {
        name: "quick_sort",
        test_filter: "quick",
        skeleton_path: "exercises/sort/src/quick.rs",
        fixture: &[10, 7, 8, 9, 1, 5],
        concept_note: "Why `isize` instead of `usize` for the partition bounds? The \
            algorithm needs a \"nothing placed yet\" sentinel of `low - 1`, which can \
            legitimately be `-1`. `usize` has no negative values — `0usize - 1` panics \
            in a debug build — so `isize` isn't a style choice here, it's required.",
    },
    Exercise {
        name: "shell_sort",
        test_filter: "shell",
        skeleton_path: "exercises/sort/src/shell.rs",
        fixture: &[12, 34, 54, 2, 3],
        concept_note: "Why guard every `j - gap` with `j >= gap` first? `j` and `gap` \
            are both `usize`, so `j - gap` when `j < gap` underflows — a panic in \
            debug builds, not just a logic bug. The guard is load-bearing for \
            correctness, the same gotcha Rust newcomers hit with unsigned subtraction.",
    },
    Exercise {
        name: "counting_sort",
        test_filter: "counting",
        skeleton_path: "exercises/sort/src/counting.rs",
        fixture: &[5, 1, 4, 2, 8, 1, 4],
        concept_note: "Notice this trace has zero Compare or Swap events — only Set. \
            Counting sort never compares two elements against each other; it counts \
            occurrences and places each value directly into its final position. The \
            same event vocabulary that traced every comparison-based sort so far \
            covers an algorithm that does no comparisons at all.",
    },
    Exercise {
        name: "radix_sort",
        test_filter: "radix",
        skeleton_path: "exercises/sort/src/radix.rs",
        fixture: &[170, 45, 75, 90, 802, 24, 2, 66],
        concept_note: "This assumes non-negative values — true for every algolings \
            fixture, but worth noticing: radix sort's digit-extraction trick doesn't \
            generalize to negative numbers without extra preprocessing (e.g. offsetting \
            them into a non-negative range first). Real signed-integer radix sorts add \
            that step explicitly rather than pretending it isn't needed.",
    },
];

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

    #[test]
    fn all_eight_curriculum_exercises_are_registered_in_order() {
        let names: Vec<&str> = EXERCISES.iter().map(|e| e.name).collect();
        assert_eq!(
            names,
            vec![
                "bubble_sort",
                "selection_sort",
                "insertion_sort",
                "merge_sort",
                "quick_sort",
                "shell_sort",
                "counting_sort",
                "radix_sort",
            ]
        );
    }

    #[test]
    fn every_exercise_fixture_is_within_the_legibility_cap() {
        // Design review: fixtures capped at <=20 elements so the terminal
        // trace stays legible.
        for exercise in EXERCISES {
            assert!(
                exercise.fixture.len() <= 20,
                "{} fixture has {} elements, exceeds the 20-element cap",
                exercise.name,
                exercise.fixture.len()
            );
        }
    }

    #[test]
    fn every_exercise_test_filter_is_distinct() {
        // Guards against the exact bug class the eng review found: two
        // exercises accidentally sharing a filter would mean one of them
        // silently tests the wrong module.
        let mut filters: Vec<&str> = EXERCISES.iter().map(|e| e.test_filter).collect();
        let original_len = filters.len();
        filters.sort_unstable();
        filters.dedup();
        assert_eq!(filters.len(), original_len, "duplicate test_filter found");
    }
}
