use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let ret = Self::iter(&nums);

        ret
    }

    fn iter(nums: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        let root_idx = nums.len() / 2;
        let root_val = nums[root_idx];
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));

        root.borrow_mut().left = Self::iter(&nums[0..root_idx].to_vec());
        root.borrow_mut().right = Self::iter(&nums[(root_idx + 1)..].to_vec());

        Some(root)
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::tree_utils::arrayToTree;

    use super::Solution;

    #[test]
    fn should_work() {
        Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]);
    }
}

pub struct Solution;

use crate::tree_utils::TreeNode;
