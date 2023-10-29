use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(that) = root {
            let mut min = i32::MAX;
            Self::tree_iter(that, &mut -1, &mut min);
            return min;
        }

        0
    }

    fn tree_iter(root: Rc<RefCell<TreeNode>>, pre: &mut i32, min: &mut i32) {
        let node = root.borrow();

        if let Some(that) = node.left.clone() {
            Self::tree_iter(that, pre, min);
        };

        if *pre != -1 {
            *min = (*min).min(node.val - *pre);
        }
        *pre = node.val;

        if let Some(that) = node.right.clone() {
            Self::tree_iter(that, pre, min);
        };
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
