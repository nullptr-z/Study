use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut result = vec![];
        let mut queue = VecDeque::new();

        if let Some(root) = root {
            queue.push_back(root);
        }

        let mut level_count = 1;
        let mut level_counts: usize = 1;
        let mut sum: i64 = 0;
        while let Some(node) = queue.pop_back() {
            let node = node.borrow();
            if let Some(node) = &node.left {
                queue.push_front(node.clone());
            }
            if let Some(node) = &node.right {
                queue.push_front(node.clone());
            }
            level_count -= 1;
            sum += node.val as i64;
            if level_count == 0 {
                result.push((sum as f64) / (level_counts as f64));
                sum = 0;
                level_count = queue.len();
                level_counts = queue.len();
            }
        }

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
