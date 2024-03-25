use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut dict = HashMap::new();
        for v in nums {
            let cur = dict.entry(v).or_insert(0);
            *cur += 1;
        }

        let mut min_heap = BinaryHeap::new();
        min_heap.push((Reverse(0), 0));
        for item in dict {
            if min_heap.len() > k {
                min_heap.pop();
            }

            min_heap.push((Reverse(item.1), item.0))
        }
        min_heap.pop();

        min_heap.into_iter().map(|item| item.1).collect()
    }
}

#[test]
fn test() {
    let ret = Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
    println!("【 ret 】==> {:?}", ret);
}
