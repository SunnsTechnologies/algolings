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
| 9 | Floyd's cycle detection | `Box` can't represent a real cycle at all (exclusive ownership forbids it) — reuses the doubly-linked list's `Rc<RefCell<Node>>` instead, comparing pointer identity, never values |

**Stacks & queues** — same value sequences, but access is restricted to one or both ends on purpose:

| # | Exercise | Idiom you'll hit |
|---|----------|-------------------|
| 1 | Stack (Vec-backed) | `Vec`'s own push/pop already operate on the end — no pointers needed for LIFO |
| 2 | Stack (linked-list-backed) | The same push_front/pop_front idiom from Insert/Remove, wrapped in its own type |
| 3 | Queue (VecDeque-backed) | Why `Vec::remove(0)` being O(n) is exactly the problem a ring buffer solves |
| 4 | Queue (linked-list-backed) | `Rc<RefCell<Node>>` for a shared `tail` reference, with no `Weak` needed — nothing here ever walks backward |

**Hash tables** — a genuinely different structure: position comes from a hash, not a sequence:

| # | Exercise | Idiom you'll hit |
|---|----------|-------------------|
| 1 | Custom hash table | `DefaultHasher` + modulo for bucket placement, chaining (`Vec<Vec<i32>>`) so collisions coexist instead of overwriting |
| 2 | HashMap entry API | `entry(key).or_insert(0)` — one lookup instead of two; the first exercise with no animated trace, since std `HashMap` has no stable index to replay against |

**Recursion & backtracking** — no data structure at all here, just the call stack itself:

| # | Exercise | Idiom you'll hit |
|---|----------|-------------------|
| 1 | Recursion basics | Tracing the CALL STACK itself — depth grows on the way down, shrinks on the way back up |
| 2 | Tail recursion | Rust doesn't guarantee tail-call optimization — the accumulator rewrite changes what's computed, not whether frames pile up |
| 3 | Subsets | Backtracking's core idiom: push a candidate, recurse, pop it back off (undo) |
| 4 | Combinations | Same push/recurse/pop, gated by a fixed result length instead of capturing every prefix |
| 5 | Permutations | `used: Vec<bool>` instead of a start index — every element has to stay available at every position |
| 6 | Permutations with duplicates | Sort first, then skip a duplicate unless its earlier twin is already placed |
| 7 | N-Queens | `positions: Vec<usize>` instead of a 2D board — same push/recurse/pop idiom as every other backtracking exercise here |

**Trees** — the full chapter, from a plain unordered binary tree up through both major kinds of self-balancing tree:

| # | Exercise | Idiom you'll hit |
|---|----------|-------------------|
| 1 | Binary tree — insert | No ordering rule to compare against, so insert is breadth-first: the trace position is a level-order slot index (root = 0, children of `i` are `2i+1`/`2i+2`), the same arithmetic the heap exercise uses, just walked via Box pointers instead of a flat Vec |
| 2 | Binary search tree | Depth-as-position, the same call-stack idiom recursion_basics used, now walking a tree instead of a flat call stack |
| 3 | BST deletion | Deleting a two-children node without losing track of an unrelated sibling at the same depth |
| 4 | Tree traversals | Same append-tracing idiom as linked-list Insert — inorder/preorder/postorder/level_order differ only in WHERE the visit happens |
| 5 | Heap (min-heap) | A "tree" that's really just a flat `Vec` and index arithmetic — reuses sorting's `cmp_lt`/`swap` unchanged |
| 6 | Self-balancing BST (AVL) — insert | Height tracking and the four rotation cases (LL/RR/LR/RL); ships without an animated trace, since a rotation restructures multiple nodes at once |
| 7 | Self-balancing BST (AVL) — delete | The same four cases, but genuinely different trigger conditions than insertion's |
| 8 | Red-black tree — insert | A left-leaning red-black tree (LLRB): three ordered checks (right-leaning red, two left reds, a 4-node split) replace AVL's four |
| 9 | Red-black tree — delete | Borrowing a red link before descending into a black node, so deletion never breaks the tree's black-height invariant |

**Graphs** — the full chapter: unweighted traversal, weighted shortest paths, spanning trees, ordering, and connectivity:

