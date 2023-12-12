use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let tt = t.as_bytes();

        let mut map = HashMap::new();
        let mut map2 = HashMap::new();

        for (index, val) in s.as_bytes().iter().enumerate() {
            let target = map.entry(val).or_insert(tt[index]);
            let target2 = map2.entry(tt[index]).or_insert(val);
            if *target != tt[index] || *target2 != val {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let r = Solution::is_isomorphic("abcd".into(), "abad".into());
        println!("【 r 】==> {:?}", r);
    }
}

pub struct Solution;
