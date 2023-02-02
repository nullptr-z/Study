use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use super::{get_index, FIFO};

pub struct TreeNode<T = i32> {
    value: T,
    left: Option<Rc<RefCell<TreeNode<T>>>>,
    right: Option<Rc<RefCell<TreeNode<T>>>>,
    pre: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T> {
    #[inline]
    fn new(value: T) -> TreeNode<T> {
        Self {
            value,
            left: None,
            right: None,
            pre: None,
        }
    }
}

pub fn is_unit_val_tree<T: Copy + Eq>(root: Option<Rc<RefCell<TreeNode<T>>>>) -> bool {
    fn is_val_eq<T: Copy + Eq>(node: Option<Rc<RefCell<TreeNode<T>>>>, val: T) -> bool {
        if let Some(t_node) = node.clone() {
            return t_node.borrow().value == val
                && is_val_eq(t_node.borrow().left.clone(), val)
                && is_val_eq(t_node.borrow().right.clone(), val);
        }
        true // 节点为 None
    }

    let is_eq = is_val_eq(root.clone(), root.unwrap().borrow().value);

    let a = is_eq;
    is_eq
}

pub fn inorder_traverse<T: Copy>(root: Option<Rc<RefCell<TreeNode<T>>>>) -> Vec<T> {
    fn dfs<T: Copy>(root: Option<Rc<RefCell<TreeNode<T>>>>, list: &mut Vec<T>) {
        if let Some(x) = root {
            dfs(x.borrow().left.clone(), list);
            list.push(x.borrow().value);
            dfs(x.borrow().right.clone(), list);
        }
    }
    let mut list: Vec<T> = vec![];
    dfs(root, &mut list);
    list
}

pub fn levelOrder<T: Copy>(root: Option<Rc<RefCell<TreeNode<T>>>>) -> Vec<T> {
    let capacity = 100usize;
    let mut fifo = FIFO::with_capacity(capacity);
    let mut vec = Vec::with_capacity(capacity);
    fifo.push(root);
    while fifo.len() > 0 {
        let count = fifo.len();
        let item = fifo.pop().unwrap();
        match item {
            Some(x) => {
                vec.push(x.borrow().value);
                fifo.push(x.borrow().left.clone());
                fifo.push(x.borrow().right.clone());
            }
            None => (),
        }
    }

    vec
}

pub fn levelOrder_z<T: Copy>(root: Option<Rc<RefCell<TreeNode<T>>>>) -> Vec<T> {
    let capacity = 100usize;
    let mut fifo = VecDeque::with_capacity(capacity);
    let mut vec = Vec::with_capacity(capacity);
    fifo.push_back(root);

    let mut index = 0;
    while fifo.len() > 0 {
        let parity = index % 2 == 0;
        let len = fifo.len();
        let a = len;
        for _ in 0..len {
            let item = match parity {
                true => fifo.pop_front().unwrap(),
                false => fifo.pop_back().unwrap(),
            };

            if item.is_some() {
                let x = item.unwrap();
                vec.push(x.borrow().value);
                if parity {
                    fifo.push_back(x.borrow().left.clone());
                    fifo.push_back(x.borrow().right.clone());
                } else {
                    fifo.push_front(x.borrow().right.clone());
                    fifo.push_front(x.borrow().left.clone());
                }
            }
        }
        index += 1;
    }
    vec
}

pub fn levelOrder_pre_sequence<T: Copy>(root: Option<Rc<RefCell<TreeNode<T>>>>) -> Vec<T> {
    let capacity = 100usize;
    let mut tep_vec = Vec::with_capacity(capacity);
    tep_vec.push(root);
    let mut vec = Vec::with_capacity(capacity);

    while tep_vec.len() > 0 {
        let item = tep_vec.pop().unwrap();
        if item.is_some() {
            let x = item.unwrap();
            vec.push(x.borrow().value);
            tep_vec.push(x.borrow().right.clone());
            tep_vec.push(x.borrow().left.clone());
        };
    }
    vec
}

pub fn levelOrder_middle_sequence<T: Copy>(root: Option<Rc<RefCell<TreeNode<T>>>>) -> Vec<T> {
    let capacity = 100usize;
    let mut tep_vec = Vec::with_capacity(capacity);
    let mut root = root.clone();
    let mut vec = Vec::with_capacity(capacity);

    while tep_vec.len() > 0 || root.is_some() {
        while root.is_some() {
            tep_vec.push(root.clone());
            root = root.clone().unwrap().borrow().left.clone();
        }

        let mut item = tep_vec.pop().unwrap();
        vec.push(item.clone().unwrap().borrow().value);
        root = item.unwrap().borrow().right.clone();
    }
    vec.into()
}

pub fn levelOrder_end_sequence<T: Copy>(root: Option<Rc<RefCell<TreeNode<T>>>>) -> Vec<T> {
    let capacity = 100usize;
    let mut tep_vec = Vec::with_capacity(capacity);
    let mut root = root.clone();
    let mut vec = Vec::with_capacity(capacity);

    while tep_vec.len() > 0 || root.is_some() {
        while root.is_some() {
            tep_vec.push(root.clone());
            tep_vec.push(None);
            root = root.clone().unwrap().borrow().left.clone();
        }

        let mut item: Option<Rc<RefCell<TreeNode<T>>>> = tep_vec.pop().unwrap();
        if item.is_none() {
            let mut right = tep_vec.pop().unwrap();
            root = right.clone().unwrap().borrow().right.clone();
            tep_vec.push(right)
        } else {
            vec.push(item.clone().unwrap().borrow().value);
        }
    }
    vec.into()
}

fn generate_binary_tree_pre_middle<T: Copy + PartialEq>(
    prev: &[T],
    center: &[T],
) -> (usize, Option<Rc<RefCell<TreeNode<T>>>>) {
    let len = prev.len();
    if len < 1 {
        return (0, None);
    }

    // 跟节点值
    let root_value = prev[0];

    let mut root: Option<Rc<RefCell<TreeNode<T>>>> =
        Some(Rc::new(RefCell::new(TreeNode::new(root_value))));

    if len == 1 {
        return (1, root);
    };
    // 左根
    let mut left_root = center[0];
    // 左树长度
    let mut left_len = 0;
    for (index, item) in prev.iter().enumerate() {
        if *item == left_root {
            left_len = index + 1;
            break;
        }
    }
    if root_value == left_root {
        return (left_len, root);
    }

    let mut flag = true;
    // 前序从1开始左树遍历,
    let mut i = 1;
    while root.is_some() && i > 0 {
        let item = &prev[i];
        let root_t = root.clone().unwrap();
        if flag {
            root_t.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(*item))));
            root_t.borrow().left.clone().unwrap().borrow_mut().pre = root.clone();
            root = root_t.borrow().left.clone();
            i += 1;
            if *item == left_root {
                flag = false;
                let pre = root.clone().unwrap().borrow().pre.clone();
                let pre_value = pre.clone().unwrap().borrow().value;
                for (index, pre_item) in center.iter().enumerate() {
                    // 找到父节点
                    if *pre_item == pre_value {
                        // 如果父节点正好在,当前节点后一个位置,则,向父节点挂载右节点(中序)
                        if center[index - 1] == *item {
                            root = pre;
                        }
                        break;
                    }
                }
                continue;
            }
        } else {
            if left_len < len {
                let mut center_start = 0;
                for (index, item) in center.iter().enumerate() {
                    if *item == root.clone().unwrap().borrow().value {
                        center_start = index + 1;
                        break;
                    }
                }
                let (len, result) = generate_binary_tree_pre_middle(
                    &prev[left_len..len],
                    &center[center_start..len],
                );
                root.clone().unwrap().borrow_mut().right = result.clone();
                if root.clone().unwrap().borrow().pre.clone().is_none() {
                    return (left_len, root);
                }
                root = root.clone().unwrap().borrow().pre.clone();
                left_len += len;
                i -= len;
            } else {
                i -= 1;
            }
        }
    }

    (left_len, root)
}

