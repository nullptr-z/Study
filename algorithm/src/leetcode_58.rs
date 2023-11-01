impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut b = s.as_bytes().to_owned();
        b.reverse();
        let mut range = (0, 0);
        for val in b {
            if val > 48 {
                range.0 += 1;
            } else if range.0 > 0 {
                break;
            }
        }

        range.0 - range.1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::length_of_last_word("Hello World ".to_string());
    }
}

pub struct Solution;
