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
    /// For most exercises this is short (e.g. "bubble"); linked-list
    /// exercises anchor it to a module path prefix (e.g. "insert::tests::")
    /// since cargo's substring filter match could otherwise cross into a
    /// sibling exercise's still-unsolved tests when they share one struct.
    pub test_filter: &'static str,
    /// The dispatch key `<package>-trace` binaries match on (a `main.rs`
    /// argv, not a cargo filter). For sort/search this equals `test_filter`
    /// exactly — they happen to be the same short name. It's a SEPARATE
    /// field because linked-list's `test_filter` is anchored (see above)
    /// while the trace dispatcher still expects the bare name; conflating
    /// them here would repeat the exact test_filter/name bug this project
    /// already got bitten by once.
    pub trace_key: &'static str,
    pub skeleton_path: &'static str,
    pub fixture: &'static [i32],
    pub concept_note: &'static str,
    /// Escalating hints (vague nudge -> more specific -> near-answer), per
    /// the design review's Pass 7 resolution. Requested one at a time via
    /// `[h]` while an exercise is failing.
    pub hints: &'static [&'static str],
    /// The value being searched for, or acted on (e.g. removed) — shown
    /// once to the learner before the trace starts. `None` when the
    /// exercise doesn't need one.
    pub target: Option<i32>,
    /// Whether the trace's starting picture should be empty rather than
    /// `fixture`'s values. `false` everywhere except exercises whose
    /// events themselves ADD the fixture's values (e.g. `insert`) — for
    /// those, starting from `fixture` and then replaying inserts of the
    /// same values would double them.
    pub starts_empty: bool,
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
    Module {
        name: "linked lists",
        package: "exercises-linked-list",
        solutions_package: "linked-list-solutions",
        watch_dir: "exercises/linked-list/src",
        exercises: LINKED_LIST_EXERCISES,
    },
];

pub const SORT_EXERCISES: &[Exercise] = &[
    Exercise {
        name: "bubble_sort",
        test_filter: "bubble",
        trace_key: "bubble",
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
        starts_empty: false,
    },
    Exercise {
        name: "selection_sort",
        test_filter: "selection",
        trace_key: "selection",
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
        starts_empty: false,
    },
    Exercise {
        name: "insertion_sort",
        test_filter: "insertion",
        trace_key: "insertion",
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
        starts_empty: false,
    },
    Exercise {
        name: "merge_sort",
        test_filter: "merge",
        trace_key: "merge",
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
        starts_empty: false,
    },
    Exercise {
        name: "quick_sort",
        test_filter: "quick",
        trace_key: "quick",
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
        starts_empty: false,
    },
    Exercise {
        name: "shell_sort",
        test_filter: "shell",
        trace_key: "shell",
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
        starts_empty: false,
    },
    Exercise {
        name: "counting_sort",
        test_filter: "counting",
        trace_key: "counting",
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
        starts_empty: false,
    },
    Exercise {
        name: "radix_sort",
        test_filter: "radix",
        trace_key: "radix",
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
        starts_empty: false,
    },
];

pub const SEARCH_EXERCISES: &[Exercise] = &[
    Exercise {
        name: "linear_search",
        test_filter: "linear",
        trace_key: "linear",
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
        starts_empty: false,
    },
    Exercise {
        name: "binary_search",
        test_filter: "binary",
        trace_key: "binary",
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
        starts_empty: false,
    },
];

