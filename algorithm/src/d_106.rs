// 中：左根右
// 后：左右根

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() {
            return None;
        }

        let mut postorder = postorder.to_owned();
        let root_val = postorder.pop().unwrap();

        let mut root = TreeNode::new(root_val);
        let sp_line = inorder.iter().position(|m| *m == root_val).unwrap();

        root.left =
            Solution::build_tree(inorder[..sp_line].to_owned(), postorder[..sp_line].to_vec());
        root.right = Solution::build_tree(
            inorder[(sp_line + 1)..].to_owned(),
            postorder[sp_line..].to_vec(),
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
        let res = Solution::build_tree([9, 3, 15, 20, 7].into(), [9, 15, 7, 20, 3].into());
        println!("【 res 】==> {:?}", res);
    }
}

pub struct Solution;

use crate::tree_utils::TreeNode;
