impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let ss = s.as_bytes();
        let mut dp = vec![vec![0; s.len()]; s.len()];

        for i in (0..s.len()).rev() {
            dp[i][i] = 1;
            for j in i + 1..s.len() {
                if ss[i] == ss[j] {
                    dp[i][j] = dp[i + 1][j - 1] + 2;
                    continue;
                }
                dp[i][j] = dp[i + 1][j].max(dp[i][j - 1]);
            }
        }

        dp[0][s.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::longest_palindrome_subseq("bbbab".into());
    }
}
pub struct Solution;
