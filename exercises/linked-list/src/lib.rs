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

// Both modules define their own `Node` — re-exporting both under the bare
// name would collide, so each gets a qualified alias.
pub use list::{Node as SinglyLinkedNode, SinglyLinkedList};
pub use doubly_list::{DoublyLinkedList, Node as DoublyLinkedNode};