| # | Exercise | Idiom you'll hit |
|---|----------|-------------------|
| 1 | Breadth-first search | A HashSet for visited, a VecDeque as the frontier — the same append-tracing idiom tree_traversals used, now walking a queue instead of a tree |
| 2 | Depth-first search | Same fixture and idiom as BFS, but committing to one branch all the way down before trying the next — same graph, genuinely different visit order |
| 3 | Cycle detection | Undirected tracks the immediate parent (Option, never a magic sentinel) so the edge you just walked in on doesn't look like a cycle; directed tracks the current recursion stack instead, since edges only go one way |
| 4 | Connected components | One running position counter across the whole call, never reset per group — resetting would make each new component's animation shove every earlier one sideways, since a trace insert is a real shift, not an overwrite |
| 5 | Dijkstra | A stable per-node trace position, assigned once — watch a distance get set, then IMPROVED to a smaller value once a cheaper path is found |
| 6 | Bellman-Ford | Same idiom as Dijkstra, but handles negative edge weights — if distances are still changing after exactly vertex-count-minus-one passes, that instability itself is the negative-cycle detector |
| 7 | Floyd-Warshall | All-pairs, not single-source — a 2D table flattened row-major like grid_paths, except the same cell can improve more than once as each vertex is tried as a route-through point |
| 8 | Minimum spanning tree — Prim's | Grows a tree outward from one vertex, a min-heap always picking the cheapest frontier edge; ships without an animated trace, since an MST edge is three values, not one |
| 9 | Minimum spanning tree — Kruskal's | Sorts every edge cheapest-first, uses a given union-find structure to skip whichever ones would close a cycle |
| 10 | Topological sort — DFS-based | Traces only the real returned order, not the raw finish-order push sequence (which runs backward relative to it) — one untraced bookkeeping pass, one traced pass |
| 11 | Topological sort — Kahn's | Peels off vertices with no remaining dependencies one at a time; builds the real order directly, no reversal needed |
| 12 | Strongly connected components | Kosaraju's two-pass algorithm — an untraced finish-order pass, then a traced pass over the reversed graph using connected_components' exact never-reset idiom |

**Dynamic programming** — the final module: optimizing recursion by caching overlapping subproblems:

| # | Exercise | Idiom you'll hit |
|---|----------|-------------------|
| 1 | Climbing stairs | Bottom-up tabulation — the same recurrence as Fibonacci, but built forward instead of recursed into, so nothing's ever recomputed |
| 2 | House robber | A DECISION at every step (skip vs. take), not a plain sum — dp[i] = dp[i-1].max(dp[i-2] + nums[i]) |
| 3 | Grid paths | The first genuinely 2D DP table, flattened row-major into one trace position — the first row and column are base cases, traced in an order that avoids double-marking the shared corner cell |

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

algolings is organized into modules (sorting, searching, linked lists, stacks & queues, hash tables, recursion & backtracking, trees, graphs, dynamic programming), each a pair of crates:

- **`algolings-trace`** — the tracing primitives (`cmp_lt`, `cmp_lt_values`, `swap`, `set_at`, `mark_sorted`, `probe`, `found`, `narrow_range`, `mark_inserted`, `mark_removed`, `mark_visited`, `mark_converging`, `mark_set`). Exercise solutions call these instead of raw `<` / `.swap()` / `==` — same behavior, but it lets algolings record what *your* solution actually did.
- **`algolings-cli`** — the `algolings` binary: watches the current module's exercise files, runs their tests, renders the trace, and moves on to the next module once one's fully solved.
- **`exercises/sort`**, **`exercises/search`**, **`exercises/linked-list`**, **`exercises/stacks-queues`**, **`exercises/hash-tables`**, **`exercises/recursion-backtracking`**, **`exercises/trees`**, **`exercises/graphs`**, **`exercises/dynamic-programming`** — the exercises you edit. Each one ships broken (`todo!()`).
- **`exercises/sort-solutions`**, **`exercises/search-solutions`**, **`exercises/linked-list-solutions`**, **`exercises/stacks-queues-solutions`**, **`exercises/hash-tables-solutions`**, **`exercises/recursion-backtracking-solutions`**, **`exercises/trees-solutions`**, **`exercises/graphs-solutions`**, **`exercises/dynamic-programming-solutions`** — reference solutions, used by CI to prove every exercise is solvable, and as the source for hints and concept notes.

When you save a file, algolings runs `cargo test` against it. If it passes, algolings runs a *separate* subprocess (`cargo run --bin <package>-trace`) with tracing enabled against your now-correct solution, and animates the result. Running the trace as its own subprocess — rather than linking your code into the long-running watch process — means it always reflects what you just saved, and a genuinely broken infinite loop can be killed outright instead of hanging the tool.

The linked-list module's trace still uses the same array-style animation as sorting/searching: a list's values, in the order you'd walk them, shown as a growable/shrinkable row rather than literal boxes and arrows. It doesn't show pointer structure directly — the Rust code and concept notes carry that idiom instead. `floyds_cycle_detection` is the one exception, and reuses `mark_set` on two STABLE positions instead: position 0 always shows the slow pointer's current value, position 1 the fast pointer's — both land on the same value the instant a cycle closes, since a list with a real cycle has no defined "end" for the array-growing idiom to build toward.

