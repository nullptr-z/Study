use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};

pub struct SingleQueue(pub VecDeque<i32>);

impl SingleQueue {
    pub fn new() -> Self {
        Self(VecDeque::<i32>::new())
    }

    pub fn push(&mut self, v: i32) {
        while let Some(b) = self.back() {
            if v > *b {
                self.pop_back();
            } else {
                break;
            }
        }
        self.push_back(v);
    }

    pub fn pop(&mut self, v: i32) {
        if Some(v) == self.front().copied() {
            self.pop_front();
        }
    }

    pub fn get_max(&self) -> i32 {
        self.front().copied().unwrap()
    }
}

impl Deref for SingleQueue {
    type Target = VecDeque<i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SingleQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
