use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut result = vec![];
        let mut rev = vec![];
        let mut rev_temp = vec![];
        let mut stack = VecDeque::new();
        stack.push_back(root);

        let mut i = stack.len();
        while i > 0 {
            i -= 1;
            if let Some(root) = stack.pop_front() {
                if let Some(node) = root {
                    result.push(node.borrow().val);
                    rev_temp.push(node.borrow().val);
                    stack.push_back(node.borrow().left.clone());
                    stack.push_back(node.borrow().right.clone());
                } else {
                    result.push(i32::MIN);
                    rev_temp.push(i32::MIN);
                }
                if i == 0 {
                    i = stack.len();
                    rev_temp.reverse();
                    rev.append(&mut rev_temp);
                    rev_temp.clear();
                }
            }
        }

        result == rev
    }

    pub fn is_symmetrics(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            return Self::compare(&node.left, &node.right);
        }
        false
    }

    fn compare(
        left: &Option<Rc<RefCell<TreeNode>>>,
        right: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if left.is_none() && right.is_none() {
            return true;
        }
        if let Some(l) = left {
            if let Some(r) = right {
                let mut cmp: bool = l.borrow().val == r.borrow().val;
                if cmp {
                    cmp = cmp && Self::compare(&r.borrow().left, &l.borrow().right);
                    cmp = cmp && Self::compare(&r.borrow().right, &l.borrow().left);
                }
                return cmp;
            }
        }

        false
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
