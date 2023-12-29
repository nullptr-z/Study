use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();

            let l = Solution::sum_of_left_leaves(node.left.clone());
            let r = Solution::sum_of_left_leaves(node.right.clone());

            if let Some(left) = node.left.clone() {
                let node = left.borrow();
                if node.left.is_none() && node.right.is_none() {
                    return node.val + l + r;
                }
            }

            return l + r;
        }
        0
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
