impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string(); // 处理空输入情况
        }

        let mut prefix = String::new();
        let chars: Vec<Vec<char>> = strs.iter().map(|s| s.chars().collect()).collect();
        let min_len = chars.iter().map(|s| s.len()).min().unwrap(); // 找到最短字符串长度

        for i in 0..min_len {
            let ch = chars[0][i];
            if chars.iter().all(|s| s[i] == ch) {
                prefix.push(ch);
            } else {
                break;
            }
        }

        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::longest_common_prefix(vec!["ab".to_string(), "a".to_string()]);
    }
}

pub struct Solution;
