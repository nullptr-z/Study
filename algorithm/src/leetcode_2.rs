// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn add_two_numbers_s(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // let list: Option<Box<ListNode>> = None;

        let mut node1 = l1;
        let mut node2 = l2;

        let mut cf = 0;
        let mut result = vec![cf];

        loop {
            let mut is_end = true;
            let val1 = match node1 {
                Some(v) => {
                    is_end = false;
                    node1 = v.next.clone();
                    (*v).val
                }
                None => 0,
            };
            let val2 = match node2 {
                Some(v) => {
                    is_end = false;
                    node2 = v.next.clone();
                    (*v).val
                }
                None => 0,
            };
            if is_end {
                if cf == 0 {
                    result.pop();
                }
                break;
            }
            let sum = val1 + val2 + result.pop().unwrap();
            if sum >= 10 {
                cf = 1;
                result.push(sum % 10);
            } else {
                cf = 0;
                result.push(sum);
            }
            result.push(cf)
        }

        println!("【 result 】==> {:?}", result);
        let sum_list = arrayToList(result);

        sum_list
    }

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut carry = 0;
        let mut dummy = Box::new(ListNode::new(0));
        let mut current = &mut dummy;

        while l1.is_some() || l2.is_some() || carry > 0 {
            let mut sum = carry;

            if let Some(node) = l1.take() {
                sum += node.val;
                l1 = node.next;
            }

            if let Some(node) = l2.take() {
                sum += node.val;
                l2 = node.next;
            }

            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();
        }

        dummy.next
    }
}

fn arrayToList(array: Vec<i32>) -> Option<Box<ListNode>> {
    let mut node: Option<Box<ListNode>> = None;

    for item in array.into_iter().rev() {
        let mut list: ListNode = ListNode::new(item);
        list.next = node;
        node = Some(Box::new(list))
    }

    node
}

#[cfg(test)]
mod tests {
    use crate::leetcode_2::arrayToList;

    use super::ListNode;
    use super::Solution;

    #[test]
    fn test_add_two_numbers() {
        let list11 = arrayToList(vec![2, 4, 3]);
        let list2 = arrayToList(vec![2, 4, 3]);
        let result = Solution::add_two_numbers(list11, list2);
        println!("【 result 】==> {:?}", result);
    }

    #[test]
    fn add_two_numbers_should_work() {
        let mut a: Option<i32> = Some(1);

        while let Some(aa) = a {
            println!("【 as 】==> {:?}", aa);
            a = None;
        }
        // Solution::add_two_numbers()
    }
}

pub struct Solution;
