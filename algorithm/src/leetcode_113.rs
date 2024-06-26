use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        pub fn recursion(
            root: Option<Rc<RefCell<TreeNode>>>,
            mut acount: Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if let Some(node) = root {
                let refs = node.borrow();
                let target_sum: i32 = acount.pop().unwrap() - refs.val;
                acount.push(refs.val);
                acount.push(target_sum);

                if refs.left.is_none() && refs.right.is_none() {
                    if acount.pop().unwrap() == 0 {
                        result.push(acount);
                        return;
                    }
                }

                recursion(refs.left.clone(), acount.clone(), result);
                recursion(refs.right.clone(), acount, result);
            }
        }
        recursion(root, vec![target_sum], &mut result);

        result
    }

    pub fn path_sum_backtrace(
        root: Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
    ) -> Vec<Vec<i32>> {
        let mut ps = PathSum {
            result: vec![],
            amount: vec![],
        };
        if root.is_some() {
            ps.path_sum(root, target_sum);
        }

        ps.result
    }
}

struct PathSum {
    result: Vec<Vec<i32>>,
    amount: Vec<i32>,
}

impl PathSum {
    pub fn path_sum(&mut self, root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) {
        let binding = root.unwrap();
        let refs = binding.borrow();
        self.amount.push(refs.val);
        let target_sum = target_sum - refs.val;

        if refs.left.is_none() && refs.right.is_none() {
            if target_sum == 0 {
                self.result.push(self.amount.clone());
                return;
            }
        }

        if refs.left.is_some() {
            self.path_sum(refs.left.clone(), target_sum);
            self.amount.pop();
        }

        if refs.right.is_some() {
            self.path_sum(refs.right.clone(), target_sum);
            self.amount.pop();
        }
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
