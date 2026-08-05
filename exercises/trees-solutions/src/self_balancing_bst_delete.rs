use crate::avl::{is_balanced, is_bst, tree_contains, AvlNode, AvlTree};

/// Reference solution for the `self_balancing_bst_delete` exercise.
impl AvlTree {
    pub fn delete(&mut self, value: i32) {
        self.root = self.root.take().and_then(|node| delete_node(node, value));
    }
}

fn delete_node(mut node: Box<AvlNode>, value: i32) -> Option<Box<AvlNode>> {
    if value < node.value {
        node.left = node.left.take().and_then(|n| delete_node(n, value));
    } else if value > node.value {
        node.right = node.right.take().and_then(|n| delete_node(n, value));
    } else {
        match (node.left.take(), node.right.take()) {
            (None, None) => return None,
            (Some(child), None) | (None, Some(child)) => return Some(child),
            (Some(left), Some(right)) => {
                let successor_value = min_value(&right);
                node.value = successor_value;
                node.right = delete_node(right, successor_value);
                node.left = Some(left);
            }
        }
    }

    node.update_height();
    let balance = node.balance_factor();

    if balance > 1 && node.left.as_ref().unwrap().balance_factor() >= 0 {
        return Some(AvlNode::rotate_right(node));
    }
    if balance > 1 && node.left.as_ref().unwrap().balance_factor() < 0 {
        node.left = Some(AvlNode::rotate_left(node.left.take().unwrap()));
        return Some(AvlNode::rotate_right(node));
    }
    if balance < -1 && node.right.as_ref().unwrap().balance_factor() <= 0 {
        return Some(AvlNode::rotate_left(node));
    }
    if balance < -1 && node.right.as_ref().unwrap().balance_factor() > 0 {
        node.right = Some(AvlNode::rotate_right(node.right.take().unwrap()));
        return Some(AvlNode::rotate_left(node));
    }

    Some(node)
}

fn min_value(node: &AvlNode) -> i32 {
    let mut current = node;
    while let Some(ref left) = current.left {
        current = left;
    }
    current.value
}

#[cfg(test)]
include!("../../tests-shared/self_balancing_bst_delete_tests.rs");
