//

use std::collections::HashMap;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut max = 0;

        for value in nums.iter() {
            if map.get(value).is_some() {
                continue;
            }
            let pre = map.get(&(value - 1)).copied().unwrap_or(0);
            let next = map.get(&(value + 1)).copied().unwrap_or(0);

            let val = pre + next + 1;
            max = max.max(val);

            // - pre和+next一定会找到map元素中存在的
            // pre和next存储的值就是，距离当前值的距离
            map.insert(*value, val);
            map.insert(value - pre, val);
            map.insert(value + next, val);
            // println!("【 value - pre 】==> {:?}", value - pre);
            // println!("【 value + nex 】==> {:?}", value + next);
            // println!("【 map 】==>{} {:?}", value, map);
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        // 最后的2会取到1和4
        // Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]);

        Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]);
    }
}

pub struct Solution;
