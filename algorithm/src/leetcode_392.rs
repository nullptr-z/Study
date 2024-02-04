impl Solution {
    pub fn is_subsequencs(s: String, t: String) -> bool {
        if s.len() < 1 {
            return true;
        }

        let s = s.as_bytes();
        let mut i = 0;
        for org in t.as_bytes() {
            if *org == s[i] {
                i += 1;
            }
            if i == s.len() {
                return true;
            }
        }

        false
    }

    // dp，最长子序列解法
    pub fn is_subsequence(s: String, t: String) -> bool {
        let ss = s.as_bytes();
        let tt = t.as_bytes();
        let mut dp = vec![vec![0; s.len() + 1]; t.len() + 1];
        for i in 1..dp.len() {
            for j in 1..dp[0].len() {
                if tt[i - 1] == ss[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1
                } else {
                    dp[i][j] = dp[i][j - 1].max(dp[i - 1][j])
                }
            }
        }

        *dp.last().unwrap().last().unwrap() == s.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string());
    }
}

pub struct Solution;
