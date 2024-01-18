use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recursion(root: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> (bool, bool) {
            if let Some(node) = root {
                let refs = node.borrow();
                let left = recursion(refs.left.clone(), result);
                let right = recursion(refs.right.clone(), result);

                if left.1 && right.1 {
                    // 左右子节点都是可见的，不点亮自己；自己的可见状态，依赖于子节点
                    return (false, left.0 || right.0);
                }

                if !(left.1 && right.1) {
                    *result += 1;
                    // 有一个子节点不可见，点亮自己，照亮他
                    return (true, true);
                }
            }

            // 空节点，设置为无摄像头，可见
            (false, true)
        }

        let mut result = 0;
        if !recursion(root, &mut result).1 {
            // 特殊处理根节点，左右子节点没有被点亮的情况,例如[0]
            result += 1;
        }

        result
    }

    pub fn min_camera_cover_op(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recursion(root: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> (bool, bool) {
            if let Some(node) = root {
                let refs = node.borrow();
                let left = recursion(refs.left.clone(), result);
                let right = recursion(refs.right.clone(), result);

                return match (left.1, right.1) {
                    // 左右子节点都是可见的，不点亮自己；自己的可见状态，依赖于子节点
                    (true, true) => (false, left.0 || right.0),
                    _ => {
                        // 有一个子节点不可见(其中之一false)，点亮自己，照亮他
                        *result += 1;
                        (true, true)
                    }
                };
            }

            // 空节点，设置为无摄像头，可见
            (false, true)
        }

        let mut result = 0;
        if !recursion(root, &mut result).1 {
            // 根节点不可见，特殊处理；左右子节点没有被点亮出现的情况,例如[0]
            result += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::tree_utils::arrayToTree;

    #[test]
    fn should_work() {
        let tree = arrayToTree(vec![0, 0, 0]);
        let ret = Solution::min_camera_cover(tree);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;

use crate::tree_utils::TreeNode;
