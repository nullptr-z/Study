use std::collections::BTreeMap;

#[derive(Debug)]
struct LRUCache {
    cache: Vec<Option<(i32, i32)>>,
    activity_level: BTreeMap<i32, i32>,
    operator_count: i32,
    capacity: i32,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            // 考虑Hash表和有序结构
            // 一般最优选择是hash + DLink
            cache: vec![None; 100001],
            activity_level: BTreeMap::new(),
            operator_count: 0,
            capacity: capacity,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        self.operator_count += 1;

        if let Some(item) = self.cache[key as usize] {
            self.activity_level
                .remove(&self.cache[key as usize].unwrap().1);
            self.activity_level.insert(self.operator_count, key);
            self.cache[key as usize] = Some((item.0, self.operator_count));
            return item.0;
        }

        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        self.operator_count += 1;

        if self.cache[key as usize].is_some() {
            self.activity_level
                .remove(&self.cache[key as usize].unwrap().1);
            self.activity_level.insert(self.operator_count, key);
            self.cache[key as usize] = Some((value, self.operator_count));
            return;
        }
        if self.capacity <= 0 {
            // if let Some((key, value)) = self.activity_level.first_key_value() {
            //     self.activity_level.remove_entry(key);
            //     self.cache[*value as usize] = None;
            //     self.capacity += 1;
            // }

            let f = self.activity_level.iter().next();
            let k = *f.unwrap().0;
            let v = *f.unwrap().1;
            self.activity_level.remove(&k);
            self.cache[v as usize] = None;
        }

        self.capacity -= 1;
        self.activity_level.insert(self.operator_count, key);
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
        obj.put(4, 4);
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
