impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut len = nums.len();
        let mut l = 0;
        let mut r = len;
        while l < r {
            let mid = (r + l) >> 1;
            if nums[mid] > nums[len - 1] {
                l = mid + 1
            } else {
                r = mid
            }
        }

        nums[l]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::find_min(vec![3, 4, 5, 1, 2]);
    }
}

pub struct Solution;
