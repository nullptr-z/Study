use std::{cell::RefCell, rc::Rc};

impl Solution {
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn recursion(root: &Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
            if let Some(root) = root {
                let mut refs = root.borrow_mut();
                recursion(&refs.right.clone(), sum);
                *sum += refs.val;
                refs.val = *sum;
                recursion(&refs.left.clone(), sum);
            }
        }

        let mut sum = 0;
        recursion(&root, &mut sum);

        root
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
