impl Solution {
    pub fn max_profit_dp(prices: Vec<i32>) -> i32 {
        let mut dp = vec![-prices[0], 0];
        for p in prices {
            dp[0] = dp[0].max(-p);
            dp[1] = dp[1].max(dp[0] + p);
        }
        dp[1]
    }

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = prices[0];
        let mut profit = 0;

        for val in prices.into_iter().skip(1) {
            if min > val {
                min = val;
            }
            profit = profit.max(val - min);
        }

        profit
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::max_profit(vec![7, 6, 4, 3, 1]);
    }
}

pub struct Solution;
