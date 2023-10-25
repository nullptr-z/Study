//

use crate::list_utils::ListNode;

impl Solution {
    // 1.先把left之前的放进dummy
    // 2.内层循环移动到right位置, 把right后面的节点拼接到第一个left后面(这个left就是反转后的right)
    // 3.退出内层循环，开始反转left..right，反转的方法就是把dummy放在，当前迭代位置cur的next上
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut heads = head;
        let mut dummy = Box::new(ListNode::new(0));
        let mut current = &mut dummy;
        let mut node_middle = None;

        let mut i = 0;
        while i < right {
            if let Some(node) = heads.take() {
                if i >= (left - 1) {
                    let mut temp = node.clone();
                    temp.next = match node_middle {
                        Some(v) => Some(v),
                        None => {
                            let mut j = 0;
                            // 这里大量迭代只是为了跳过不需要的节点，但是全部clone了，感觉划不着
                            let mut temp = Some(node.clone());
                            while j < (right - i) {
                                temp = temp.unwrap().next;
                                j += 1;
                            }
                            temp
                        }
                    };
                    node_middle = Some(temp);

                    if (i + 1) == right {
                        current.next = node_middle;
                        break;
                    }
                } else {
                    current.next = Some(Box::new(ListNode::new(node.val)));
                    current = current.next.as_mut().unwrap();
                }
                heads = node.next;
            }
            i += 1;
        }

        // println!("【 dummy 】==> {:?}", dummy.next);
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use crate::list_utils::arrayToList;

    use super::Solution;

    #[test]
    fn should_work() {
        Solution::reverse_between(arrayToList(vec![1, 2, 3, 4, 5]), 2, 4);
        Solution::reverse_between(arrayToList(vec![5]), 1, 1);
        Solution::reverse_between(arrayToList(vec![1, 2, 3]), 1, 1);
        Solution::reverse_between(arrayToList(vec![1, 2, 3, 4, 5]), 2, 2);
    }
}

pub struct Solution;
