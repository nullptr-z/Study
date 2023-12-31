// 中：左根右
// 后：左右根

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() || preorder.is_empty() {
            return None;
        }

        let (pre, preorder) = preorder.split_at(1);
        let root_val = pre[0];
        let mut root = TreeNode::new(root_val);
        let sp_line = inorder.iter().position(|m| *m == root_val).unwrap();

        root.left = Self::build_tree(preorder[..sp_line].to_vec(), inorder[..sp_line].to_vec());
        root.right = Self::build_tree(
            preorder[sp_line..].to_vec(),
            inorder[(sp_line + 1)..].to_vec(),
        );

        Some(Rc::new(RefCell::new(root)))
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::tree_utils::arrayToTree;

    use super::Solution;

    #[test]
    fn should_work() {
        // let res = Solution::build_tree([9, 3, 15, 20, 7].into(), [9, 15, 7, 20, 3].into());
        // println!("【 res 】==> {:?}", res);
        let res = Solution::build_tree([3, 9, 20, 15, 7].into(), [9, 3, 15, 20, 7].into());
        println!("【 res 】==> {:?}", res);
    }
}

pub struct Solution;

use crate::tree_utils::TreeNode;
