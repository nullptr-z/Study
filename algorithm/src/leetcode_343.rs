impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[2] = 1;

        for i in 3..=n {
            for j in 1..i - 1 {
                dp[i] = dp[i].max((i - j) * j).max(dp[i - j] * j);
            }
        }

        dp[n] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::integer_break(10);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
