impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in 1..prices.len() {
            let diff = prices[i] - prices[i - 1];
            if diff > 0 {
                sum += diff;
            }
        }

        sum
    }

    pub fn max_profit_dp(prices: Vec<i32>) -> i32 {
        // [买, 不买]
        let mut dp = vec![-prices[0], 0];
        for p in prices {
            dp[0] = dp[0].max(dp[1] - p);
            dp[1] = dp[1].max(dp[0] + p);
        }
        dp[1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::max_profit_dp(vec![7, 1, 5, 3, 6, 4]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
