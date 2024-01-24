impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum % 2 > 0 {
            return false;
        }
        let pack = (sum / 2) as usize;
        let mut dp = vec![0; pack + 1];

        for i in nums {
            // 滚动数据写法才在 i 位置结束
            for cap in (i as usize..=pack).rev() {
                let remain = (cap as i32) - i;
                dp[cap] = dp[cap].max(dp[remain as usize] + i);
                if dp[cap] == pack as i32 {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::can_partition(vec![2, 2, 1, 1]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
