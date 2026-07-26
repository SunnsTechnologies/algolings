//! Shared "what does this event do" logic used by both the `--plain`
//! renderer and the interactive `ratatui` player, so the two presentations
//! can never silently diverge on what a step actually means.

use algolings_trace::Event;

/// Applies `event` to `arr` in place, returning the positions to highlight
/// and a short human-readable description of what happened.
pub fn apply_event(arr: &mut [i32], event: &Event) -> (Vec<usize>, String) {
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
}
