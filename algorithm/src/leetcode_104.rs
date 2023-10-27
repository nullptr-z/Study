use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
        if let Some(r) = root {
            queue.push_back(r);
        }
        let mut level_count = 1;

        let mut level = 0;
        while queue.len() > 0 {
            level_count -= 1;
            if let Some(node) = queue.pop_front() {
                let node = node.borrow();
                if let Some(n) = node.left.clone() {
                    queue.push_back(n);
                }
                if let Some(n) = node.right.clone() {
                    queue.push_back(n);
                }
            }

            if level_count == 0 {
                level_count = queue.len();
                level += 1;
            }
        }

        level
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::tree_utils::arrayToTree;

    use super::Solution;

    #[test]
    fn should_work() {
        Solution::max_depth(arrayToTree(vec![3, 9, 20, -1, -1, 15, 7]));
    }
}

pub struct Solution;

use crate::tree_utils::TreeNode;
