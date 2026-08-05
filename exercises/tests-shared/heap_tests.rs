// Shared between exercises/trees/src/heap.rs (the learner-facing
// skeleton) and exercises/trees-solutions/src/heap.rs (the reference
// solution), via `include!()`.
#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    #[test]
    fn insert_keeps_the_minimum_at_the_root() {
        let mut heap = MinHeap::new();
        for value in [5, 3, 8, 1, 9, 2] {
            heap.insert(value);
        }
        assert_eq!(heap.peek(), Some(1));
    }

    #[test]
    fn peek_does_not_remove_the_value() {
        let heap = MinHeap::from_values(&[3, 5, 8]);
        assert_eq!(heap.peek(), Some(3));
        assert_eq!(heap.len(), 3);
    }

    #[test]
    fn extract_min_returns_values_in_ascending_order() {
        let mut heap = MinHeap::from_values(&[5, 3, 8, 1, 9, 2]);
        let mut extracted = Vec::new();
        while let Some(min) = heap.extract_min() {
            extracted.push(min);
        }
        assert_eq!(extracted, vec![1, 2, 3, 5, 8, 9]);
    }

    #[test]
    fn extract_min_on_an_empty_heap_returns_none() {
        let mut heap = MinHeap::new();
        assert_eq!(heap.extract_min(), None);
    }

    #[test]
    fn extract_min_on_a_single_element_heap_empties_it() {
        let mut heap = MinHeap::from_values(&[42]);
        assert_eq!(heap.extract_min(), Some(42));
        assert!(heap.is_empty());
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let mut a = MinHeap::from_values(&[5, 3, 8]);
        let min_a = a.extract_min();
        enable();
        let mut b = MinHeap::from_values(&[5, 3, 8]);
        let min_b = b.extract_min();
        disable();
        assert_eq!(min_a, min_b);
    }

    #[test]
    fn tracing_enabled_captures_the_exact_sequence_for_insert() {
        let mut heap = MinHeap::from_values(&[5]);

        enable();
        heap.insert(3);
        let events = take_events();
        disable();

        assert_eq!(
            events,
            vec![Event::Compare { i: 1, j: 0 }, Event::Swap { i: 1, j: 0 }]
        );
    }

    #[test]
    fn tracing_enabled_captures_the_exact_sequence_for_extract_min() {
        let mut heap = MinHeap::from_values(&[1, 3, 8, 5]);

        enable();
        let min = heap.extract_min();
        let events = take_events();
        disable();

        assert_eq!(min, Some(1));
        assert_eq!(
            events,
            vec![
                Event::Compare { i: 1, j: 0 },
                Event::Compare { i: 2, j: 1 },
                Event::Swap { i: 0, j: 1 },
            ]
        );
    }
}
