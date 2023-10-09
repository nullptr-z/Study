//
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut dp: Vec<i32> = vec![0; 10001];

        let mut start = nums[0];
        let mut end = nums[0];
        for val in nums.iter() {
            dp[*val as usize] += *val;
            start = start.min(*val);
            end = end.max(*val);
        }

        for i in start..=end {
            let idx = i as usize;
            let pre = dp[idx - 1];
            let cur = match idx == 1 {
                true => 0,
                false => dp[idx - 2],
            };
            dp[idx] = pre.max(cur + dp[idx]);
        }

        let max = dp[end as usize];

        max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::delete_and_earn(vec![3, 1]);
    }
}