The hash-table module's trace is a real simplification, not just a different skin on the same idea: `custom_hash_table`'s row shows one value per BUCKET (via `mark_set`), so it can demonstrate hash-based placement but not two colliding keys coexisting in the same bucket — the tests verify chaining survives collisions regardless of what the trace can draw. `hash_entry` has no trace at all, the first exercise without one: std `HashMap` has no stable index or iteration order for a replay to point at.

The recursion & backtracking module's trace doesn't show a data structure at all — `recursion_basics`/`tail_recursion` trace the CALL STACK itself (one frame per recursion depth), and `subsets`/`combinations`/`permutations`/`permutations_with_duplicates`/`n_queens` all reuse the same `Insert`/`Remove`/`Found` events for backtracking's push-candidate/recurse/pop-to-undo idiom — no new trace infrastructure was needed for this whole module.

The trees module reuses more than it invents. `binary_tree_insert` traces a level-order SLOT INDEX instead of depth — a plain binary tree has no ordering rule to descend by comparison, so insert is breadth-first (a queue of `(node, index)` pairs), and a node at index `i` has children at `2i+1`/`2i+2`, the same arithmetic `heap` uses for its flat Vec, just walked via Box pointers instead of indexing an array directly. `binary_search_tree`/`bst_deletion` extend recursion_basics's call-stack idiom to a tree descent (depth stands in for position, safe there specifically because a single insert/contains call only ever walks one root-to-leaf path); `tree_traversals` reuses linked-list Insert's append-tracing idiom; `heap` reuses sorting's `cmp_lt`/`swap` completely unchanged, since a heap really is just a flat `Vec` under the hood. The two self-balancing exercise pairs (AVL and red-black) ship with no animated trace at all — a rotation or color flip restructures several nodes' pointers (and, for red-black, colors) simultaneously, which no existing trace event can represent faithfully. Their rotation/color-flip mechanics are given to you fully implemented; the lesson in each is the case-decision logic that decides when to call them.

The graphs module traces most exercises as visit order, not tree depth or array index — `bfs`/`dfs` reuse tree_traversals' append idiom directly (`mark_inserted(order.len(), node)`), and `cycle_detection`/`connected_components`/`strongly_connected_components` use one running position counter across the WHOLE call rather than resetting per component or per branch, since a trace `Insert` is a real array shift, not an overwrite. `dijkstra`/`bellman_ford` instead assign every vertex a STABLE position once up front and `mark_set` it on every distance improvement — the same slot can update more than once over a single run, which is exactly how the trace shows a cheaper path being found. `mst_prim`/`mst_kruskal` ship with no animated trace at all: a minimum-spanning-tree edge is three values (from, to, weight), and the existing trace events only carry one.

The dynamic programming module reuses the `Set` idiom `counting_sort`/`radix_sort`/`custom_hash_table` already established — a fixed-size, zeroed starting picture, filled in as the DP table builds — rather than inventing anything new. `grid_paths` is the one exercise with a genuinely 2D table, flattened row-major into a single trace position; its first row and column (both DP base cases) have to be traced in a specific order (the full first row, then the first column starting one row down) or the shared corner cell gets marked twice.

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
exercises/hash-tables/             hash-table exercises you edit (start broken)
exercises/hash-tables-solutions/   hash-table reference solutions (used by CI + hints)
exercises/recursion-backtracking/            recursion & backtracking exercises you edit (start broken)
exercises/recursion-backtracking-solutions/  recursion & backtracking reference solutions (used by CI + hints)
exercises/trees/                   trees exercises you edit (start broken)
exercises/trees-solutions/         trees reference solutions (used by CI + hints)
exercises/graphs/                  graphs exercises you edit (start broken)
exercises/graphs-solutions/        graphs reference solutions (used by CI + hints)
exercises/dynamic-programming/            dynamic programming exercises you edit (start broken)
exercises/dynamic-programming-solutions/  dynamic programming reference solutions (used by CI + hints)
exercises/tests-shared/            one test suite per exercise, shared by both crates in its module
```

## Contributing

The full curriculum from the design doc (sorting → searching → linked lists → stacks & queues → hash tables → recursion & backtracking → trees → graphs → dynamic programming) now has at least one exercise per module, and the linked-list, graphs, and trees modules cover their entire tutorial chapters. Found a bug, or want to help elsewhere? Issues and PRs welcome.

To verify the whole repo (not just the exercises you're working on), use:

```sh
cargo test --workspace --no-fail-fast
```

The plain `cargo test --workspace` isn't enough here: the exercise skeletons in `exercises/sort/` are *supposed* to fail (they start unsolved), and Cargo's default fail-fast behavior means it stops at that first failing package before ever reaching `exercises/sort-solutions/`'s tests. `--no-fail-fast` makes sure the reference solutions actually get checked.

## License

MIT — see [LICENSE](LICENSE).
