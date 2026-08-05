use crate::avl::{is_balanced, is_bst, tree_contains, AvlNode, AvlTree};

/// Reference solution for the `self_balancing_bst` exercise.
impl AvlTree {
    pub fn insert(&mut self, value: i32) {
        self.root = Some(match self.root.take() {
            Some(node) => insert_node(node, value),
            None => Box::new(AvlNode::new(value)),
        });
    }
}

fn insert_node(mut node: Box<AvlNode>, value: i32) -> Box<AvlNode> {
    if value < node.value {
        node.left = Some(match node.left.take() {
            Some(left) => insert_node(left, value),
            None => Box::new(AvlNode::new(value)),
        });
    } else if value > node.value {
        node.right = Some(match node.right.take() {
            Some(right) => insert_node(right, value),
            None => Box::new(AvlNode::new(value)),
        });
    } else {
        return node;
    }

    node.update_height();
    let balance = node.balance_factor();

    if balance > 1 && value < node.left.as_ref().unwrap().value {
        return AvlNode::rotate_right(node);
    }
    if balance < -1 && value > node.right.as_ref().unwrap().value {
        return AvlNode::rotate_left(node);
    }
    if balance > 1 && value > node.left.as_ref().unwrap().value {
        node.left = Some(AvlNode::rotate_left(node.left.take().unwrap()));
        return AvlNode::rotate_right(node);
    }
    if balance < -1 && value < node.right.as_ref().unwrap().value {
        node.right = Some(AvlNode::rotate_right(node.right.take().unwrap()));
        return AvlNode::rotate_left(node);
    }

    node
}

#[cfg(test)]
include!("../../tests-shared/self_balancing_bst_tests.rs");
