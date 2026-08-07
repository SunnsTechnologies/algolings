//! Dispatcher binary: runs the requested exercise's function(s) with
//! tracing enabled, and prints the recorded events as JSON to stdout.
//!
//! Four of these exercises (self_balancing_bst, self_balancing_bst_delete,
//! red_black_tree, red_black_tree_delete) are untraced — a rotation or
//! color flip restructures multiple nodes at once, which no existing
//! trace event can represent — so their `fixture` argument is ignored
//! (`_fixture`) and `enable()` naturally records zero events, same as
//! hash-tables' `hash_entry`.
//!
//! Exercises that need a "starting tree" not built by the operation being
//! traced (bst_deletion, tree_traversals, heap, and the delete variants
//! above) build it via each scaffolding module's `from_values` BEFORE
//! calling `enable()`, so only the graded operation's own events show up —
//! matching the same principle their shared tests use: never depend on
//! another exercise's (possibly unsolved) `insert`.
//!
//! Run as a fresh subprocess (`cargo run -p exercises-trees --bin
//! exercises-trees-trace -- <name> <fixture-json> [target]`) so it always
//! reflects the CURRENT on-disk solution code.

use algolings_trace::{enable, take_events};
use exercises_trees::{inorder, AvlTree, BinaryTree, Bst, MinHeap, RbTree};

fn main() {
    let mut args = std::env::args().skip(1);
    let name = args
        .next()
        .unwrap_or_else(|| usage_error("missing exercise name"));
    let fixture_json = args
        .next()
        .unwrap_or_else(|| usage_error("missing fixture JSON"));
    let fixture: Vec<i32> =
        serde_json::from_str(&fixture_json).unwrap_or_else(|e| usage_error(&e.to_string()));
    let target: Option<i32> = args.next().and_then(|s| s.parse().ok());

    match name.as_str() {
        "binary_search_tree" => {
            let mut tree = Bst::new();
            enable();
            for value in fixture {
                tree.insert(value);
            }
            tree.contains(target.unwrap_or(4));
        }
        "bst_deletion" => {
            let mut tree = Bst::from_values(&fixture);
            enable();
            tree.delete(target.unwrap_or(5));
        }
        "tree_traversals" => {
            let tree = Bst::from_values(&fixture);
            enable();
            let mut result = Vec::new();
            inorder(&tree.root, &mut result);
        }
        "binary_tree_insert" => {
            let mut tree = BinaryTree::new();
            enable();
            for value in fixture {
                tree.insert(value);
            }
        }
        "heap" => {
            let (initial, pushed) = fixture.split_at(fixture.len().saturating_sub(1));
            let mut heap = MinHeap::from_values(initial);
            enable();
            if let Some(&value) = pushed.first() {
                heap.insert(value);
            }
        }
        "self_balancing_bst" => {
            let mut tree = AvlTree::new();
            enable();
            for value in [30, 20, 40, 10, 25, 35, 45] {
                tree.insert(value);
            }
        }
        "self_balancing_bst_delete" => {
            let mut tree = AvlTree::from_values(&[30, 20, 40, 10, 25, 35, 45]);
            enable();
            tree.delete(target.unwrap_or(40));
        }
        "red_black_tree" => {
            let mut tree = RbTree::new();
            enable();
            for value in [10, 20, 30, 15, 25, 5] {
                tree.insert(value);
            }
        }
        "red_black_tree_delete" => {
            let mut tree = RbTree::from_values(&[10, 20, 30, 15, 25, 5]);
            enable();
            tree.delete(target.unwrap_or(20));
        }
        other => {
            eprintln!("unknown exercise: {other}");
            std::process::exit(2);
        }
    }
    let events = take_events();

    println!(
        "{}",
        serde_json::to_string(&events).expect("events always serialize")
    );
}

fn usage_error(message: &str) -> ! {
    eprintln!("usage: trace <exercise-name> <fixture-json> [target]\n{message}");
    std::process::exit(2);
}
