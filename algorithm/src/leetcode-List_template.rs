impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::list_utils::arrayToList;

    #[test]
    fn should_work() {
        Solution::sort_list(arrayToList(vec![]));
    }
}

pub struct Solution;
use crate::list_utils::ListNode;
