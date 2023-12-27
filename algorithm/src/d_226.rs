use std::cell::RefCell;
use std::collections::VecDeque;
use std::mem::swap;
use std::rc::Rc;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = vec![root.clone()];

        while let Some(root) = stack.pop() {
            if let Some(node) = root {
                let (left, right) = (node.borrow().left.clone(), node.borrow().right.clone());
                stack.push(right.clone());
                stack.push(left.clone());
                node.borrow_mut().left = right;
                node.borrow_mut().right = left;
            }
        }

        root
    }

    pub fn invert_tree_recursion(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.clone() {
            let (left, right) = (node.borrow().left.clone(), node.borrow().right.clone());
            node.borrow_mut().left = Solution::invert_tree_recursion(right);
            node.borrow_mut().right = Solution::invert_tree_recursion(left);
        }

        root
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::tree_utils::arrayToTree;

    use super::Solution;

    #[test]
    fn should_work() {
        Solution::invert_tree(arrayToTree(vec![4, 2, 7, 1, 3, 6, 9]));
    }
}

pub struct Solution;

use crate::tree_utils::TreeNode;
