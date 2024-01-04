use std::{cell::RefCell, rc::Rc};

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        let len = nums.len() / 2;
        let mut root = TreeNode::new(nums[len]);
        let left = Self::sorted_array_to_bst(nums[..len].to_vec());
        root.left = left;
        let right = Self::sorted_array_to_bst(nums[len + 1..].to_vec());
        root.right = right;

        Some(Rc::new(RefCell::new(root)))
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
