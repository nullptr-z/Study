impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let ss = s.as_bytes();
        let tt = t.as_bytes();
        let s_len = ss.len() + 1;
        let t_len = tt.len() + 1;
        let mut dp = vec![vec![0; t_len]; s_len];
        dp.iter_mut().for_each(|item| item[0] = 1);

        for i in 1..s_len {
            for j in 1..t_len {
                if ss[i - 1] == tt[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j]
                } else {
                    dp[i][j] = dp[i - 1][j]
                }
                // println!("【 dp 】==>{:?} {} {} ", dp[i], ss[i - 1], tt[j - 1]);
            }
        }

        for d in &dp {
            println!("【 d 】==> {:?}", d);
        }

        dp[s_len - 1][t_len - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::num_distinct("babgbag".to_string(), "bag".to_string());
    }
}

pub struct Solution;
