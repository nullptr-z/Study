use std::cmp::max;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // (买入,已卖出,卖出,冻结)
        let (mut buy, mut sale, mut sell, mut freeze) = (-prices[0], 0, 0, 0);

        for p in prices.iter().skip(1) {
            let temp_freeze = freeze;
            freeze = sell; // 前一天是卖出；今天被冻结的唯一条件
            sell = buy + p; // 前一天是买入；这里不能延续前一天，如果前一天是卖出，今天必然是冻结，更不可能是已卖出

            buy = max(max(buy, sale - p), temp_freeze - p); // 延续前一天原值(已买入含义)，前一天已卖出，前一天冻结；如果前一天正好卖出，因为今天是冻结期
            sale = max(sale, temp_freeze); //  延续前一天原值，或者前一天是冻结期
        }

        max(max(freeze, sale), sell)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::max_profit(vec![1, 2, 3, 0, 2]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
