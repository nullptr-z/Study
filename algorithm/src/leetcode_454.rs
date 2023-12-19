use std::collections::HashMap;

impl Solution {
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        let mut count = 0;
        let mut map = HashMap::new();
        for a in nums1 {
            for b in &nums2 {
                let sum = a + b;
                let cur = map.entry(sum).or_insert(0);
                *cur += 1;
            }
        }

        for a in nums3 {
            for b in &nums4 {
                let sum = a + *b;
                let diff = 0 - sum;
                let cur = map.entry(diff).or_insert(i32::MIN);
                if *cur != i32::MIN {
                    count += *cur;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
