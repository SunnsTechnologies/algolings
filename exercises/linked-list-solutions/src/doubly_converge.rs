use crate::doubly_list::DoublyLinkedList;
use algolings_trace::{found, mark_converging};

/// Reference solution for the `doubly_converge` exercise.
impl DoublyLinkedList {
    pub fn contains_converging(&self, target: i32) -> bool {
        if self.is_empty() {
            return false;
        }

        let mut left = self.head.clone();
        let mut right = self.tail.clone();
        let mut li = 0;
        let mut ri = self.len() - 1;

        while li <= ri {
            let left_node = left.clone().expect("li stays within bounds");
            let right_node = right.clone().expect("ri stays within bounds");

            mark_converging(li, ri);

            if left_node.borrow().value == target {
                found(li);
                return true;
            }
            if li != ri && right_node.borrow().value == target {
                found(ri);
                return true;
            }

            if li == ri {
                break;
            }

            left = left_node.borrow().next.clone();
            right = right_node.borrow().prev.as_ref().and_then(|w| w.upgrade());
            li += 1;
            ri -= 1;
        }

        false
    }
}

#[cfg(test)]
include!("../../tests-shared/doubly_converge_tests.rs");
