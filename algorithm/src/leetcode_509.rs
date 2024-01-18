impl Solution {
    pub fn fib(n: i32) -> i32 {
        let len = n as usize;
        let mut dp = vec![0; len + 2]; // 这里多加是为了方便处理，不用单独判断
        dp[1] = 1;
        for i in 2..dp.len() {
            dp[i] = dp[i - 1] + dp[i - 2];
        }

        dp[len]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::fib(4);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
