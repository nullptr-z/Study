use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::invert_tree_recursive(&root);
        root
    }

    fn invert_tree_recursive(node: &Option<Rc<RefCell<TreeNode>>>) {
        if let Some(n) = node {
            let mut n = n.borrow_mut();
            Self::invert_tree_recursive(&n.left);
            Self::invert_tree_recursive(&n.right);
            let temp = n.left.clone();
            n.left = n.right.clone();
            n.right = temp;
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn should_work() {}
}

pub struct Solution;

use crate::tree_utils::TreeNode;
