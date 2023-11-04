impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut xor = 0;
        for val in nums.into_iter() {
            xor ^= val;
        }

        xor
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::single_number(vec![2, 2, 1]);
    }
}

pub struct Solution;
