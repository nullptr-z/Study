impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut dp = vec![1; n as usize + 1];
        dp[0] = 0;
        for _i in 0..(m - 1) as usize {
            for j in 1..dp.len() {
                dp[j] += dp[j - 1];
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
        let ret = Solution::unique_paths(3, 2);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
