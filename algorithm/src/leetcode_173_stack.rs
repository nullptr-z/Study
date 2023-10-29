use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
    cur: Option<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self {
            stack: vec![],
            cur: root,
        }
    }

    fn next(&mut self) -> i32 {
        while let Some(node) = self.cur.take() {
            self.stack.push(node.clone());
            self.cur = node.borrow().left.clone();
        }

        if let Some(top) = self.stack.pop() {
            self.cur = top.borrow().right.clone();
            return top.borrow().val;
        }

        0
    }

    fn has_next(&self) -> bool {
        return self.stack.len() > 0 || self.cur.is_some();
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
