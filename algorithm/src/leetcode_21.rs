use crate::list_utils::ListNode;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut new_list = Box::new(ListNode::new(0));
        let mut list = &mut new_list;
        let mut one = &list1;
        let mut two = &list2;

        while let (Some(o), Some(t)) = (one, two) {
            if o.val <= t.val {
                // 原地修改，新建next = 0
                // node.next = Some(Box::new(ListNode::new(0)));
                // node.val = o;
                // list = &mut node.next;
                list.next = Some(Box::new(ListNode::new(o.val)));
                one = &o.next;
            } else {
                // 原地修改，新建next = 0
                // node.next = Some(Box::new(ListNode::new(0)));
                // node.val = t;
                // list = &mut node.next;
                list.next = Some(Box::new(ListNode::new(t.val)));
                two = &t.next;
            }
            list = list.next.as_mut().unwrap();
        }

        list.next = match one.is_some() {
            true => one.to_owned(),
            false => two.to_owned(),
        };

        // *list = None; //如果使用了`原地修改`的方案,需要把最后`新建next = 0`清除
        new_list.next
    }

    /// 递归版，更清晰
    fn merge_two_lists_rec(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(n1), None) => Some(n1),
            (None, Some(n2)) => Some(n2),
            (Some(mut n1), Some(mut n2)) => {
                if n1.val < n2.val {
                    let n1_next = n1.next.take();
                    let merged = Self::merge_two_lists(n1_next, Some(n2));
                    Some(Box::new(ListNode {
                        val: n1.val,
                        next: merged,
                    }))
                } else {
                    let n2_next = n2.next.take();
                    let merged = Self::merge_two_lists(Some(n1), n2_next);
                    Some(Box::new(ListNode {
                        val: n2.val,
                        next: merged,
                    }))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::list_utils::arrayToList;

    use super::Solution;

    #[test]
    fn should_work() {
        Solution::merge_two_lists(arrayToList(vec![1, 2, 4]), arrayToList(vec![1, 3, 4]));
    }
}

pub struct Solution;
