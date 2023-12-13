impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![1; nums.len()];

        for i in 1..nums.len() {
            ans[i] = nums[i - 1] * ans[i - 1];
        }

        let mut temp = *nums.last().take().unwrap();
        for i in (0..nums.len() - 1).rev() {
            ans[i] *= temp;
            temp *= nums[i];
        }

        ans
    }

    pub fn product_except_selfs(nums: Vec<i32>) -> Vec<i32> {
        let end = nums.len();
        let mut left: Vec<i32> = vec![1; end];
        let mut right: Vec<i32> = vec![1; end];

        for i in 0..end - 1 {
            left[i + 1] = nums[i] * left[i];
            right[i + 1] = nums[end - i - 1] * right[i];
        }

        right.reverse();
        for i in 0..end {
            left[i] *= right[i];
        }

        left
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::product_except_selfs(vec![1, 2, 3, 4]);
    }
}

pub struct Solution;
