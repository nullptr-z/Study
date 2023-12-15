impl Solution {
    /// 1. l左指针，r右指针
    /// 2. 如果sum小于target，r+1,一直累加
    /// 3. 一旦sum大于target，l+1,直到小于target回到步骤2
    /// 4. 退出条件为l = r
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut short_len, mut sum) = (usize::MAX, nums[0]);
        let (mut l, mut r) = (0, 1);

        while l < r {
            if sum < target && r <= nums.len() - 1 {
                sum += nums[r];
                r += 1;
                continue;
            }
            if sum >= target {
                short_len = short_len.min(r - l);
                sum -= nums[l];
            }
            l += 1;
        }

        if short_len == usize::MAX {
            return 0;
        }

        short_len as i32
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
