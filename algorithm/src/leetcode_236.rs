use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut ancestry_node = None;

        if let Some(node) = root {
            Self::search_ancestry(
                &node,
                p.unwrap().borrow().val,
                q.unwrap().borrow().val,
                &mut ancestry_node,
            );
        }

        ancestry_node.clone()
    }

    fn search_ancestry(
        root: &Rc<RefCell<TreeNode>>,
        p: i32,
        q: i32,
        ancestry_node: &mut Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let node = root.borrow();

        let left = if let Some(node) = &node.left {
            let result = Self::search_ancestry(&node, p, q, ancestry_node);
            result
        } else {
            false
        };
        let right = if let Some(node) = &node.right {
            let result = Self::search_ancestry(&node, p, q, ancestry_node);
            result
        } else {
            false
        };

        if ancestry_node.is_some() {
            return true;
        }

        let flag = node.val == q || node.val == p;

        if (left && right) || (flag && (left || right)) {
            *ancestry_node = Some(root.clone())
        }

        right || left || flag
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
