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
    Module {
        name: "stacks & queues",
        package: "exercises-stacks-queues",
        solutions_package: "stacks-queues-solutions",
        watch_dir: "exercises/stacks-queues/src",
        exercises: STACKS_QUEUES_EXERCISES,
    },
    Module {
        name: "hash tables",
        package: "exercises-hash-tables",
        solutions_package: "hash-tables-solutions",
        watch_dir: "exercises/hash-tables/src",
        exercises: HASH_TABLES_EXERCISES,
    },
    Module {
        name: "recursion & backtracking",
        package: "exercises-recursion-backtracking",
        solutions_package: "recursion-backtracking-solutions",
        watch_dir: "exercises/recursion-backtracking/src",
        exercises: RECURSION_BACKTRACKING_EXERCISES,
    },
    Module {
        name: "trees",
        package: "exercises-trees",
        solutions_package: "trees-solutions",
        watch_dir: "exercises/trees/src",
        exercises: TREES_EXERCISES,
    },
    Module {
        name: "graphs",
        package: "exercises-graphs",
        solutions_package: "graphs-solutions",
        watch_dir: "exercises/graphs/src",
        exercises: GRAPHS_EXERCISES,
    },
    Module {
        name: "dynamic programming",
        package: "exercises-dynamic-programming",
        solutions_package: "dynamic-programming-solutions",
        watch_dir: "exercises/dynamic-programming/src",
        exercises: DYNAMIC_PROGRAMMING_EXERCISES,
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
    Exercise {
        name: "doubly_contains",
        test_filter: "doubly_contains::tests::",
        trace_key: "doubly_contains",
        skeleton_path: "exercises/linked-list/src/doubly_contains.rs",
        fixture: &[10, 20, 30, 40, 50],
        concept_note: "This is almost the same code as the singly-linked list's \
            `contains` — the only real difference is ownership. `Option<Box<Node>>` \
            walks with `.as_deref()`; `Option<Rc<RefCell<Node>>>` walks by cloning \
            the `Rc` and reading through `.borrow()`. The search itself is still \
            purely forward, one node at a time, exactly like a singly-linked list — \
            having `prev` doesn't help here at all.",
        hints: &[
            "Walk from `self.head`, cloning each node's `Rc` to move to the next \
             one — `Option<Rc<RefCell<Node>>>` doesn't support `.as_deref()` the \
             way `Option<Box<Node>>` does.",
            "Call `mark_visited(i)` for every node you check, and `found(i)` the \
             moment you find a match.",
            "Read a node's value through `.borrow()` — `node.borrow().value`.",
        ],
        target: Some(40),
        starts_empty: false,
    },
    Exercise {
        name: "doubly_converge",
        test_filter: "doubly_converge::tests::",
        trace_key: "doubly_converge",
        skeleton_path: "exercises/linked-list/src/doubly_converge.rs",
        fixture: &[10, 20, 30, 40, 50],
        concept_note: "This is the one search a singly-linked list structurally \
            can't do: check both ends and work inward, without first walking the \
            whole list to even find the far end's neighbor. Each step covers two \
            nodes for the price of one meeting-in-the-middle — worst case still \
            O(n), but the search space shrinks from both sides at once.",
        hints: &[
            "Guard the empty-list case first — `self.len() - 1` underflows a \
             `usize` when the list is empty.",
            "Track `li` (starts at 0) and `ri` (starts at `self.len() - 1`); when \
             `li == ri` they're the SAME node — check it once and stop, don't \
             decrement `ri` past that point.",
            "Call `mark_converging(li, ri)` once per step, checking both nodes \
             against `target`; advance `li` forward via `next` and `ri` backward \
             via `prev`.",
        ],
        target: Some(30),
        starts_empty: false,
    },
];

