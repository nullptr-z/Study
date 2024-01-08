impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        fn backtracking(s: &[u8], result: &mut Vec<Vec<String>>, temp: &mut Vec<String>) {
            if s.is_empty() {
                // 如果到最后一直都是满足回文条件的，则加入到结果集
                result.push(temp.to_owned());
                return;
            }
            for i in 0..s.len() {
                let sub_str = &s[0..i + 1];
                // 子串是否是回文，不是的话，本层向后探索
                if !is_palindrome(sub_str) {
                    continue;
                }
                // 如果子串是回文，记录这个子串
                temp.push(unsafe { String::from_utf8_unchecked(sub_str.to_vec()) });
                // 开始向下一个位置开始探索，剩余的子串回文情况
                backtracking(&s[i + 1..], result, temp);
                // 探索其他分支
                temp.pop();
            }
        }

        let mut result = vec![];
        let mut temp = vec![];
        backtracking(&s.as_bytes(), &mut result, &mut temp);

        result
    }
}

fn is_palindrome(str: &[u8]) -> bool {
    let (mut l, mut r) = (0, str.len() - 1);
    while l < r {
        if str[l] != str[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::partition("aab".into());
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
