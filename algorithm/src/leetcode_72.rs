impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (w1, w2) = (word1.as_bytes(), word2.as_bytes());
        let (m, n) = (w1.len(), w2.len());
        let mut dp = vec![vec![0; n + 1]; m + 1];
        dp[0] = (0..=n).into_iter().collect();
        for i in 0..=m {
            dp[i][0] = i
        }

        for i in 1..dp.len() {
            for j in 1..dp[1].len() {
                if w1[i - 1] == w2[j - 1] {
                    // 字符相同，什么也不做，保持原样就好
                    dp[i][j] = dp[i - 1][j - 1]
                } else {
                    // 字符不相同，当前一个元素，前一个元素，两者都不是
                    // 增删除改任取其一，操作次数 +1;
                    // 注意，如果w1.len==w2.len，在这里`删`一个，后面必然会有一个`增`
                    dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]) + 1
                }
            }
        }

        dp[m][n] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::min_distance("inte".into(), "xedas".into());
    }
}

pub struct Solution;
