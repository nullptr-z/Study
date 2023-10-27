use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::tree_utils::arrayToTree;

    use super::Solution;

    #[test]
    fn should_work() {
        Solution::max_depth(Some(Rc::new(RefCell::new(arrayToTree(vec![
            3, 9, 20, -1, -1, 15, 7,
        ])))));
    }
}

pub struct Solution;

use crate::tree_utils::TreeNode;
