impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut slow = 0;
        for pos in (0..nums.len()) {
            if nums[pos] != val {
                nums[slow] = nums[pos];
                slow += 1;
            }
        }

        slow as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::remove_element(&mut vec![3, 2, 2, 3], 3);

        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
