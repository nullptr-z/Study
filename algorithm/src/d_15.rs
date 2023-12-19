impl Solution {
    /// ps: 哈希法相当于枚举所有组合，去重，测试用例会超时
    /// 排序后枚举i，另外两个数使用双指针进行收缩即可
    /// L= i+1; R= len
    /// 1. 排序 nums.sort
    /// 2. 大循环for i in 0..=len
    /// 3. sum = nums[i]+[l]+[r]
    /// 4. sum > 0, l—1，反之 l+1
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];

        nums.sort();

        for i in 0..nums.len() - 2 {
            // 排序后第一个元素大0，就不可能得到解
            if nums[i] > 0 {
                break;
            }

            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let (mut l, mut r) = (i + 1, nums.len() - 1);

            while l < r {
                let sum = nums[l] + nums[i] + nums[r];
                if sum > 0 {
                    r -= 1;
                } else if sum < 0 {
                    l += 1;
                } else {
                    result.push(vec![nums[l], nums[i], nums[r]]);

                    while l < r && nums[l + 1] == nums[l] {
                        l += 1;
                    }
                    while r > l && nums[r - 1] == nums[r] {
                        r -= 1;
                    }

                    l += 1;
                    r -= 1;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::three_sum(vec![1, -1, -1, 0]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
