//

pub struct Solution;

impl Solution {
    pub fn remove_duplicatesed(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();
        let mut count_repeat = 0;
        let mut count_that = 0;
        let mut that = nums[0] + 1;

        let mut i = 0;
        while i < len {
            if nums[i] != that {
                that = nums[i];
                count_that = 1;
            } else if count_that == 2 {
                nums[i] = i32::MAX;
                count_repeat += 1;
            } else {
                count_that += 1;
            }

            i += 1;
        }
        nums.sort();

        (len - count_repeat) as i32
    }

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();
        if len <= 2 {
            return len as i32;
        }
        let mut fast = 2;
        let mut slow = 2;
        while fast < len {
            if nums[slow - 2] != nums[fast] {
                nums[slow] = nums[fast];
                slow += 1;
            }
            fast += 1;
        }

        slow as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::remove_duplicates(&mut vec![1, 1, 1, 2, 2, 3]);
    }
}
// [1, 1, 1, 1, 1, 2, 2, 2, 3]
