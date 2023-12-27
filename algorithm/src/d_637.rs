use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut result: Vec<f64> = vec![];
        let mut level_sum: f64 = 0.0;
        let mut level_count: usize = 0;
        let mut stack = VecDeque::new();

        if root.is_some() {
            stack.push_back(root.unwrap());
        }

        let mut count = stack.len();
        while let Some(root) = stack.pop_front() {
            count -= 1;
            level_sum += root.borrow().val as f64;
            level_count += 1;
            if let Some(left) = &root.borrow().left {
                stack.push_back(left.clone());
            }
            if let Some(right) = &root.borrow().right {
                stack.push_back(right.clone());
            }
            if count == 0 {
                result.push(level_sum / level_count as f64);
                level_count = 0;
                level_sum = 0.0;
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
