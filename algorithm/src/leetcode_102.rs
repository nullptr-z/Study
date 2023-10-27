use std::cell::RefCell;
use std::collections::VecDeque;
use std::mem::swap;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut queue = VecDeque::<Rc<RefCell<TreeNode>>>::new();
        if let Some(root) = root {
            queue.push_back(root);
        }

        let mut level_size = 1;

        let mut items: Vec<i32> = Vec::new();
        while !queue.is_empty() {
            if let Some(node) = queue.pop_front() {
                let node = node.borrow();
                items.push(node.val);
                if let Some(left) = node.left.clone() {
                    queue.push_back(left);
                }
                if let Some(right) = node.right.clone() {
                    queue.push_back(right);
                }
            }

            level_size -= 1;
            if level_size == 0 {
                if items.len() > 0 {
                    result.push(items.clone());
                }
                items.resize(0, 0);
                level_size = queue.len();
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::tree_utils::arrayToTree;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn should_work() {
        let tree = arrayToTree(vec![3, 9, 20, -1, -1, 15, 7]);
        println!("【 tree 】==> {:#?}", tree);
        // Solution::level_order(tree);
        // Solution::level_order(Some(Rc::new(RefCell::new(arrayToTree(vec![
        //     3, 9, 20, -1, -1, 15, 7,
        // ])))));
    }
}

pub struct Solution;

use crate::search_struct::PriorityQueue;
use crate::tree_utils::TreeNode;
