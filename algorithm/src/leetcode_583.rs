impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let w1 = word1.as_bytes();
        let w2 = word2.as_bytes();
        let mut dp = vec![vec![0; w2.len() + 1]; w1.len() + 1];

        for i in 1..dp.len() {
            for j in 1..dp[i].len() {
                if w1[i - 1] == w2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1; // 对角线
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]); // 左边和上边
                }
            }
        }

        (w1.len() + w2.len()) as i32 - (dp[w1.len()][w2.len()] << 1)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        // let ret = Solution::min_distance("sea".into(), "eat".into());
        // println!("【 ret 】==> {:?}", ret);
        let ret = Solution::min_distance("leetcode".into(), "etco".into());
        println!("【 ret 】==> {:?}", ret);
        // let ret = Solution::min_distance("a".into(), "b".into());
        // println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
