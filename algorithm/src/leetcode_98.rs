use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut pre = i64::MIN;
        Self::tree_iter(root, &mut pre)
    }

    fn tree_iter(root: Option<Rc<RefCell<TreeNode>>>, pre: &mut i64) -> bool {
        if let Some(node) = root {
            let that = node.clone();
            let mut l = Self::tree_iter(that.borrow().left.clone(), pre);
            if *pre != i64::MIN {
                l = (*pre as i32) < that.borrow().val && l;
            }
            *pre = that.borrow().val as i64;
            let r = Self::tree_iter(that.borrow().right.clone(), pre);
            return l && r;
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
    fn should_work() {}
}

pub struct Solution;
use crate::tree_utils::TreeNode;
