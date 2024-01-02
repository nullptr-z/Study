use std::{cell::RefCell, rc::Rc};

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root == p || root == q || root.is_none() {
            return root;
        }
        if let Some(node) = root {
            let refs = node.borrow();
            let left = Self::lowest_common_ancestor(refs.left.clone(), p.clone(), q.clone());
            let right = Self::lowest_common_ancestor(refs.right.clone(), p, q);
            match (&left, &right) {
                // 不会出现None None
                (Some(_), None) => left,
                (Some(_), Some(_)) => Some(node.clone()),
                _ => right,
            }
        } else {
            None
        }
    }

    pub fn lowest_common_ancestors(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut cur = None;
        fn recursion(
            root: &Option<Rc<RefCell<TreeNode>>>,
            p: i32,
            q: i32,
            cur: &mut Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            if let Some(node) = root {
                let refs = node.borrow();
                let l = recursion(&refs.left, p, q, cur);
                let r = recursion(&refs.right, p, q, cur);
                if cur.is_none() && l && r {
                    *cur = Some(node.clone());
                    return false;
                }
                if refs.val == q || refs.val == p {
                    if cur.is_none() && (l || r) {
                        *cur = Some(node.clone());
                        return false;
                    }
                    return true;
                }
                return l || r;
            }
            false
        }

        recursion(
            &root,
            p.unwrap().borrow().val,
            q.unwrap().borrow().val,
            &mut cur, // 修正为可变引用
        );

        cur
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
