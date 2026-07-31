# algolings

**Learn algorithms by fixing them — in idiomatic Rust, with a live trace of your own solution.**

algolings is [rustlings](https://github.com/rust-lang/rustlings) for algorithms. You clone the repo, run one command, and it watches your work: save a broken exercise and see why it fails, fix it and watch your own solution run as an animated trace in your terminal, then read a short note on the Rust idiom your solution just used.

![algolings solving bubble_sort: a real save, a real test run, a real trace of the learner's own comparisons and swaps, then the concept note](demo.gif)

*(shown in `--plain` mode for a reliable recording — the default mode renders the same trace as an animated terminal UI with step-through and auto-play controls.)*

## Why

Rustlings teaches Rust syntax through small exercises. LeetCode-style tools teach algorithm correctness. Neither teaches you *why* a correct Rust solution looks the way it does — why a sort takes `&mut [i32]` instead of `Vec<i32>`, why `isize` shows up where you'd expect `usize`, why one line copies a value before an array gets mutated out from under it.

algolings sits at the intersection: real algorithms, idiomatic Rust, and a trace of *your own* comparisons and swaps — not a canned animation.

## Quickstart

```sh
git clone https://github.com/SunnsTechnologies/algolings
cd algolings
cargo run
```

That's it. `algolings` starts with the sorting module, watching `exercises/sort/src/`. Open `exercises/sort/src/bubble.rs` in your editor, replace the `todo!()` with a real bubble sort, and save. algolings will:

1. Run the exercise's tests.
2. If they fail, show you why.
3. If they pass, replay your solution as an animated trace and show a note on the Rust idiom it relies on.
4. Move you on to the next exercise — and once a module's done, straight into the next one (sorting, then searching, then linked lists, then whatever comes after).

Prefer plain sequential text instead of the animated terminal view (useful for screen readers, CI, or piping output)?

```sh
cargo run -- --plain
```

## What you'll learn

**Sorting** — eight algorithms, in the order you solve them:

| # | Exercise | Idiom you'll hit |
|---|----------|-------------------|
| 1 | Bubble sort | Why a step-tracing helper function is just a normal call, not magic |
| 2 | Selection sort | Tracking an index instead of a value |
| 3 | Insertion sort | Copying a value out before the array mutates under it |
| 4 | Merge sort | Recursing on index ranges instead of `split_at_mut` sub-slices |
| 5 | Quick sort | `isize` vs. `usize`, and why `-1` sometimes has to be a valid bound |
| 6 | Shell sort | Guarding unsigned subtraction against underflow |
| 7 | Counting sort | An algorithm with zero comparisons |
| 8 | Radix sort | Where the non-negative-integer assumption actually matters |

**Searching** — next up once sorting's done:

| # | Exercise | Idiom you'll hit |
|---|----------|-------------------|
| 1 | Linear search | Why this returns `Option<usize>` instead of a sentinel like `-1` |
| 2 | Binary search | `left + (right - left) / 2` instead of `(left + right) / 2`, to avoid overflow |

**Linked lists** — after searching, a first taste of pointer-based structures:

| # | Exercise | Idiom you'll hit |
|---|----------|-------------------|
| 1 | Insert (push_front/push_back) | Why reaching "the end" means walking from `head`, not indexing |
| 2 | Remove | `current.take()` to move a node out of the list before deciding what to do with it |
| 3 | Traverse | `.as_deref()` to walk `Option<Box<Node<T>>>` with shared references, not ownership |
| 4 | Reverse | Reversing in place with three tracked pointers (`prev`/`current`/`next`), no new list allocated |
| 5 | Doubly-linked push | `Rc<RefCell<Node>>` for forward links, `Weak` for backward — a strong reference both ways would leak the whole list |
| 6 | Doubly-linked pop | `prev` turns "find the node before the tail" from an O(n) walk into an O(1) upgrade |
| 7 | Doubly-linked contains | Same forward-only search as Traverse, but walking `Rc<RefCell<Node>>` by cloning and borrowing instead of `.as_deref()` |
| 8 | Converging search | Searching from both ends at once — the one thing a singly-linked list structurally can't do |

**Stacks & queues** — same value sequences, but access is restricted to one or both ends on purpose:

| # | Exercise | Idiom you'll hit |
|---|----------|-------------------|
| 1 | Stack (Vec-backed) | `Vec`'s own push/pop already operate on the end — no pointers needed for LIFO |
| 2 | Stack (linked-list-backed) | The same push_front/pop_front idiom from Insert/Remove, wrapped in its own type |
| 3 | Queue (VecDeque-backed) | Why `Vec::remove(0)` being O(n) is exactly the problem a ring buffer solves |
| 4 | Queue (linked-list-backed) | `Rc<RefCell<Node>>` for a shared `tail` reference, with no `Weak` needed — nothing here ever walks backward |

Each exercise's full write-up (complexity analysis, walkthrough, alternative implementations) is also available at [learn.sunnstech.com](https://learn.sunnstech.com) if you want the deeper version.

## While you're solving an exercise

```
[ 5 | 1* | 4* | 2 | 8 ]
compare [1] and [0] -> [ 5* | 1* | 4 | 2 | 8 ]
```

**In the trace view** (after an exercise passes), single keypresses:
- **space** step through one comparison/swap at a time
- **a** toggle auto-play
- **q** quit the trace and move on to the concept note

**While an exercise is failing**, type a command and press enter:
- **h** then enter — show the next hint. Hints escalate (a vague nudge, then more specific, then near the answer) and stay in order until you solve the exercise.

Compared elements are marked with both color *and* a `*` glyph — never color alone, so the trace stays legible if you're colorblind or running `--plain`.

## How it works

algolings is organized into modules (sorting, searching, linked lists, stacks & queues, ...), each a pair of crates:

- **`algolings-trace`** — the tracing primitives (`cmp_lt`, `cmp_lt_values`, `swap`, `set_at`, `mark_sorted`, `probe`, `found`, `narrow_range`, `mark_inserted`, `mark_removed`, `mark_visited`, `mark_converging`). Exercise solutions call these instead of raw `<` / `.swap()` / `==` — same behavior, but it lets algolings record what *your* solution actually did.
- **`algolings-cli`** — the `algolings` binary: watches the current module's exercise files, runs their tests, renders the trace, and moves on to the next module once one's fully solved.
- **`exercises/sort`**, **`exercises/search`**, **`exercises/linked-list`**, **`exercises/stacks-queues`** — the exercises you edit. Each one ships broken (`todo!()`).
- **`exercises/sort-solutions`**, **`exercises/search-solutions`**, **`exercises/linked-list-solutions`**, **`exercises/stacks-queues-solutions`** — reference solutions, used by CI to prove every exercise is solvable, and as the source for hints and concept notes.

When you save a file, algolings runs `cargo test` against it. If it passes, algolings runs a *separate* subprocess (`cargo run --bin <package>-trace`) with tracing enabled against your now-correct solution, and animates the result. Running the trace as its own subprocess — rather than linking your code into the long-running watch process — means it always reflects what you just saved, and a genuinely broken infinite loop can be killed outright instead of hanging the tool.

The linked-list module's trace still uses the same array-style animation as sorting/searching: a list's values, in the order you'd walk them, shown as a growable/shrinkable row rather than literal boxes and arrows. It doesn't show pointer structure directly — the Rust code and concept notes carry that idiom instead.

## Project layout

```
algolings-trace/                  tracing primitives shared by exercises and the CLI
algolings-cli/                     the algolings binary
exercises/sort/                    sorting exercises you edit (start broken)
exercises/sort-solutions/          sorting reference solutions (used by CI + hints)
exercises/search/                  searching exercises you edit (start broken)
exercises/search-solutions/        searching reference solutions (used by CI + hints)
exercises/linked-list/             linked-list exercises you edit (start broken)
exercises/linked-list-solutions/   linked-list reference solutions (used by CI + hints)
exercises/stacks-queues/           stacks & queues exercises you edit (start broken)
exercises/stacks-queues-solutions/ stacks & queues reference solutions (used by CI + hints)
exercises/tests-shared/            one test suite per exercise, shared by both crates in its module
```

## Contributing

Found a bug, or want to help finish the linked-list module (Floyd's cycle detection) or port the next one (hash tables, recursion & backtracking, trees, graphs, dynamic programming)? Issues and PRs welcome.

To verify the whole repo (not just the exercises you're working on), use:

```sh
cargo test --workspace --no-fail-fast
```

The plain `cargo test --workspace` isn't enough here: the exercise skeletons in `exercises/sort/` are *supposed* to fail (they start unsolved), and Cargo's default fail-fast behavior means it stops at that first failing package before ever reaching `exercises/sort-solutions/`'s tests. `--no-fail-fast` makes sure the reference solutions actually get checked.

## License

MIT — see [LICENSE](LICENSE).