pub const LINKED_LIST_EXERCISES: &[Exercise] = &[
    Exercise {
        name: "insert",
        test_filter: "insert::tests::",
        trace_key: "insert",
        skeleton_path: "exercises/linked-list/src/insert.rs",
        fixture: &[10, 20, 30],
        concept_note: "Why does push_back need to walk the whole list instead of \
            jumping straight to the end the way an array's push does? There's no \
            stored length or tail pointer — reaching \"the end\" means following \
            `next` pointers from `head` until one is `None`, an O(n) walk arrays \
            never pay for indexed access.",
        hints: &[
            "push_front attaches at index 0 directly. push_back needs to walk to \
             the end first — there's no shortcut.",
            "Use `self.head.take()` to move the current head out, then wrap it in \
             a new `Box` as the new node's `next`.",
            "Call `mark_inserted(i, value)` right where you attach the new node — \
             `i` is 0 for push_front, or `self.len()` for push_back.",
        ],
        target: None,
        starts_empty: true,
    },
    Exercise {
        name: "remove",
        test_filter: "remove::tests::",
        trace_key: "remove",
        skeleton_path: "exercises/linked-list/src/remove.rs",
        fixture: &[10, 20, 30],
        concept_note: "Why `current.take()` instead of just reading through \
            `current`? `.take()` moves the boxed node OUT of the list temporarily, \
            leaving `None` behind, so you can decide whether to keep it, splice \
            around it, or put it back — you can't make that kind of \
            ownership-transferring decision through a shared reference.",
        hints: &[
            "Walk the list looking for a node whose value matches, tracking the \
             index as you go.",
            "Use `current.take()` to move a node out of the list temporarily, so \
             you can either keep it or splice around it.",
            "If the value matches, replace `*current` with `boxed_node.next.take()` \
             and call `mark_removed(i)`; otherwise put `boxed_node` back with \
             `*current = Some(boxed_node)` and advance to \
             `&mut current.as_mut().unwrap().next`.",
        ],
        target: Some(20),
        starts_empty: false,
    },
    Exercise {
        name: "traverse",
        test_filter: "traverse::tests::",
        trace_key: "traverse",
        skeleton_path: "exercises/linked-list/src/traverse.rs",
        fixture: &[10, 20, 30],
        concept_note: "Why `.as_deref()` instead of just following `.next` on the \
            `Box` directly? `Option<Box<Node<T>>>` doesn't chain the way you'd \
            hope — `.as_deref()` converts it to `Option<&Node<T>>` so you can walk \
            node to node using shared references, without taking ownership of \
            anything.",
        hints: &[
            "Walk from `self.head` using `.as_deref()` to get shared references, \
             not ownership.",
            "Compare each node's value to the target, calling `mark_visited(i)` \
             for every position you check.",
            "Call `found(i)` and return as soon as you find a match; if you reach \
             `None` without matching, the value isn't in the list.",
        ],
        target: Some(20),
        starts_empty: false,
    },
    Exercise {
        name: "reverse",
        test_filter: "reverse::tests::",
        trace_key: "reverse",
        skeleton_path: "exercises/linked-list/src/reverse.rs",
        fixture: &[1, 2, 3],
        concept_note: "Notice the trace shows values leaving the front of one \
            section and joining the front of another — that's exactly what the \
            algorithm does. Each node gets `.take()`n out and re-linked with its \
            `next` pointing at whatever was reversed so far, one node at a time, \
            in place, with no new list ever allocated.",
        hints: &[
            "Track three things as you walk: the node before (`prev`, starts \
             `None`), the current node, and the node after it (saved before you \
             touch anything).",
            "Use `current.take()` to move a node out, then point its `next` at \
             `prev` before advancing both.",
            "Call `mark_removed(0)` and `mark_inserted(remaining_len, value)` for \
             each node — `remaining_len` is how many nodes are still unprocessed \
             after this one leaves.",
        ],
        target: None,
        starts_empty: false,
    },
    Exercise {
        name: "doubly_push",
        test_filter: "doubly_push::tests::",
        trace_key: "doubly_push",
        skeleton_path: "exercises/linked-list/src/doubly_push.rs",
        fixture: &[1, 2, 3],
        concept_note: "Why is push_back O(1) here when the singly-linked version \
            had to walk the whole list? The stored `tail` pointer — not being \
            doubly-linked itself — is what gets you there; a singly-linked list \
            with its own tail pointer would be just as fast. What backward links \
            (`prev`, via `Weak`) actually buy you — O(1) removal from a known \
            node, walking backward — is what a later exercise puts to use.",
        hints: &[
            "push_front and push_back both need to wire `prev` too, not just \
             `next` — use `Rc::downgrade(&new_node)` to make a `Weak` reference, \
             never a strong `Rc`, or you'll create a reference cycle.",
            "The list keeps a `tail` pointer specifically so push_back never has \
             to walk to find the end — update it directly instead of searching.",
            "Call `mark_inserted(0, value)` for push_front, or \
             `mark_inserted(self.len(), value)` for push_back (before updating \
             `len`) — and remember to update `self.len` yourself, it isn't \
             computed by walking here.",
        ],
        target: None,
        starts_empty: true,
    },
    Exercise {
        name: "doubly_pop",
        test_filter: "doubly_pop::tests::",
        trace_key: "doubly_pop",
        skeleton_path: "exercises/linked-list/src/doubly_pop.rs",
        fixture: &[1, 2, 3, 4],
        concept_note: "Why does pop_back need `prev` at all, instead of just knowing \
            where `tail` is? Removing the tail means someone else has to become the \
            new tail — and you need to find that node. A singly-linked list, even \
            with its own stored tail pointer, has no way to reach the node BEFORE \
            the tail without walking from `head` all the way there: it's O(n) hiding \
            behind an O(1)-looking pointer. `prev` is what makes finding it, and \
            fixing it up, actually O(1).",
        hints: &[
            "pop_front needs to null out the NEW head's `prev` — it's still \
             pointing (via `Weak`) at the node you just removed.",
            "pop_back needs to find the new tail by upgrading the removed node's \
             `prev` — a singly-linked list has no way to do this in O(1), which is \
             exactly what backward links buy you.",
            "Call `mark_removed(0)` for pop_front, or `mark_removed(self.len() - 1)` \
             for pop_back — read the length before updating it, same as \
             doubly_push's push_back did for insertion.",
        ],
        target: None,
        starts_empty: false,
    },
];

