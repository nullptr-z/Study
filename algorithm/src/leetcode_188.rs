impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let mut dp = vec![(-prices[0], 0); 1 + k as usize];
        dp[0] = (0, 0);
        for p in prices.iter().skip(1) {
            for j in 1..dp.len() {
                dp[j].0 = dp[j].0.max(dp[j - 1].1 - p);
                dp[j].1 = dp[j].1.max(dp[j].0 + p)
            }
        }

        dp.last().unwrap().1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
