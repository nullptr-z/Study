use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut pre = None;
        Self::pre_iter(root, &mut pre);
    }

    fn pre_iter(root: &mut Option<Rc<RefCell<TreeNode>>>, pre: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            Self::pre_iter(&mut node.borrow_mut().right, pre);
            Self::pre_iter(&mut node.borrow_mut().left, pre);
            node.borrow_mut().left = None;
            node.borrow_mut().right = pre.take();
            *pre = Some(node.clone())
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
