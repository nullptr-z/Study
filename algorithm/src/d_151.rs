impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut strb = s.chars().collect::<Vec<char>>();

        let mut slow = 0;
        for fast in 0..strb.len() {
            if strb[fast] != ' ' {
                strb[slow] = strb[fast];
                slow += 1;
            } else if slow > 0 && strb[slow - 1] != ' ' {
                strb[slow] = ' ';
                slow += 1;
            }
        }

        if strb[slow - 1] == ' ' {
            // 清除末尾空格
            slow -= 1;
        }
        strb.resize(slow, ' ');
        strb.reverse();

        let mut l = 0;
        let len = strb.len();
        // +1为了让最后一个元素得到翻转
        for r in 0..len + 1 {
            if r == len || strb[r] == ' ' {
                strb[l..r].reverse();
                l = r + 1;
            }
        }

        strb.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::reverse_words("  hello world  ".into());
        println!("【 ret 】==> {:?}", ret);
        let ret = Solution::reverse_words("the sky is blue".into());
        println!("【 ret 】==> {:?}", ret);
        let ret = Solution::reverse_words(" asdasd df f".into());
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
