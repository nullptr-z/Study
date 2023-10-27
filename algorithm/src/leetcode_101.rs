use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
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
