impl Solution {
    // 二维
    pub fn one_two_backpack(weight: Vec<i32>, price: Vec<i32>, n: i32) -> i32 {
        let mut dp = vec![vec![0; n as usize + 1]; weight.len() + 1];

        for i in 0..weight.len() {
            for cap in 0..=n as usize {
                let remain = (cap as i32) - weight[i];
                if remain >= 0 {
                    dp[i + 1][cap] = dp[i][cap].max(price[i] + dp[i][remain as usize])
                } else {
                    dp[i + 1][cap] = dp[i][cap]
                }
            }
        }

        *dp.last().unwrap().last().unwrap()
    }
    // 一维，滚动数组
    pub fn one_two_backpacks(weight: Vec<i32>, price: Vec<i32>, n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];

        for i in 0..weight.len() {
            for cap in (0..=n as usize).rev() {
                let remain = (cap as i32) - weight[i];
                if remain >= 0 {
                    dp[cap] = dp[cap].max(price[i] + dp[remain as usize])
                }
            }
        }

        *dp.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::one_two_backpacks(vec![2, 3, 4], vec![3, 5, 6], 6);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
