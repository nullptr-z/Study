use std::{cell::RefCell, rc::Rc};

impl Solution {
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let mut refs = node.borrow_mut();
            if refs.val < low {
                // 当前节点小于low，左子树都是更小的必然不满足直接抛弃
                return Self::trim_bst(refs.right.take(), low, high);
            }
            if refs.val > high {
                // 当前节点大high，右子树都是更大的必然不满足直接抛弃
                return Self::trim_bst(refs.left.take(), low, high);
            }
            // 当前节点在`low-hig`区间范围内的执行流程，向下迭代看还有没有不符合条件的
            refs.left = Self::trim_bst(refs.left.take(), low, high);
            refs.right = Self::trim_bst(refs.right.take(), low, high);
            return Some(node.clone());
        }

        None
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