/// Cargo's `--lib <filter>` matches as a plain substring anywhere in the
/// full test path — not just an exact match or a module prefix. Two
/// exercises whose `test_filter`s are different STRINGS can still collide
/// if one is contained in the other (e.g. `"insert::tests::"` is a
/// substring of `"doubly_insert::tests::"`), which the simpler
/// exact-duplicate check can't catch. Returns the first colliding pair's
/// indices found, if any.
#[cfg(test)]
fn find_substring_collision(filters: &[&str]) -> Option<(usize, usize)> {
    for (i, a) in filters.iter().enumerate() {
        for (j, b) in filters.iter().enumerate() {
            if i != j && b.contains(a) {
                return Some((i, j));
            }
        }
    }
    None
}

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
    fn find_substring_collision_detects_a_real_collision() {
        // "insert::tests::" is a genuine substring of
        // "doubly_insert::tests::" — this is the exact collision that
        // would have shipped undetected without this check.
        let filters = ["insert::tests::", "doubly_insert::tests::"];
        assert_eq!(find_substring_collision(&filters), Some((0, 1)));
    }

    #[test]
    fn find_substring_collision_ignores_genuinely_unrelated_filters() {
        let filters = ["insert::tests::", "remove::tests::", "traverse::tests::"];
        assert_eq!(find_substring_collision(&filters), None);
    }

    #[test]
    fn no_exercise_test_filter_collides_with_another_within_its_module() {
        // Scoped per-module, same reasoning as the exact-duplicate check
        // above — cargo test -p only ever runs one package's tests at a
        // time, so two DIFFERENT modules sharing a substring relationship
        // is harmless.
        for module in MODULES {
            let filters: Vec<&str> = module.exercises.iter().map(|e| e.test_filter).collect();
            if let Some((i, j)) = find_substring_collision(&filters) {
                panic!(
                    "{:?}'s test_filter {:?} is a substring of {:?}'s test_filter {:?} in \
                     module {:?} — cargo's filter match would pick up both exercises' tests \
                     when running either one",
                    module.exercises[i].name,
                    filters[i],
                    module.exercises[j].name,
                    filters[j],
                    module.name
                );
            }
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
    fn every_trace_key_is_a_bare_dispatch_name_not_an_anchored_filter() {
        // Regression test: caught live when test_filter's D14 anchoring
        // ("insert::tests::") accidentally also broke the trace dispatch
        // binary's argv match, which expects the bare name ("insert").
        // trace_key must never carry the "::" anchor test_filter uses.
        for exercise in all_exercises() {
            assert!(
                !exercise.trace_key.contains("::"),
                "{}'s trace_key ({:?}) looks like an anchored test_filter, not a bare \
                 dispatch key",
                exercise.name,
                exercise.trace_key
            );
        }
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
