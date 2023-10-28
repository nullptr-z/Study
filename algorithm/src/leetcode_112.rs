use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(root) = root {
            return Self::has_path_sum_helper(&root, target_sum - root.borrow().val);
        }

        false
    }

    fn has_path_sum_helper(root: &Rc<RefCell<TreeNode>>, target_sum: i32) -> bool {
        let node = root.borrow();

        if node.left.is_none() && node.right.is_none() {
            return target_sum == 0;
        }

        let left = if let Some(node) = &node.left {
            Self::has_path_sum_helper(&node, target_sum - node.borrow().val)
        } else {
            false
        };

        let right = if let Some(node) = &node.right {
            Self::has_path_sum_helper(&node, target_sum - node.borrow().val)
        } else {
            false
        };

        left || right
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::tree_utils::arrayToTree;

    use super::Solution;

    #[test]
    fn should_work() {}
}

pub struct Solution;

use crate::tree_utils::TreeNode;
