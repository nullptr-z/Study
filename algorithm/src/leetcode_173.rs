use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct BSTIterator {
    middle_sequence: Vec<i32>,
    cur_ptr: usize,
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut that = Self {
            middle_sequence: vec![],
            cur_ptr: 0,
        };
        if let Some(node) = root {
            BSTIterator::middle_iter(&mut that, &node);
        }

        that
    }

    fn next(&mut self) -> i32 {
        if self.cur_ptr < self.middle_sequence.len() {
            let val = self.middle_sequence[self.cur_ptr];
            self.cur_ptr += 1;

            return val;
        }

        0
    }

    fn has_next(&self) -> bool {
        self.cur_ptr < self.middle_sequence.len()
    }

    fn middle_iter(&mut self, root: &Rc<RefCell<TreeNode>>) {
        let root = root.borrow();
        if let Some(node) = &root.left {
            self.middle_iter(&node)
        }
        self.middle_sequence.push(root.val);
        if let Some(node) = &root.right {
            self.middle_iter(&node)
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
