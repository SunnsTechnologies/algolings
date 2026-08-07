mod list;
mod doubly_list;
mod insert;
mod remove;
mod reverse;
mod traverse;
mod doubly_push;
mod doubly_pop;
mod doubly_contains;
mod doubly_converge;
mod floyds_cycle_detection;

// Both modules define their own `Node` — re-exporting both under the bare
// name would collide, so each gets a qualified alias.
pub use list::{Node as SinglyLinkedNode, SinglyLinkedList};
pub use doubly_list::{DoublyLinkedList, Node as DoublyLinkedNode};

#[cfg(test)]
mod sync_tests {
    /// Regression guard: list.rs is hand-duplicated between this crate and
    /// exercises/linked-list (there's no shared dependency between
    /// skeleton and solution crates), with nothing else enforcing they
    /// stay in sync. Same for doubly_list.rs. Both are pure scaffolding —
    /// no todo!()s — so byte-identical is exactly what "in sync" means.
    #[test]
    fn list_rs_matches_the_skeleton_crate() {
        let solutions = include_str!("list.rs");
        let skeleton = include_str!("../../linked-list/src/list.rs");
        assert_eq!(
            solutions, skeleton,
            "list.rs has diverged between linked-list-solutions and linked-list"
        );
    }

    #[test]
    fn doubly_list_rs_matches_the_skeleton_crate() {
        let solutions = include_str!("doubly_list.rs");
        let skeleton = include_str!("../../linked-list/src/doubly_list.rs");
        assert_eq!(
            solutions, skeleton,
            "doubly_list.rs has diverged between linked-list-solutions and linked-list"
        );
    }
}
