impl Solution {
    /// 不完全匹配：之前已经匹配了一些了
    /// 如果遇到不完全匹配，回溯到开始匹配位置的后一位
    pub fn str_str_backtrace(haystack: String, needle: String) -> i32 {
        let needle = needle.as_bytes();
        let haystack = haystack.as_bytes();
        let mut n = 0;

        let mut i = 0;
        while i < haystack.len() {
            let v = haystack[i];
            if v == needle[n] {
                if n == needle.len() - 1 {
                    return ((i) - n) as i32;
                }
                n += 1;
            } else {
                if n > 0 {
                    i -= n;
                }
                n = 0;
            }
            i += 1;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        // let ret = Solution::str_str("sadbutsad".into(), "sad".into());
        // println!("【 ret 】==> {:?}", ret);
        // let ret = Solution::str_str("leetcode".into(), "eet".into());
        // println!("【 ret 】==> {:?}", ret);
        // let ret = Solution::str_str("mississippi".into(), "issip".into());
        // println!("【 ret 】==> {:?}", ret);
        // let ret = Solution::str_str("missississippi".into(), "ississip".into());
        // println!("【 ret 】==> {:?}", ret);
        let ret = Solution::str_str("issississippi".into(), "ississip".into());
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
