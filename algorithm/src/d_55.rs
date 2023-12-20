impl Solution {
    pub fn rotate_left(s: String, i: usize) -> String {
        let mut strb = s.as_bytes().to_owned();
        strb.reverse();
        strb[0..i].reverse();
        strb[i..].reverse();

        String::from_utf8(strb).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::rotate_left("abcdefg".into(), 2);
        let ret = Solution::rotate_left("defgabc".into(), 3);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
