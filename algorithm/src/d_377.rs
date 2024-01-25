impl Solution {
    pub fn combination_sum4(mut nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;
        let mut dp = vec![0; target + 1];
        dp[0] = 1;
        for cap in 1..dp.len() {
            for &n in nums.iter() {
                let n = n as usize;
                if cap >= n {
                    dp[cap] += dp[cap - n as usize];
                }
            }
        }

        *dp.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::combination_sum4(vec![1, 2, 3], 4);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
