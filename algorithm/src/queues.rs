use std::collections::VecDeque;
#[derive(Default, Debug)]
pub struct Queue(pub VecDeque<i32>);

impl Queue {
    pub fn push(&mut self, x: i32) {
        self.0.push_back(x)
    }

    pub fn pop(&mut self) -> i32 {
        match self.0.pop_front() {
            Some(v) => v,
            None => -1,
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
