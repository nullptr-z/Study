use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
        let mut result = 0;
        Self::tree_iter(root, &mut k, &mut result);

        result
    }

    fn tree_iter(root: Option<Rc<RefCell<TreeNode>>>, k: &mut i32, result: &mut i32) {
        if *k <= 0 {
            return;
        }
        if let Some(node) = root {
            let that = node.clone();
            Self::tree_iter(that.borrow().left.clone(), k, result);
            *k -= 1;
            if *k == 0 {
                *result = that.borrow().val;
            }
            Self::tree_iter(that.borrow().right.clone(), k, result);
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
