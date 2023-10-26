#[derive(Debug)]
struct LRUCache {
    cache: Vec<Option<(i32, i32)>>,
    operator_count: i32,
    capacity: i32,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            cache: vec![None; 100001],
            operator_count: 0,
            capacity: capacity,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        self.operator_count += 1;

        if let Some(item) = self.cache[key as usize] {
            self.cache[key as usize] = Some((item.0, self.operator_count));
            return item.0;
        }

        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        self.operator_count += 1;

        // let that = &mut self.cache[key as usize];
        if self.cache[key as usize].is_some() {
            // that.unwrap().0 = value;
            // that.unwrap().1 = self;
            self.cache[key as usize] = Some((value, self.operator_count));
            return;
        }
        if self.capacity <= 0 {
            let mut min = (i32::MAX, i32::MAX);
            let mut idx: usize = 0;
            for (i, item) in self.cache.iter().enumerate() {
                if let Some(m) = item {
                    if m.1 < min.1 {
                        min = m.clone();
                        idx = i
                    }
                }
            }
            self.cache[idx] = None;
            self.capacity += 1;
        }

        self.capacity -= 1;
        self.cache[key as usize] = Some((value, self.operator_count))
    }
}

#[cfg(test)]
mod tests {
    use super::LRUCache;

    #[test]
    fn should_work() {
        let mut obj = LRUCache::new(2);
        obj.put(1, 1);
        obj.put(2, 2);
        let r = obj.get(1);
        println!("【 r 】==> {:?}", r);
        obj.put(3, 3);
        let r = obj.get(2);
        println!("【 r 】==> {:?}", r);
        obj.put(4, 4);
        let r = obj.get(1);
        println!("【 r 】==> {:?}", r);
        let r = obj.get(3);
        println!("【 r 】==> {:?}", r);
        let r = obj.get(4);
        println!("【 r 】==> {:?}", r);

        println!("【 obj 】==> {:?}", obj);
    }

    #[test]
    fn should_work2() {
        let mut obj = LRUCache::new(2);
        obj.put(1, 0);
        obj.put(2, 2);
        let r = obj.get(1);
        println!("【 r 】==> {:?}", r);
        obj.put(3, 3);
        let r = obj.get(2);
        println!("【 r 】==> {:?}", r);
        println!("【 obj 】==> {:?}", obj);
        obj.put(4, 4);
        println!("【 obj 】==> {:?}", obj);
        let r = obj.get(1);
        println!("【 r 】==> {:?}", r);
        let r = obj.get(3);
        println!("【 r 】==> {:?}", r);
        let r = obj.get(4);
        println!("【 r 】==> {:?}", r);

        println!("【 obj 】==> {:?}", obj);
    }
}

pub struct Solution;
