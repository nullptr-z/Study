use std::collections::VecDeque;

#[derive(Default)]
struct Queue(VecDeque<i32>);

impl Queue {
    fn push(&mut self, x: i32) {
        self.0.push_back(x)
    }

    fn pop(&mut self) -> i32 {
        match self.0.pop_front() {
            Some(v) => v,
            None => -1,
        }
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Default)]
struct MyStack {
    queue: Queue,
}

impl MyStack {
    fn new() -> Self {
        Self {
            queue: Default::default(),
        }
    }

    fn push(&mut self, x: i32) {
        self.queue.push(x);
    }

    fn pop(&mut self) -> i32 {
        let mut len = self.queue.len() - 1;
        let mut top = -1;
        while len > 0 {
            top = self.queue.pop();
            self.push(top);
            len -= 1;
        }
        top = self.queue.pop();

        top
    }

    fn top(&mut self) -> i32 {
        let top = self.pop();
        self.push(top);

        top
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::MyStack;

    #[test]
    fn should_work() {
        let mut obj = MyStack::new();
        obj.push(1);
        let ret_2: i32 = obj.pop();
        println!("【 ret_2 】==> {:?}", ret_2);
        let ret_3: i32 = obj.top();
        let ret_4: bool = obj.empty();
    }
}

pub struct Solution;
