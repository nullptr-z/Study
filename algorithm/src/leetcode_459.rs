impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        // let ss = format!("{}{}", s, s);
        let ss = s.repeat(2);
        ss[1..ss.len() - 1].contains(&s)
    }

    pub fn repeated_substring_pattern_kmp(s: String) -> bool {
        if s.len() < 2 {
            return false;
        }

        let next = Self::get_next(&s);
        let len = next.len();
        let last = next.last().unwrap();

        *last > 0 && len % (len - last) == 0
    }

    pub fn get_next(s: &str) -> Vec<usize> {
        let len = s.len();
        let chars = s.as_bytes();
        let mut next = vec![0; len];

        let mut k = 0;
        for i in 1..len {
            while k > 0 && chars[i] != chars[k] {
                k = next[k - 1];
            }
            if chars[i] == chars[k] {
                k += 1;
                next[i] = k;
            }
        }

        next
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        // let ret = Solution::repeated_substring_pattern("abaababaab".into());
        // println!("【 ret 】==> {:?}", ret);
        // let ret = Solution::repeated_substring_pattern("abac".into());
        // println!("【 ret 】==> {:?}", ret);
        // let ret = Solution::repeated_substring_pattern("abab".into());
        // println!("【 ret 】==> {:?}", ret);
        let ret = Solution::repeated_substring_pattern("ababab".into());
        println!("【 ret 】==> {:?}", ret);
        let ret = Solution::repeated_substring_pattern("abcabcabcabc".into());
        println!("【 ret 】==> {:?}", ret);
    }

    #[test]
    fn get_next_should_work() {
        let ne = vec![
            "abaababaab",
            "aabaaf",
            "abcdabcdabcdabcd",
            "sad",
            "isisip",
            "isisip",
        ];
        for m in ne {
            let res = Solution::get_next(m);
            println!("【 res 】==> {:?}", res);
        }
    }
}

pub struct Solution;
