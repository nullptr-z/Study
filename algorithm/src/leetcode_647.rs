use std::usize;

impl Solution {
    // dp
    pub fn count_substrings_dp(s: String) -> i32 {
        let ss = s.as_bytes();
        let mut dp = vec![vec![false; s.len()]; s.len()];
        let mut res = 0;

        for i in (0..s.len()).rev() {
            for j in i..s.len() {
                // j-1 <= 1代表相同位置，字符串本身就是一个回文
                // dp[i + 1][j - 1] 保存着上一次的比较结果，是否是回文串；
                // 注意：i+1是因为 i 是反向迭代的
                if ss[i] == ss[j] && (j - i <= 1 || dp[i + 1][j - 1]) {
                    dp[i][j] = true;
                    res += 1;
                }
            }
        }
        res
    }

    // 双指针
    pub fn count_substrings(s: String) -> i32 {
        fn extends(s: &[u8], mut i: usize, mut j: usize) -> i32 {
            let mut count = 0;
            while i < s.len() && j < s.len() && s[i] == s[j] {
                count += 1;
                j += 1;
                i = i.wrapping_sub(1);
            }
            return count;
        }

        let ss = s.as_bytes();
        let mut count = 0;

        for i in 0..s.len() {
            count += extends(ss, i, i);
            count += extends(ss, i, i + 1);
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::count_substrings_dp("aaa".into());
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
