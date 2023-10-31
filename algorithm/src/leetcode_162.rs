impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l < r {
            let middle = (l + r) / 2;
            if nums[middle] < nums[middle + 1] {
                l = middle + 1;
            } else {
                r = middle
            }
        }

        l as i32
    }

    pub fn find_peak_element_iter(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut idx = 0;
        for (i, val) in nums.into_iter().enumerate() {
            if val > max {
                max = val;
                idx = i;
            }
        }

        idx as i32
    }
}
#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn should_work() {
        let result = Solution::find_peak_element(vec![1]);
        println!("【 result 】==> {:?}", result);
    }
}

pub struct Solution;
