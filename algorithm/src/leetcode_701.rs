use std::{cell::RefCell, rc::Rc};

impl Solution {
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = &root {
            let mut refs = node.borrow_mut();

            if val < refs.val {
                if refs.left.is_some() {
                    Self::insert_into_bst(refs.left.clone(), val);
                } else {
                    refs.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                }
            } else if val > refs.val {
                if refs.right.is_some() {
                    Self::insert_into_bst(refs.right.clone(), val);
                } else {
                    refs.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                }
            }
        } else {
            return Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }

        root
    }

    // 感觉应该这样才对，需要切断链条；虽然这个答案没有AC
    pub fn insert_into_bsts(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = &root {
            let mut refs = node.borrow_mut();

            if val < refs.val {
                let mut new_node = TreeNode::new(val);
                if let Some(left) = &refs.left {
                    if val < left.borrow().val {
                        Self::insert_into_bst(refs.left.clone(), val);
                    } else {
                        new_node.left = refs.left.take();
                    }
                }
                refs.left = Some(Rc::new(RefCell::new(new_node)));
            } else if val > refs.val {
                let mut new_node = TreeNode::new(val);
                if let Some(right) = &refs.right {
                    if val > right.borrow().val {
                        Self::insert_into_bst(refs.right.clone(), val);
                    } else {
                        new_node.right = refs.right.take();
                    }
                }
                refs.right = Some(Rc::new(RefCell::new(new_node)));
            }
        }

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