pub const STACKS_QUEUES_EXERCISES: &[Exercise] = &[
    Exercise {
        name: "stack_vec",
        test_filter: "stack_vec::tests::",
        trace_key: "stack_vec",
        skeleton_path: "exercises/stacks-queues/src/stack_vec.rs",
        fixture: &[10, 20, 30, 40],
        concept_note: "A stack's whole contract is LIFO — last in, first out — and \
            Vec's own push/pop already operate on the end of the buffer, which is \
            exactly what that means. No pointers, no allocation per element beyond \
            the backing buffer, cache-friendly: Vec is usually the RIGHT choice for \
            a stack, not just the easy one.",
        hints: &[
            "push appends to the end of self.data — call mark_inserted(self.len(), \
             value) BEFORE pushing, since self.len() is about to change.",
            "pop and peek both need to check is_empty() first — self.len() - 1 \
             underflows a usize on an empty stack.",
            "peek reads the last element without removing it — \
             self.data.last().copied(), traced with mark_visited.",
        ],
        target: None,
        starts_empty: true,
    },
    Exercise {
        name: "stack_linked_list",
        test_filter: "stack_linked_list::tests::",
        trace_key: "stack_linked_list",
        skeleton_path: "exercises/stacks-queues/src/stack_linked_list.rs",
        fixture: &[10, 20, 30, 40],
        concept_note: "This is the same push_front/head-removal idiom the \
            linked-list module's insert/remove exercises already taught you, just \
            wrapped in its own Stack type. That's not a coincidence — a stack that \
            only ever touches ONE end of a singly-linked list IS push_front/pop_front, \
            whatever you name the struct.",
        hints: &[
            "push and pop both always operate on self.head — LIFO means the newest \
             value is always at index 0.",
            "push: build a new node whose `next` is the OLD head (self.head.take()), \
             then make it the new head.",
            "pop: self.head.take().map(...) — the closure gets the old head, unwraps \
             its next as the new head, and returns its value.",
        ],
        target: None,
        starts_empty: true,
    },
    Exercise {
        name: "queue_vecdeque",
        test_filter: "queue_vecdeque::tests::",
        trace_key: "queue_vecdeque",
        skeleton_path: "exercises/stacks-queues/src/queue_vecdeque.rs",
        fixture: &[10, 20, 30, 40],
        concept_note: "A queue needs O(1) access at BOTH ends — enqueue at the back, \
            dequeue at the front. Vec can do the back in O(1) but its remove(0) is \
            O(n): every remaining element has to shift down one slot. VecDeque is a \
            ring buffer built specifically to make both ends O(1), which is why it \
            exists as its own type instead of everyone just using Vec.",
        hints: &[
            "enqueue pushes onto the BACK — self.data.push_back(value), traced with \
             mark_inserted(self.len(), value).",
            "dequeue and peek both act on the FRONT, always index 0 — check \
             is_empty() first.",
            "dequeue: self.data.pop_front(), traced with mark_removed(0).",
        ],
        target: None,
        starts_empty: true,
    },
    Exercise {
        name: "queue_linked_list",
        test_filter: "queue_linked_list::tests::",
        trace_key: "queue_linked_list",
        skeleton_path: "exercises/stacks-queues/src/queue_linked_list.rs",
        fixture: &[10, 20, 30, 40],
        concept_note: "A stack only ever touches ONE end of a linked list, so \
            push_front/pop_front (or their Box<Node> equivalents) are all it needs. \
            A queue touches BOTH ends — enqueue at the tail, dequeue at the head — so \
            it needs a stored `tail` reference, or every enqueue would have to walk \
            the whole list just to find where to attach. `Rc<RefCell<Node>>` gives \
            you that shared tail reference without needing `Weak` anywhere: nothing \
            here ever removes from the tail or walks backward, so there's no \
            reference cycle to guard against the way the doubly-linked list's \
            pop_back has to.",
        hints: &[
            "enqueue: if the list is empty, the new node becomes BOTH head and \
             tail. Otherwise, wire the OLD tail's `next` to the new node first, \
             THEN make the new node the tail.",
            "dequeue: take the head; if there's no next node, the queue is now \
             empty — null `tail` too, or the next enqueue wires onto an orphaned \
             node nothing points to anymore.",
            "At len() == 1, head and tail are the SAME Rc<RefCell<Node>> — never \
             hold a borrow() and borrow_mut() on both fields at once, or it panics.",
        ],
        target: None,
        starts_empty: true,
    },
];

