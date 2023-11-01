impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = vec![-1, -1];
        if nums.is_empty() {
            return result;
        }
        if nums.is_empty() {
            return result;
        }

        let mut l = 0;
        let mut r = nums.len();
        while l < r {
            let mid = l + (r - l) / 2;
            let mid_val = nums[mid];

            if mid_val >= target {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        if r < nums.len() && nums[r] == target {
            result[0] = r as i32;
        }

        let mut l = 0;
        let mut r = nums.len();
        while l < r {
            let mid = l + (r - l) / 2;
            let mid_val = nums[mid];

            if mid_val <= target {
                l = mid + 1;
            } else {
                if mid > 0 {
                    r = mid;
                } else {
                    r = l
                }
            }
        }
        if l > 0 && nums[l - 1] == target {
            result[1] = (l - 1) as i32;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        // let result = Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8);
        // let result = Solution::search_range(vec![1, 3], 2);
        // let result = Solution::search_range(vec![1], 0);
        let result = Solution::search_range(vec![1], 1);
        // let result = Solution::search_range(vec![2, 2], 3);
        println!("【 result 】==> {:?}", result);
    }
}

pub struct Solution;
