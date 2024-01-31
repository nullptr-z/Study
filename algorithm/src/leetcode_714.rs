impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut dp = (-prices[0], 0);
        for p in prices {
            dp.0 = dp.0.max(dp.1 - p);
            dp.1 = dp.1.max(dp.0 + p - fee);
        }

        dp.1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::max_profit(vec![1, 3, 2, 8, 4, 9], 2);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
