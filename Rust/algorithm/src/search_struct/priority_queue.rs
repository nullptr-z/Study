use std::{borrow::BorrowMut, mem::swap};

pub enum PRIORITY {
    MaxHeap = 1,
    MinHeap = 2,
}

pub struct PriorityQueue<T> {
    inner: Vec<T>,
    /** 深度 */
    heap_depth: usize,
    /** 优先级 */
    priority: PRIORITY,
    /** 容量 */
    heap_size: usize,
}

impl<T: Clone + PartialOrd> PriorityQueue<T> {
    pub fn new(vec: Vec<T>, priority: PRIORITY) -> Self {
        let len = vec.len();
        let mut init = Self {
            inner: PriorityQueue::default().inner,
            // inner: Vec::with_capacity(len),
            heap_depth: f32::log2(len as f32) as usize,
            priority,
            heap_size: len,
        };

        for i in 0..len {
            init.inner.push(vec[i].clone());
            init.up(i);
        }

        init
    }

    fn up(&mut self, mut that_index: usize) {
        while that_index > 0 {
            let pre_index = self.get_index_by_prev(that_index);

            match self.compare(that_index, pre_index) {
                true => {
                    self.swap(that_index, pre_index);
                    that_index = pre_index;
                }
                false => break,
            }
        }
    }

    fn down(&mut self, mut that_index: usize) {
        while self.get_index_by_heap_depth(that_index) < self.heap_depth {
            let next_index = that_index * 2 + 1;
            if next_index >= self.len() {
                return;
            }
            let next_right = next_index + 1;

            let compar_l_r = next_right < self.len() && self.compare(next_right, next_index);
            match compar_l_r && self.compare(next_right, that_index) {
                true => {
                    self.swap(next_right, that_index);
                    that_index = next_right;
                }
                false => match self.compare(next_index, that_index) {
                    true => {
                        self.swap(next_index, that_index);
                        that_index = next_index;
                    }
                    false => break,
                },
            }
        }
    }

    fn compare(&self, that_index: usize, pre_index: usize) -> bool {
        match self.priority {
            PRIORITY::MaxHeap => self.inner[that_index] > self.inner[pre_index],
            PRIORITY::MinHeap => self.inner[that_index] < self.inner[pre_index],
        }
    }

    fn get_index_by_heap_depth(&self, index: usize) -> usize {
        f32::log2(index as f32) as usize
    }

    pub fn get_index_by_prev(&self, index: usize) -> usize {
        (index - 1) / 2
    }

    pub fn get_index_by_item(&self, index: usize) -> &T {
        &self.inner[index]
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    fn swap(&mut self, that_index: usize, swap_index: usize) {
        let temp = self.inner[that_index].clone();
        self.inner[that_index] = self.inner[swap_index].clone();
        self.inner[swap_index] = temp;
    }
}

impl<T: Clone + PartialOrd> PriorityQueue<T> {
    pub fn empty(&self) -> bool // 如果优先队列为空，返回真
    {
        self.heap_size == 0
    }
    pub fn pop(&mut self) -> Option<T> // 弹出第一个元素
    {
        let len = self.inner.len();
        if len > 0 {
            self.swap(0, len - 1);
            let pop = self.inner.pop();
            self.heap_depth = self.get_index_by_heap_depth(len - 1);
            self.down(0);
            return pop;
        }
        None
    }
    pub fn push(&mut self, item: T) // 添加一个元素x
    {
        self.inner.push(item);
        let len = self.inner.len();
        self.heap_depth = self.get_index_by_heap_depth(len - 1);
        self.up(len - 1);
    }
    pub fn size() // 返回优先队列中拥有的元素的个数
    {
        todo!()
    }
    pub fn top() // 返回优先队列中有最高优先级的元素
    {
        todo!()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut default = PriorityQueue::default();
        default.inner = Vec::with_capacity(capacity);
        default
    }
}

impl<T> Default for PriorityQueue<T> {
    fn default() -> Self {
        Self {
            inner: Vec::with_capacity(0),
            heap_depth: 0,
            priority: PRIORITY::MaxHeap,
            heap_size: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::PriorityQueue;

    #[test]
    fn test_priority_queue() {
        let array = vec![3, 5, 8, 9, 1, 2, 11, 4, 6, 12, 30];
        // 其父节点:dad = (i - 1) / 2
        // 左/右子节点:left = 2 * i + 1
        // right = 2 * i + 2
        //                 30
        //         12              9
        //     6       11      2       5
        // 3       4 1    8

        let mut pq = PriorityQueue::new(array, super::PRIORITY::MaxHeap);

        let pop = pq.pop().unwrap();
        let pop = pq.pop().unwrap();
        let pop = pq.pop().unwrap();
        let pop = pq.pop().unwrap();
        let pop = pq.pop().unwrap();
        let pop = pq.pop().unwrap();
        let pop = pq.pop().unwrap();
        let pop = pq.pop().unwrap();
        let pop = pq.pop().unwrap();
        let pop = pq.pop().unwrap();
        let pop = pq.pop().unwrap();
        pq.push(pop);
        assert_eq!(pop, 1);
    }
}
