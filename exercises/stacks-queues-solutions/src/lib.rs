mod vec_stack;
mod stack_vec;
mod linked_stack;
mod stack_linked_list;
mod vec_queue;
mod queue_vecdeque;
mod linked_queue;
mod queue_linked_list;

pub use vec_stack::VecStack;
pub use linked_stack::{LinkedStack, Node as LinkedStackNode};
pub use vec_queue::VecQueue;
pub use linked_queue::{LinkedQueue, Node as LinkedQueueNode};

#[cfg(test)]
mod sync_tests {
    /// Regression guard: scaffolding files are hand-duplicated between this
    /// crate and exercises/stacks-queues (there's no shared dependency
    /// between skeleton and solution crates), with nothing else enforcing
    /// they stay in sync. Each is pure scaffolding — no todo!()s — so
    /// byte-identical is exactly what "in sync" means. Same convention as
    /// linked-list-solutions' list.rs/doubly_list.rs sync tests.
    #[test]
    fn vec_stack_rs_matches_the_skeleton_crate() {
        let solutions = include_str!("vec_stack.rs");
        let skeleton = include_str!("../../stacks-queues/src/vec_stack.rs");
        assert_eq!(
            solutions, skeleton,
            "vec_stack.rs has diverged between stacks-queues-solutions and stacks-queues"
        );
    }

    #[test]
    fn linked_stack_rs_matches_the_skeleton_crate() {
        let solutions = include_str!("linked_stack.rs");
        let skeleton = include_str!("../../stacks-queues/src/linked_stack.rs");
        assert_eq!(
            solutions, skeleton,
            "linked_stack.rs has diverged between stacks-queues-solutions and stacks-queues"
        );
    }

    #[test]
    fn vec_queue_rs_matches_the_skeleton_crate() {
        let solutions = include_str!("vec_queue.rs");
        let skeleton = include_str!("../../stacks-queues/src/vec_queue.rs");
        assert_eq!(
            solutions, skeleton,
            "vec_queue.rs has diverged between stacks-queues-solutions and stacks-queues"
        );
    }

    #[test]
    fn linked_queue_rs_matches_the_skeleton_crate() {
        let solutions = include_str!("linked_queue.rs");
        let skeleton = include_str!("../../stacks-queues/src/linked_queue.rs");
        assert_eq!(
            solutions, skeleton,
            "linked_queue.rs has diverged between stacks-queues-solutions and stacks-queues"
        );
    }
}
