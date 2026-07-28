use crate::list::SinglyLinkedList;
use algolings_trace::mark_removed;

/// Reference solution for the `remove` exercise.
impl SinglyLinkedList {
    pub fn remove(&mut self, target: i32) -> bool {
        let mut current = &mut self.head;
        let mut i = 0;

        while let Some(mut boxed_node) = current.take() {
            if boxed_node.value == target {
                mark_removed(i);
                *current = boxed_node.next.take();
                return true;
            } else {
                *current = Some(boxed_node);
                current = &mut current.as_mut().unwrap().next;
                i += 1;
            }
        }

        false
    }
}

#[cfg(test)]
include!("../../tests-shared/remove_tests.rs");
