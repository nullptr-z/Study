use std::{cell::RefCell, rc::Rc};

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        Self::preorder(root.clone(), &mut result);

        let mut result = vec![];
        Self::inorder(root.clone(), &mut result);

        let mut result = vec![];
        Self::postorder(root.clone(), &mut result);

        result
    }

    fn preorder(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node) = root {
            result.push(node.borrow().val);
            Self::preorder(node.borrow_mut().left.take(), result);
            Self::preorder(node.borrow_mut().right.take(), result);
        }
    }

    fn inorder(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node) = root {
            Self::inorder(node.borrow_mut().left.take(), result);
            result.push(node.borrow().val);
            Self::inorder(node.borrow_mut().right.take(), result);
        }
    }

    fn postorder(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node) = root {
            Self::postorder(node.borrow_mut().left.take(), result);
            Self::postorder(node.borrow_mut().right.take(), result);
            result.push(node.borrow().val);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tree_utils::arrayToTree;

    use super::Solution;

    #[test]
    fn should_work() {
        // let ret = Solution::preorder_traversal(arrayToTree(vec![1, -1, 2, 3]));
        // println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
use crate::tree_utils::TreeNode;
