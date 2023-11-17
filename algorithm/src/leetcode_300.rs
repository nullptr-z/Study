impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut max_len = 0;

        let mut dp: Vec<i32> = vec![0; nums.len()];
        for i in 0..dp.len() {
            dp[i] = 1;
            for j in 1..i {
                if nums[j] < nums[i] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
            max_len = max_len.max(dp[i]);
        }

        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
