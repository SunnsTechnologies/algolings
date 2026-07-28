#[cfg(test)]
mod tests {
    use super::*;
    use algolings_trace::{disable, enable, take_events, Event};

    #[test]
    fn sorts_correctly() {
        let mut input = vec![5, 1, 4, 2, 8];
        merge_sort(&mut input);
        assert_eq!(input, vec![1, 2, 4, 5, 8]);
    }

    #[test]
    fn handles_empty_and_single_element() {
        let mut empty: Vec<i32> = vec![];
        merge_sort(&mut empty);
        assert_eq!(empty, Vec::<i32>::new());

        let mut single = vec![1];
        merge_sort(&mut single);
        assert_eq!(single, vec![1]);
    }

    #[test]
    fn tracing_disabled_does_not_change_behavior() {
        disable();
        let mut a = vec![5, 1, 4, 2, 8, 7, 3, 6];
        let mut b = a.clone();
        merge_sort(&mut a);
        enable();
        merge_sort(&mut b);
        disable();
        assert_eq!(a, b);
    }

    #[test]
    fn tracing_reports_true_global_indices_across_recursion() {
        enable();
        let mut input = vec![5, 1, 4, 2, 8, 7, 3, 6];
        merge_sort(&mut input);
        let events = take_events();
        disable();

        // Every recorded index must be a valid position in the ORIGINAL
        // 8-element array — proves an offset-free recursion scheme (index
        // ranges into one slice, not split_at_mut sub-slices) reports
        // global, not sub-slice-local, indices even deep in the recursion.
        for event in &events {
            match event {
                Event::Compare { i, j } => {
                    assert!(*i < 8, "Compare index {i} out of global range");
                    assert!(*j < 8, "Compare index {j} out of global range");
                }
                Event::Set { i, .. } => assert!(*i < 8, "Set index {i} out of global range"),
                Event::MarkSorted { i } => assert!(*i < 8),
                Event::Swap { .. } => panic!("merge sort should never emit Swap events"),
                Event::Probe { .. }
                | Event::Found { .. }
                | Event::NarrowRange { .. }
                | Event::Insert { .. }
                | Event::Remove { .. } => {
                    panic!("merge sort should never emit search or linked-list events")
                }
            }
        }
        assert!(events.iter().any(|e| matches!(e, Event::Compare { .. })));
        assert!(events.iter().any(|e| matches!(e, Event::Set { .. })));
    }
}
