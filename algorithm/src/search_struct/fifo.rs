pub struct FIFO<T> {
    inner: Vec<T>,
    start_ptr: usize,
    end_ptr: usize,
}

impl<T: Clone> FIFO<T> {
    pub fn push(&mut self, item: T) {
        self.inner.push(item);
        self.end_ptr += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len() > 0 {
            let temp = self.inner[self.start_ptr].clone();
            self.inner = self.inner[self.start_ptr + 1..].to_vec();
            self.end_ptr -= 1;
            Some(temp)
        } else {
            None
        }
    }

    pub fn front(&self) -> Option<&T> {
        match self.len() > 0 {
            true => Some(&self.inner[self.start_ptr]),
            false => None,
        }
    }

    pub fn back(&self) -> Option<&T> {
        match self.len() > 0 {
            true => Some(&self.inner[self.end_ptr]),
            false => None,
        }
    }

    pub fn len(&self) -> usize {
        self.end_ptr - self.start_ptr
    }
}

impl<T> FIFO<T> {
    pub fn with_capacity(wite: usize) -> FIFO<T> {
        Self {
            inner: Vec::with_capacity(wite),
            start_ptr: 0,
            end_ptr: 0,
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_fifo_struct() {}
}