// let inorder = ['G', 'D', 'H', 'B', 'E', 'A', 'F', 'I', 'C'];
// let preorder =  ['G', 'H', 'D', 'E', 'B', 'I', 'F', 'C', 'A'];
fn generate_binary_tree_middle_back<T: Copy + PartialEq>(
    middle: &[T],
    back: &[T],
) -> Option<Rc<RefCell<TreeNode<T>>>> {
    let middle_len = middle.len();
    // let back_len = back.len();

    let mut vec = Vec::with_capacity(100);

    let left_root_value = *middle.first().unwrap();
    let back_left_root_value = *back.first().unwrap();

    let mut left_child_len = get_index(back, &left_root_value).unwrap() + 1;
    let mut flag = true;
    // let left = Some(Rc::new(RefCell::new(TreeNode::new(left_root_value))));
    let left = match left_root_value != back_left_root_value {
        true => {
            // 如果中序首 不等于 后序首则说明，左根还有子节点
            // 后序子节点长度 = 0 ~ 左根
            // 左根前 = 后序左根下标 -1
            // 中序子节点长度 = 左根 ~ 后序左根前
            left_child_len = 0;
            None
        }
        false => Some(Rc::new(RefCell::new(TreeNode::new(left_root_value)))),
    };
    if middle_len == 1 {
        return left;
    }

    let mut next_child_index = left_child_len;
    let mut root = None;
    while next_child_index < middle_len {
        let root_value = middle[next_child_index];
        let mut root_temp = root.clone();
        root = Some(Rc::new(RefCell::new(TreeNode::new(root_value))));
        root.clone().unwrap().borrow_mut().left = root_temp;
        if flag && left.is_some() {
            root.clone().unwrap().borrow_mut().left = left.clone();
            vec.push(left.clone().unwrap().borrow().value);
            flag = false;
        }
        vec.push(root_value);
        if middle_len - next_child_index == 1 {
            return root;
        }

        let right_child_len = get_index(back, &root_value).unwrap() - next_child_index;
        if right_child_len == 0 {
            next_child_index += 1;
            continue;
        };
        let right = generate_binary_tree_middle_back_right_child(
            &middle[next_child_index..],
            &back[next_child_index..],
            right_child_len,
        );
        root.clone().unwrap().borrow_mut().right = right.clone();
        vec.push(right.clone().unwrap().borrow().value);
        next_child_index += right_child_len + 1;
    }
    root.clone()
}