pub const HASH_TABLES_EXERCISES: &[Exercise] = &[
    Exercise {
        name: "custom_hash_table",
        test_filter: "custom_hash_table::tests::",
        trace_key: "custom_hash_table",
        skeleton_path: "exercises/hash-tables/src/custom_hash_table.rs",
        fixture: &[0, 0, 0, 0, 0, 0, 0, 0],
        concept_note: "Every other exercise in this project traces a VALUE SEQUENCE — a \
            row you can point at and say \"this is what's stored, in this order.\" A hash \
            table isn't that: `bucket_index` scatters keys across a fixed set of slots \
            based on a hash, not insertion order. The trace here shows which bucket a key \
            landed in — but since it can only show one value per slot, two keys colliding \
            into the same bucket will show the most RECENT one, not both. That's a \
            limitation of the visualization, not of your implementation: the tests fully \
            verify that chaining keeps every colliding key findable, regardless of what \
            the trace can draw.",
        hints: &[
            "bucket_index: hash the key with DefaultHasher, then reduce it to a valid \
             position with `% self.buckets.len()` — the same hash-then-modulo idiom \
             every hash table variant uses under the hood.",
            "insert is a SET operation, not a growing list — check whether the key's \
             bucket already contains it before pushing, or you'll store duplicates.",
            "remove's trace shows 0 if the bucket is now empty, or one of the keys \
             still chained there if others survive — call mark_set(index, ...) either \
             way, after mutating the bucket.",
        ],
        target: None,
        starts_empty: false,
    },
    Exercise {
        name: "hash_entry",
        test_filter: "hash_entry::tests::",
        trace_key: "hash_entry",
        skeleton_path: "exercises/hash-tables/src/hash_entry.rs",
        fixture: &[],
        concept_note: "This is the first exercise in algolings with no animated trace — \
            std HashMap has no stable iteration order or index to point a replay at, \
            unlike every array/list/bucket-array model every other exercise traces \
            against. The idiom worth learning here isn't visual: `entry(key).or_insert(0)` \
            looks up the key ONCE and hands you a `&mut` either way, instead of the \
            naive `contains_key` + `get_mut`/`insert` pattern hashing the same key twice.",
        hints: &[
            "Start from an empty HashMap::new().",
            "For each value, call `counts.entry(value).or_insert(0)` — this returns a \
             `&mut usize`, a fresh 0 if the key wasn't there yet, or the existing count \
             if it was.",
            "Dereference and add 1 to whatever `entry(...).or_insert(0)` returns: \
             `*counts.entry(value).or_insert(0) += 1;`",
        ],
        target: None,
        starts_empty: true,
    },
];

