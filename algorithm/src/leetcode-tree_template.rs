use std::{cell::RefCell, rc::Rc};

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
        let tree = arrayToTree(vec![0, 0, -1, 0, 0]);
    }
}

pub struct Solution;

use crate::tree_utils::TreeNode;
