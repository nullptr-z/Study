//

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    /// 动态转移方程：dp[i] = dp[j] && find(s[j..i]);
    /// dp[j]记录字典中是否存在的单词着那个位置
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let word_dict_set: HashSet<String> = word_dict.into_iter().collect();
        let mut dp = vec![false; s.len() + 1];

        dp[0] = true;
        for i in 1..=s.len() {
            for j in 0..i {
                if dp[j] && word_dict_set.contains(&s[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }

        println!("【 dp 】==> {:?}", dp);
        dp[s.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::word_break(
            "leetccode".to_string(),
            vec!["leet".to_string(), "leetc".to_string(), "code".to_string()],
        );
    }
}
