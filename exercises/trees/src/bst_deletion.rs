use crate::bst::{tree_contains, Bst, Node};
use algolings_trace::{mark_removed, mark_set, mark_visited};

// Implement delete for a binary search tree — the three classic cases:
// a leaf just goes away, a node with one child gets spliced out and
// replaced by that child, and a node with two children gets its VALUE
// replaced by its in-order successor (the leftmost node of its right
// subtree), with that successor's own (now-duplicate) node removed from
// the right subtree.
//
// Trace the descent the same way binary_search_tree does: mark_visited(depth)
// at every node you compare against. For a leaf or one-child deletion,
// mark_removed(depth) at the node you splice out — same as before.
//
// The two-children case is the one that needs care: don't restart the
// depth counter at 0 for the successor search — CONTINUE incrementing it
// from where you found the target (search into the right subtree at
// depth + 1, then keep going left). Restarting at 0 would collide with
// whatever unrelated sibling node happens to share that depth. Once you
// find the successor, call mark_set(target_depth, successor_value) —
// the target's slot didn't get removed, its VALUE changed — and
// mark_removed(successor_depth) for the successor's own node being
// spliced out.
impl Bst {
    pub fn delete(&mut self, value: i32) {
        self.root = delete_node(self.root.take(), value, 0);
    }
}

fn delete_node(node: Option<Box<Node>>, value: i32, depth: usize) -> Option<Box<Node>> {
    todo!("descend to find value (mark_visited), then remove/splice/replace depending on how many children it has")
}

fn extract_min(node: Box<Node>, depth: usize) -> (i32, Option<Box<Node>>) {
    todo!("descend left as far as possible (mark_visited each step), mark_removed at the leftmost node, return its value and what should take its place")
}

#[cfg(test)]
include!("../../tests-shared/bst_deletion_tests.rs");
