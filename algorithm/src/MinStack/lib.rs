struct MinStack {
    data: Vec<i32>,
    min: Option<i32>,
    min_count: u32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        Self {
            data: Vec::new(),
            min: None,
            min_count: 0,
        }
    }

    fn push(&mut self, x: i32) {
        self.data.push(x);
        self.change_min(x);
    }

    fn pop(&mut self) {
        if self.data.len() > 0 {
            let pop_val = self.data.pop().unwrap();
            if pop_val == self.min.unwrap() {
                self.min_count -= 1;
                if self.min_count == 0 {
                    self.item_clear();
                }
            }
        }
    }

    fn item_clear(&mut self) {
        self.min = None;
        for itm in &self.data {
            // self.change_min(*itm);
            let item = *itm;
            if self.min.is_none() || self.min.unwrap() > item {
                self.min = Some(item);
                self.min_count = 1;
            } else if self.min.unwrap() == item {
                self.min_count += 1;
            }
        }
    }

    fn change_min(&mut self, item: i32) {
        if self.min.is_none() || self.min.unwrap() > item {
            self.min = Some(item);
            self.min_count = 1;
        } else if self.min.unwrap() == item {
            self.min_count += 1;
        }
    }

    fn top(&self) -> i32 {
        self.data.last().unwrap().clone()
    }

    fn min(&self) -> i32 {
        self.min.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::MinStack;

    #[test]
    fn min_statck() {
        // ["MinStack","push","push","push","top","pop","min","pop","min","pop","push","top","min","push","top","min","pop","min"]
        // [[],[],[],[],[],[],[],[],[],[],[],[],[],[-2147483648],[],[],[],[]]
        let mut minStack = MinStack::new();
        minStack.push(2147483646);
        minStack.push(2147483646);
        minStack.push(2147483647);
        minStack.top();
        minStack.pop();
        minStack.min();
        minStack.pop();
        minStack.min();
        minStack.pop();
        minStack.push(2147483647);
        minStack.top();
        let aa = minStack.min();
        minStack.push(-2147483648);
        minStack.top();
        minStack.min();
        minStack.pop();
        minStack.min();
    }
}
