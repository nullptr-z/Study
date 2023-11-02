impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s = s.as_bytes();
        let mut i = 0;
        if s.len() > 0 {
            for org in t.as_bytes() {
                if *org == s[i] {
                    i += 1;
                }
                if i == s.len() {
                    break;
                }
            }
        } else {
            return true;
        }

        if i < s.len() {
            return false;
        }

        true
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
