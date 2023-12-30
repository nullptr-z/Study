use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(node) = root {
            let refs = node.borrow();
            let sum = target_sum - refs.val;

            if refs.left.is_none() && refs.right.is_none() {
                return sum == 0;
            }

            let l = Solution::has_path_sum(refs.left.clone(), sum);
            let r = Solution::has_path_sum(refs.right.clone(), sum);

            return l || r;
        }

        false
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
