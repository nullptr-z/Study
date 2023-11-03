impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() < 1 {
            return true;
        }

        let s = s.as_bytes();
        let mut i = 0;
        for org in t.as_bytes() {
            if *org == s[i] {
                i += 1;
            }
            if i == s.len() {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string());
    }
}

pub struct Solution;
