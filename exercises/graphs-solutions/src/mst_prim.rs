use crate::weighted_graph::WeightedGraph;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

/// Reference solution for the `mst_prim` exercise.
pub fn prim_mst(graph: &WeightedGraph, start: i32) -> Vec<(i32, i32, i32)> {
    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::new();
    let mut mst = Vec::new();

    visited.insert(start);
    for &(to, weight) in graph.neighbors(start) {
        heap.push(Reverse((weight, start, to)));
    }

    while let Some(Reverse((weight, from, to))) = heap.pop() {
        if visited.contains(&to) {
            continue;
        }

        visited.insert(to);
        mst.push((from, to, weight));

        for &(next, w) in graph.neighbors(to) {
            if !visited.contains(&next) {
                heap.push(Reverse((w, to, next)));
            }
        }
    }

    mst
}

#[cfg(test)]
include!("../../tests-shared/mst_prim_tests.rs");
