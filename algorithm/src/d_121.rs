impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut dp = 0;
        let mut min = prices[0];

        for cur in 1..prices.len() {
            dp = dp.max(prices[cur] - min);
            min = min.min(prices[cur])
        }

        dp
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::max_profit(vec![7, 1, 5, 3, 6, 4]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
