impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x_str = x.to_string();
        let x_nums = x_str.as_bytes();
        let len = x_nums.len();
        for i in 0..(len / 2) {
            if x_nums[i] != x_nums[len - i - 1] {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::is_palindrome(121);
    }
}

pub struct Solution;
