use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut depth = 0;
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
                depth += 1;
                count = stack.len();
            }
        }

        depth
    }

    pub fn max_depth_recursion(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recursion(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(node) = root {
                let l = recursion(node.borrow().left.clone());
                let r = recursion(node.borrow().right.clone());
                return l.max(r) + 1;
            }

            0
        }

        recursion(root)
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
