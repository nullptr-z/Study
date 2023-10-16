//

pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut len = nums.len();
        let mut i = 0;

        while i < len {
            let end_idx = len - 1;
            if nums[end_idx] == val {
                len -= 1;
                continue;
            }
            if nums[i] == val {
                nums[i] = nums[end_idx];
                len -= 1;
            }
            i += 1;
        }

        len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::remove_element(&mut vec![3, 2, 2, 3], 3);
    }
}
