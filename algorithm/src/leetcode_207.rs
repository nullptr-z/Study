impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut header: Box<ListNode> = Box::new(ListNode { val: 0, next: head });
        let mut current: &mut Box<ListNode> = &mut header;

        'outer: loop {
            let value = match &current.next {
                None => break 'outer,
                Some(n) => n.val,
            };
            if value == val {
                current.next = current.next.as_mut().unwrap().next.take();
            } else {
                current = current.next.as_mut().unwrap();
            }
        }

        header.next
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
