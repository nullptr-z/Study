use std::{cell::RefCell, rc::Rc};

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = &root {
            let lt = p.as_ref().unwrap().borrow().val;
            let lg = q.as_ref().unwrap().borrow().val;
            let refs = node.borrow();
            if lt < refs.val && lg < refs.val {
                return Self::lowest_common_ancestor(refs.left.clone(), p, q);
            } else if lt > refs.val && lg > refs.val {
                return Self::lowest_common_ancestor(refs.right.clone(), p, q);
            }
            return Some(node.clone());
        }
        None
    }

    pub fn lowest_common_ancestor_s(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = vec![root];
        let q = q.unwrap().borrow().val;
        let p = p.unwrap().borrow().val;
        let (q, p) = (q.min(p), q.max(p)); // 分两行写的话一定记得要换变量名

        while let Some(Some(current)) = stack.pop() {
            let refs = current.borrow();
            if refs.val > p {
                stack.push(refs.left.clone());
            } else if refs.val < q {
                stack.push(refs.right.clone());
            } else {
                return Some(current.clone());
            }
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