pub const RECURSION_BACKTRACKING_EXERCISES: &[Exercise] = &[
    Exercise {
        name: "recursion_basics",
        test_filter: "recursion_basics::tests::",
        trace_key: "recursion_basics",
        skeleton_path: "exercises/recursion-backtracking/src/recursion_basics.rs",
        fixture: &[],
        concept_note: "There's no array here — the thing worth tracing is the CALL STACK \
            itself. Each recursive call is one frame deeper; the trace's row IS the stack, \
            growing on the way down and shrinking on the way back up. Notice fibonacci's \
            trace: one whole branch (insert then remove at some depth) always finishes \
            before the next branch begins at that SAME depth — that's not a coincidence, \
            it's why nothing ever collides.",
        hints: &[
            "You need a private helper that carries a `depth: usize` alongside `n` — the \
             public factorial/fibonacci just start that at 0.",
            "Call mark_inserted(depth, n as i64) the moment a call begins, before \
             recursing further.",
            "Call mark_removed(depth) right before returning — after the recursive call \
             comes back, but before you hand the result up.",
        ],
        target: None,
        starts_empty: true,
    },
    Exercise {
        name: "tail_recursion",
        test_filter: "tail_recursion::tests::",
        trace_key: "tail_recursion",
        skeleton_path: "exercises/recursion-backtracking/src/tail_recursion.rs",
        fixture: &[],
        concept_note: "This LOOKS like it should need no stack at all — the accumulator \
            carries the answer forward, so there's no work left once the recursive call \
            returns. Watch the trace anyway: it grows to the same depth recursion_basics's \
            factorial does. Some languages guarantee tail-call optimization collapses this \
            into a loop; Rust doesn't. The rewrite changes WHAT gets computed at each step, \
            not whether frames pile up.",
        hints: &[
            "Same call-stack tracing pattern as recursion_basics: a private helper \
             carrying `depth`, starting at 0.",
            "The base case returns `acc` directly — no more multiplication needed, it's \
             already been folded in on the way down.",
            "Recurse with `n - 1` and `n * acc` — the accumulator absorbs the \
             multiplication BEFORE the call, not after.",
        ],
        target: None,
        starts_empty: true,
    },
    Exercise {
        name: "subsets",
        test_filter: "subsets::tests::",
        trace_key: "subsets",
        skeleton_path: "exercises/recursion-backtracking/src/subsets.rs",
        fixture: &[1, 2, 3],
        concept_note: "Backtracking's whole idiom in one loop: push a candidate onto the \
            working path, recurse, then pop it back off before trying the next one. Every \
            state along the way — including the empty path — is a real subset, which is \
            why found() fires on nearly every call here, not just at some final depth.",
        hints: &[
            "Capture `current` into `result` at the START of every call — the empty \
             subset counts too.",
            "mark_inserted(current.len(), value) BEFORE pushing, mark_removed(current.len() \
             - 1) BEFORE popping — same convention as every stack push/pop in this project.",
            "The loop starts at `index`, not 0 — that's what stops [1,2] and [2,1] from \
             both appearing (only ever add elements AFTER where you started).",
        ],
        target: None,
        starts_empty: true,
    },
    Exercise {
        name: "combinations",
        test_filter: "combinations::tests::",
        trace_key: "combinations",
        skeleton_path: "exercises/recursion-backtracking/src/combinations.rs",
        fixture: &[1, 2, 3],
        concept_note: "Same push/recurse/pop idiom as subsets, but with a length gate: \
            only capture `current` once it reaches exactly `k` elements, and stop \
            exploring further from there (return immediately) instead of continuing to \
            add more.",
        hints: &[
            "Check `current.len() == k` FIRST, before the loop — if it's already length \
             k, capture and return without trying to add more.",
            "Same tracing convention as subsets: mark_inserted before push, mark_removed \
             before pop, found() when you capture.",
            "The loop still starts at `start`, not 0, for the same reason as subsets — no \
             re-visiting earlier elements.",
        ],
        target: None,
        starts_empty: true,
    },
    Exercise {
        name: "permutations",
        test_filter: "permutations::tests::",
        trace_key: "permutations",
        skeleton_path: "exercises/recursion-backtracking/src/permutations.rs",
        fixture: &[1, 2, 3],
        concept_note: "Subsets and combinations only ever look FORWARD through the input \
            (the loop starts at `index`/`start`). Permutations needs every element \
            available at every position, in any order — so instead of a start index, track \
            which ones are already placed with `used: Vec<bool>`, and the loop always runs \
            0..len, skipping whatever's already used.",
        hints: &[
            "The loop runs `0..nums.len()` every time, not from some start index — skip \
             indices where `used[i]` is true.",
            "Capture once `current.len() == nums.len()`, same length-gate idiom as \
             combinations.",
            "Set `used[i] = true` before recursing, and back to `false` on the way out — \
             it's part of the undo, same as popping `current`.",
        ],
        target: None,
        starts_empty: true,
    },
    Exercise {
        name: "permutations_with_duplicates",
        test_filter: "permutations_with_duplicates::tests::",
        trace_key: "permutations_with_duplicates",
        skeleton_path: "exercises/recursion-backtracking/src/permutations_with_duplicates.rs",
        fixture: &[1, 1, 2],
        concept_note: "Without a duplicate check, [1, 1, 2] would produce the SAME \
            permutation twice — once for each identical `1`, since the algorithm can't \
            otherwise tell them apart. Sorting groups equal values together, so the fix \
            becomes local: at each position, only ever start with the FIRST unused copy \
            of an equal run, never a later one.",
        hints: &[
            "Sort `nums` first — the duplicate check only works when equal values are \
             adjacent.",
            "Same used[]/length-gate structure as permutations, plus one more skip \
             condition in the loop.",
            "Skip index i if `nums[i] == nums[i - 1] && !used[i - 1]` — that means the \
             earlier identical value hasn't been placed yet in THIS branch, so placing \
             the later one first would just re-derive an ordering another branch already \
             covers.",
        ],
        target: None,
        starts_empty: true,
    },
    Exercise {
        name: "n_queens",
        test_filter: "n_queens::tests::",
        trace_key: "n_queens",
        skeleton_path: "exercises/recursion-backtracking/src/n_queens.rs",
        fixture: &[],
        concept_note: "The tutorial's literal version tracks a full 2D board — this \
            exercise simplifies that to `positions: Vec<usize>` (the column chosen for \
            each row), since queens go one per row in order, which is exactly the \
            push/recurse/pop idiom every other exercise in this module already uses. The \
            conflict-tracking (`cols`/`diag1`/`diag2`) is unchanged from the original — \
            only the STATE representation got simpler, not the algorithm.",
        hints: &[
            "`row` is just `positions.len()` — you don't need to pass it separately.",
            "Only call mark_inserted/push (and later mark_removed/pop) INSIDE the \
             \"column doesn't conflict\" branch — a row where every column conflicts \
             should never call either, since nothing was ever placed there.",
            "diag1 uses `row + n - col` and diag2 uses `row + col` to turn \"same \
             diagonal\" into \"same array index\" without ever going negative.",
        ],
        target: Some(4),
        starts_empty: true,
    },
];

