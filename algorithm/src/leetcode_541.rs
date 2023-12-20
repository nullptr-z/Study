impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let k = k as usize;
        let mut strb = s.as_bytes().to_owned();

        let k2 = 2 * k;
        let mut i = k2;
        loop {
            let mut l = i - k2;
            let mut r = l + k - 1;
            if l >= s.len() {
                break;
            }
            if r >= s.len() {
                r = s.len() - 1;
            }

            while l < r {
                strb[l] ^= strb[r];
                strb[r] ^= strb[l];
                strb[l] ^= strb[r];

                l += 1;
                r -= 1;
            }

            i += k2;
        }

        String::from_utf8(strb).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::reverse_str("abcdefg".into(), 8);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
