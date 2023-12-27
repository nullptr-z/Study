use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        let mut level_max = i32::MIN;
        let mut stack = VecDeque::new();

        if root.is_some() {
            stack.push_back(root.unwrap());
        }

        let mut count = stack.len();
        while let Some(root) = stack.pop_front() {
            count -= 1;
            level_max = level_max.max(root.borrow().val);
            if let Some(left) = &root.borrow().left {
                stack.push_back(left.clone());
            }
            if let Some(right) = &root.borrow().right {
                stack.push_back(right.clone());
            }
            if count == 0 {
                result.push(level_max);
                level_max = i32::MIN;
                count = stack.len();
            }
        }

        result
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
