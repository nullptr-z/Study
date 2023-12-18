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

    pub fn is_anagram_map_ascii(s: String, t: String) -> bool {
        let mut record = vec![0; 26];

        let baseChar = 'a';

        for byte in s.bytes() {
            record[byte as usize - baseChar as usize] += 1;
        }
        for byte in t.bytes() {
            record[byte as usize - baseChar as usize] -= 1;
        }

        record.iter().filter(|x| **x != 0).count() == 0
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
