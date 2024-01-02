use std::{cell::RefCell, rc::Rc};

impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ret = vec![];
        fn recursion(root: Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<(i32, usize)>) {
            if let Some(node) = root {
                let refs = node.borrow();
                recursion(refs.left.clone(), ret);
                if !ret.is_empty() {
                    if ret.last().unwrap().0 == refs.val {
                        ret.last_mut().unwrap().1 += 1;
                    } else {
                        remove_small(ret);
                        ret.push((refs.val, 1));
                    }
                } else {
                    ret.push((refs.val, 1));
                }
                recursion(refs.right.clone(), ret);
            }
        }

        recursion(root, &mut ret);
        remove_small(&mut ret);

        ret.iter().map(|v| v.0).collect()
    }
}

fn remove_small(vec: &mut Vec<(i32, usize)>) {
    if vec.len() > 2 {
        return;
    }
    if let Some(pre) = vec.pop() {
        if let Some(last) = vec.last() {
            if pre.1 < vec.last().unwrap().1 {
                return;
            } else if pre.1 > last.1 {
                vec.clear();
            }
        }
        vec.push(pre.clone());
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
