use std::mem::swap;

impl Solution {
    pub fn rotate_idx(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        let mut result = vec![0; len];

        for idx in 0..len {
            result[(idx + k as usize) % len] = nums[idx];
        }
        swap(nums, &mut result)
    }

    // 实测结果好像比_idx版性能好
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        let k = k as usize % len;
        let mut result = Vec::with_capacity(len);

        nums.reverse();
        for idx in 0..k {
            result.push(nums[idx]);
        }
        nums.reverse();
        result.reverse();
        for idx in 0..(len - k) {
            result.push(nums[idx]);
        }
        swap(nums, &mut result)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::rotate(&mut vec![1, 2, 3, 4, 5, 6, 7], 3);
    }
}

pub struct Solution;
