impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut b = s.as_bytes().to_owned();
        b.reverse();
        let mut count = 0;

        for val in b {
            if val > 48 {
                count += 1;
            } else if count > 0 {
                break;
            }
        }

        count
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
