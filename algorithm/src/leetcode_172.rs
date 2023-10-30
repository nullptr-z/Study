impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        if n < 5 {
            return 0;
        }

        let mut count = 0;

        let mut c = n;
        while c > 0 {
            c /= 5;
            count += c;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::trailing_zeroes(13);
    }
}

pub struct Solution;