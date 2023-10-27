use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn arrayToTree(array: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut tree = TreeNode::new(0);
    for (i, item) in array.iter().enumerate() {
        tree.val = *item;
    }

    Some(Rc::new(RefCell::new(tree)))
}

// pub fn generate_tree(tree: &mut Option<Rc<RefCell<TreeNode>>>, array: &Vec<i32>) {

//     for (i, item) in array.iter().enumerate() {
//         tree.val = *item;
//     }
// }