fn generate_binary_tree_middle_back_right_child<T: PartialEq + Copy>(
    middle: &[T],
    back: &[T],
    right_child_len: usize,
) -> Option<Rc<RefCell<TreeNode<T>>>> {
    // 以右节点为根的遍历, root指根 右节点的父节点
    let root_value = middle[0];
    if middle.len() == 1 {
        return Some(Rc::new(RefCell::new(TreeNode::new(root_value))));
    }
    let right_len = get_index(back, &root_value).unwrap() + 1;
    let right = match right_child_len != 1 {
        true => generate_binary_tree_middle_back(&middle[1..right_len], &back[0..right_len - 1]),
        false => Some(Rc::new(RefCell::new(TreeNode::new(middle[1])))),
    };

    right
}

/**
 * 根节点对应中下标，后序前一个位置为左，去根的后序最后一个为又
 */
fn generate_binary_tree_middle_back2<T: Copy + PartialEq>(
    middle: &[T],
    back: &[T],
) -> Option<Rc<RefCell<TreeNode<T>>>> {
    let middle_len = middle.len();

    let mut vec = Vec::with_capacity(100);

    let mut divider_vec = VecDeque::with_capacity(middle_len);
    let mut root_vec = VecDeque::with_capacity(middle_len * 2);

    let root_value = *back.last().unwrap();
    let root = Some(Rc::new(RefCell::new(TreeNode::new(root_value))));
    vec.push(root_value);
    root_vec.push_back(root.clone());
    divider_vec.push_back(root_value);

    while root_vec.len() > 0 {
        let root_t = root_vec.pop_front().unwrap().clone().unwrap();

        let back_end_value = divider_vec.pop_front().unwrap();
        let back_end = get_index(&back, &back_end_value).unwrap();
        let divider = get_index(&middle, &back_end_value).unwrap();

        // 分割线小于尾部,代表后序中左根有子节点, 需要切换到后序左节点位置
        let mut divide = divider.min(back_end);

        let left_range = &middle[..divide];
        let left_value = *left_range.last().unwrap();
        root_t.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(left_value))));
        vec.push(left_value);
        if left_range.len() > 2 {
            root_vec.push_back(root_t.borrow().left.clone());
            divider_vec.push_back(left_value);
        }

        let right_range = &middle[divide..back_end];
        let right_value = *right_range.last().unwrap();
        root_t.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(right_value))));
        vec.push(right_value);
        if right_range.len() > 2 {
            root_vec.push_back(root_t.borrow().right.clone());
            divider_vec.push_back(right_value);
        }
    }
    let a = 1;
    root
}