pub const TREES_EXERCISES: &[Exercise] = &[
    Exercise {
        name: "binary_search_tree",
        test_filter: "binary_search_tree::tests::",
        trace_key: "binary_search_tree",
        skeleton_path: "exercises/trees/src/binary_search_tree.rs",
        fixture: &[5, 3, 8, 1, 4, 7, 9],
        concept_note: "Notice the trace position is DEPTH, not an array index — the same \
            call-stack-as-position idiom recursion_basics used for factorial/fibonacci, \
            extended to a tree descent. It's safe here for the same reason: a single \
            insert or contains call only ever walks ONE path from the root, so no two \
            probes in this trace can ever collide on the same depth.",
        hints: &[
            "Write private helpers that carry a `depth: usize` alongside the node, same \
             pattern as recursion_basics — the public insert/contains just start it at 0.",
            "Call mark_visited(depth) every time you step into a Some(node), before \
             comparing against its value.",
            "insert: call mark_inserted(depth, value) at the None case where you place \
             the new node. contains: call found(depth) right before returning true on a \
             match.",
        ],
        target: Some(4),
        starts_empty: true,
    },
    Exercise {
        name: "bst_deletion",
        test_filter: "bst_deletion::tests::",
        trace_key: "bst_deletion",
        skeleton_path: "exercises/trees/src/bst_deletion.rs",
        fixture: &[10, 5, 7, 3],
        concept_note: "Watch the depth counter through the two-children case: it does NOT \
            restart at 0 for the in-order-successor search — it keeps counting up from the \
            target's own depth. Restarting would let an unrelated sibling at that same \
            depth collide with the successor in the trace. The Set event you see at the \
            end is the target's OWN position changing value, not a swap.",
        hints: &[
            "Leaf and one-child cases: mark_visited(depth) on the way down, then \
             mark_removed(depth) where the node is actually dropped or replaced by its \
             one child.",
            "Two-children case: find the in-order successor (the minimum of the right \
             subtree) using a helper that keeps incrementing depth from depth + 1 — never \
             restart it at 0.",
            "Once you have the successor's value, call mark_set(depth, successor_value) \
             at the TARGET's original depth, copy the value into the target node, then \
             remove the successor's own (now-duplicate) leaf with mark_removed at its \
             depth.",
        ],
        target: Some(5),
        starts_empty: false,
    },
    Exercise {
        name: "tree_traversals",
        test_filter: "tree_traversals::tests::",
        trace_key: "tree_traversals",
        skeleton_path: "exercises/trees/src/tree_traversals.rs",
        fixture: &[5, 3, 8, 1, 4, 7, 9],
        concept_note: "Same append-tracing idiom insert.rs (linked lists) already taught \
            you: mark_inserted(result.len(), value) right where a value gets pushed. The \
            trace you're watching is inorder specifically, but all four traversals \
            (inorder/preorder/postorder/level_order) use this exact same idiom — only the \
            ORDER they visit nodes in differs.",
        hints: &[
            "inorder/preorder/postorder are recursive: the only difference between them \
             is WHERE the mark_inserted(result.len(), value)/result.push(value) pair sits \
             relative to the two recursive calls on left and right.",
            "level_order is iterative, not recursive — use a VecDeque as a queue, pushing \
             children onto the back as you pop and visit each node from the front.",
            "inorder: left, then visit, then right. preorder: visit, then left, then \
             right. postorder: left, then right, then visit.",
        ],
        target: None,
        starts_empty: true,
    },
    Exercise {
        name: "heap",
        test_filter: "heap::tests::",
        trace_key: "heap",
        skeleton_path: "exercises/trees/src/heap.rs",
        fixture: &[1, 3, 2, 0],
        concept_note: "No new trace events here at all — a heap is stored as a flat Vec, \
            so it reuses cmp_lt/swap exactly as written, the same helpers bubble_sort/ \
            selection_sort/insertion_sort already called. A heap is a genuinely flat value \
            sequence under the hood; the tree picture is just how humans like to draw it.",
        hints: &[
            "A node at index i has parent (i - 1) / 2, and children 2i + 1 / 2i + 2 — \
             that arithmetic is the entire structure, there's no pointer-based tree here.",
            "insert: push the value onto the end, then bubble it UP — swap with its \
             parent while it's smaller, using cmp_lt(&self.data, index, parent).",
            "extract_min: pop the last element, swap it into index 0 (replacing the old \
             minimum), then bubble it DOWN — repeatedly swap with whichever child is \
             smaller, until neither child is smaller than it.",
        ],
        target: None,
        starts_empty: false,
    },
    Exercise {
        name: "self_balancing_bst",
        test_filter: "self_balancing_bst::tests::",
        trace_key: "self_balancing_bst",
        skeleton_path: "exercises/trees/src/self_balancing_bst.rs",
        fixture: &[],
        concept_note: "No animated trace for this one — unlike a stack push or a single \
            node swap, a rotation restructures several nodes' pointers AND their heights \
            at once, and no existing trace event (or reasonable approximation) can \
            represent that faithfully. rotate_left/rotate_right/height tracking are given \
            to you fully implemented in avl.rs; the lesson is knowing which of the four \
            cases (LL/RR/LR/RL) applies after each insert.",
        hints: &[
            "Insert like a normal BST first (recursively), then on the way back up: call \
             update_height(), then check balance_factor() at the current node.",
            "Compare the balance factor's SIGN against whether the inserted value went \
             left or right of that node's own left/right child, to tell an LL/RR case \
             apart from an LR/RL case.",
            "LL: balance_factor() > 1 and value < left child's value → rotate_right. RR: \
             mirror image → rotate_left. LR/RL: rotate the CHILD first, then rotate the \
             node itself — both given as AvlNode::rotate_left/rotate_right.",
        ],
        target: None,
        starts_empty: true,
    },
    Exercise {
        name: "self_balancing_bst_delete",
        test_filter: "self_balancing_bst_delete::tests::",
        trace_key: "self_balancing_bst_delete",
        skeleton_path: "exercises/trees/src/self_balancing_bst_delete.rs",
        fixture: &[],
        concept_note: "Same BST-delete cases as bst_deletion (leaf, one child, two \
            children via in-order successor), but rebalancing has to happen on the way \
            back up, same as self_balancing_bst's insert. The four conditions are \
            genuinely different from insertion's, though: deletion can require a rotation \
            even when the relevant child's OWN balance factor is already 0, which \
            insertion never triggers — so don't just copy insertion's four checks over \
            unchanged.",
        hints: &[
            "BST delete first: leaf returns None, one child returns that child, two \
             children replace the value with the in-order successor and delete the \
             successor from the right subtree.",
            "Call update_height() and check balance_factor() at EVERY level on the way \
             back up, not just where the node was actually removed — the imbalance can \
             surface several levels above the deletion.",
            "LL: balance_factor() > 1 and the left child's OWN balance_factor() >= 0 \
             (not strictly > 0, unlike insertion) → rotate_right, and the mirror for RR \
             uses <= 0. Otherwise it's the two-step LR/RL case, same as insertion.",
        ],
        target: None,
        starts_empty: true,
    },
    Exercise {
        name: "red_black_tree",
        test_filter: "red_black_tree::tests::",
        trace_key: "red_black_tree",
        skeleton_path: "exercises/trees/src/red_black_tree.rs",
        fixture: &[],
        concept_note: "Untraced, same reasoning as self_balancing_bst — a rotation or \
            color flip restructures/recolors multiple nodes at once. This is a \
            left-leaning red-black tree (LLRB): red links only ever lean left, which \
            collapses the textbook's twelve-ish insertion cases down to three checks, \
            run in order, after every recursive insert call returns.",
        hints: &[
            "New nodes are always red (RbNode::new already does this) — insert like a \
             normal BST, then fix the shape on the way back up, in this exact order.",
            "1) Right-leaning red: right child red, left child not → rotate_left. 2) Two \
             consecutive left reds: left child red AND left child's left child red → \
             rotate_right.",
            "3) Both children red (a 4-node): flip_colors — this one can fire right after \
             case 2 resolves it, so check it unconditionally, not as an else-branch. All \
             three helpers (is_red/rotate_left/rotate_right/flip_colors) are given in rb.rs.",
        ],
        target: None,
        starts_empty: true,
    },
    Exercise {
        name: "red_black_tree_delete",
        test_filter: "red_black_tree_delete::tests::",
        trace_key: "red_black_tree_delete",
        skeleton_path: "exercises/trees/src/red_black_tree_delete.rs",
        fixture: &[],
        concept_note: "Untraced, same reasoning as red_black_tree. The hard part isn't \
            the BST-shape delete cases (same three as bst_deletion) — it's that you can \
            only safely delete THROUGH a red link. Before descending into a child that's \
            about to become a black 2-node, you have to borrow a red link from its \
            sibling first, using the given move_red_left/move_red_right — or the deletion \
            can lose a black link along that path and break the tree's black-height \
            invariant.",
        hints: &[
            "Guard against deleting a value that isn't in the tree FIRST (use \
             tree_contains) — move_red_left/move_red_right assume the search path they're \
             on actually continues, and will panic on a value that was never there.",
            "Descending left: if the left child and ITS left child are both black, call \
             move_red_left before recursing left. Descending right (or when the target \
             equals the current node and needs its right subtree): the mirror image with \
             move_red_right.",
            "Prime the root to Red before the very first recursive call if both its \
             children are black, and force it back to Black afterward — otherwise the \
             very first move_red_left/move_red_right has no red link to borrow from. Call \
             RbNode::fix_up (given) on the way back up at every level, same three checks \
             red_black_tree.rs's insert used.",
        ],
        target: Some(20),
        starts_empty: true,
    },
];

