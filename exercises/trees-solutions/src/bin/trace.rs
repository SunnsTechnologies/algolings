//! Same dispatcher as exercises/trees/src/bin/trace.rs, run against the
//! reference solutions instead of the learner-facing skeletons — used by
//! algolings-cli's tests so they don't need to mutate the shared skeleton
//! file to exercise the success path.

use algolings_trace::{enable, take_events};
use trees_solutions::{inorder, AvlTree, Bst, MinHeap, RbTree};

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
