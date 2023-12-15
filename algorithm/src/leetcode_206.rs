impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = None;
        let mut cur = head.take();

        while let Some(mut node) = cur.take() {
            cur = node.next.take();
            node.next = dummy;
            dummy = Some(node);
        }

        dummy
    }

    pub fn reverse_list_recursion(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn recursion(
            head: Option<Box<ListNode>>,
            pre: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            if let Some(mut node) = head {
                let next = node.next.take();
                node.next = pre;
                return recursion(next, Some(node));
            }

            pre
        }

        recursion(head, None)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::list_utils::arrayToList;

    #[test]
    fn should_work() {
        let res = Solution::reverse_list_recursion(arrayToList(vec![1, 2, 3, 4, 5]));
        println!("【 ret 】==> {:?}", res);
    }
}

pub struct Solution;
use crate::list_utils::ListNode;
