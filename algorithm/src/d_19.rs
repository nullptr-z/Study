//

use crate::list_utils::ListNode;

pub struct Solution;

impl Solution {
    /// 快指针先移动 N 次
    /// 然后快慢指针一起移动，块指针结束的时候，移除慢指针next即可
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, mut n: i32) -> Option<Box<ListNode>> {
        let binding = head.clone();
        let mut next = binding.as_ref();

        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;
        let mut cur = dummy.as_mut();

        while let Some(node) = next.as_ref() {
            if n == 0 {
                break;
            }
            next = node.next.as_ref();
            n -= 1;
        }

        while let Some(node) = next.as_ref() {
            next = node.next.as_ref();
            cur = cur.next.as_mut().unwrap();
        }
        cur.next = cur.next.as_mut().unwrap().next.take();

        dummy.next
    }

    //  写错了，这是删除指定下标
    pub fn remove_nth_from_ends(head: Option<Box<ListNode>>, mut n: i32) -> Option<Box<ListNode>> {
        let mut next = head.clone();
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;
        // let mut dummy = Box::new(dummy);
        let mut cur = dummy.as_mut();

        while let Some(mut node) = next.take() {
            if n == 0 {
                cur.next = node.next.take();
                break;
            }
            n -= 1;
            next = node.next;
            cur = cur.next.as_mut().unwrap();
        }

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use crate::list_utils::arrayToList;

    use super::Solution;

    #[test]
    fn should_work() {
        let res = Solution::remove_nth_from_end(arrayToList(vec![1, 2, 3, 4, 5]), 2);
        println!("【 res 】==> {:?}", res);
    }
}

// TODO: 双指针，快指针提前遍历n次，然后两个指针开始同步移动，
// 快指针到尾部时，慢指针就是需要删除的位置，
// 考虑让慢指针停在需要删除节点的前驱
