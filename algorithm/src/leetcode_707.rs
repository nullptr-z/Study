use std::cell::RefCell;
use std::rc::Rc;

/// 实际上这道题不需要，首尾指针
/// 每个函数当成独立的一道题，各自实现即可
struct MyLinkedList {
    n: i32,
    head: Rc<RefCell<MyLinkedListNode>>,
    tail: Rc<RefCell<MyLinkedListNode>>,
}

struct MyLinkedListNode {
    val: i32,
    next: Option<Rc<RefCell<MyLinkedListNode>>>,
}

impl MyLinkedList {
    fn new() -> Self {
        let v_node = Rc::new(RefCell::new(MyLinkedListNode { val: 0, next: None }));
        Self {
            n: 0,
            head: v_node.clone(),
            tail: v_node.clone(),
        }
    }

    fn get(&self, index: i32) -> i32 {
        if index < 0 || index >= self.n {
            return -1;
        }
        let mut pre = self.head.borrow().next.clone();
        for _ in 0..index {
            pre = pre.and_then(|node| node.borrow().next.clone());
        }
        pre.map(|node| node.borrow().val).unwrap()
    }

    fn add_at_head(&mut self, val: i32) {
        self.add_at_index(0, val);
    }

    fn add_at_tail(&mut self, val: i32) {
        let new_node = Rc::new(RefCell::new(MyLinkedListNode { val, next: None }));
        self.tail.borrow_mut().next = Some(new_node.clone());
        self.tail = new_node;
        self.n += 1;
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index < 0 || index > self.n {
            return;
        }
        if index == self.n {
            self.add_at_tail(val);
            return;
        }

        let mut pre = Some(self.head.clone());
        for _ in 0..index {
            pre = pre.and_then(|node| node.borrow().next.clone());
        }
        let next = pre.clone().and_then(|node| node.borrow().next.clone());
        let new = Some(Rc::new(RefCell::new(MyLinkedListNode { val, next })));
        pre.map(|node| node.borrow_mut().next = new);
        self.n += 1;
    }

    fn delete_at_index(&mut self, index: i32) {
        if index < 0 || index >= self.n {
            return;
        }
        let mut pre = Some(self.head.clone());
        for _ in 0..index {
            pre = pre.and_then(|node| node.borrow().next.clone());
        }
        let next = pre
            .clone()
            .and_then(|node| node.borrow().next.clone())
            .and_then(|node| node.borrow().next.clone());
        pre.clone().map(|node| node.borrow_mut().next = next);
        if index == self.n - 1 {
            self.tail = pre.unwrap();
        }
        self.n -= 1;
    }
}

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */
#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::list_utils::arrayToList;

    #[test]
    fn should_work() {}
}

pub struct Solution;
use crate::list_utils::ListNode;