#[test]
fn test_binary_tree() {
    let sign_num = 1;
    let sign = Rc::new(RefCell::new(TreeNode::new(sign_num)));
    let mut tree_sign: TreeNode<i32> = TreeNode::new(sign_num);
    tree_sign.left = Some(sign.clone());
    tree_sign.right = Some(sign.clone());
    let root = Some(Rc::new(RefCell::new(tree_sign)));
    let _is_eq = is_unit_val_tree(root);

    let mut tree1: TreeNode<char> = TreeNode::new('A');
    {
        let mut tree2: TreeNode<char> = TreeNode::new('B');
        let mut tree3: TreeNode<char> = TreeNode::new('C');
        let mut tree4: TreeNode<char> = TreeNode::new('D');
        let mut tree5: TreeNode<char> = TreeNode::new('E');
        let mut tree6: TreeNode<char> = TreeNode::new('F');
        let mut tree7: TreeNode<char> = TreeNode::new('G');
        let mut tree8: TreeNode<char> = TreeNode::new('H');
        let mut tree9: TreeNode<char> = TreeNode::new('I');

        tree6.right = Some(Rc::new(RefCell::new(tree9)));
        tree4.left = Some(Rc::new(RefCell::new(tree7)));
        tree4.right = Some(Rc::new(RefCell::new(tree8)));
        tree3.left = Some(Rc::new(RefCell::new(tree6)));
        tree2.left = Some(Rc::new(RefCell::new(tree4)));
        tree2.right = Some(Rc::new(RefCell::new(tree5)));
        tree1.left = Some(Rc::new(RefCell::new(tree2)));
        tree1.right = Some(Rc::new(RefCell::new(tree3)));
    }

    let root = Some(Rc::new(RefCell::new(tree1)));

    let result = inorder_traverse(root.clone());
    println!("{:?}", result);

    let arr = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I'];
    let result = levelOrder(root.clone());
    assert_eq!(arr, result.as_slice());

    let arr = ['A', 'C', 'B', 'D', 'E', 'F', 'I', 'H', 'G'];
    let result = levelOrder_z(root.clone());
    assert_eq!(arr, result.as_slice());

    let arr = ['A', 'B', 'D', 'G', 'H', 'E', 'C', 'F', 'I'];
    let result = levelOrder_pre_sequence(root.clone());
    assert_eq!(arr, result.as_slice());

    let arr = ['G', 'D', 'H', 'B', 'E', 'A', 'F', 'I', 'C'];
    let result = levelOrder_middle_sequence(root.clone());
    assert_eq!(arr, result.as_slice());

    let arr = ['G', 'H', 'D', 'E', 'B', 'I', 'F', 'C', 'A'];
    let result = levelOrder_end_sequence(root.clone());
    assert_eq!(arr, result.as_slice());

    let preorder = ['A', 'B', 'D', 'G', 'H', 'E', 'C', 'F', 'I'];
    let inorder = ['G', 'D', 'H', 'B', 'E', 'A', 'F', 'I', 'C'];
    let (len, result) = generate_binary_tree_pre_middle(&preorder, &inorder);
    let result = levelOrder_pre_sequence(result.clone());
    assert_eq!(preorder, result.as_slice());

    let inorder = ['G', 'D', 'H', 'B', 'E', 'A', 'F', 'I', 'C'];
    let preorder = ['G', 'H', 'D', 'E', 'B', 'I', 'F', 'C', 'A'];
    let result = generate_binary_tree_middle_back(&inorder, &preorder);
    let result = levelOrder_middle_sequence(result.clone());
    assert_eq!(inorder, result.as_slice());

    let inorder = [9, 3, 15, 20, 7];
    let preorder = [9, 15, 7, 20, 3];
    // let result = generate_binary_tree_middle_back2(&inorder, &preorder);
    // let result = levelOrder_middle_sequence(result.clone());
    // assert_eq!(inorder, result.as_slice());
}