pub const GRAPHS_EXERCISES: &[Exercise] = &[
    Exercise {
        name: "bfs",
        test_filter: "bfs::tests::",
        trace_key: "bfs",
        skeleton_path: "exercises/graphs/src/bfs.rs",
        // A flattened edge list, not a value sequence: consecutive
        // (from, to) pairs describing a small diamond — 1 connects to 2
        // and 3, both of which connect to 4.
        fixture: &[1, 2, 1, 3, 2, 4, 3, 4],
        concept_note: "Notice the trace position is VISIT ORDER, not a tree depth or a call- \
            stack frame — the same append-tracing idiom tree_traversals used for inorder/ \
            preorder/postorder/level_order, just walking a graph's queue instead of a tree's \
            recursion. A HashSet, not an array, is what keeps BFS from ever revisiting a node \
            it's already seen.",
        hints: &[
            "Track visited nodes in a HashSet, and the frontier to explore in a VecDeque — \
             push_back to enqueue, pop_front to dequeue, that's what makes it breadth-FIRST.",
            "The moment you pop a node off the queue, call mark_inserted(order.len(), node) \
             right before pushing it into `order` — visit order IS trace position here, no \
             separate counter needed.",
            "When you look at a popped node's neighbors, mark each unvisited one visited \
             AND push it onto the queue in the SAME step — marking it only when it's dequeued \
             later would let it get enqueued twice.",
        ],
        target: Some(1),
        starts_empty: true,
    },
    Exercise {
        name: "dfs",
        test_filter: "dfs::tests::",
        trace_key: "dfs",
        skeleton_path: "exercises/graphs/src/dfs.rs",
        fixture: &[1, 2, 1, 3, 2, 4, 3, 4],
        concept_note: "Same fixture as bfs, same append-tracing idiom (mark_inserted(order.len(), \
            node)) — but watch the visit order diverge: DFS commits to one branch and follows \
            it all the way down before ever trying the next one, where BFS explores everything \
            one step away before going any further. Same graph, same starting point, genuinely \
            different order.",
        hints: &[
            "Write a private recursive helper that carries `visited`/`order` alongside the \
             current node — the public dfs() just kicks it off from `start`.",
            "Call mark_inserted(order.len(), node) the moment you enter a node (mark it \
             visited and push it into `order`), before you look at any of its neighbors.",
            "Loop over the current node's neighbors — for each unvisited one, recurse into \
             it immediately, don't just queue it up. That immediate recursion is what makes \
             this depth-FIRST instead of breadth-first.",
        ],
        target: Some(1),
        starts_empty: true,
    },
    Exercise {
        name: "cycle_detection",
        test_filter: "cycle_detection::tests::",
        trace_key: "cycle_detection",
        skeleton_path: "exercises/graphs/src/cycle_detection.rs",
        fixture: &[1, 2, 2, 3, 3, 1],
        concept_note: "Two different algorithms sharing one file: undirected cycle detection \
            has to ignore the edge you just walked IN on (the immediate parent) or every \
            single edge would look like a cycle — a directed graph doesn't have this problem, \
            since edges only go one way, so it tracks the CURRENT recursion stack instead. \
            Notice there's no depth counter here — trace position is visit order, same as \
            bfs/dfs, which is also what lets found() point back at exactly where the closing \
            node already sits in the trace.",
        hints: &[
            "Undirected: thread an Option<i32> parent through the recursion (None at the \
             very start) — not a magic sentinel value, since a real vertex could collide \
             with one. A cycle is an already-visited neighbor that ISN'T Some(that parent).",
            "Directed: track which nodes are on the CURRENT call stack (a HashSet you add to \
             on entry, remove from before returning) separately from which nodes have EVER \
             been visited — a cycle is a neighbor that's on the stack right now, not just \
             visited at some point in the past.",
            "Both: mark_inserted(*position, node) the first time you visit a node (then \
             increment position — never reset it, even across disconnected components), and \
             found(original_position) using the position you stored for whichever node closes \
             the cycle.",
        ],
        target: None,
        starts_empty: true,
    },
    Exercise {
        name: "connected_components",
        test_filter: "connected_components::tests::",
        trace_key: "connected_components",
        skeleton_path: "exercises/graphs/src/connected_components.rs",
        fixture: &[1, 2, 3, 4, 4, 5],
        concept_note: "One running position counter across the WHOLE call, not reset for \
            each new group — an earlier draft of this exercise reset it per component, which \
            looked fine on paper but would have made every later group's animation shove \
            every earlier group sideways, since inserting into the trace's display array is a \
            real shift, not an overwrite. Component grouping still exists correctly in the \
            returned Vec<Vec<i32>> and in the tests; it just isn't drawn as separate visual \
            groups.",
        hints: &[
            "Loop over every vertex; whichever ones you haven't visited yet each start a \
             fresh DFS that collects everything reachable into its own Vec<i32>.",
            "Write your own private DFS helper here rather than calling dfs() from dfs.rs — \
             this exercise's tests shouldn't depend on that one being solved.",
            "Thread ONE &mut usize position counter through every DFS call this function \
             makes, not a fresh counter per component — mark_inserted(*position, node) then \
             increment it, exactly once per vertex, for the whole call.",
        ],
        target: None,
        starts_empty: true,
    },
];

