use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, mut postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let result = Self::generate(&mut postorder, &inorder);

        return result;
    }

    pub fn generate(preorder: &mut Vec<i32>, inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() || inorder.is_empty() {
            return None;
        }
        let val = preorder.pop().unwrap();

        let mut root = TreeNode::new(val);
        let mut sp = inorder.split(|v| *v == val);
        let left = sp.next();
        let right = sp.next();
        if let Some(right) = right {
            root.right = Self::generate(preorder, right);
        }
        if let Some(left) = left {
            root.left = Self::generate(preorder, left);
        }
        Some(Rc::new(RefCell::new(root)))
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::Solution;

    #[test]
    fn should_work() {
        Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]);
    }
}

pub struct Solution;

use crate::tree_utils::TreeNode;
