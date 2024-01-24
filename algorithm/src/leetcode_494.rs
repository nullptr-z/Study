impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum = nums.iter().sum::<i32>();
        let diff = sum - target;
        if sum < target || diff % 2 == 1 {
            return 0;
        }
        let pack = (diff / 2) as usize;
        let mut dp = vec![0; pack + 1];
        dp[0] = 1;
        for i in nums {
            for j in ((i as usize)..dp.len()).rev() {
                dp[j] += dp[j - i as usize];
            }
        }

        dp[pack]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        // let ret = Solution::find_target_sum_ways(vec![1], 2);
        let ret = Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
