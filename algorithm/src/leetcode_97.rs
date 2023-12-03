impl Solution {
    // [true, false, false, false, false, false]
    // [true, false, false, false, false, false]
    // [true, true, true, true, true, false]
    // [false, true, true, false, true, false]
    // [false, false, true, true, true, true]
    // [false, false, false, true, false, true]

    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }

        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();

        // dp[i][j] i的含义是s1[0..i]，j的含义是s2[0..j]
        // 递归公式 s1[0..i] 和 s2[0..j]能否交错形成s3[0..(i+j)]
        // 每次新加入一个字符，这个字符加上之前的结果看有没有组合的可能
        let mut dp = vec![vec![false; s2.len() + 1]; s1.len() + 1];
        dp[0][0] = true;

        for i in 0..=s1.len() {
            for j in 0..=s2.len() {
                let inx = i + j;
                if i > 0 {
                    dp[i][j] = dp[i - 1][j] && s1[i - 1] == s3[inx - 1];
                }
                if j > 0 && !dp[i][j] {
                    dp[i][j] = dp[i][j - 1] && s2[j - 1] == s3[inx - 1];
                }
            }
        }

        for it in dp.iter() {
            println!("{:?}", it);
        }
        dp[s1.len()][s2.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::is_interleave(
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbcbcac".to_string(),
        );
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
