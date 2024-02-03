impl Solution {
    // 贪心
    pub fn max_sub_arrays(nums: Vec<i32>) -> i32 {
        let mut max_sum = i32::MIN;
        let mut sum = 0;
        for val in nums {
            sum += val;
            max_sum = max_sum.max(sum);
            sum = sum.max(0);
        }
        max_sum
    }

    // DP
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut dp = vec![0; nums.len()];
        dp[0] = nums[0];
        for j in 1..nums.len() {
            let sum = nums[j] + dp[j - 1];
            if dp[j - 1] > 0 {
                dp[j] = sum
            } else {
                dp[j] = nums[j]
            }
            max = max.max(dp[j])
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]);
        println!("【 ret 】==> {:?}", ret);
        let ret = Solution::max_sub_array(vec![-2, -1]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
