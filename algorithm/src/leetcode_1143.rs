impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (s1, s2) = (text1.as_bytes(), text2.as_bytes());
        let (n, m) = (s1.len(), s2.len());
        let mut dp = vec![vec![0; n + 1]; m + 1];

        for i in 1..=m {
            for j in 1..=n {
                if s2[i - 1] == s1[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1; // 对角线
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]); // 左边和上方
                }
            }
        }

        dp[m][n]
    }

    // 打印出来看的更清晰
    pub fn longest_common_subsequencess(text1: String, text2: String) -> i32 {
        let mut count: i32 = 0;
        let ss = text1.as_bytes();
        let mut dp = vec![vec![(11, 11); text1.len()]; text2.len()];

        for (i, &s2) in text2.as_bytes().iter().enumerate() {
            for (j, &s1) in ss.iter().enumerate() {
                if s1 == s2 {
                    count += 1;
                    dp[i][j] = (i, j);
                }
            }
        }
        for d in &dp {
            println!("【 d 】==> {:?}", d);
        }

        let n = text2.len() + 1;
        let m = text1.len() + 1;
        let mut dps = vec![vec![0; m]; n];

        for i in 1..n {
            for j in 1..m {
                let cur = dp[i - 1][j - 1];
                if cur.0 != 11 {
                    dps[i][j] = dps[i - 1][j - 1] + 1;
                } else {
                    // dps[i][j] = dps[i - 1][j - 1]
                    dps[i][j] = dps[i - 1][j].max(dps[i][j - 1])
                }
            }
        }

        for d in &dps {
            println!("【 d 】==> {:?}", d);
        }

        dps[n - 1][m - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        // let ret = Solution::longest_common_subsequencess("bsbininm".into(), "jmjkbkjkv".into());
        // println!("【 ret 】==> {:?} {}", ret, 1);
        // let ret = Solution::longest_common_subsequence("oxcpqrsvwf".into(), "shmtulqrypy".into());
        // println!("【 ret 】==> {:?} {}", ret, 2);
        let ret = Solution::longest_common_subsequence("abcde".into(), "ace".into());
        println!("【 ret 】==> {:?}", ret);
        // let ret = Solution::longest_common_subsequence("ezupkr".into(), "ubmrapg".into());
        // println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
