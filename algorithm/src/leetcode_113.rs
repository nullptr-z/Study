use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        pub fn recursion(
            root: Option<Rc<RefCell<TreeNode>>>,
            mut acount: Vec<i32>,
            result: &mut Vec<Vec<i32>>,
            target_sum: i32,
        ) {
            if let Some(node) = root {
                let refs = node.borrow();
                let sum = acount.pop().unwrap() + refs.val;
                acount.push(refs.val);
                acount.push(sum);

                if refs.left.is_none() && refs.right.is_none() {
                    if acount.pop().unwrap() == target_sum {
                        result.push(acount);
                        return;
                    }
                }

                recursion(refs.left.clone(), acount.clone(), result, target_sum);
                recursion(refs.right.clone(), acount, result, target_sum);
            }
        }
        recursion(root, vec![0], &mut result, target_sum);

        result
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::tree_utils::arrayToTree;

    use super::Solution;

    #[test]
    fn should_work() {}
}

pub struct Solution;

use crate::tree_utils::TreeNode;
