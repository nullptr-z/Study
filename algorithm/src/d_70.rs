impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let len = n as usize;
        let mut dp = vec![1; len + 2];
        for i in 2..dp.len() {
            dp[i] = dp[i - 1] + dp[i - 2]
        }
        dp[len]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::climb_stairs(3);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
