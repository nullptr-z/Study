use std::{cell::RefCell, rc::Rc};

impl Solution {
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = &root {
            let mut refs = root.borrow_mut();

            if key < refs.val {
                // 小于，向左边递归
                refs.left = Self::delete_node(refs.left.take(), key);
            } else if key > refs.val {
                // 大于，向右边递归
                refs.right = Self::delete_node(refs.right.take(), key);
            } else {
                return match (refs.left.take(), refs.right.take()) {
                    // 没有找到key
                    (None, None) => None,
                    // 找到，没有左子树，返回右子树
                    (None, Some(right)) => Some(right),
                    // 找到，没有右子树，返回左子树
                    (Some(left), None) => Some(left),
                    // 找到，左右子树都在，把左子树追加到，右子树第一个节点的左叶子节点
                    (Some(left), Some(right)) => {
                        let mut cur = right.clone();
                        while let Some(ref left) = cur.clone().borrow().left {
                            cur = left.clone();
                        }
                        cur.borrow_mut().left = Some(left.clone());
                        Some(right)
                    }
                };
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
    fn should_work() {
        let ret = Solution::delete_node(arrayToTree(vec![5, 3, 6, 2, 4, -1, 7]), 3);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;

use crate::tree_utils::TreeNode;
