impl Solution {
    pub fn zz(nums: Vec<i32>) -> Vec<i32> {
        let end = nums.len() - 1;
        let mut left: Vec<i32> = vec![1; nums.len()];
        let mut right: Vec<i32> = vec![1; nums.len()];

        for i in 1..=end {
            left[i] = nums[i] * left[i - 1];
            right[i] = nums[nums.len() - i] * right[i - 1];
        }

        println!("【 left 】==> {:?}", left);
        println!("【 right 】==> {:?}", right);
        right.reverse();
        let mut result = Vec::with_capacity(nums.len());
        for (i, val) in left.iter().enumerate().rev() {
            result.push(right[i] * *val);
        }

        println!("【 result 】==> {:?}", result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::zz(vec![11, 3, 2, 5]);
    }
}

pub struct Solution;
