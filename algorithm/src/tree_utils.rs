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
    let mut index = 0;
    array_to_tree(&array, &mut index)
}

pub fn array_to_tree(array: &Vec<i32>, index: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
    if *index >= array.len() || array[*index] == -1 {
        *index += 1;
        return None;
    }

    let val = array[*index];

    let node = Rc::new(RefCell::new(TreeNode::new(val)));
    *index += 1;
    node.borrow_mut().left = array_to_tree(array, index);
    *index += 1;
    node.borrow_mut().right = array_to_tree(array, index);

    Some(node)
}
