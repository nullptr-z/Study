use std::mem::swap;

struct CQueue {
    data: Vec<i32>,
    cache: Vec<i32>,
}

/**
* `&self` means the method takes an immutable reference.
* If you need a mutable reference, change it to `&mut self` instead.
*/
impl CQueue {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            cache: Vec::new(),
        }
    }

    fn append_tail(&mut self, value: i32) {
        self.cache.push(value);
    }

    fn delete_head(&mut self) -> i32 {
        if self.data.len() > 0 {
            return self.data.pop().unwrap();
        }

        let case_len = self.cache.len();
        if case_len < 1 {
            return -1;
        }
        // self.cache.as_mut_slice().reverse();
        // swap(&mut self.data, &mut self.cache)
        for item in 0..case_len - 1 {
            self.data.push(self.cache.pop().unwrap())
        }
        self.cache.pop().unwrap()
    }
}

/**
* Your CQueue object will be instantiated and called as such:
* let obj = CQueue::new();
* obj.append_tail(value);
* let ret_2: i32 = obj.delete_head();
*/

#[cfg(test)]
mod tests {
    use super::CQueue;

    #[test]
    fn defaulat() {
        let mut obj = CQueue::new();
        obj.append_tail(1);
        obj.append_tail(2);
        obj.append_tail(3);
        let ret_2: i32 = obj.delete_head();
        assert_eq!(ret_2, 1);
        let ret_2: i32 = obj.delete_head();
        assert_eq!(ret_2, 2);
        let ret_2: i32 = obj.delete_head();
        assert_eq!(ret_2, 3);
    }
}
