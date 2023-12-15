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
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::list_utils::arrayToList;

    #[test]
    fn should_work() {
        let res = Solution::reverse_list(arrayToList(vec![1, 2, 3, 4, 5]));
        println!("【 ret 】==> {:?}", res);
    }
}

pub struct Solution;
use crate::list_utils::ListNode;
