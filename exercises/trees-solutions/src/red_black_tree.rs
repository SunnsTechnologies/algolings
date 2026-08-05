use crate::rb::{is_black_balanced, is_bst, no_red_red, tree_contains, Color, RbNode, RbTree};

/// Reference solution for the `red_black_tree` exercise.
impl RbTree {
    pub fn insert(&mut self, value: i32) {
        self.root = Some(insert_at(self.root.take(), value));
        if let Some(ref mut root) = self.root {
            root.color = Color::Black;
        }
    }
}

fn insert_at(node: Option<Box<RbNode>>, value: i32) -> Box<RbNode> {
    let mut node = match node {
        None => return Box::new(RbNode::new(value)),
        Some(node) => node,
    };

    if value < node.value {
        node.left = Some(insert_at(node.left.take(), value));
    } else if value > node.value {
        node.right = Some(insert_at(node.right.take(), value));
    }

    if RbNode::is_red(&node.right) && !RbNode::is_red(&node.left) {
        node = RbNode::rotate_left(node);
    }
    if RbNode::is_red(&node.left) && RbNode::is_red(&node.left.as_ref().unwrap().left) {
        node = RbNode::rotate_right(node);
    }
    if RbNode::is_red(&node.left) && RbNode::is_red(&node.right) {
        RbNode::flip_colors(&mut node);
    }
    node
}

#[cfg(test)]
include!("../../tests-shared/red_black_tree_tests.rs");
