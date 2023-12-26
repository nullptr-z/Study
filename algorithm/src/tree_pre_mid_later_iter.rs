use std::{cell::RefCell, rc::Rc};

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        let mut stack = vec![];
        stack.push(root);
        while let Some(mut tree) = stack.pop() {
            if let Some(node) = tree.take() {
                result.push(node.borrow().val);
                stack.push(node.borrow_mut().right.take());
                stack.push(node.borrow_mut().left.take());
            }
        }

        result
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        let mut stack = vec![];
        stack.push(root);
        while let Some(tree) = stack.pop() {
            if let Some(node) = tree.clone() {
                stack.push(node.borrow_mut().right.take());
                if node.borrow().left.is_none() {
                    result.push(node.borrow().val);
                } else {
                    stack.push(tree);
                }
                stack.push(node.borrow_mut().left.take());
            }
        }

        result
    }

    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        let mut stack = vec![];
        stack.push(root);
        while let Some(tree) = stack.pop() {
            if let Some(node) = tree.clone() {
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    result.push(node.borrow().val);
                } else {
                    stack.push(tree);
                }
                stack.push(node.borrow_mut().right.take());
                stack.push(node.borrow_mut().left.take());
            }
        }

        result
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
        let ret = Solution::postorder_traversal(arrayToTree(vec![1, -1, 2, 3]));
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
use crate::tree_utils::TreeNode;
