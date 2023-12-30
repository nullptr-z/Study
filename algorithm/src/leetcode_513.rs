use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};
use std::rc::Rc;

impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut map = BTreeMap::new();
        fn recursion(
            root: Option<Rc<RefCell<TreeNode>>>,
            depth: usize,
            map: &mut BTreeMap<usize, i32>,
        ) {
            if let Some(node) = root {
                let refs = node.borrow();
                if refs.left.is_none() && refs.right.is_none() {
                    if map.get(&depth).is_none() {
                        map.insert(depth, node.borrow().val);
                    }
                }
                recursion(refs.left.clone(), depth + 1, map);
                recursion(refs.right.clone(), depth + 1, map);
            }
        }

        recursion(root, 0, &mut map);

        let result = map.last_key_value().map_or(0, |m| *m.1);

        result
    }

    pub fn find_bottom_left_value_sequence(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack = VecDeque::new();
        let mut result = 0;
        let mut i = 1;

        if let Some(node) = root {
            result = node.borrow().val;
            stack.push_back(Some(node));
        }

        while let Some(root) = stack.pop_front() {
            if let Some(node) = root {
                let refs = node.borrow();
                if let Some(node) = refs.left.clone() {
                    stack.push_back(Some(node.clone()));
                }
                if let Some(node) = refs.right.clone() {
                    stack.push_back(Some(node.clone()));
                }
            }

            i -= 1;
            if i == 0 {
                i = stack.len();
                if let Some(node) = stack.front() {
                    result = node.clone().unwrap().borrow().val;
                }
            }
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
