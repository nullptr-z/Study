impl Solution {
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        // 创建一个虚拟头节点，简化删除操作
        let mut dummy = Some(Box::new(ListNode { val: 0, next: None }));
        let mut current = &mut dummy;

        while let Some(mut node) = head.take() {
            head = node.next.take();
            if node.val != val {
                current.as_mut().unwrap().next = Some(node);
                current = &mut current.as_mut().unwrap().next;
            }
        }

        // 返回删除后的链表头节点
        dummy.unwrap().next
    }

    pub fn remove_elements2(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummyHead = Box::new(ListNode::new(0));
        dummyHead.next = head;
        let mut cur = dummyHead.as_mut();
        // 使用take()替换std::men::replace(&mut node.next, None)达到相同的效果，并且更普遍易读
        while let Some(nxt) = cur.next.take() {
            if nxt.val == val {
                cur.next = nxt.next;
            } else {
                cur.next = Some(nxt);
                cur = cur.next.as_mut().unwrap();
            }
        }
        dummyHead.next
    }
}

#[cfg(test)]
mod tests {
    use crate::list_utils::arrayToList;

    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::remove_elements(arrayToList(vec![1, 2, 6, 3, 4, 5, 6].into()), 6);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
use crate::list_utils::ListNode;
