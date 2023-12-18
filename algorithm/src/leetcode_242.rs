use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut s_map = HashMap::new();
        for &v in s.as_bytes() {
            let count = s_map.entry(v).or_insert(0);
            *count += 1;
        }

        for v in t.bytes() {
            let count = s_map.entry(v).or_insert(0);
            if *count == 0 {
                return false;
            }
            *count -= 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::is_anagram("anagram".into(), "nagaram".into());
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
