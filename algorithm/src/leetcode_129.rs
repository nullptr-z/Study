use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let mut sum = 0;
            Self::has_path_sum_helper(&root, root.borrow().val, &mut sum);

            return sum;
        }

        0
    }

    fn has_path_sum_helper(root: &Rc<RefCell<TreeNode>>, cur_val: i32, sum: &mut i32) {
        let node = root.borrow();

        if node.left.is_none() && node.right.is_none() {
            *sum += cur_val;
            return;
        }

        if let Some(node) = &node.left {
            Self::has_path_sum_helper(&node, cur_val * 10 + node.borrow().val, sum);
        }

        if let Some(node) = &node.right {
            Self::has_path_sum_helper(&node, cur_val * 10 + node.borrow().val, sum);
        }
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
