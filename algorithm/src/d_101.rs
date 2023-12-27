use std::cell::RefCell;
use std::collections::VecDeque;
use std::mem::swap;
use std::rc::Rc;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn recursion(
            left: &Option<Rc<RefCell<TreeNode>>>,
            right: &Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            if left.is_some() && right.is_some() {
                let l = left.clone().unwrap();
                let r = right.clone().unwrap();
                let cmp = r.borrow().val == l.borrow().val;
                let cmp1 = recursion(&l.borrow().left, &r.borrow().right);
                let cmp2 = recursion(&l.borrow().right, &r.borrow().left);
                return cmp & cmp1 & cmp2;
            }

            if left.is_none() && right.is_none() {
                return true;
            }

            false
        }

        if let Some(node) = root {
            let node = node.borrow();
            return recursion(&node.left, &node.right);
        }

        false
    }

    pub fn is_symmetric_iter(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut node_vals = VecDeque::new();
        let mut queue = VecDeque::new();
        let mut queue1 = VecDeque::new();
        queue.push_back(root);

        while let Some(root) = queue.pop_front() {
            if let Some(node) = root {
                node_vals.push_front(node.borrow().val);
                queue1.push_back(node.borrow().left.clone());
                queue1.push_back(node.borrow().right.clone());
            } else {
                node_vals.push_front(i32::MIN);
            }
            if queue.is_empty() {
                while !node_vals.is_empty() && node_vals.len() > 1 {
                    let l = node_vals.pop_back().unwrap();
                    let r = node_vals.pop_front().unwrap();
                    if l != r {
                        return false;
                    }
                }
                node_vals = VecDeque::new();
                swap(&mut queue, &mut queue1);
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::tree_utils::arrayToTree;

    use super::Solution;

    #[test]
    fn should_work() {
        Solution::is_symmetric(arrayToTree(vec![1, 2, 2, 3, 4, 4, 3]));
    }
}

pub struct Solution;

use crate::tree_utils::TreeNode;
