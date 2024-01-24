impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum % 2 > 0 {
            return false;
        }
        let n = (sum / 2) as usize;
        let mut dp = vec![0; n + 1];

        for i in nums {
            for cap in (i as usize..=n).rev() {
                let remain = (cap as i32) - i;
                dp[cap] = dp[cap].max(dp[remain as usize] + i);
                if dp[cap] == n as i32 {
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
