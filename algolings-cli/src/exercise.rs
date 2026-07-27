//! The exercise-harness registry: one place declaring what an exercise is
//! (skeleton path, trace fixture, concept-note text, staged hints) so the
//! watch loop, the CLI, and future exercise-porting (design doc Next Steps
//! step 4) all read from a single source instead of copy-pasted per-exercise
//! glue.
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
    /// Escalating hints (vague nudge -> more specific -> near-answer), per
    /// the design review's Pass 7 resolution. Requested one at a time via
    /// `[h]` while an exercise is failing.
    pub hints: &'static [&'static str],
    /// The value being searched for, shown once to the learner before the
    /// trace starts. `None` for sorting exercises, which don't search for
    /// anything; `Some(n)` for search exercises.
    pub target: Option<i32>,
}

/// One module of the curriculum: a package of skeleton exercises, its
/// reference-solution package, the directory to watch, and the exercises
/// themselves, in solving order. `run_module` (main.rs) runs one of these
/// to completion before moving to the next.
pub struct Module {
    pub name: &'static str,
    pub package: &'static str,
    pub solutions_package: &'static str,
    pub watch_dir: &'static str,
    pub exercises: &'static [Exercise],
}

pub const MODULES: &[Module] = &[
    Module {
        name: "sorting",
        package: "exercises-sort",
        solutions_package: "sort-solutions",
        watch_dir: "exercises/sort/src",
        exercises: SORT_EXERCISES,
    },
    Module {
        name: "searching",
        package: "exercises-search",
        solutions_package: "search-solutions",
        watch_dir: "exercises/search/src",
        exercises: SEARCH_EXERCISES,
    },
];

pub const SORT_EXERCISES: &[Exercise] = &[
    Exercise {
        name: "bubble_sort",
        test_filter: "bubble",
        skeleton_path: "exercises/sort/src/bubble.rs",
        fixture: &[5, 1, 4, 2, 8],
        concept_note: "Why `cmp_lt`/`swap` instead of `<`/`.swap()`? They're normal \
            function calls with the same behavior — the only difference is they let \
            algolings record which comparisons and swaps your solution actually made, \
            so the trace you just watched was your own reasoning, not a canned replay.",
        hints: &[
            "Compare each pair of adjacent elements — if they're out of order, swap them.",
            "Loop over the array repeatedly. A full pass with no swaps means you're done.",
            "Use `cmp_lt(arr, i, i - 1)` to check order, `swap(arr, i - 1, i)` to fix it, \
             and `mark_sorted(n)` once a pass makes no swaps.",
        ],
        target: None,
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
        hints: &[
            "For each position, find the smallest remaining value and put it there.",
            "Track the INDEX of the smallest value you've seen in the unsorted region, \
             not the value itself.",
            "Use `cmp_lt(arr, j, min_index)` in your inner loop, then `swap(arr, i, \
             min_index)` only if `min_index != i`.",
        ],
        target: None,
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
        hints: &[
            "Grow a sorted region on the left, one element at a time.",
            "Save the value you're inserting into a local variable before you start \
             shifting other elements.",
            "Use `cmp_lt_values(&key, &arr[j - 1], i, j - 1)` to check whether to keep \
             shifting, and `set_at` for both the shift and the final placement.",
        ],
        target: None,
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
        hints: &[
            "Split the array in half, sort each half, then merge them back together.",
            "Recurse using `lo`/`hi` index bounds into the SAME array, not sub-slices.",
            "In the merge step, snapshot the segment into a `Vec` first, then use \
             `cmp_lt_values` and `set_at` to write the merged result back into `arr`.",
        ],
        target: None,
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
        hints: &[
            "Pick a pivot, partition the array around it, then recursively sort each side.",
            "Use `isize`, not `usize`, for your partition bounds — you need a value that \
             can legitimately go to -1.",
            "In `partition`, use `cmp_lt(arr, high, j)` negated to check `arr[j] <= \
             pivot`, and `swap` to move qualifying elements left.",
        ],
        target: None,
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
        hints: &[
            "It's insertion sort, but comparing elements `gap` apart instead of adjacent.",
            "Start with a large gap and halve it each pass, down to 1.",
            "Guard every `j - gap` with `j >= gap` first, to avoid a `usize` subtraction \
             underflow.",
        ],
        target: None,
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
        hints: &[
            "Count how many times each value appears, instead of comparing elements.",
            "Turn your counts into prefix sums, then walk the input in reverse for a \
             stable result.",
            "Use `set_at(&mut output, index, value)` to place each value at the index \
             given by its running count, then copy `output` back into `arr`.",
        ],
        target: None,
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
        hints: &[
            "Sort by one digit at a time, starting from the ones place.",
            "Use counting sort as your per-digit subroutine, multiplying the divisor by \
             10 each pass.",
            "Loop while `max_value / exp > 0`, and use `set_at` for every write into the \
             output buffer.",
        ],
        target: None,
    },
];