pub const DYNAMIC_PROGRAMMING_EXERCISES: &[Exercise] = &[
    Exercise {
        name: "climbing_stairs",
        test_filter: "climbing_stairs::tests::",
        trace_key: "climbing_stairs",
        skeleton_path: "exercises/dynamic-programming/src/climbing_stairs.rs",
        // A fixed-size, zeroed starting picture — six positions for
        // n=5 (target) — filled in by Set as the dp table builds,
        // exactly like counting_sort/radix_sort's output array.
        // fixture.len() must always be target + 1.
        fixture: &[0, 0, 0, 0, 0, 0],
        concept_note: "Same recurrence as Fibonacci (dp[i] = dp[i-1] + dp[i-2]) — the idiom \
            here isn't the math, it's BOTTOM-UP TABULATION: build every smaller answer first, \
            in order, so nothing ever gets recomputed. Notice this reuses mark_set exactly like \
            counting_sort/radix_sort already do for a value landing in a fixed-size output \
            array — no new trace event needed for the whole DP module.",
        hints: &[
            "dp[i] is the number of ways to reach step i. There's exactly one way to reach \
             step 0 (don't move) and step 1 (one single step) — those are your base cases.",
            "For every step after that, you either arrived via a 1-step move from i-1, or a \
             2-step move from i-2 — so dp[i] = dp[i-1] + dp[i-2].",
            "Call mark_set(i, dp[i] as i32) every time you fill a position — including \
             dp[0] and dp[1], not just inside the loop.",
        ],
        target: Some(5),
        starts_empty: false,
    },
    Exercise {
        name: "house_robber",
        test_filter: "house_robber::tests::",
        trace_key: "house_robber",
        skeleton_path: "exercises/dynamic-programming/src/house_robber.rs",
        // Unlike every other DP fixture, this IS the actual input — rob()
        // takes nums directly, so fixture doubles as both the display
        // picture and the function argument.
        fixture: &[2, 7, 9, 3, 1],
        concept_note: "A genuinely different recurrence from climbing_stairs' plain sum — this \
            one is a DECISION at every step: dp[i] = dp[i-1].max(dp[i-2] + nums[i]), either skip \
            house i and keep whatever you already had, or rob it and add to what you had two \
            houses back (never one, since adjacent houses can't both be hit). Same mark_set \
            idiom as climbing_stairs either way.",
        hints: &[
            "dp[i] is the most you can rob from houses 0..=i. Base cases: dp[0] = nums[0], \
             dp[1] = nums[0].max(nums[1]) — you can only take one of the first two.",
            "At each house after that, you're choosing between two options: don't rob it \
             (dp[i-1]) or rob it (dp[i-2] + nums[i]) — take whichever is bigger.",
            "Call mark_set(i, dp[i]) every time you fill a position, including dp[0] and \
             dp[1] before the loop starts.",
        ],
        target: None,
        starts_empty: false,
    },
    Exercise {
        name: "grid_paths",
        test_filter: "grid_paths::tests::",
        trace_key: "grid_paths",
        skeleton_path: "exercises/dynamic-programming/src/grid_paths.rs",
        // A 3x3 grid, flattened row-major into 9 zeroed positions.
        // fixture.len() must always be a multiple of target (cols);
        // rows is derived as fixture.len() / target in the dispatcher.
        fixture: &[0, 0, 0, 0, 0, 0, 0, 0, 0],
        concept_note: "The first genuinely 2D DP table in this curriculum, flattened row-major \
            into a single trace position (row * cols + col) since this project's trace events \
            only understand a flat array. The first row and first column are both base cases — \
            trace row 0 in FULL first (that includes the top-left corner), then column 0 \
            starting from row 1, not row 0, or the corner cell gets marked twice.",
        hints: &[
            "dp[row][col] is the number of paths to reach that cell. The entire first row \
             and first column are base case 1 — moving only right or only down, there's \
             exactly one way to get there.",
            "For every other cell, dp[row][col] = dp[row-1][col] + dp[row][col-1] — the paths \
             to reach it either came from above or from the left.",
            "mark_set(row * cols + col, value) for every cell. Trace the ENTIRE first row \
             (including the corner), then the first column starting at row 1 — starting \
             column 0's loop at row 0 instead would mark_set the corner twice.",
        ],
        target: Some(3),
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
