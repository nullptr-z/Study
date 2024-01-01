use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut pre_val = i64::MIN;
        fn recursion(root: Option<Rc<RefCell<TreeNode>>>, pre_val: &mut i64) -> bool {
            if let Some(node) = root {
                let refs = node.borrow();
                let l = recursion(refs.left.clone(), pre_val);
                if !l || (refs.val as i64) <= *pre_val {
                    return false;
                }
                *pre_val = refs.val as i64;
                return recursion(refs.right.clone(), pre_val);
            }
            true
        }

        recursion(root, &mut pre_val)
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::tree_utils::arrayToTree;

    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::is_valid_bst(arrayToTree(vec![5, 4, 6, -1, -1, 3, 7]));
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;

use crate::tree_utils::TreeNode;
