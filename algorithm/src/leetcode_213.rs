impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let l = Solution::robs(&nums[1..]);
        let r = Solution::robs(&nums[..&nums.len() - 1]);
        l.max(r)
    }

    pub fn robs(nums: &[i32]) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut dp = vec![0; nums.len()];
        dp[0] = nums[0];
        dp[1] = nums[0].max(nums[1]);
        for i in 2..dp.len() {
            dp[i] = dp[i - 1].max(nums[i] + dp[i - 2]);
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
