use crate::graph::Graph;
use algolings_trace::{disable, enable, mark_inserted, take_events, Event};
use std::collections::HashSet;

// Topological sort (DFS-based): order every vertex so each one comes
// before everything it points to. The idea: run DFS from every
// not-yet-visited vertex, and the moment a vertex has no more unvisited
// work left below it (every neighbor is done), push it onto a stack.
// That stack ends up in REVERSE topological order — a vertex only gets
// pushed once everything reachable from it is already accounted for, so
// reversing the stack puts "comes first" at the front.
//
// A cycle makes this undefined — detect one via a `visiting` set that
// tracks the CURRENT recursion path (separate from `visited`, which
// tracks everything ever finished): reaching a node that's still
// `visiting` means you've looped back on yourself.
//
// Trace ONLY the actual returned order, not the raw finish-order stack
// (which runs backward relative to what this function returns) — build
// the finish-order stack untraced, reverse it, THEN walk the reversed
// stack calling mark_inserted(order.len(), node) into a fresh `order`
// vec. This is the one extra pass that keeps "the trace IS the return
// value" true here too, the same as every other traced exercise.
pub fn topological_sort_dfs(graph: &Graph) -> Option<Vec<i32>> {
    todo!("DFS from every unvisited node (untraced), pushing onto a finish-order stack; detect a cycle via a `visiting` set; reverse the stack, then build+trace the real `order` from it")
}

#[cfg(test)]
include!("../../tests-shared/topological_sort_recursive_tests.rs");
