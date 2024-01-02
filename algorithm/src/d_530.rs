use std::{cell::RefCell, rc::Rc};

impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut pre_val = i32::MAX;
        let mut difference = i32::MAX;
        let mut stack = vec![root];
        while let Some(root) = stack.pop() {
            if let Some(node) = root {
                let mut refs = node.borrow_mut();
                stack.push(refs.right.take());
                if refs.left.is_some() {
                    stack.push(Some(node.clone()));
                } else {
                    difference = difference.min((refs.val - pre_val).abs()); // abs为了处理第一次相减
                    pre_val = refs.val;
                }
                stack.push(refs.left.take());
            }
        }

        difference
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
