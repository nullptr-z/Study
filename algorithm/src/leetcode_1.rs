use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, v) in nums.iter().enumerate() {
            let diff = target - *v;
            if map.get(&diff).is_some() {
                return vec![i as i32, *map.get(&diff).unwrap() as i32];
            }
            map.insert(*v, i);
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::two_sum(vec![3, 2, 4], 6);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
