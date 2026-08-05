use crate::rb::{is_black_balanced, is_bst, no_red_red, tree_contains, Color, RbNode, RbTree};

/// Reference solution for the `red_black_tree_delete` exercise.
impl RbTree {
    pub fn delete(&mut self, value: i32) {
        if !tree_contains(&self.root, value) {
            return;
        }

        if let Some(ref mut root) = self.root {
            if !RbNode::is_red(&root.left) && !RbNode::is_red(&root.right) {
                root.color = Color::Red;
            }
        }

        self.root = self.root.take().and_then(|root| delete_node(root, value));
        if let Some(ref mut root) = self.root {
            root.color = Color::Black;
        }
    }
}

fn delete_node(mut h: Box<RbNode>, value: i32) -> Option<Box<RbNode>> {
    if value < h.value {
        if !RbNode::is_red(&h.left) && !RbNode::is_red(&h.left.as_ref().unwrap().left) {
            h = RbNode::move_red_left(h);
        }
        h.left = h.left.take().and_then(|left| delete_node(left, value));
    } else {
        if RbNode::is_red(&h.left) {
            h = RbNode::rotate_right(h);
        }
        if value == h.value && h.right.is_none() {
            return None;
        }
        if !RbNode::is_red(&h.right) && !RbNode::is_red(&h.right.as_ref().unwrap().left) {
            h = RbNode::move_red_right(h);
        }
        if value == h.value {
            let min = RbNode::min_value(h.right.as_ref().unwrap());
            h.value = min;
            h.right = h.right.take().and_then(delete_min);
        } else {
            h.right = h.right.take().and_then(|right| delete_node(right, value));
        }
    }
    Some(RbNode::fix_up(h))
}

fn delete_min(mut h: Box<RbNode>) -> Option<Box<RbNode>> {
    if h.left.is_none() {
        return None;
    }
    if !RbNode::is_red(&h.left) && !RbNode::is_red(&h.left.as_ref().unwrap().left) {
        h = RbNode::move_red_left(h);
    }
    h.left = h.left.take().and_then(delete_min);
    Some(RbNode::fix_up(h))
}

#[cfg(test)]
include!("../../tests-shared/red_black_tree_delete_tests.rs");
