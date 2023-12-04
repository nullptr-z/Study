impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut start_i, mut count, mut ret) = (0, 0, usize::MAX);

        for (end_i, value) in nums.iter().enumerate() {
            count += value;
            while count >= target {
                ret = ret.min(end_i - start_i + 1);
                count -= nums[start_i as usize];
                start_i += 1;
            }
        }

        if ret == usize::MAX {
            ret = 0;
        }

        ret as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
