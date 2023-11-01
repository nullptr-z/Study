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

    pub fn longest_common_prefix_s(strs: Vec<String>) -> String {
        let mut s = "".to_string();

        let chats: Vec<Vec<char>> = strs.into_iter().map(|m| m.chars().collect()).collect();

        let mut i = 0;
        loop {
            let mut flag = true;
            if i >= chats[0].len() {
                break;
            }
            let ch = chats[0][i];
            for item_str in chats.iter() {
                if i >= item_str.len() {
                    flag = false;
                    break;
                } else {
                    flag = flag && item_str[i] == ch;
                }
            }

            if flag {
                s += &ch.to_string();
            } else {
                break;
            }

            i += 1;
        }

        s.to_string()
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
