//! Shared "what does this event do" logic used by both the `--plain`
//! renderer and the interactive `ratatui` player, so the two presentations
//! can never silently diverge on what a step actually means.

use algolings_trace::Event;

/// Applies `event` to `arr` in place, returning the positions to highlight
/// and a short human-readable description of what happened. Takes a `Vec`
/// rather than a slice: sort/search never change the array's length, but
/// linked-list exercises' Insert/Remove events do.
pub fn apply_event(arr: &mut Vec<i32>, event: &Event) -> (Vec<usize>, String) {
    match *event {
        Event::Compare { i, j } => (vec![i, j], format!("compare [{i}] and [{j}]")),
        Event::Swap { i, j } => {
            arr.swap(i, j);
            (vec![i, j], format!("swap [{i}] and [{j}]"))
        }
        Event::Set { i, value } => {
            arr[i] = value as i32;
            (vec![i], format!("set [{i}] = {value}"))
        }
        Event::MarkSorted { i } => (vec![], format!("[{i}] is now sorted")),
        Event::Probe { i } => (vec![i], format!("check [{i}]")),
        Event::Found { i } => (vec![i], format!("found [{i}]!")),
        Event::NarrowRange { left, right } => {
            (vec![], format!("search range is now [{left}, {right})"))
        }
        Event::Insert { i, value } => {
            arr.insert(i, value as i32);
            (vec![i], format!("insert {value} at [{i}]"))
        }
        Event::Remove { i } => {
            let highlighted = vec![i];
            let description = format!("remove [{i}]");
            arr.remove(i);
            (highlighted, description)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_highlights_both_without_changing_values() {
        let mut arr = vec![5, 1];
        let (highlighted, desc) = apply_event(&mut arr, &Event::Compare { i: 0, j: 1 });
        assert_eq!(arr, vec![5, 1]);
        assert_eq!(highlighted, vec![0, 1]);
        assert!(desc.contains("compare"));
    }

    #[test]
    fn swap_actually_swaps_the_values() {
        let mut arr = vec![5, 1];
        apply_event(&mut arr, &Event::Swap { i: 0, j: 1 });
        assert_eq!(arr, vec![1, 5]);
    }

    #[test]
    fn set_writes_the_given_value() {
        let mut arr = vec![5, 1];
        apply_event(&mut arr, &Event::Set { i: 0, value: 9 });
        assert_eq!(arr, vec![9, 1]);
    }

    #[test]
    fn mark_sorted_highlights_nothing_and_changes_nothing() {
        let mut arr = vec![5, 1];
        let (highlighted, _) = apply_event(&mut arr, &Event::MarkSorted { i: 1 });
        assert_eq!(arr, vec![5, 1]);
        assert!(highlighted.is_empty());
    }

    #[test]
    fn probe_highlights_the_checked_index_without_changing_values() {
        let mut arr = vec![3, 7, 2, 9, 5];
        let (highlighted, desc) = apply_event(&mut arr, &Event::Probe { i: 3 });
        assert_eq!(arr, vec![3, 7, 2, 9, 5]);
        assert_eq!(highlighted, vec![3]);
        assert!(desc.contains("[3]"));
    }

    #[test]
    fn found_highlights_the_matching_index_without_changing_values() {
        let mut arr = vec![3, 7, 2, 9, 5];
        let (highlighted, desc) = apply_event(&mut arr, &Event::Found { i: 3 });
        assert_eq!(arr, vec![3, 7, 2, 9, 5]);
        assert_eq!(highlighted, vec![3]);
        assert!(desc.to_lowercase().contains("found"));
    }

    #[test]
    fn narrow_range_highlights_nothing_and_changes_nothing() {
        let mut arr = vec![3, 7, 2, 9, 5];
        let (highlighted, desc) = apply_event(&mut arr, &Event::NarrowRange { left: 2, right: 5 });
        assert_eq!(arr, vec![3, 7, 2, 9, 5]);
        assert!(highlighted.is_empty());
        assert!(desc.contains('2') && desc.contains('5'));
    }

    #[test]
    fn insert_grows_the_vec_at_the_given_index() {
        let mut arr = vec![1, 3];
        let (highlighted, desc) = apply_event(&mut arr, &Event::Insert { i: 1, value: 2 });
        assert_eq!(arr, vec![1, 2, 3]);
        assert_eq!(highlighted, vec![1]);
        assert!(desc.contains('2'));
    }

    #[test]
    fn insert_at_the_end_appends() {
        let mut arr = vec![1, 2];
        apply_event(&mut arr, &Event::Insert { i: 2, value: 3 });
        assert_eq!(arr, vec![1, 2, 3]);
    }

    #[test]
    fn insert_into_an_empty_vec_works() {
        let mut arr: Vec<i32> = vec![];
        apply_event(&mut arr, &Event::Insert { i: 0, value: 42 });
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn remove_shrinks_the_vec_at_the_given_index() {
        let mut arr = vec![1, 2, 3];
        let (highlighted, desc) = apply_event(&mut arr, &Event::Remove { i: 1 });
        assert_eq!(arr, vec![1, 3]);
        assert_eq!(highlighted, vec![1]);
        assert!(desc.contains('1'));
    }
}
