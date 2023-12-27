use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        let mut stack = VecDeque::new();

        if root.is_some() {
            stack.push_back(root.unwrap());
        }

        let mut count = stack.len();
        while let Some(root) = stack.pop_front() {
            count -= 1;
            if let Some(left) = &root.borrow().left {
                stack.push_back(left.clone());
            }
            if let Some(right) = &root.borrow().right {
                stack.push_back(right.clone());
            }
            if count == 0 {
                result.push(root.borrow().val);
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
