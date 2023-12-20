impl Solution {
    pub fn replace_number(s: String) -> String {
        let mut len = s.len();
        let number = "number".chars().collect::<Vec<char>>();
        let num_len = number.len();
        let mut strb = s.chars().collect::<Vec<char>>();
        let mut i = 0;

        while i < len {
            if strb[i] >= '0' && strb[i] <= '9' {
                let n = strb.len() - i;
                strb.resize(len + num_len - 1, '0');
                len = strb.len();
                for j in 1..n {
                    let t = len - num_len - j + 1;
                    strb[len - j] = strb[t];
                }
                for (j, v) in number.iter().enumerate() {
                    strb[i + j] = *v;
                }
                i += num_len;
            } else {
                i += 1;
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
        let ret = Solution::replace_number("a1b2c3".into());
        println!("【 ret 】==> {:?}", ret);
        let ret = Solution::replace_number("a5b".into());
        println!("【 ret 】==> {:?}", ret);
        let ret = Solution::replace_number("a1b2c3d4f5g6h6".into());
        println!("【 ret 】==> {:?}", ret);
        let ret = Solution::replace_number("123".into());
        println!("【 ret 】==> {:?}", ret);
        let ret = Solution::replace_number("abc".into());
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
