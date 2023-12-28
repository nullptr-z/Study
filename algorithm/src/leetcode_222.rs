use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    // 求是否满二叉，时间复杂度 < O(n)
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let binding = root.unwrap();
        let ref_root = binding.borrow();

        let (mut left, mut right) = (ref_root.left.clone(), ref_root.right.clone());
        let (mut left_depth, mut right_depth) = (0, 0);
        while let Some(node) = left {
            left = node.borrow().left.clone();
            left_depth += 1;
        }
        while let Some(node) = right {
            right = node.borrow().right.clone();
            right_depth += 1;
        }
        if left_depth == right_depth {
            return (2 << left_depth) - 1;
        }

        1 + Solution::count_nodes(ref_root.left.clone())
            + Solution::count_nodes(ref_root.right.clone())
    }

    // 迭代计数
    pub fn count_nodes_iter(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut stack: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        stack.push_back(root.unwrap());
        let mut i = 1;
        while let Some(node) = stack.pop_front() {
            let node = node.borrow();
            if let Some(l) = &node.left {
                stack.push_back(l.clone());
                i += 1;
            } else {
                break;
            }
            if let Some(l) = &node.right {
                stack.push_back(l.clone());
                i += 1;
            } else {
                break;
            }
        }

        i
    }

    // 递归计数
    pub fn count_node_s(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let node = node.borrow();
                let left = Solution::count_nodes(node.left.clone());
                let right = Solution::count_nodes(node.right.clone());
                left + right + 1
            }
            None => 0,
        }
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
