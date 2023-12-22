struct MyQueue {
    stack: Vec<i32>,
    l: usize,
    r: usize,
}

impl MyQueue {
    fn new() -> Self {
        Self {
            stack: vec![],
            l: 0,
            r: 0,
        }
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x);
        self.r += 1;
    }

    fn pop(&mut self) -> i32 {
        if self.l == self.r {
            return -1;
        }
        let val = self.stack[self.l];
        self.l += 1;

        val
    }

    fn peek(&self) -> i32 {
        if self.l == self.r {
            return -1;
        }
        self.stack[self.l]
    }

    fn empty(&self) -> bool {
        self.r == self.l
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
#[cfg(test)]
mod tests {
    use super::MyQueue;

    #[test]
    fn should_work() {
        let mut obj = MyQueue::new();
        obj.push(1);
        let ret_2: i32 = obj.pop();
        let ret_3: i32 = obj.peek();
        let ret_4: bool = obj.empty();
    }
}

pub struct Solution;
