impl Solution {
    pub fn is_palindrome(mut s: String) -> bool {
        let s_ascii = unsafe { s.as_bytes_mut() };
        let mut s1 = "".to_string();

        for val in s_ascii.iter() {
            if (*val >= 97 && *val <= 122)
                || (*val >= 65 && *val <= 90)
                || (*val >= 48 && *val <= 57)
            {
                s1.push((*val as char).to_ascii_lowercase())
            }
        }
        let s: String = s1.chars().rev().collect();

        s == s1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let r = Solution::is_palindrome("A man, a plan, a canal: Panama".to_string());
        println!("【 r 】==> {:?}", r);
    }
}

pub struct Solution;
