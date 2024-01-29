use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // (偷，不偷)
        fn recursion(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            if let Some(node) = root {
                let mut refs = node.borrow_mut();
                let left = recursion(refs.left.take());
                let right = recursion(refs.right.take());

                let steal = refs.val + left.1 + right.1;
                let not_steal = left.0.max(left.1) + right.0.max(right.1);
                return (steal, not_steal);
            }
            (0, 0)
        }

        let dp = recursion(root);
        dp.0.max(dp.1)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {}
}

pub struct Solution;

use crate::tree_utils::TreeNode;
