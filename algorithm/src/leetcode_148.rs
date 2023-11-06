use crate::list_utils::ListNode;

// Leetcoode速度快的都是现将链表转换为了数组，虽然速度更快，但是不符合考察的题意
impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let ret = Self::sort_lists(&head);
        println!("【 ret 】==> {:?}", ret);

        ret
    }

    fn sort_lists(head: &Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head.clone();
        }

        let mut fast = head;
        let mut head = head.clone();
        let mut slow = &mut head;

        while let Some(next) = fast {
            fast = &next.next;
            if let Some(nn) = fast {
                fast = &nn.next;
                if let Some(nn) = slow {
                    slow = &mut nn.next;
                }
            }
        }
        let mid = slow.take();

        let left = Self::sort_lists(&head);
        let right = Self::sort_lists(&mid);
        Solution::merge_two_lists(left, right)
    }

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
                list.next = Some(Box::new(ListNode::new(o.val)));
                one = &o.next;
            } else {
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
}

#[cfg(test)]
mod tests {
    use crate::list_utils::arrayToList;

    use super::Solution;

    #[test]
    fn should_work() {
        Solution::sort_list(arrayToList(vec![4, 2, 1, 3]));
    }
}

pub struct Solution;
