//! Step-tracing primitives shared between exercise solutions and the
//! algolings CLI's replay/trace renderer.
//!
//! Resolves the "whose code gets traced" question from the design review:
//! exercise solutions call these helpers (`cmp_lt`, `swap`, `set_at`,
//! `mark_sorted`) instead of raw `<` / `.swap()` / index assignment. The
//! public function signature a learner writes stays fully idiomatic
//! (`fn bubble_sort(arr: &mut [i32])`) — only the operations *inside* the
//! body go through these helpers, so tracing a learner's own solution and
//! tracing the reference solution both fall out of the same mechanism, with
//! no special-casing of which one "really" gets traced.

use serde::{Deserialize, Serialize};
use std::cell::{Cell, RefCell};

/// Serializable so the trace binary (a subprocess, run fresh against the
/// current on-disk exercise code) can print the recorded trace as JSON for
/// the algolings CLI's parent process to read back.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Event {
    Compare { i: usize, j: usize },
    Swap { i: usize, j: usize },
    Set { i: usize, value: i64 },
    MarkSorted { i: usize },
    /// Search algorithms checking index `i` against a target value. The
    /// target itself isn't carried here — it's constant for the whole
    /// trace, shown once by the CLI instead of repeated on every event.
    Probe { i: usize },
    /// The target was found at index `i`.
    Found { i: usize },
    /// Binary search narrowed its remaining search space to `[left, right)`.
    NarrowRange { left: usize, right: usize },
    /// A value was inserted at index `i` (a linked list's value sequence
    /// growing by one).
    Insert { i: usize, value: i64 },
    /// The value at index `i` was removed (the value sequence shrinking by
    /// one).
    Remove { i: usize },
    /// Two pointers, walking toward each other from opposite ends, are
    /// simultaneously checking `left` and `right`. Unlike `Probe` (one
    /// index at a time), this reports both positions in a single event so
    /// the renderer can highlight them together.
    Converge { left: usize, right: usize },
}

thread_local! {
    static ENABLED: Cell<bool> = const { Cell::new(false) };
    static EVENTS: RefCell<Vec<Event>> = const { RefCell::new(Vec::new()) };
}

/// Enables tracing on the current thread and clears any prior events.
pub fn enable() {
    ENABLED.with(|e| e.set(true));
    EVENTS.with(|events| events.borrow_mut().clear());
}

/// Disables tracing on the current thread. Normal `cargo test` runs never
/// call this because they never call `enable()` in the first place —
/// tracing defaults to off.
pub fn disable() {
    ENABLED.with(|e| e.set(false));
}

pub fn is_enabled() -> bool {
    ENABLED.with(|e| e.get())
}

/// Drains and returns all events recorded since the last `enable()` or
/// `take_events()` call.
pub fn take_events() -> Vec<Event> {
    EVENTS.with(|events| std::mem::take(&mut *events.borrow_mut()))
}

fn record(event: Event) {
    if is_enabled() {
        EVENTS.with(|events| events.borrow_mut().push(event));
    }
}

/// Compares `arr[i] < arr[j]`, recording a `Compare` event when tracing is
/// enabled. A no-op (beyond the enabled check) when tracing is disabled.
///
/// Use this for algorithms that compare two *live* positions in the array
/// being sorted (bubble/selection/insertion sort). For algorithms that
/// compare values held in a temporary buffer while still wanting to report
/// the values' *original* global positions (merge sort's snapshot-based
/// merge step), use [`cmp_lt_values`] instead — it decouples "what data gets
/// compared" from "what index gets reported".
pub fn cmp_lt<T: PartialOrd>(arr: &[T], i: usize, j: usize) -> bool {
    record(Event::Compare { i, j });
    arr[i] < arr[j]
}

/// Compares two arbitrary values directly, recording a `Compare` event with
/// caller-supplied `report_i`/`report_j` indices rather than indexing into a
/// slice. Needed whenever the values being compared don't live at those
/// indices in any single slice right now (e.g. a merge-sort snapshot buffer)
/// but the trace should still report the algorithm's true global positions.
pub fn cmp_lt_values<T: PartialOrd>(a: &T, b: &T, report_i: usize, report_j: usize) -> bool {
    record(Event::Compare { i: report_i, j: report_j });
    a < b
}

/// Swaps `arr[i]` and `arr[j]`, recording a `Swap` event when tracing is
/// enabled.
pub fn swap<T>(arr: &mut [T], i: usize, j: usize) {
    record(Event::Swap { i, j });
    arr.swap(i, j);
}

/// Writes `value` into `out[i]`, recording a `Set` event. Used by algorithms
/// (e.g. counting sort, merge sort's write-back) that write into a buffer
/// rather than swapping two live positions.
pub fn set_at<T>(out: &mut [T], i: usize, value: T)
where
    T: Copy + Into<i64>,
{
    record(Event::Set { i, value: value.into() });
    out[i] = value;
}

/// Records that position `i` is now in its final sorted place.
pub fn mark_sorted(i: usize) {
    record(Event::MarkSorted { i });
}

/// Checks `arr[i]` against `target`, recording a `Probe` event when tracing
/// is enabled. Returns whether it matches.
pub fn probe<T: PartialEq>(arr: &[T], i: usize, target: &T) -> bool {
    record(Event::Probe { i });
    &arr[i] == target
}

