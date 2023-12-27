use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
        let mut depth = 0;

        if root.is_some() {
            queue.push_back(root);
        }
        let mut level_count = 1;

        while let Some(root) = queue.pop_front() {
            level_count -= 1;
            if let Some(node) = root {
                let node_ref = node.borrow();
                if node_ref.left.is_none() && node_ref.right.is_none() {
                    return depth + 1;
                }
                queue.push_back(node_ref.left.clone());
                queue.push_back(node_ref.right.clone());
            }

            if level_count == 0 {
                depth += 1;
                level_count = queue.len();
            }
        }

        depth
    }

    pub fn min_depth_recursion(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let l = Solution::min_depth(root.borrow().left.clone());
            let r = Solution::min_depth(root.borrow().right.clone());

            // 如果left空，则不被考虑,使用right的结果
            if root.borrow().left.is_none() {
                return r + 1;
            }
            // 如果right空，则不被考虑，使用left的结果
            if root.borrow().right.is_none() {
                return l + 1;
            }

            // 都不为空
            return l.min(r) + 1;
        }
        return 0;
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
