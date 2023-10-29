use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut result2 = vec![];
        let mut queue = VecDeque::new();

        if let Some(root) = root {
            queue.push_front(root);
        }

        let mut level_count = 1;
        let mut level_count1 = 1;
        while let Some(node) = queue.pop_back() {
            let node = node.borrow();
            if let Some(node) = &node.left {
                queue.push_front(node.clone());
            }
            if let Some(node) = &node.right {
                queue.push_front(node.clone());
            }
            level_count -= 1;
            result2.push(node.val);
            if level_count == 0 {
                // 不是很符合题意，可以通过改变`queue`弹出顺序来做
                if level_count1 % 2 == 0 {
                    result2.reverse();
                }
                result.push(result2);
                result2 = vec![];
                level_count = queue.len();
                level_count1 += 1;
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
