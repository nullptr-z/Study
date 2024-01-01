use std::{cell::RefCell, rc::Rc};

impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node1) = &root1 {
            if let Some(node2) = root2 {
                let mut n1 = node1.borrow_mut();
                let n2 = node2.borrow_mut();
                n1.val += n2.val;
                n1.left = Self::merge_trees(n1.left.clone(), n2.left.clone());
                n1.right = Self::merge_trees(n1.right.clone(), n2.right.clone());
                return Some(node1.clone());
            }
        }

        if root1.is_some() {
            return root1;
        }
        if root2.is_some() {
            return root2;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::tree_utils::arrayToTree;

    use super::Solution;

    #[test]
    fn should_work() {
        Solution::merge_trees(
            arrayToTree(vec![1, 3, 2, 5]),
            arrayToTree(vec![2, 1, 3, -1, 4, -1, 7]),
        );
    }
}

pub struct Solution;

use crate::tree_utils::TreeNode;
