use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        pub fn recursion(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(node) = root {
                let l = recursion(node.borrow().left.clone());
                let r = recursion(node.borrow().right.clone());

                if l == -1 || r == -1 || (l - r).abs() > 1 {
                    return -1;
                }

                return l.max(r) + 1;
            }
            0
        }
        let result = recursion(root);

        result != -1
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
