//

use crate::list_utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut head_ptr = head;
        let mut current = &mut head_ptr;

        while let Some(node) = current {
            current = &mut node.next;
            len += 1;
        }

        let end = len - n;
        let mut dummy_head = ListNode {
            val: 0,
            next: head_ptr.take(),
        };
        let mut current = &mut dummy_head;

        let mut i = 0;
        while let Some(ref mut node) = current.next {
            if i == end {
                current.next = node.next.take();
                break;
            } else {
                current = current.next.as_mut().unwrap();
            }
            i += 1;
        }

        dummy_head.next
    }

    // TODO: 双指针，快指针提前遍历n次，然后两个指针开始同步移动，
    // 快指针到尾部时，慢指针就是需要删除的位置，
    // 考虑让慢指针停在需要删除节点的前驱
}

#[cfg(test)]
mod tests {
    use crate::list_utils::arrayToList;

    use super::Solution;

    #[test]
    fn should_work() {
        Solution::remove_nth_from_end(arrayToList(vec![1, 2, 3, 4, 5]), 2);
    }
}
