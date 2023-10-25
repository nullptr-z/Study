#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl From<Vec<i32>> for ListNode {
    fn from(value: Vec<i32>) -> Self {
        let that = arrayToList(value).to_owned().unwrap();

        *that
    }
}

pub fn arrayToList(array: Vec<i32>) -> Option<Box<ListNode>> {
    let mut node: Option<Box<ListNode>> = None;

    for item in array.into_iter().rev() {
        let mut list: ListNode = ListNode::new(item);
        list.next = node;
        node = Some(Box::new(list))
    }

    node
}
