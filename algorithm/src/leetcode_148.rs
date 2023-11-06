use crate::list_utils::ListNode;

impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::find_middle(&head);

        head
    }

    fn find_middle(head: &Option<Box<ListNode>>) {
        let mut mid = head;
        let mut flash = head;

        while let Some(next) = flash {
            if let Some(nn) = &next.next {
                mid = &mid.as_ref().unwrap().next;
                flash = &nn.next;
            }
        }
        let left = mid.to_owned().take();
        println!("【 left 】==> {:?}", head);
        println!("【 left 】==> {:?}", left);
        println!("【 mid 】==> {:?}", mid);
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
