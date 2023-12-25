use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut map = HashMap::new();
        let mut heap = BinaryHeap::with_capacity(k);

        for v in nums {
            let cur = map.entry(v).or_insert(0);
            *cur += 1;
        }

        for v in map {
            if heap.len() >= k {
                if (Reverse(v.1), v.0) < *heap.peek().unwrap() {
                    heap.pop();
                }
            }
            if heap.len() < k {
                // 通过Reverse反转比较结果，将小顶堆变成大顶堆
                heap.push((Reverse(v.1), v.0));
            }
        }

        heap.into_iter().map(|v| v.1).collect()
    }

    pub fn top_k_frequent_map(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for v in nums {
            let cur = map.entry(v).or_insert(0);
            *cur += 1;
        }

        let mut result: Vec<(i32, i32)> = map.into_iter().collect();
        result.sort_by(|f, b| b.1.cmp(&f.1));
        let result: Vec<i32> = result[0..(k as usize)].into_iter().map(|m| m.0).collect();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::top_k_frequent(vec![5, 5, 6, 6, 6, 7, 7, 7, 7], 2);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
