impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut dp = vec![0; nums.len() + 1];
        dp[1] = nums[0];
        for i in 2..dp.len() {
            dp[i] = dp[i - 1].max(nums[i - 1] + dp[i - 2]);
        }

        *dp.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::rob(vec![0, 0, 0]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
