use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        Self::tree_iter(root, &mut result);

        result
    }

    fn tree_iter(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node) = root {
            let that = node.clone();
            result.push(that.borrow().val);
            Self::tree_iter(that.borrow().left.clone(), result);
            Self::tree_iter(that.borrow().right.clone(), result);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::tree_utils::arrayToTree;

    use super::Solution;

    #[test]
    fn should_work() {
        // Solution::max_depth(Some(Rc::new(RefCell::new(arrayToTree(vec![1, -1, 2, 3])))));
    }
}

pub struct Solution;
use crate::tree_utils::TreeNode;