pub const SEARCH_EXERCISES: &[Exercise] = &[
    Exercise {
        name: "linear_search",
        test_filter: "linear",
        skeleton_path: "exercises/search/src/linear.rs",
        fixture: &[3, 7, 2, 9, 5],
        concept_note: "Why does this return `Option<usize>` instead of, say, -1 for \
            \"not found\"? `usize` can't hold -1, and even if it could, a caller could \
            forget to check for the sentinel and use it as a real index. `Option<usize>` \
            makes \"maybe there's no answer\" part of the type — the compiler won't let \
            you use the index without unwrapping it first.",
        hints: &[
            "Check each element in order until you find the target or reach the end.",
            "Use `probe(arr, i, &target)` to check each index — it returns true on a \
             match.",
            "Call `found(i)` right before returning `Some(i)`, so the trace shows \
             exactly where the search succeeded.",
        ],
        target: Some(9),
    },
    Exercise {
        name: "binary_search",
        test_filter: "binary",
        skeleton_path: "exercises/search/src/binary.rs",
        fixture: &[2, 5, 8, 12, 16, 23, 38, 56, 72, 91],
        concept_note: "Why `left + (right - left) / 2` instead of `(left + right) / 2`? \
            Both give the same midpoint mathematically, but `left + right` can overflow \
            for large bounds even when the true midpoint is nowhere near overflowing — \
            a classic bug real binary search implementations have shipped with. \
            Subtracting first keeps the intermediate value bounded by the range itself.",
        hints: &[
            "The array is sorted — use that. Check the middle element and eliminate half \
             the remaining range each time.",
            "Track `left`/`right` bounds; narrow them based on whether the middle is too \
             high or too low.",
            "Use `probe(arr, mid, &target)` to check the middle, `narrow_range(left, \
             right)` after adjusting the bounds, and `found(mid)` right before returning \
             `Some(mid)`.",
        ],
        // 16, not 23, on purpose — it lands away from the first midpoint
        // this implementation checks, so the trace actually shows a
        // narrowing step or two instead of finding it on the first probe.
        target: Some(16),
    },
];

pub fn find_by_skeleton_path(path: &str) -> Option<&'static Exercise> {
    MODULES
        .iter()
        .flat_map(|m| m.exercises)
        .find(|e| e.skeleton_path == path)
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
    fn find_by_skeleton_path_also_finds_a_search_exercise() {
        // Regression test: find_by_skeleton_path used to search only the
        // sort module's exercises.
        let exercise = find_by_skeleton_path("exercises/search/src/linear.rs");
        assert_eq!(exercise.map(|e| e.name), Some("linear_search"));
    }

    #[test]
    fn all_eight_curriculum_exercises_are_registered_in_order() {
        let names: Vec<&str> = SORT_EXERCISES.iter().map(|e| e.name).collect();
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

    fn all_exercises() -> impl Iterator<Item = &'static Exercise> {
        MODULES.iter().flat_map(|m| m.exercises)
    }

    #[test]
    fn every_exercise_fixture_is_within_the_legibility_cap() {
        // Design review: fixtures capped at <=20 elements so the terminal
        // trace stays legible.
        for exercise in all_exercises() {
            assert!(
                exercise.fixture.len() <= 20,
                "{} fixture has {} elements, exceeds the 20-element cap",
                exercise.name,
                exercise.fixture.len()
            );
        }
    }

    #[test]
    fn every_exercise_test_filter_is_distinct_within_its_module() {
        // Guards against the exact bug class the eng review found: two
        // exercises accidentally sharing a filter would mean one of them
        // silently tests the wrong module. Scoped per-module (not globally)
        // since `cargo test -p` only ever runs one package's tests at a
        // time — two DIFFERENT modules coincidentally sharing a filter
        // string would be harmless.
        for module in MODULES {
            let mut filters: Vec<&str> = module.exercises.iter().map(|e| e.test_filter).collect();
            let original_len = filters.len();
            filters.sort_unstable();
            filters.dedup();
            assert_eq!(
                filters.len(),
                original_len,
                "duplicate test_filter found in module {:?}",
                module.name
            );
        }
    }

    #[test]
    fn every_exercise_has_at_least_two_staged_hints() {
        for exercise in all_exercises() {
            assert!(
                exercise.hints.len() >= 2,
                "{} has only {} hint(s), expected an escalating staged set",
                exercise.name,
                exercise.hints.len()
            );
            for hint in exercise.hints {
                assert!(!hint.is_empty());
            }
        }
    }

    #[test]
    fn binary_search_fixture_is_sorted() {
        // binary_search only works correctly on sorted input — an
        // accidentally-unsorted fixture would make the reference solution's
        // own trace look buggy even though the algorithm is correct.
        let exercise = all_exercises()
            .find(|e| e.name == "binary_search")
            .expect("binary_search should be registered");
        let mut sorted = exercise.fixture.to_vec();
        sorted.sort_unstable();
        assert_eq!(
            exercise.fixture,
            sorted.as_slice(),
            "binary_search's fixture must be sorted"
        );
    }

    #[test]
    fn every_search_exercise_has_a_target() {
        for exercise in SEARCH_EXERCISES {
            assert!(
                exercise.target.is_some(),
                "{} is a search exercise but has no target",
                exercise.name
            );
        }
    }

    #[test]
    fn every_sort_exercise_has_no_target() {
        for exercise in SORT_EXERCISES {
            assert!(
                exercise.target.is_none(),
                "{} is a sort exercise but has a target",
                exercise.name
            );
        }
    }
}
