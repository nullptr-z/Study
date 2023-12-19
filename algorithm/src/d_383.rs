impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut map = vec![0; 26];
        for v in magazine.bytes() {
            let key = (v - b'a') as usize;
            map[key] += 1;
        }

        for v in ransom_note.bytes() {
            let key = (v - b'a') as usize;
            if map[key] == 0 {
                return false;
            }

            map[key] -= 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::can_construct("a".into(), "ab".into());
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
