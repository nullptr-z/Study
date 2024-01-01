use std::{cell::RefCell, rc::Rc};

impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        let (mut max_index, mut max_value) = (0, i32::MIN);
        for (index, &value) in nums.iter().enumerate() {
            if value > max_value {
                (max_index, max_value) = (index, value);
            }
        }

        let mut root = TreeNode::new(max_value);
        root.left = Self::construct_maximum_binary_tree(nums[..max_index].to_owned());
        root.right = Self::construct_maximum_binary_tree(nums[(max_index + 1)..].to_owned());

        Some(Rc::new(RefCell::new(root)))
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::construct_maximum_binary_tree(vec![3, 2, 1, 6, 0, 5]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;

use crate::tree_utils::TreeNode;
