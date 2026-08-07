use crate::doubly_list::DoublyLinkedList;
use algolings_trace::{disable, enable, mark_set, take_events, Event};
use std::rc::Rc;

// Floyd's cycle detection (tortoise and hare): does this list loop back
// on itself instead of ending in None? Two pointers walk the SAME list
// at different speeds — slow moves one node at a time, fast moves two.
// If there's no cycle, fast reaches the end first. If there IS a cycle,
// fast eventually laps slow and they land on the exact same node.
//
// "Same node" means POINTER identity (Rc::ptr_eq), not equal values —
// two different nodes can hold the same i32 by coincidence, and that
// must never look like a cycle.
//
// Trace with two STABLE display positions: position 0 always shows
// slow's current value, position 1 always shows fast's current value —
// call mark_set(0, ...) / mark_set(1, ...) every time each pointer
// actually advances to a real node (skip it when a pointer becomes
// None, since there's nothing to display). Watch both slots end up
// showing the SAME value the instant a cycle is found.
impl DoublyLinkedList {
    pub fn has_cycle(&self) -> bool {
        todo!("slow moves one node via next, fast moves two — mark_set each pointer's new value on every real advance, and check Rc::ptr_eq(slow, fast) after each step")
    }
}

#[cfg(test)]
include!("../../tests-shared/floyds_cycle_detection_tests.rs");