/// Records that the target was found at index `i`.
pub fn found(i: usize) {
    record(Event::Found { i });
}

/// Records that the remaining search space narrowed to `[left, right)`.
pub fn narrow_range(left: usize, right: usize) {
    record(Event::NarrowRange { left, right });
}

/// Records that `value` was inserted at index `i`. Record-only, unlike
/// `swap`/`set_at`: a linked list has no array to operate through, so the
/// caller's own `Box`/`Option` mutation does the real work — this just logs
/// it happened, the same way `mark_sorted` already does.
pub fn mark_inserted<T: Copy + Into<i64>>(i: usize, value: T) {
    record(Event::Insert { i, value: value.into() });
}

/// Records that the value at index `i` was removed. Record-only, same
/// reasoning as [`mark_inserted`].
pub fn mark_removed(i: usize) {
    record(Event::Remove { i });
}

/// Records that index `i` was visited while walking a structure with no
/// indexable array to check against directly (e.g. a linked list traversal)
/// — the caller does its own comparison and calls this purely to log the
/// step, unlike [`probe`] which both checks and records.
pub fn mark_visited(i: usize) {
    record(Event::Probe { i });
}

/// Records that `left` and `right` are being checked simultaneously by two
/// pointers converging from opposite ends. Record-only, same reasoning as
/// [`mark_visited`] — the caller does its own comparisons.
pub fn mark_converging(left: usize, right: usize) {
    record(Event::Converge { left, right });
}

/// Records that position `i` was overwritten with `value`. Record-only,
/// same reasoning as [`mark_inserted`]/[`mark_removed`] — for callers
/// whose storage isn't a literal `&mut [T]` (so [`set_at`] doesn't apply),
/// e.g. a hash table writing into one of its buckets rather than a slice
/// index.
pub fn mark_set<T: Copy + Into<i64>>(i: usize, value: T) {
    record(Event::Set {
        i,
        value: value.into(),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_by_default_and_records_nothing() {
        disable();
        let arr = [3, 1, 2];
        let _ = cmp_lt(&arr, 0, 1);
        assert!(take_events().is_empty());
    }

    #[test]
    fn enabled_records_compare_and_swap() {
        enable();
        let mut arr = [3, 1];
        assert!(!cmp_lt(&arr, 0, 1));
        swap(&mut arr, 0, 1);
        let events = take_events();
        assert_eq!(
            events,
            vec![
                Event::Compare { i: 0, j: 1 },
                Event::Swap { i: 0, j: 1 },
            ]
        );
        disable();
    }

    #[test]
    fn probe_records_the_index_and_reports_whether_it_matches_the_target() {
        enable();
        let arr = [3, 7, 2, 9, 5];
        assert!(!probe(&arr, 0, &9));
        assert!(probe(&arr, 3, &9));
        assert_eq!(
            take_events(),
            vec![Event::Probe { i: 0 }, Event::Probe { i: 3 }]
        );
        disable();
    }

    #[test]
    fn found_records_the_matching_index() {
        enable();
        found(3);
        assert_eq!(take_events(), vec![Event::Found { i: 3 }]);
        disable();
    }

    #[test]
    fn narrow_range_records_the_new_bounds() {
        enable();
        narrow_range(2, 5);
        assert_eq!(take_events(), vec![Event::NarrowRange { left: 2, right: 5 }]);
        disable();
    }

    #[test]
    fn disabled_probe_found_and_narrow_range_record_nothing() {
        disable();
        let arr = [3, 7, 2, 9, 5];
        let _ = probe(&arr, 0, &9);
        found(0);
        narrow_range(0, 5);
        assert!(take_events().is_empty());
    }

    #[test]
    fn mark_inserted_records_the_index_and_value() {
        enable();
        mark_inserted(2, 42i32);
        assert_eq!(take_events(), vec![Event::Insert { i: 2, value: 42 }]);
        disable();
    }

    #[test]
    fn mark_removed_records_the_index() {
        enable();
        mark_removed(1);
        assert_eq!(take_events(), vec![Event::Remove { i: 1 }]);
        disable();
    }

    #[test]
    fn mark_visited_records_a_probe_event() {
        enable();
        mark_visited(4);
        assert_eq!(take_events(), vec![Event::Probe { i: 4 }]);
        disable();
    }

    #[test]
    fn disabled_insert_remove_and_visit_record_nothing() {
        disable();
        mark_inserted(0, 1i32);
        mark_removed(0);
        mark_visited(0);
        assert!(take_events().is_empty());
    }

    #[test]
    fn mark_converging_records_both_positions_in_one_event() {
        enable();
        mark_converging(1, 3);
        assert_eq!(take_events(), vec![Event::Converge { left: 1, right: 3 }]);
        disable();
    }

    #[test]
    fn disabled_mark_converging_records_nothing() {
        disable();
        mark_converging(1, 3);
        assert!(take_events().is_empty());
    }

    #[test]
    fn mark_set_records_a_set_event() {
        enable();
        mark_set(2, 42i32);
        assert_eq!(take_events(), vec![Event::Set { i: 2, value: 42 }]);
        disable();
    }

    #[test]
    fn disabled_mark_set_records_nothing() {
        disable();
        mark_set(2, 42i32);
        assert!(take_events().is_empty());
    }
}
