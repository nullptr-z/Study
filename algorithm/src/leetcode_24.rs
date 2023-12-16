impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;
        let mut cur = dummy.as_mut();

        while let Some(mut node) = cur.next.take() {
            // node是第一个节点
            // next是第二个节点
            if let Some(mut next) = node.next.take() {
                // 将第三个节点挂在第一个节点上
                node.next = next.next.take();
                // 将第一个节点挂在第二个节点上
                next.next = Some(node);
                // 第一个节点（交换前的二）挂在虚节点上
                cur.next = Some(next);
                // 完成交换，向后移动2位，第二个节点充当虚节点
                cur = cur.next.as_mut().unwrap().next.as_mut().unwrap();
            } else {
                // 循环还剩一个的时候也会被take()，需要链上它
                cur.next = Some(node);
                // 这一步是为了防止产生环
                cur = cur.next.as_mut().take().unwrap();
            }
        }

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::list_utils::arrayToList;

    #[test]
    fn should_work() {
        let res = Solution::swap_pairs(arrayToList(vec![]));
        println!("【 ret 】==> {:?}", res);
    }
}

pub struct Solution;
use crate::list_utils::ListNode;
