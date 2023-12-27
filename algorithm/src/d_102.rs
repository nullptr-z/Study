use std::cell::RefCell;
use std::collections::VecDeque;
use std::mem::swap;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut layers = vec![];
        let mut stack = VecDeque::new();
        let mut stack1 = VecDeque::new();

        stack.push_back(root);

        while let Some(mut root) = stack.pop_front() {
            if let Some(node) = root.take() {
                layers.push(node.borrow().val);
                stack1.push_back(node.borrow_mut().left.take());
                stack1.push_back(node.borrow_mut().right.take());
            }
            if stack.is_empty() && !layers.is_empty() {
                result.push(vec![]);
                swap(&mut stack, &mut stack1);
                swap(result.last_mut().unwrap(), &mut layers);
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
