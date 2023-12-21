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

    pub fn get_next(str: &str) -> Vec<usize> {
        // 初始化成kmp长度可以在不匹配的适合不做处理，反之则要push(0)
        let mut next = vec![0; str.len()];
        let chars = str.chars().collect::<Vec<char>>();

        let mut k = 0;
        for i in 1..chars.len() {
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

    pub fn str_str(haystack: String, needle: String) -> i32 {
        let (haystack_len, needle_len) = (haystack.len(), needle.len());
        let next = Self::get_next(&needle);
        let needle = needle.as_bytes();
        let haystack = haystack.as_bytes();

        let mut j = 0;
        for i in 0..haystack_len {
            while j > 0 && haystack[i] != needle[j] {
                j = next[j - 1];
            }
            if haystack[i] == needle[j] {
                j += 1;
                if j == needle_len {
                    return ((i + 1) - needle_len) as i32; // 这个先+1再减对于rust很必要
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn get_next_should_work() {
        let res = Solution::get_next("aabaaf".into());
        println!("【 res 】==> {:?}", res);
    }

    #[test]
    fn should_work() {
        let ret = Solution::str_str("leetcode".into(), "leeto".into());
        println!("【 ret 】==> {:?}", ret);
        let ret = Solution::str_str("sadbutsad".into(), "sad".into());
        println!("【 ret 】==> {:?}", ret);
        let ret = Solution::str_str("mississippi".into(), "issip".into());
        println!("【 ret 】==> {:?}", ret);
        let ret = Solution::str_str("missississippi".into(), "ississip".into());
        println!("【 ret 】==> {:?}", ret);
        let ret = Solution::str_str("mississippi".into(), "issip".into());
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
