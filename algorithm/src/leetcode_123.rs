impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut day1_buy, mut day1_sale, mut day2_buy, mut day2_sale) =
            (-prices[0], 0, -prices[0], 0);

        for p in prices.iter().skip(1) {
            day1_buy = day1_buy.max(-p);
            day1_sale = day1_sale.max(p + day1_buy);
            // day2 其实相当于第二大次卖出，加上第一次卖出的价格
            day2_buy = day2_buy.max(day1_sale - p);
            day2_sale = day2_sale.max(day2_buy + p);
        }
        day2_sale
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::max_profit(vec![3, 2, 6, 5, 0, 3]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
