use crate::list_utils::ListNode;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut dummy_head = ListNode {
            val: 0,
            next: head.take(),
        };
        let mut current = &mut dummy_head;

        while current.next.as_ref().is_some() && current.next.as_ref().unwrap().next.is_some() {
            if current.next.as_ref().unwrap().val
                == current.next.as_ref().unwrap().next.as_ref().unwrap().val
            {
                let x = current.next.as_ref().unwrap().val;
                while current.next.is_some() && current.next.as_ref().unwrap().val == x {
                    current.next = current.next.as_mut().unwrap().next.take();
                }
            } else {
                current = current.next.as_mut().unwrap();
            }
        }
        println!("【 dummy_head 】==> {:?}", dummy_head);

        dummy_head.next
    }
}

#[cfg(test)]
mod tests {
    use crate::list_utils::arrayToList;

    use super::Solution;

    #[test]
    fn should_work() {
        Solution::delete_duplicates(arrayToList(vec![1, 2, 3, 3, 4, 4, 5]));
    }
}

pub struct Solution;
