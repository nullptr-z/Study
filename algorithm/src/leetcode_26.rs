//

pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut pre: Option<i32> = None;

        let mut f = 0;

        let mut i = 0;
        while i < nums.len() {
            if f > 1 && nums[i] == nums[0] {
                break;
            }
            if pre.is_none() || pre.unwrap() != nums[i] {
                nums[f] = nums[i];
                pre = Some(nums[i]);
                f += 1;
            }
            i += 1;
        }

        f as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::remove_duplicates(&mut vec![-1, 0, 0, 0, 0, 3, 3]);
    }
}
