use crate::weighted_graph::WeightedGraph;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

// Prim's algorithm: grow a minimum spanning tree one vertex at a time,
// always adding the CHEAPEST edge that reaches a not-yet-included
// vertex. Starting from one vertex, keep a min-heap of "frontier" edges
// (one endpoint already in the tree, the other not) and repeatedly take
// the cheapest one that doesn't just reconnect to something already
// included.
//
// Ships untraced — a growing list of (from, to, weight) triples doesn't
// fit the existing trace events, which only carry one i32 value each.
pub fn prim_mst(graph: &WeightedGraph, start: i32) -> Vec<(i32, i32, i32)> {
    todo!("visited set + min-heap of (weight, from, to) frontier edges — pop the cheapest, skip if `to` is already visited, otherwise add it to the MST and push its own frontier edges")
}

#[cfg(test)]
include!("../../tests-shared/mst_prim_tests.rs");
