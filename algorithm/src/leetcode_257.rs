use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut result = vec![];

        fn recursion(root: Option<Rc<RefCell<TreeNode>>>, path: &str, result: &mut Vec<String>) {
            if let Some(node) = root {
                let mut path_join = format!("{}", node.borrow().val);
                if !path.is_empty() {
                    path_join = format!("{}->{}", path, path_join);
                };

                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    result.push(path_join.into());
                    return;
                }

                recursion(node.borrow().left.clone(), &path_join, result);
                recursion(node.borrow().right.clone(), &path_join, result);
            }
        }

        recursion(root, "", &mut result);

        result
    }

    pub fn binary_tree_pathss(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut result = vec![];

        fn recursion(node: Rc<RefCell<TreeNode>>, path: &str, result: &mut Vec<String>) {
            let node = node.borrow();
            let mut path_join = format!("{}", node.val);
            if !path.is_empty() {
                path_join = format!("{}->{}", path, path_join);
            };

            if node.left.is_none() && node.right.is_none() {
                result.push(path_join.to_string());
                return;
            }
            if let Some(n) = &node.left {
                recursion(n.clone(), &path_join, result);
            }
            if let Some(n) = &node.right {
                recursion(n.clone(), &path_join, result);
            }
        }

        if root.is_some() {
            recursion(root.unwrap(), "", &mut result);
        }

        result
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
