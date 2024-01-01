use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let refs = node.borrow();

            return match refs.val.cmp(&val) {
                std::cmp::Ordering::Equal => Some(node.clone()),
                std::cmp::Ordering::Less => Self::search_bst(refs.right.clone(), val),
                std::cmp::Ordering::Greater => Self::search_bst(refs.left.clone(), val),
            };
        }
        None
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
