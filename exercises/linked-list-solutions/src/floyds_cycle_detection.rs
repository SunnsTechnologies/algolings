use crate::doubly_list::DoublyLinkedList;
use algolings_trace::{disable, enable, mark_set, take_events, Event};
use std::rc::Rc;

/// Reference solution for the `floyds_cycle_detection` exercise.
impl DoublyLinkedList {
    pub fn has_cycle(&self) -> bool {
        let mut slow = self.head.clone();
        let mut fast = self.head.clone();

        while let (Some(s), Some(f)) = (slow.clone(), fast.clone()) {
            let next_slow = s.borrow().next.clone();
            slow = next_slow;
            if let Some(ref sv) = slow {
                mark_set(0, sv.borrow().value);
            }

            let next_fast = match f.borrow().next.clone() {
                Some(one_step) => one_step.borrow().next.clone(),
                None => return false,
            };
            fast = next_fast;
            if let Some(ref fv) = fast {
                mark_set(1, fv.borrow().value);
            }

            if let (Some(s2), Some(f2)) = (&slow, &fast) {
                if Rc::ptr_eq(s2, f2) {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
include!("../../tests-shared/floyds_cycle_detection_tests.rs");
