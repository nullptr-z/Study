use crate::list_utils::ListNode;

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 0 || head.is_none() {
            return head;
        }

        let mut len: i32 = 0;
        let mut head = head;
        let mut current = &mut head;
        // 拿到链表长度
        while let Some(node) = current {
            len += 1;
            current = &mut node.next;
        }
        // 处理很大的k；长度等于链表长度的时候不需要旋转
        let k = k % len;
        if k == 0 {
            return head;
        }

        // 取出后半部分
        let mut current = &mut head;
        let mut i = len - k;
        while let Some(ref mut node) = current {
            current = &mut node.next;
            i -= 1;
            if i <= 0 {
                break;
            }
        }

        // 将后半部分移动到尾部，链上前半部分，完成旋转
        let mut latter = current.take();
        let mut current = &mut latter;
        while let Some(ref mut node) = current {
            if node.next.is_none() {
                node.next = head;
                break;
            } else {
                current = &mut node.next;
            }
        }
        // println!("【 latter 】==> {:?}", latter);

        latter
    }
}

#[cfg(test)]
mod tests {
    use crate::list_utils::arrayToList;

    use super::Solution;

    #[test]
    fn should_work() {
        Solution::rotate_right(arrayToList(vec![1, 2, 3, 4, 5]), 2);
        Solution::rotate_right(arrayToList(vec![1, 2]), 1);
    }
}

pub struct Solution;
