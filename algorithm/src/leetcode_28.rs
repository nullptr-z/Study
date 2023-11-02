impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if haystack.len() < needle.len() {
            return -1;
        }

        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();

        let mut start = 0;
        let mut i = 0;
        while i < haystack.len() {
            if start == needle.len() {
                break;
            }
            if haystack[i] == needle[start] {
                i += 1;
                start += 1;
                continue;
            }

            if start > 0 {
                i = i - start + 1;
            } else {
                i += 1;
            }
            start = 0;
        }
        if start <= needle.len() - 1 {
            // 如果==说明start=0
            return -1;
        }

        (i - start) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let r = Solution::str_str("mississippi".to_string(), "a".to_string());
        println!("【 r 】==> {:?}", r);
    }
}

pub struct Solution;
