use crate::doubly_list::DoublyLinkedList;
use algolings_trace::mark_removed;

/// Reference solution for the `doubly_pop` exercise.
impl DoublyLinkedList {
    pub fn pop_front(&mut self) -> Option<i32> {
        self.head.take().map(|old_head| {
            let value = old_head.borrow().value;
            mark_removed(0);
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
                None => {
                    self.tail = None;
                }
            }
            self.len -= 1;
            value
        })
    }

    pub fn pop_back(&mut self) -> Option<i32> {
        self.tail.take().map(|old_tail| {
            let value = old_tail.borrow().value;
            mark_removed(self.len - 1);
            let prev_weak = old_tail.borrow().prev.clone();
            match prev_weak {
                Some(weak) => {
                    let new_tail = weak.upgrade().expect(
                        "prev pointed at a node that's already gone — check that pop_front \
                         (or whatever last removed a node) set the new head's prev to None",
                    );
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head = None;
                }
            }
            self.len -= 1;
            value
        })
    }
}

#[cfg(test)]
include!("../../tests-shared/doubly_pop_tests.rs");
