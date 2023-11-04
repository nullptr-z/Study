use std::collections::VecDeque;

impl Solution {
    pub fn reverse_words(mut s: String) -> String {
        s.push(' ');

        let mut strs = String::new();
        let mut s_queue = VecDeque::new();

        for s in s.as_bytes() {
            if *s != 32 {
                strs.push(*s as char)
            } else if strs.len() > 0 {
                s_queue.push_front(strs);
                strs = String::new();
            }
        }

        let mut ret = String::new();
        let q_len = s_queue.len() - 1;
        for (i, q) in s_queue.into_iter().enumerate() {
            ret += &q;
            if i < q_len {
                ret += " "
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::reverse_words("the sky is blue".to_string());
    }
}

pub struct Solution;
