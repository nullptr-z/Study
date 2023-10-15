//

pub struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let sb = s.as_bytes();
        let len = s.len();
        let mut dp = vec![0; len + 1];
        dp[0] = 1;

        for i in 1..=len {
            let s_idx = i - 1; // s需要从下标0开始访问
            if sb[s_idx] != b'0' {
                // 只要s当前位置不是0
                dp[i] += dp[s_idx];
            }
            if i > 1
                && sb[s_idx - 1] != b'0'
                && ((sb[s_idx - 1] - b'0') * 10 + (sb[s_idx] - b'0')) <= 26
            {
                dp[i] += dp[s_idx - 1];
            }
        }

        println!("【 dp 】==> {:?}", dp);
        dp[len]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::num_decodings("226".to_string());
    }
}
