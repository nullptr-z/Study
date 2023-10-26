use crate::list_utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
        match head {
            Some(node) => {
                let ve = Solution::reverse_print(node.next);
                [ve, vec![node.val]].concat()
            }
            None => vec![],
        }

        // if head.is_none() {
        //     return vec![];
        // }
        // let root = head.unwrap();
        // let v1 = vec![root.val];
        // if root.next.is_some() {
        //     let ve = Solution::reverse_print(root.next);
        //     [ve, v1].concat()
        // } else {
        //     v1
        // }
    }
}

#[cfg(test)]

mod tests {
    use super::{ListNode, Solution};

    #[test]
    fn oofer06() {
        let mut v1 = ListNode::new(1);
        let mut v2 = ListNode::new(3);
        let v3 = ListNode::new(2);
        v2.next = Some(Box::new(v3));
        v1.next = Some(Box::new(v2));

        let vec = Solution::reverse_print(Some(Box::new(v1)));

        assert_eq!(vec, [2, 3, 1])
    }
}
