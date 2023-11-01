impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l <= r {
            let middle = (r + l) / 2;
            if nums[middle] == target {
                return middle as i32;
            }

            if nums[l] <= nums[middle] {
                if target >= nums[l] && target < nums[middle] {
                    r = middle - 1;
                } else {
                    l = middle + 1;
                }
            } else {
                if target <= nums[r] && target > nums[middle] {
                    l = middle + 1;
                } else {
                    r = middle - 1;
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        // let result = Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0);
        let result = Solution::search(vec![1, 3], 2);
        println!("【 result 】==> {:?}", result);
    }
}

pub struct Solution;
