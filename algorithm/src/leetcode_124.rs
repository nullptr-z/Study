use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let mut max = i32::MIN;
            Self::max_path_sum_helper(&root, &mut max);

            return max;
        }

        0
    }

    fn max_path_sum_helper(root: &Rc<RefCell<TreeNode>>, max: &mut i32) -> i32 {
        let node = root.borrow();
        let val = node.val;

        let l_count = if let Some(node) = &node.left {
            Self::max_path_sum_helper(&node, max).max(0)
        } else {
            0
        };

        let r_count = if let Some(node) = &node.right {
            Self::max_path_sum_helper(&node, max).max(0)
        } else {
            0
        };

        *max = *max.max(&mut (val + l_count + r_count));

        let cur_max = val + l_count.max(r_count);

        cur_max
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
